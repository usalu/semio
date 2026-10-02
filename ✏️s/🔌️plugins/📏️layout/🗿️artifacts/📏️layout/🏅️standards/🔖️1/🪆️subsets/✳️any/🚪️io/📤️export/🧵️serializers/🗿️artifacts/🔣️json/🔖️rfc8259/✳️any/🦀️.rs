//! 🧾️ Serialize layout through the first-party JSON artifact codec.
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::{parse_json_text, JsonSnapshot};

pub fn register() {}

/// 📤️ The layout document as a `stdio.json` artifact: its RFC 8259 text parsed by the JSON artifact's own parser (the
/// artifact's DSL is the derived `.dsl.semio` grammar, not JSON text).
pub fn serialize(from: &LayoutSnapshot) -> Result<JsonSnapshot, store::PackError> {
    parse_json_text(&serialize_text(from)?).map(JsonSnapshot::from_value).map_err(|error| store::PackError::Schema(error.to_string()))
}

pub fn serialize_text(from: &LayoutSnapshot) -> Result<String, store::PackError> {
    Ok(dsl::os_pack::json::to_json_string(from))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
