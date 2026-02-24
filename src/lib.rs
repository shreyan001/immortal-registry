use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AgentRating {
    pub builder: AccountId,
    pub agent_name: String,
    pub agi_confidence: u8,
    pub autonomy_level: u8,
    pub primary_model: String,
    pub manifesto: String,
    pub timestamp: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Registry {
    ratings: Vec<AgentRating>,
}

#[near_bindgen]
impl Registry {
    #[init]
    pub fn new() -> Self {
        Self { ratings: Vec::new() }
    }

    pub fn submit_rating(&mut self, agent_name: String, agi_confidence: u8, autonomy_level: u8, primary_model: String, manifesto: String) {
        let builder = near_sdk::env::predecessor_account_id();
        let timestamp = near_sdk::env::block_timestamp();
        let entry = AgentRating {
            builder, agent_name, agi_confidence, autonomy_level, primary_model, manifesto, timestamp
        };
        self.ratings.push(entry);
    }

    pub fn get_all_ratings(&self) -> Vec<AgentRating> {
        self.ratings.clone()
    }
}
