//! playground <- zip
use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub fn deserialize(from: &ZipSnapshot) -> Result<PlaygroundSnapshot, store::TextError> {
    let _ = STDIO_ZIP_DOCUMENT_SCHEMA;
    let value = dsl::ToValue::to_value(from);
    dsl::FromValue::from_value(value).map_err(|e| store::TextError::new(format!("playground<-zip: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<PlaygroundSnapshot, store::TextError> {
    let wire = <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&wire)
}
