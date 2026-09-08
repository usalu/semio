//! 🧾️ Deserialize layout from the first-party JSON artifact codec.
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::{write_json_text, JsonSnapshot};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<LayoutSnapshot, store::TextError> {
    deserialize_text(&write_json_text(&from.value))
}

pub fn deserialize_text(text: &str) -> Result<LayoutSnapshot, store::TextError> {
    crate::schema::parse_layout_document(text).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1)))
}
