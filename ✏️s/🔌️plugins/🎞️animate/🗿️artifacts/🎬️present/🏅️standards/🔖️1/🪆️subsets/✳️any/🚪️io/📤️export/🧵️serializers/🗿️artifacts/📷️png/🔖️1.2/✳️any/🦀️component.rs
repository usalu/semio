//! present -> png
use crate::artifacts::present::schema::snapshot::PresentSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &PresentSnapshot) -> Result<PngSnapshot, store::TextError> {
    let _ = STDIO_PNG_DOCUMENT_SCHEMA;
    let bytes = <PresentSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <PngSnapshot as store::ArtifactPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &PresentSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<PngSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
