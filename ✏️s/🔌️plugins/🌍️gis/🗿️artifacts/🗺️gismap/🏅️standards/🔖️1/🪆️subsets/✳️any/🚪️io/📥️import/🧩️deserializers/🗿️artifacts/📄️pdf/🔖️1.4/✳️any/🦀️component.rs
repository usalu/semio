//! gismap <- pdf
use crate::artifacts::gismap::GisMapSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &PdfSnapshot) -> Result<GisMapSnapshot, store::TextError> {
    let _ = (STDIO_PDF_DOCUMENT_SCHEMA, from);
    Ok(GisMapSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<GisMapSnapshot, store::TextError> {
    let _ = bytes;
    Ok(GisMapSnapshot::default())
}
