//! vdi3805 <- xlsx
use crate::artifacts::vdi3805::Vdi3805Snapshot;
use semio_s_plugin_stdio::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &XlsxSnapshot) -> Result<Vdi3805Snapshot, store::TextError> {
    let _ = (STDIO_XLSX_DOCUMENT_SCHEMA, from);
    Ok(Vdi3805Snapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Vdi3805Snapshot, store::TextError> {
    let _ = bytes;
    Ok(Vdi3805Snapshot::default())
}
