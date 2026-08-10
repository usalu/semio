//! wires -> svg
use crate::artifacts::wires::schema::snapshot::WiresSnapshot;
use semio_s_plugin_stdio::artifacts::svg::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &WiresSnapshot) -> Result<SvgSnapshot, store::TextError> {
    let _ = STDIO_SVG_DOCUMENT_SCHEMA;
    let bytes = <WiresSnapshot as store::DocumentPack>::encode_pack(snapshot);
    <SvgSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &WiresSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<SvgSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
