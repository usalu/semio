//! en1998 <- zip
use crate::artifacts::en1998::En1998Snapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &ZipSnapshot) -> Result<En1998Snapshot, store::TextError> {
    let _ = (STDIO_ZIP_DOCUMENT_SCHEMA, from);
    Ok(En1998Snapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<En1998Snapshot, store::TextError> {
    let _ = bytes;
    Ok(En1998Snapshot::default())
}
