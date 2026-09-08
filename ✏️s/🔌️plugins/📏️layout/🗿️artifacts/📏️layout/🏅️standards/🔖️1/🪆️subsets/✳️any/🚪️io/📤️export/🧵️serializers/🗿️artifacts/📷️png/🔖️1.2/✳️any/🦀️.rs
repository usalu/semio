//! Serialize layout to stdio.png.
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_png::PngSnapshot;

pub fn register() {}

pub fn serialize(from: &LayoutSnapshot) -> Result<PngSnapshot, store::PackError> {
    <PngSnapshot as dsl::FromValue>::from_value(dsl::ToValue::to_value(from)).map_err(|error| store::PackError::Schema(error.to_string()))
}
