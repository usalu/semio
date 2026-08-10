//! en1995 <- xlsx
use crate::artifacts::en1995::En1995Snapshot;
use semio_s_plugin_stdio::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &XlsxSnapshot) -> Result<En1995Snapshot, store::TextError> {
    let _ = (STDIO_XLSX_DOCUMENT_SCHEMA, from);
    Ok(En1995Snapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<En1995Snapshot, store::TextError> {
    let _ = bytes;
    Ok(En1995Snapshot::default())
}
