//! program -> zip
use crate::ProgramSnapshot;
pub use semio_s_artifact_stdio_zip::schema::snapshot::ZipEntry;
pub use semio_s_artifact_stdio_zip::ZipSnapshot;
use semio_s_artifact_stdio_zip::STDIO_ZIP_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn serialize(snapshot: &ProgramSnapshot) -> Result<ZipSnapshot, semio_framework_diagnostic::TextError> {
    let tables = crate::standards::v1::subsets::any::io::program_export_tables(snapshot).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let entries = tables
        .into_iter()
        .map(|table| {
            let rows = semio_framework_value::DslValue::Array(table.rows.into_iter().map(semio_framework_value::DslValue::Object).collect());
            ZipEntry { name: format!("{}.json", table.name), data: semio_framework_pack_json::to_json_string(&rows).into_bytes(), ..Default::default() }
        })
        .collect::<Vec<_>>();
    Ok(ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries, comment: "s.architect.program@1/*".into(), ..Default::default() })
}

pub fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(<ZipSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}

pub fn serialize_raw_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    let archive = serialize(snapshot)?;
    semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::encode_zip(&archive).map_err(|error| { let error = error.into_value_error(); semio_framework_diagnostic::TextError::new(error.kind, format!("program->zip: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
