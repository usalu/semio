//! en1996 <- zip
use crate::artifacts::en1996::En1996Snapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &ZipSnapshot) -> Result<En1996Snapshot, store::TextError> {
    let _ = (STDIO_ZIP_DOCUMENT_SCHEMA, from);
    Ok(En1996Snapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<En1996Snapshot, store::TextError> {
    let _ = bytes;
    Ok(En1996Snapshot::default())
}
