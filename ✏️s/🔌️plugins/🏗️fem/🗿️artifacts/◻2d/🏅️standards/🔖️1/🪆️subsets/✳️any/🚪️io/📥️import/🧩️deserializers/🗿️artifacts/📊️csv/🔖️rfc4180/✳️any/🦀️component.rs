//! fem2d <- csv
use crate::artifacts::fem2d::Fem2dSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &CsvSnapshot) -> Result<Fem2dSnapshot, store::TextError> {
    let _ = (STDIO_CSV_DOCUMENT_SCHEMA, from);
    Ok(Fem2dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Fem2dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Fem2dSnapshot::default())
}
