//! Deserialize layout via stdio.png.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_artifact_stdio_png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PngSnapshot) -> Result<LayoutSnapshot, store::TextError> {
    let _ = STDIO_PNG_DOCUMENT_SCHEMA;
    <LayoutSnapshot as dsl::FromValue>::from_value(dsl::ToValue::to_value(from)).map_err(|error| store::TextError::new(format!("layout<-png: {error}"), dsl::TextSpan::at(1, 1)))
}
