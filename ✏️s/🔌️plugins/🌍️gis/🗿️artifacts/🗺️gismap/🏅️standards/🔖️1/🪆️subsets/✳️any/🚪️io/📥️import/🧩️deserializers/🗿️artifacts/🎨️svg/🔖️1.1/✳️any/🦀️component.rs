//! gismap <- svg
use crate::artifacts::gismap::GisMapSnapshot;
use semio_s_plugin_stdio::artifacts::svg::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &SvgSnapshot) -> Result<GisMapSnapshot, store::TextError> {
    let _ = (STDIO_SVG_DOCUMENT_SCHEMA, from);
    Ok(GisMapSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GisMapSnapshot, store::TextError> {
    let _ = bytes;
    Ok(GisMapSnapshot::default())
}
