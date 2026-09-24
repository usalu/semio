//! 📏️ layout ← dxf — a DXF file read by `s.stdio.semio/v1/drawing`'s own import leaf and taken as a trace:
//! every closed axis-aligned rectangle frames a page (the whole canvas when there is none) and the
//! drawing itself becomes the document's background (`layout_document_json_from_drawing`).
//!
//! 🔖 `IoFidelity::Lossy`: frames, stories and styles are not recovered from line work.
use crate::io::layout_document_json_from_drawing;
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{decode_drawing, SemioDrawingFormat};

pub fn register() {}

pub fn deserialize_text(text: &str) -> Result<LayoutSnapshot, store::TextError> {
    let error = |message: String| store::TextError::new(format!("layout←dxf: {message}"), dsl::TextSpan::at(1, 1));
    let drawing = decode_drawing(text.as_bytes(), SemioDrawingFormat::Dxf).map_err(error)?;
    let value = layout_document_json_from_drawing(&drawing, "dxf", "Imported DXF").map_err(error)?;
    <LayoutSnapshot as dsl::FromValue>::from_value(value).map_err(|e| error(e.to_string()))
}
