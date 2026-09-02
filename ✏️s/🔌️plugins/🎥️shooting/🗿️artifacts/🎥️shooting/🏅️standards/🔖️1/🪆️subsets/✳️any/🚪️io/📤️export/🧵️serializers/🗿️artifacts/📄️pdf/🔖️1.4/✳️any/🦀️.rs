//! shooting -> pdf
use crate::artifacts::shooting::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &ShootingSnapshot) -> Result<PdfSnapshot, store::TextError> {
    let _ = STDIO_PDF_DOCUMENT_SCHEMA;
    let value = dsl::ToValue::to_value(snapshot);
    dsl::FromValue::from_value(value).map_err(|e| store::TextError::new(format!("shooting->pdf: {e}"), dsl::TextSpan::at(1, 1)))
}

pub async fn serialize_bytes(snapshot: &ShootingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<PdfSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
