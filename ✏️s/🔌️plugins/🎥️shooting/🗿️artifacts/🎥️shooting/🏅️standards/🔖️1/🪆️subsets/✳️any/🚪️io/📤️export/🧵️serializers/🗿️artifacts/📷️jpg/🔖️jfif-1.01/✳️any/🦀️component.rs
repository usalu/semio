//! shooting -> jpg
use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::jpg::{JpgSnapshot, STDIO_JPG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &ShootingSnapshot) -> Result<JpgSnapshot, store::TextError> {
    let _ = STDIO_JPG_DOCUMENT_SCHEMA;
    let bytes = <ShootingSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <JpgSnapshot as store::ArtifactPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &ShootingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<JpgSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
