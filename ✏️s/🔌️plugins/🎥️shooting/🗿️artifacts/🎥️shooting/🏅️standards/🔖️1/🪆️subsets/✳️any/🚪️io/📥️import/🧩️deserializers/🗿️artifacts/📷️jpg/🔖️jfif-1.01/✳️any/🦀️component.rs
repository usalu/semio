//! shooting <- jpg
use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::jpg::{JpgSnapshot, STDIO_JPG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JpgSnapshot) -> Result<ShootingSnapshot, store::TextError> {
    let _ = STDIO_JPG_DOCUMENT_SCHEMA;
    let bytes = <JpgSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, store::TextError> {
    <ShootingSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| {
        <ShootingSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}
