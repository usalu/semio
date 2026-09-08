//! Serialize layout to stdio.png.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::png::PngSnapshot;

pub fn register() {}

pub fn serialize(from: &LayoutSnapshot) -> Result<PngSnapshot, store::PackError> {
    <PngSnapshot as dsl::FromValue>::from_value(dsl::ToValue::to_value(from)).map_err(|error| store::PackError::Schema(error.to_string()))
}
