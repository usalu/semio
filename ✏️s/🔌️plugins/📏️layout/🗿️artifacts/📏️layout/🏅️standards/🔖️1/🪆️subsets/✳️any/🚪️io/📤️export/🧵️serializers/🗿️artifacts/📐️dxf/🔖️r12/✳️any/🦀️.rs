//! 📏️ layout → dxf — the pages laid side by side as a drawing (`layout_snapshot_to_semio_drawing`: page
//! boundaries, frames with their fills and strokes, and any imported background trace) written as
//! DXF R12 entities by `s.stdio.semio/v1/drawing`'s own export leaf.
//!
//! 🔖 `IoFidelity::Lossy`: a picture of the spreads — story text, styles and links are not drawn.
use crate::layout_snapshot_to_semio_drawing;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use {semio_s_artifact_stdio_semio_drawing_export::encode_drawing, semio_s_artifact_stdio_semio_drawing::io::SemioDrawingFormat};

pub fn register() {}

pub fn serialize_bytes(from: &LayoutSnapshot) -> Result<Vec<u8>, store::PackError> {
    encode_drawing(&layout_snapshot_to_semio_drawing(from), SemioDrawingFormat::Dxf).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("layout→dxf: {error}"))))
}
