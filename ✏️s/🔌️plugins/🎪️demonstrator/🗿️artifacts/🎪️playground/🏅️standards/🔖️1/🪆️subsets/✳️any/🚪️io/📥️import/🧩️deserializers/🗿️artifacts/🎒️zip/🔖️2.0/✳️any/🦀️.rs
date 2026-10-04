//! playground ← zip — the DSL member of a document archive written by the sibling export leaf (or any
//! archive carrying that member), parsed as this artifact's own DSL (`IoFidelity::Exact`).
use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_s_artifact_stdio_zip::io::decode_document_archive;

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<PlaygroundSnapshot, semio_framework_diagnostic::TextError> {
    decode_document_archive(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("playground←zip: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}
