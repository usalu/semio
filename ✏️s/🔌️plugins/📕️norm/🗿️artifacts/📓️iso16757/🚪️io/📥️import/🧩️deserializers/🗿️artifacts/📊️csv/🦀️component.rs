//! iso16757 <- csv
use crate::artifacts::iso16757::Iso16757Snapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &CsvSnapshot) -> Result<Iso16757Snapshot, store::TextError> {
    let _ = (STDIO_CSV_DOCUMENT_SCHEMA, from);
    Ok(Iso16757Snapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Iso16757Snapshot, store::TextError> {
    let _ = bytes;
    Ok(Iso16757Snapshot::default())
}
