//! program -> zip
use crate::artifacts::program::ProgramSnapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &ProgramSnapshot) -> Result<ZipSnapshot, store::TextError> {
    let _ = STDIO_ZIP_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("program->zip: {e}"), dsl::TextSpan::at(1, 1)))
}

pub async fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<ZipSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
