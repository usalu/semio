//! generation2d -> json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — the deletion task itself
//! never touched this file; see w5b-close-report.md): `JsonSnapshot::from_value`/stdio's own real
//! `write_json_pretty` do the structural conversion — no hand-rolled bridge needed here.
use crate::Generation2dSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::{JsonSnapshot};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::{write_json_pretty};
use semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn serialize(snapshot: &Generation2dSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(snapshot));
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &Generation2dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
