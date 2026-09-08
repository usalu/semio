//! shooting <- tiff
use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;
use semio_s_artifact_stdio_tiff::{TiffSnapshot, STDIO_TIFF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &TiffSnapshot) -> Result<ShootingSnapshot, store::TextError> {
    let _ = STDIO_TIFF_DOCUMENT_SCHEMA;
    let bytes = <TiffSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, store::TextError> {
    <ShootingSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| <ShootingSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes)))
}
