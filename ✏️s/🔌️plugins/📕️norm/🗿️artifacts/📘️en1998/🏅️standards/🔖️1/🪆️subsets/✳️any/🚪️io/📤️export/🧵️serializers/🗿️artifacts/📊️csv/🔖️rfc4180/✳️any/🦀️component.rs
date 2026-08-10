//! en1998 -> csv
use crate::artifacts::en1998::En1998Snapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &En1998Snapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        headers: vec!["payload".into()],
        rows: vec![vec![<En1998Snapshot as store::ArtifactDsl>::print_dsl(snapshot)]],
    })
}

pub fn serialize_bytes(snapshot: &En1998Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
