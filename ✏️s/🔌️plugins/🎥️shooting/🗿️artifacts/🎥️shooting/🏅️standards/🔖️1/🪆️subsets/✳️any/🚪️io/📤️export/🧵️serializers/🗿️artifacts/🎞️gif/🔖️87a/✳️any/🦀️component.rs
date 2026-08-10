//! shooting -> gif
use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::gif::{GifSnapshot, STDIO_GIF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &ShootingSnapshot) -> Result<GifSnapshot, store::TextError> {
    let _ = STDIO_GIF_DOCUMENT_SCHEMA;
    let bytes = <ShootingSnapshot as store::DocumentPack>::encode_pack(snapshot);
    <GifSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &ShootingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<GifSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
