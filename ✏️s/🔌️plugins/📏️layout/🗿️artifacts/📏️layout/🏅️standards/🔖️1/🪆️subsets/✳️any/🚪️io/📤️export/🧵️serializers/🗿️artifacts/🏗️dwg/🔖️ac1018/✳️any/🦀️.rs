//! 📏️ layout → dwg — the pages laid side by side as a drawing (`layout_snapshot_to_semio_drawing`: page
//! boundaries, frames with their fills and strokes, and any imported background trace) written as
//! DWG entities by `s.stdio.semio/v1/drawing`'s own export leaf.
//!
//! 🔖 `IoFidelity::Lossy`: a picture of the spreads — story text, styles and links are not drawn.
use crate::io::layout_snapshot_to_semio_drawing;
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{encode_drawing, SemioDrawingFormat};

pub fn register() {}

pub fn serialize_bytes(from: &LayoutSnapshot) -> Result<Vec<u8>, store::PackError> {
    encode_drawing(&layout_snapshot_to_semio_drawing(from), SemioDrawingFormat::Dwg).map_err(|error| store::PackError::Schema(format!("layout→dwg: {error}")))
}
