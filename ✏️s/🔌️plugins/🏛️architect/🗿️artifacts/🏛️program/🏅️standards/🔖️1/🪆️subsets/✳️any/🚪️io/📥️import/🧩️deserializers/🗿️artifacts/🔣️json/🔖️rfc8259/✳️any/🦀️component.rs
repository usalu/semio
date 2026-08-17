//! program <- json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired export leaf's doc comment (same wave,
//! `JsonSnapshot.value: serde_json::Value` -> stdio's own `JsonValue`). Mirrors it with the
//! reverse structural converter and stdio's own real `parse_json_text` for `deserialize_bytes`.
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::ARCHITECT_PROGRAM_SCHEMA;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<ProgramSnapshot, store::TextError> {
    let _ = ARCHITECT_PROGRAM_SCHEMA;
    let mut out: ProgramSnapshot = serde_json::from_value(from.to_serde_value()).map_err(|e| store::TextError::new(format!("program<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = ARCHITECT_PROGRAM_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ProgramSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
