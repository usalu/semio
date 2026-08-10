//! present <- json
use crate::artifacts::present::PresentSnapshot;
use crate::artifacts::present::PRESENT_DOCUMENT_SCHEMA;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<PresentSnapshot, store::TextError> {
    let _ = PRESENT_DOCUMENT_SCHEMA;
    let mut out: PresentSnapshot = serde_json::from_value(from.value.clone())
        .map_err(|e| store::TextError::new(format!("present<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = PRESENT_DOCUMENT_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<PresentSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
