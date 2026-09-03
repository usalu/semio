//! shooting -> json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — see w5b-close-report.md):
//! `JsonSnapshot.value` was retyped from `serde_json::Value` to stdio's own lexeme-preserving
//! `JsonValue` by a concurrent stdio wave, breaking this pre-existing leaf's compile (surfaced only
//! after the sibling stale-glue.rs `document`→`artifact` mount was fixed). Mirrors 🗒️note's own
//! identical fix (`✏️s/🔌️plugins/🗒️note/…/🔣️json/…/🦀️.rs`): the structural conversion goes
//! through `dsl::os_pack::json::from_dsl_value` (`DslValue` -> first-party `pack::JsonValue`) then
//! stdio's own `From<pack::JsonValue> for JsonValue` bridge, and stdio's own real
//! `write_json_pretty` — no `serde_json` anywhere in this leaf.
use crate::artifacts::shooting::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{write_json_pretty, JsonSnapshot};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

pub async fn register() {}

pub async fn serialize(snapshot: &ShootingSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(snapshot));
    Ok(JsonSnapshot::from_value(value))
}

pub async fn serialize_bytes(snapshot: &ShootingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
