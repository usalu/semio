//! present <- png
use crate::artifacts::present::schema::snapshot::PresentSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PngSnapshot) -> Result<PresentSnapshot, store::TextError> {
    let _ = STDIO_PNG_DOCUMENT_SCHEMA;
    let bytes = <PngSnapshot as store::DocumentPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<PresentSnapshot, store::TextError> {
    <PresentSnapshot as store::DocumentPack>::decode_pack(bytes).or_else(|_| {
        <PresentSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}
