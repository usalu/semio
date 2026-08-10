//! vdi3805 -> csv
use crate::artifacts::vdi3805::Vdi3805Snapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Vdi3805Snapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        headers: vec!["payload".into()],
        rows: vec![vec![<Vdi3805Snapshot as store::DocumentDsl>::print_dsl(snapshot)]],
    })
}

pub fn serialize_bytes(snapshot: &Vdi3805Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}
