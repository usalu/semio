//! shooting <- gif
use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::gif::{GifSnapshot, STDIO_GIF_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &GifSnapshot) -> Result<ShootingSnapshot, store::TextError> {
    let _ = STDIO_GIF_DOCUMENT_SCHEMA;
    let bytes = <GifSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, store::TextError> {
    <ShootingSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| <ShootingSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes)))
}
