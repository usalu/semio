//! playground <- xlsx
use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_s_plugin_stdio::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

pub fn deserialize(from: &XlsxSnapshot) -> Result<PlaygroundSnapshot, store::TextError> {
    let _ = STDIO_XLSX_DOCUMENT_SCHEMA;
    let value = dsl::ToValue::to_value(from);
    dsl::FromValue::from_value(value).map_err(|e| store::TextError::new(format!("playground<-xlsx: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<PlaygroundSnapshot, store::TextError> {
    let wire = <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&wire)
}
