//! shooting <- tiff
use crate::artifacts::shooting::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::tiff::{TiffSnapshot, STDIO_TIFF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &TiffSnapshot) -> Result<ShootingSnapshot, store::TextError> {
    let _ = STDIO_TIFF_DOCUMENT_SCHEMA;
    let bytes = semio_s_plugin_stdio::artifacts::tiff::engine::encode_tiff(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <ShootingSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <ShootingSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, store::TextError> {
    deserialize(&semio_s_plugin_stdio::artifacts::tiff::engine::decode_tiff(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}
