//! shooting <- json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — see w5b-close-report.md):
//! see the paired export leaf's doc comment. Mirrors it going through stdio's own
//! `JsonSnapshot::to_serde_value` bridge and stdio's own real `parse_json_text`.
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{parse_json_text, JsonSnapshot};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<ShootingSnapshot, store::TextError> {
    let _ = SHOOTING_DOCUMENT_SCHEMA;
    let mut out: ShootingSnapshot = serde_json::from_value(from.to_serde_value())
        .map_err(|e| store::TextError::new(format!("shooting<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = SHOOTING_DOCUMENT_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(value))
}
