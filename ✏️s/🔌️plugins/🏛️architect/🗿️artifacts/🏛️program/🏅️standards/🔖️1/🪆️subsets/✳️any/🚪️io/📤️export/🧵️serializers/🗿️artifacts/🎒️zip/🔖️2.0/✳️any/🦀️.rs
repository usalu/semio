//! program -> zip
use crate::ProgramSnapshot;
use semio_s_artifact_stdio_zip::STDIO_ZIP_DOCUMENT_SCHEMA;
pub use semio_s_artifact_stdio_zip::ZipSnapshot;
pub use semio_s_artifact_stdio_zip::schema::snapshot::ZipEntry;

pub fn register() {}

pub fn serialize(snapshot: &ProgramSnapshot) -> Result<ZipSnapshot, store::TextError> {
    let tables = crate::io::program_export_tables(snapshot).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))?;
    let entries = tables
        .into_iter()
        .map(|table| {
            let rows = dsl::DslValue::Array(table.rows.into_iter().map(dsl::DslValue::Object).collect());
            ZipEntry { name: format!("{}.json", table.name), data: dsl::json::to_json_string(&rows).into_bytes() }
        })
        .collect::<Vec<_>>();
    Ok(ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries, comment: "s.architect.program@1/*".into() })
}

pub fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<ZipSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}

pub fn serialize_raw_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, store::TextError> {
    let archive = serialize(snapshot)?;
    semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::encode_zip(&archive).map_err(|error| store::TextError::new(format!("program->zip: {error}"), dsl::TextSpan::at(1, 1)))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
