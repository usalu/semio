use crate::error::Result;
use crate::kit::{Kit, KitDto, KitRef};

impl Kit {
    /// Parse a kit from a JSON string into a fully hydrated graph.
    pub fn from_json_str(s: &str) -> Result<KitRef> {
        let dto: KitDto = serde_json::from_str(s)?;
        Ok(Kit::from_dto(dto))
    }

    /// Serialize this kit to pretty JSON.
    pub fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.to_dto())?)
    }

    /// Serialize this kit to compact JSON.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self.to_dto())?)
    }
}
