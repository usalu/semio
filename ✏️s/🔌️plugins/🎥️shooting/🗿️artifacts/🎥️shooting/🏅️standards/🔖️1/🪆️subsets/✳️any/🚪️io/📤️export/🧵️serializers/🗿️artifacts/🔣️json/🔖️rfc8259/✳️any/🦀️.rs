//! shooting -> json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — see w5b-close-report.md):
//! `JsonSnapshot.value` was retyped from `serde_json::Value` to stdio's own lexeme-preserving
//! `JsonValue` by a concurrent stdio wave, breaking this pre-existing leaf's compile (surfaced only
//! after the sibling stale-glue.rs `document`→`artifact` mount was fixed). Mirrors 🗒️note's own
//! identical fix (`✏️s/🔌️plugins/🗒️note/…/🔣️json/…/🦀️.rs`): a real structural
//! `serde_json::Value -> JsonValue` conversion goes through stdio's own
//! `From<serde_json::Value> for JsonValue` bridge, and stdio's own real `write_json_pretty`.
use crate::artifacts::shooting::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{write_json_pretty, JsonSnapshot};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

pub async fn register() {}

pub async fn serialize(snapshot: &ShootingSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}

pub async fn serialize_bytes(snapshot: &ShootingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
