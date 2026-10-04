//! generation2d → svg — the page drawing (`generation2d_drawing`) written as SVG 1.1 by `s.stdio.semio/v1/drawing`'s own
//! export leaf, the one svg writer every owner shares.
//!
//! 🔖 `IoFidelity::Lossy`: a page drawing, not the document — there is no svg import.
use crate::standards::v1::subsets::any::io::generation2d_drawing;
use crate::Generation2dSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{encode_drawing, SemioDrawingFormat};

pub fn register() {}

pub fn serialize_bytes(snapshot: &Generation2dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    encode_drawing(&generation2d_drawing(snapshot).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("generation2d→svg: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?, SemioDrawingFormat::Svg).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("generation2d→svg: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}
