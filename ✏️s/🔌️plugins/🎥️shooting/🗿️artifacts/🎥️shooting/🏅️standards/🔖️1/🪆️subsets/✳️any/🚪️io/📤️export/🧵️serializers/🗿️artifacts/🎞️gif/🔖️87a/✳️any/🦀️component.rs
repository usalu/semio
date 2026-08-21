//! shooting -> gif
use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::gif::{GifSnapshot, STDIO_GIF_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &ShootingSnapshot) -> Result<GifSnapshot, store::TextError> {
    let _ = STDIO_GIF_DOCUMENT_SCHEMA;
    let bytes = <ShootingSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <GifSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub async fn serialize_bytes(snapshot: &ShootingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<GifSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
