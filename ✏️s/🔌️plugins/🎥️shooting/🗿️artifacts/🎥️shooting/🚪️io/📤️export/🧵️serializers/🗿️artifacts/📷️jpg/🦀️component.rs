//! shooting -> jpg
use crate::artifacts::shooting::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::jpg::{JpgSnapshot, STDIO_JPG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &ShootingSnapshot) -> Result<JpgSnapshot, store::TextError> {
    let bytes = <ShootingSnapshot as store::DocumentPack>::encode_pack(snapshot)
        .or_else(|_| Ok(<ShootingSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes()))?;
    semio_s_plugin_stdio::artifacts::jpg::engine::decode_jpg(&bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &ShootingSnapshot) -> Result<Vec<u8>, store::TextError> {
    semio_s_plugin_stdio::artifacts::jpg::engine::encode_jpg(&serialize(snapshot)?)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
