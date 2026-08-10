//! shooting -> bmp
use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::bmp::{BmpSnapshot, STDIO_BMP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &ShootingSnapshot) -> Result<BmpSnapshot, store::TextError> {
    let _ = STDIO_BMP_DOCUMENT_SCHEMA;
    let bytes = <ShootingSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <BmpSnapshot as store::ArtifactPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &ShootingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<BmpSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
