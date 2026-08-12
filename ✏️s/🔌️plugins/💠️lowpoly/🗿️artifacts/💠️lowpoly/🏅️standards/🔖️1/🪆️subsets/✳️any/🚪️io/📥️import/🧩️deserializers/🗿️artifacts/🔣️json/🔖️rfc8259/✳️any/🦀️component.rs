//! lowpoly <- json
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, write_json_text};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let mut out: LowpolySnapshot = serde_json::from_value(from.to_serde_value())
        .map_err(|e| store::TextError::new(format!("lowpoly<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = LOWPOLY_DOCUMENT_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
