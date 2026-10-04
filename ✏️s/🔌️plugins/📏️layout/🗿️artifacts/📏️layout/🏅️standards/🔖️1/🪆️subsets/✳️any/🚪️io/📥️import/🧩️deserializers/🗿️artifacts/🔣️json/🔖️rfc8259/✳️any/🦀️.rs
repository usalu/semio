//! 🧾️ Deserialize layout from the first-party JSON artifact codec.
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::{write_json_text, JsonSnapshot};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<LayoutSnapshot, semio_framework_diagnostic::TextError> {
    deserialize_text(&write_json_text(&from.value))
}

pub fn deserialize_text(text: &str) -> Result<LayoutSnapshot, semio_framework_diagnostic::TextError> {
    crate::standards::v1::subsets::any::schema::parse_layout_document(text).map_err(|error| match error{crate::io::LayoutError::Json(cause)=>semio_framework_diagnostic::TextError::from_value_error(cause,semio_framework_diagnostic::TextSpan::at(1,1)),other=>semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,other.to_string(),semio_framework_diagnostic::TextSpan::at(1,1))})
}
