//! lowpoly -> png
use crate::artifacts::lowpoly::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<PngSnapshot, store::TextError> {
    let bytes = <LowpolySnapshot as store::DocumentPack>::encode_pack(snapshot)
        .or_else(|_| Ok(<LowpolySnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes()))?;
    semio_s_plugin_stdio::artifacts::png::engine::decode_png(&bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    semio_s_plugin_stdio::artifacts::png::engine::encode_png(&serialize(snapshot)?)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
