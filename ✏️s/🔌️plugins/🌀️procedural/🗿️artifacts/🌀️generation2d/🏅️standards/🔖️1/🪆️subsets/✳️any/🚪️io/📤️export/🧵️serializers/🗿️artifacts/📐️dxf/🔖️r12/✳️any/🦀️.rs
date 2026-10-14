//! generation2d → dxf — the page drawing (`generation2d_drawing`) written as DXF R12 entities by `s.stdio.semio/v1/drawing`'s own
//! export leaf, the one dxf writer every owner shares.
//!
//! 🔖 `IoFidelity::Lossy`: a page drawing, not the document — there is no dxf import.
use crate::generation2d_drawing;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use {semio_s_artifact_stdio_semio_drawing_export::encode_drawing, semio_s_artifact_stdio_semio_drawing::io::SemioDrawingFormat};

pub fn register() {}

pub fn serialize_bytes(snapshot: &Generation2dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    encode_drawing(&generation2d_drawing(snapshot).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("generation2d→dxf: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?, SemioDrawingFormat::Dxf).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("generation2d→dxf: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}
