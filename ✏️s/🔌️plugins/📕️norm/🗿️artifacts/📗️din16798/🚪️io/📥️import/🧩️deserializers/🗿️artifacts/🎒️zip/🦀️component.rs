//! din16798 <- zip
use crate::artifacts::din16798::Din16798Snapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &ZipSnapshot) -> Result<Din16798Snapshot, store::TextError> {
    let _ = (STDIO_ZIP_DOCUMENT_SCHEMA, from);
    Ok(Din16798Snapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Din16798Snapshot, store::TextError> {
    let _ = bytes;
    Ok(Din16798Snapshot::default())
}
