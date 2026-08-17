//! gisterrain <- json
use crate::artifacts::gisterrain::{GisTerrainSnapshot};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let snap: GisTerrainSnapshot = serde_json::from_value(from.to_serde_value())
        .map_err(|e| store::TextError::new(format!("gisterrain<-json: {e}"), dsl::TextSpan::at(1, 1)))?;

    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GisTerrainSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let raw: serde_json::Value = serde_json::from_str(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(raw))
}
