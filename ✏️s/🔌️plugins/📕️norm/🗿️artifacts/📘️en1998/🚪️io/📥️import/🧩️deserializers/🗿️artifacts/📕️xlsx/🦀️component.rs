//! en1998 <- xlsx
use crate::artifacts::en1998::En1998Snapshot;
use semio_s_plugin_stdio::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &XlsxSnapshot) -> Result<En1998Snapshot, store::TextError> {
    let _ = (STDIO_XLSX_DOCUMENT_SCHEMA, from);
    Ok(En1998Snapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<En1998Snapshot, store::TextError> {
    let _ = bytes;
    Ok(En1998Snapshot::default())
}
