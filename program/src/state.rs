use crate::error::OfferError;
use {
    bonfida_utils::BorshSize,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::AccountInfo, program_error::ProgramError, pubkey, pubkey::Pubkey,
    },
};

pub const ROOT_DOMAIN_ACCOUNT: Pubkey = pubkey!("58PwtjSDuFHuUkYjH9BYnnQKHfwo9reZhC2zMJv9JPkx");

pub const FEE: u64 = 5;

#[derive(BorshSerialize, BorshDeserialize, BorshSize, PartialEq)]
#[allow(missing_docs)]
pub enum Tag {
    Uninitialized,
    CentralState,
    ActiveRecord,
    InactiveRecord,
}

#[derive(BorshSerialize, BorshDeserialize, BorshSize)]
#[allow(missing_docs)]
pub struct CentralState {
    pub tag: Tag,
}

impl CentralState {
    pub fn find_key(program_id: &Pubkey) -> (Pubkey, u8) {
        let seeds: &[&[u8]] = &[&program_id.to_bytes()];
        Pubkey::find_program_address(seeds, program_id)
    }

    pub fn save(&self, mut dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap()
    }
}

#[derive(BorshSerialize, BorshDeserialize, BorshSize)]
#[allow(missing_docs)]
pub struct TokenRecord {
    /// Tag
    pub tag: Tag,

    /// Nonce
    pub nonce: u8,

    /// Name account of the offer
    pub name_account: Pubkey,

    /// Offer owner
    pub owner: Pubkey,

    /// NFT mint
    pub nft_mint: Pubkey,
}

#[allow(missing_docs)]
impl TokenRecord {
    pub const SEED: &'static [u8; 12] = b"token_record";

    pub fn new(nonce: u8, owner: Pubkey, name_account: Pubkey, nft_mint: Pubkey) -> Self {
        Self {
            tag: Tag::ActiveRecord,
            nonce,
            owner,
            name_account,
            nft_mint,
        }
    }

    pub fn find_key(
        owner: &Pubkey,
        name_account: &Pubkey,
        quote_mint: &Pubkey,
        program_id: &Pubkey,
    ) -> (Pubkey, u8) {
        let seeds: &[&[u8]] = &[TokenRecord::SEED, &name_account.to_bytes()];
        Pubkey::find_program_address(seeds, program_id)
    }

    pub fn save(&self, mut dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap()
    }

    pub fn from_account_info(a: &AccountInfo, tag: Tag) -> Result<TokenRecord, ProgramError> {
        let mut data = &a.data.borrow() as &[u8];
        if data[0] != tag as u8 && data[0] != Tag::Uninitialized as u8 {
            return Err(OfferError::DataTypeMismatch.into());
        }
        let result = TokenRecord::deserialize(&mut data)?;
        Ok(result)
    }
}
