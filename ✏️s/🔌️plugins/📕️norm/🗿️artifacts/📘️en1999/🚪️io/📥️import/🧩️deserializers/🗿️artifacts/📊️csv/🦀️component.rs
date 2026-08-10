//! en1999 <- csv
use crate::artifacts::en1999::En1999Snapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &CsvSnapshot) -> Result<En1999Snapshot, store::TextError> {
    let _ = (STDIO_CSV_DOCUMENT_SCHEMA, from);
    Ok(En1999Snapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<En1999Snapshot, store::TextError> {
    let _ = bytes;
    Ok(En1999Snapshot::default())
}
