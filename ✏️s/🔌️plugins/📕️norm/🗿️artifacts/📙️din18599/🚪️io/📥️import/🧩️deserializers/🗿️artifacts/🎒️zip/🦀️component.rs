//! din18599 <- zip
use crate::artifacts::din18599::Din18599Snapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &ZipSnapshot) -> Result<Din18599Snapshot, store::TextError> {
    let _ = (STDIO_ZIP_DOCUMENT_SCHEMA, from);
    Ok(Din18599Snapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Din18599Snapshot, store::TextError> {
    let _ = bytes;
    Ok(Din18599Snapshot::default())
}
