//! 🧾️ Serialize layout as RFC 8259 JSON text.
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;

pub fn register() {}

pub fn serialize_text(from: &LayoutSnapshot) -> Result<String, store::PackError> {
    Ok(semio_framework_pack_json::to_json_string(from))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
