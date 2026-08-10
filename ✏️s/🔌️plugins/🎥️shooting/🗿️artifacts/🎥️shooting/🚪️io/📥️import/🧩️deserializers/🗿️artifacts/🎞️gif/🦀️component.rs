//! shooting <- gif
use crate::artifacts::shooting::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::gif::{GifSnapshot, STDIO_GIF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &GifSnapshot) -> Result<ShootingSnapshot, store::TextError> {
    let _ = STDIO_GIF_DOCUMENT_SCHEMA;
    let bytes = semio_s_plugin_stdio::artifacts::gif::engine::encode_gif(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <ShootingSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <ShootingSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, store::TextError> {
    deserialize(&semio_s_plugin_stdio::artifacts::gif::engine::decode_gif(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}
