//! din18599 -> json
use crate::artifacts::din18599::Din18599Snapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Din18599Snapshot) -> Result<JsonSnapshot, store::TextError> {
    Ok(JsonSnapshot {
        schema: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        value: serde_json::to_value(snapshot)
            .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?,
    })
}

pub fn serialize_bytes(snapshot: &Din18599Snapshot) -> Result<Vec<u8>, store::TextError> {
    serde_json::to_vec_pretty(&serialize(snapshot)?.value)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}
