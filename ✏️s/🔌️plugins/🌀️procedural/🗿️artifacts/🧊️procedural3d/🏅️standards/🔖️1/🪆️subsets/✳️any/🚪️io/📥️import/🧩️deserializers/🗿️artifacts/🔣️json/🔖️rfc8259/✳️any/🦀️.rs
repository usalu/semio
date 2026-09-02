//! procedural3d <- json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — see the paired export
//! leaf's doc comment and w5b-close-report.md): `JsonSnapshot::to_serde_value`/stdio's own real
//! `parse_json_text` do the structural conversion — no hand-rolled bridge needed here.
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, JsonSnapshot};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<Procedural3dSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let snap = <Procedural3dSnapshot as protocol::FromValue>::from_value(protocol::DslValue::from(from.to_serde_value())).map_err(|e| store::TextError::new(format!("procedural3d<-json: {e}"), dsl::TextSpan::at(1, 1)))?;

    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Procedural3dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(value))
}
