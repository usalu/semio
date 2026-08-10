//! din18599 <- json
use crate::artifacts::din18599::{Din18599Snapshot};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<Din18599Snapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let mut snap: Din18599Snapshot = serde_json::from_value(from.value.clone())
        .map_err(|e| store::TextError::new(format!("din18599<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Din18599Snapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
