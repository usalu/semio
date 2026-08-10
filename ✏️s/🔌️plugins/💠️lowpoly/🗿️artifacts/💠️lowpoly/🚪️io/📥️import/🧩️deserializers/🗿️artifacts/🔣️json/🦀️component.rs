//! lowpoly <- json
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let _ = LOWPOLY_DOCUMENT_SCHEMA;
    let mut out: LowpolySnapshot = serde_json::from_value(from.value.clone())
        .map_err(|e| store::TextError::new(format!("lowpoly<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = LOWPOLY_DOCUMENT_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
