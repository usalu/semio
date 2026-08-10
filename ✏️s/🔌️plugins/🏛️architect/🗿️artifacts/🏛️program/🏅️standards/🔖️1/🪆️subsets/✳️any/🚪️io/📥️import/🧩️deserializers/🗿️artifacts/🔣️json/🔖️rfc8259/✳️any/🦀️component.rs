//! program <- json
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::ARCHITECT_PROGRAM_SCHEMA;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<ProgramSnapshot, store::TextError> {
    let _ = ARCHITECT_PROGRAM_SCHEMA;
    let mut out: ProgramSnapshot = serde_json::from_value(from.value.clone())
        .map_err(|e| store::TextError::new(format!("program<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = ARCHITECT_PROGRAM_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ProgramSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
