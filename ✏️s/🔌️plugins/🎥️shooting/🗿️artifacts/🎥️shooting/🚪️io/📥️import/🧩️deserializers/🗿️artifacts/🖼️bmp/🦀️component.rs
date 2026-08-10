//! shooting <- bmp
use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::bmp::{BmpSnapshot, STDIO_BMP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &BmpSnapshot) -> Result<ShootingSnapshot, store::TextError> {
    let _ = STDIO_BMP_DOCUMENT_SCHEMA;
    let bytes = <BmpSnapshot as store::DocumentPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, store::TextError> {
    <ShootingSnapshot as store::DocumentPack>::decode_pack(bytes).or_else(|_| {
        <ShootingSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}
