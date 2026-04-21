use crate::error::Result;
use crate::kit::{KitFullDto, KitStore, KitStoreRef};

impl KitStore {
    /// Parse a kit from a JSON string into a fully hydrated graph.
    pub fn from_json_str(s: &str) -> Result<KitStoreRef> {
        let dto: KitFullDto = serde_json::from_str(s)?;
        Ok(KitStore::from_full_dto(dto))
    }

    pub fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.to_full_dto())?)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self.to_full_dto())?)
    }
}
