//! fem2d <- json. `stdio.json`'s real `JsonSnapshot` shape (`value: JsonValue`, a lexeme-
//! preserving custom tree, not `serde_json::Value`) landed after this leaf was first written —
//! lagging call site fixed to match (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-
//! MEDIA-FORMAT-RETIREMENT W5a): `JsonSnapshot::to_serde_value` walks the real `JsonValue` tree
//! back into `serde_json::Value` so `serde_json::from_value` still works; `deserialize_bytes`
//! parses through stdio's own real RFC 8259 text codec (`parse_json_text`), not a re-derived parser.
use crate::artifacts::fem2d::Fem2dSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;

pub async fn register() {}

pub async fn deserialize(from: &JsonSnapshot) -> Result<Fem2dSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let raw = from.to_serde_value();
    let snap: Fem2dSnapshot = serde_json::from_value(raw)
        .map_err(|e| store::TextError::new(format!("fem2d<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(snap)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Fem2dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
