//! program -> json
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's csv/tsv scope): `JsonSnapshot.value`
//! was retyped from `serde_json::Value` to stdio's own lexeme-preserving `JsonValue`
//! (`#[serde(tag = "kind")]`, NOT structurally plain JSON by design) by a concurrent stdio wave,
//! breaking this pre-existing placeholder leaf's compile. Fixed as a minimal lagging-call-site
//! update, mirroring the same pattern animate/fem used for the identical gap: a real, honest
//! structural `serde_json::Value -> JsonValue` converter (stdio provides no such bridge) plus
//! stdio's own real `write_json_pretty` text codec for `serialize_bytes`.
use crate::ProgramSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &ProgramSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let projected=crate::standards::v1::subsets::any::io::program_json::convert(dsl::ToValue::to_value(snapshot),false).map_err(|message|store::TextError::new(message,dsl::TextSpan::at(1,1)))?;
    let value=dsl::json::from_dsl_value(&projected);
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
