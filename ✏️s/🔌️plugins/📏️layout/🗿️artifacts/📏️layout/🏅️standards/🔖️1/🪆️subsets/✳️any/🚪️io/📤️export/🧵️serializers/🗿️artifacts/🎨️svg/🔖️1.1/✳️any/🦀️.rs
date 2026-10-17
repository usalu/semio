//! 📏️ layout → svg — the pages laid side by side as a drawing (`layout_snapshot_to_scene`: page
//! boundaries, frames with their fills and strokes, and any imported background trace) written as
//! SVG 1.1 by the codec bound to the background drawing slot.
//!
//! 🔖 `IoFidelity::Lossy`: a picture of the spreads — story text, styles and links are not drawn.
use crate::layout_snapshot_to_format;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &LayoutSnapshot) -> Result<Vec<u8>, store::PackError> {
    layout_snapshot_to_format(from, "svg").map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("layout→svg: {error}"))))
}

pub fn serialize_text(from: &LayoutSnapshot) -> Result<String, store::PackError> {
    String::from_utf8(serialize_bytes(from)?).map_err(|error| store::PackError::from(semio_framework_value::ValueError::from(error)))
}
