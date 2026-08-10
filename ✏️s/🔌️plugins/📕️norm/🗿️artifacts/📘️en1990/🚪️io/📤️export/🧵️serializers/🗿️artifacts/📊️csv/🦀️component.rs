//! en1990 -> csv
use crate::artifacts::en1990::En1990Snapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &En1990Snapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        headers: vec!["payload".into()],
        rows: vec![vec![<En1990Snapshot as store::DocumentDsl>::print_dsl(snapshot)]],
    })
}

pub fn serialize_bytes(snapshot: &En1990Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
