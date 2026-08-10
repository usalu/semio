//! shooting -> bmp
use crate::artifacts::shooting::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::bmp::{BmpSnapshot, STDIO_BMP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &ShootingSnapshot) -> Result<BmpSnapshot, store::TextError> {
    let bytes = <ShootingSnapshot as store::DocumentPack>::encode_pack(snapshot)
        .or_else(|_| Ok(<ShootingSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes()))?;
    semio_s_plugin_stdio::artifacts::bmp::engine::decode_bmp(&bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &ShootingSnapshot) -> Result<Vec<u8>, store::TextError> {
    semio_s_plugin_stdio::artifacts::bmp::engine::encode_bmp(&serialize(snapshot)?)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
