//! fem3d <- csv
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &CsvSnapshot) -> Result<Fem3dSnapshot, store::TextError> {
    let _ = (STDIO_CSV_DOCUMENT_SCHEMA, from);
    Ok(Fem3dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Fem3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Fem3dSnapshot::default())
}
