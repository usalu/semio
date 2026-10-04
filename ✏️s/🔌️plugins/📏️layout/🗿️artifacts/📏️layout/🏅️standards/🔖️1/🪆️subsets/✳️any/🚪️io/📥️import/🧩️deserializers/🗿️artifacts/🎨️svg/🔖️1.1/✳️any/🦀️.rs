//! 📏️ layout ← svg — a SVG file read by `s.stdio.semio/v1/drawing`'s own import leaf and taken as a trace:
//! every closed axis-aligned rectangle frames a page (the whole canvas when there is none) and the
//! drawing itself becomes the document's background (`layout_document_json_from_drawing`).
//!
//! 🔖 `IoFidelity::Lossy`: frames, stories and styles are not recovered from line work.
use crate::io::layout_document_json_from_drawing;
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{decode_drawing, SemioDrawingFormat};

pub fn register() {}

pub fn deserialize_text(text: &str) -> Result<LayoutSnapshot, semio_framework_diagnostic::TextError> {
    let error = |message: String| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("layout←svg: {message}"), semio_framework_diagnostic::TextSpan::at(1, 1));
    let drawing = decode_drawing(text.as_bytes(), SemioDrawingFormat::Svg).map_err(error)?;
    let value = layout_document_json_from_drawing(&drawing, "svg", "Imported SVG").map_err(error)?;
    <LayoutSnapshot as semio_framework_value::FromValue>::from_value(value).map_err(|cause|semio_framework_diagnostic::TextError::from_value_error(cause,semio_framework_diagnostic::TextSpan::at(1,1)))
}
