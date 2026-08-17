//! fem3d -> json. `stdio.json`'s real `JsonSnapshot` shape (`value: JsonValue`, a lexeme-
//! preserving custom tree, not `serde_json::Value`) landed after this leaf was first written —
//! lagging call site fixed to match (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-
//! MEDIA-FORMAT-RETIREMENT W5a): `serde_json::to_value(snapshot)` still produces the real
//! structured JSON tree (every `Fem3dSnapshot` field, not a single blob like the csv/md leaves),
//! walked into the target `JsonValue` shape by `JsonSnapshot::from_value`; `serialize_bytes`
//! writes it through stdio's own real RFC 8259 text codec (`write_json_text`), not a re-derived encoder.
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_text;

pub fn register() {}

pub fn serialize(snapshot: &Fem3dSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(raw))
}

pub fn serialize_bytes(snapshot: &Fem3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_text(&serialize(snapshot)?.value).into_bytes())
}
