//! Serialize layout to stdio.png.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(from: &LayoutSnapshot) -> Result<PngSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    serde_json::from_value(value).map_err(|e| store::PackError::Schema(e.to_string()))
}
