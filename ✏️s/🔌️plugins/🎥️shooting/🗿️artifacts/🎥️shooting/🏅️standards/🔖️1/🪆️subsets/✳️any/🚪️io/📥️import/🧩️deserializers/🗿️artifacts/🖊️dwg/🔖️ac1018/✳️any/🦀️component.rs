//! shooting <- dwg
use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::{DwgSnapshot, STDIO_DWG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &DwgSnapshot) -> Result<ShootingSnapshot, store::TextError> {
    let _ = STDIO_DWG_DOCUMENT_SCHEMA;
    let bytes = <DwgSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, store::TextError> {
    <ShootingSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| {
        <ShootingSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}
