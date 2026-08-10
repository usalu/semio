//! fem2d -> csv
use crate::artifacts::fem2d::Fem2dSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &Fem2dSnapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        headers: vec!["payload".into()],
        rows: vec![vec![<Fem2dSnapshot as store::ArtifactDsl>::print_dsl(snapshot)]],
    })
}

pub fn serialize_bytes(snapshot: &Fem2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
