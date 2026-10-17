//! 🧾️ Deserialize layout from RFC 8259 JSON text.
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;

pub fn register() {}

pub fn deserialize_text(text: &str) -> Result<LayoutSnapshot, semio_framework_diagnostic::TextError> {
    semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire2_codec::parse_layout_document(text).map_err(|error| match error{semio_s_artifact_layout_layout_model::LayoutError::Json(cause)=>semio_framework_diagnostic::TextError::from_value_error(cause,semio_framework_diagnostic::TextSpan::at(1,1)),other=>semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,other.to_string(),semio_framework_diagnostic::TextSpan::at(1,1))})
}
