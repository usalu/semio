//! en1993 <- zip
use crate::artifacts::en1993::En1993Snapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &ZipSnapshot) -> Result<En1993Snapshot, store::TextError> {
    let _ = (STDIO_ZIP_DOCUMENT_SCHEMA, from);
    Ok(En1993Snapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<En1993Snapshot, store::TextError> {
    let _ = bytes;
    Ok(En1993Snapshot::default())
}
