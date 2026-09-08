//! 🧾️ Serialize layout through the first-party JSON artifact codec.
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::JsonSnapshot;

pub fn register() {}

pub fn serialize(from: &LayoutSnapshot) -> Result<JsonSnapshot, store::PackError> {
    <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&serialize_text(from)?).map_err(|error| store::PackError::Schema(error.to_string()))
}

pub fn serialize_text(from: &LayoutSnapshot) -> Result<String, store::PackError> {
    Ok(dsl::os_pack::json::to_json_string(from))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
