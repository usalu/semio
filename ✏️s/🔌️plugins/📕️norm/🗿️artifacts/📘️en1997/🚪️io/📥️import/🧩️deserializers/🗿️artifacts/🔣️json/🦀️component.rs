//! en1997 <- json
use crate::artifacts::en1997::{En1997Snapshot};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<En1997Snapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let mut snap: En1997Snapshot = serde_json::from_value(from.value.clone())
        .map_err(|e| store::TextError::new(format!("en1997<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<En1997Snapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
