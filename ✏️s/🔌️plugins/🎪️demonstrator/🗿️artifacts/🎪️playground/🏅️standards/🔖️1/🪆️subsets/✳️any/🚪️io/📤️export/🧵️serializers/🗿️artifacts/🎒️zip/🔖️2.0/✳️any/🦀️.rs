//! playground -> zip
use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub fn serialize(snapshot: &PlaygroundSnapshot) -> Result<ZipSnapshot, store::TextError> {
    let _ = STDIO_ZIP_DOCUMENT_SCHEMA;
    let value = dsl::ToValue::to_value(snapshot);
    dsl::FromValue::from_value(value).map_err(|e| store::TextError::new(format!("playground->zip: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &PlaygroundSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<ZipSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
