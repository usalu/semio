//! 📏️ layout ← dxf — a DXF file read by the codec bound to the background drawing slot and taken as a trace:
//! every closed axis-aligned rectangle frames a page (the whole canvas when there is none) and the
//! drawing itself becomes the document's background (`layout_document_json_from_format`).
//!
//! 🔖 `IoFidelity::Lossy`: frames, stories and styles are not recovered from line work.
use crate::layout_document_json_from_format;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;

pub fn register() {}

pub fn deserialize_text(text: &str) -> Result<LayoutSnapshot, semio_framework_diagnostic::TextError> {
    let error = |message: String| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("layout←dxf: {message}"), semio_framework_diagnostic::TextSpan::at(1, 1));
    let value = layout_document_json_from_format("dxf", text.as_bytes(), "Imported DXF").map_err(error)?;
    <LayoutSnapshot as semio_framework_value::FromValue>::from_value(value).map_err(|cause|semio_framework_diagnostic::TextError::from_value_error(cause,semio_framework_diagnostic::TextSpan::at(1,1)))
}
