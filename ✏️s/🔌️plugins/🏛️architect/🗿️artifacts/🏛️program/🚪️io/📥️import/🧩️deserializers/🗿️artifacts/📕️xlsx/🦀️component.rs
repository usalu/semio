//! program <- xlsx
use crate::artifacts::program::ProgramSnapshot;
use semio_s_plugin_stdio::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &XlsxSnapshot) -> Result<ProgramSnapshot, store::TextError> {
    let _ = STDIO_XLSX_DOCUMENT_SCHEMA;
    let bytes = semio_s_plugin_stdio::artifacts::xlsx::engine::encode_xlsx(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <ProgramSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <ProgramSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ProgramSnapshot, store::TextError> {
    deserialize(&semio_s_plugin_stdio::artifacts::xlsx::engine::decode_xlsx(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}
