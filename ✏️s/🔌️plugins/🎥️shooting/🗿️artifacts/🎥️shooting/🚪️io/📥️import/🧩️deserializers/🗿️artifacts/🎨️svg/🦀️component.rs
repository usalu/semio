//! shooting <- svg
use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::svg::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &SvgSnapshot) -> Result<ShootingSnapshot, store::TextError> {
    let _ = STDIO_SVG_DOCUMENT_SCHEMA;
    let bytes = <SvgSnapshot as store::DocumentPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, store::TextError> {
    <ShootingSnapshot as store::DocumentPack>::decode_pack(bytes).or_else(|_| {
        <ShootingSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}
