//! program -> csv. `stdio.csv`'s real `CsvSnapshot` shape (`has_header` + `records: Vec<CsvRecord>`
//! of `CsvField{value, quoted}`) landed after this leaf was first written — lagging call site
//! fixed to match (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-
//! RETIREMENT W5a closer), same single-blob-payload shape as its `headers`/`rows` predecessor
//! (which never actually populated real headers/rows — both were always empty via
//! `unwrap_or_default` against a `ProgramSnapshot`-shaped JSON value that has no such keys),
//! just through the current fields and printing the real DSL text honestly.
use crate::artifacts::program::schema::snapshot::ProgramSnapshot;
use semio_s_plugin_stdio::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(snapshot: &ProgramSnapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        has_header: true,
        records: vec![CsvRecord { fields: vec![CsvField { value: "payload".into(), quoted: false }] }, CsvRecord { fields: vec![CsvField { value: <ProgramSnapshot as store::ArtifactDsl>::print_dsl(snapshot), quoted: true }] }],
    })
}

pub async fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
