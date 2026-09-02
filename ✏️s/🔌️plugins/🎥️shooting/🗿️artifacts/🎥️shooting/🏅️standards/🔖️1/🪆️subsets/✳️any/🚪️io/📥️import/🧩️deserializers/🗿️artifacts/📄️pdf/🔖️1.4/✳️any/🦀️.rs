//! shooting <- pdf
use crate::artifacts::shooting::ShootingSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &PdfSnapshot) -> Result<ShootingSnapshot, store::TextError> {
    let _ = STDIO_PDF_DOCUMENT_SCHEMA;
    let dsl_value = dsl::ToValue::to_value(from);
    dsl::FromValue::from_value(dsl_value).map_err(|e| store::TextError::new(format!("shooting<-pdf: {e}"), dsl::TextSpan::at(1, 1)))
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, store::TextError> {
    let wire = <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&wire)
}
