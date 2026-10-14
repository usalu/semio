//! 🧾️ Deserialize layout from the first-party JSON artifact codec.
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::{JsonSnapshot};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::{write_json_text};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<LayoutSnapshot, semio_framework_diagnostic::TextError> {
    deserialize_text(&write_json_text(&from.value))
}

pub fn deserialize_text(text: &str) -> Result<LayoutSnapshot, semio_framework_diagnostic::TextError> {
    semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire2_codec::parse_layout_document(text).map_err(|error| match error{semio_s_artifact_layout_layout_model::LayoutError::Json(cause)=>semio_framework_diagnostic::TextError::from_value_error(cause,semio_framework_diagnostic::TextSpan::at(1,1)),other=>semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,other.to_string(),semio_framework_diagnostic::TextSpan::at(1,1))})
}
