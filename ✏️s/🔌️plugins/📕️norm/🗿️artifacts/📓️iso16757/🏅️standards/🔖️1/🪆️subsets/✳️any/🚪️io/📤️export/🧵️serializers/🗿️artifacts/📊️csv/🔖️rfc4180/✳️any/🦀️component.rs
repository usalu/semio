//! iso16757 -> csv
use crate::artifacts::iso16757::Iso16757Snapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Iso16757Snapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        headers: vec!["payload".into()],
        rows: vec![vec![<Iso16757Snapshot as store::ArtifactDsl>::print_dsl(snapshot)]],
    })
}

pub fn serialize_bytes(snapshot: &Iso16757Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
