//! din4108 <- xlsx
use crate::artifacts::din4108::Din4108Snapshot;
use semio_s_plugin_stdio::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &XlsxSnapshot) -> Result<Din4108Snapshot, store::TextError> {
    let _ = (STDIO_XLSX_DOCUMENT_SCHEMA, from);
    Ok(Din4108Snapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Din4108Snapshot, store::TextError> {
    let _ = bytes;
    Ok(Din4108Snapshot::default())
}
