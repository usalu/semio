//! en1991 -> csv
use crate::artifacts::en1991::En1991Snapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &En1991Snapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        headers: vec!["payload".into()],
        rows: vec![vec![<En1991Snapshot as store::ArtifactDsl>::print_dsl(snapshot)]],
    })
}

pub fn serialize_bytes(snapshot: &En1991Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
