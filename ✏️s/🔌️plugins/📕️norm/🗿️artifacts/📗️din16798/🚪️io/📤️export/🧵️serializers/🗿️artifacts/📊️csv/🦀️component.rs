//! din16798 -> csv
use crate::artifacts::din16798::Din16798Snapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Din16798Snapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        headers: vec!["payload".into()],
        rows: vec![vec![<Din16798Snapshot as store::DocumentDsl>::print_dsl(snapshot)]],
    })
}

pub fn serialize_bytes(snapshot: &Din16798Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
