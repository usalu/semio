//! generation2d <- json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — see the paired export
//! leaf's doc comment and w5b-close-report.md): `JsonSnapshot::to_serde_value`/stdio's own real
//! `parse_json_text` do the structural conversion — no hand-rolled bridge needed here.
use crate::Generation2dSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::{parse_json_text, JsonSnapshot};
use semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<Generation2dSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let snap = <Generation2dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(from.to_serde_value())).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("generation2d<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;

    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation2dSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
