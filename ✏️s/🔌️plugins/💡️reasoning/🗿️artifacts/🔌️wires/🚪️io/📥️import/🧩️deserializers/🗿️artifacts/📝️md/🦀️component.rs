//! wires <- md
use crate::artifacts::wires::schema::snapshot::WiresSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &MdSnapshot) -> Result<WiresSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <WiresSnapshot as store::DocumentDsl>::parse_dsl(&from.body)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<WiresSnapshot, store::TextError> {
    let md = <MdSnapshot as store::DocumentPack>::decode_pack(bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&md)
}
