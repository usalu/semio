//! sequence -> csv
//!
//! 🐛️ Pre-migration content here built a `{headers, rows}` value `CsvSnapshot` no longer has
//! (see the sibling deserializer's doc comment) — a pre-existing bug this pass fixes outright,
//! symmetric with that deserializer's `id + one JSON value column` row shape.
use crate::artifacts::sequence::schema::snapshot::SequenceSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};

pub fn register() {}

pub fn serialize(snapshot: &SequenceSnapshot) -> Result<CsvSnapshot, store::TextError> {
    let fixture = snapshot.to_fixture();
    let records = fixture
        .steps
        .iter()
        .map(|step| {
            let value = serde_json::to_string(&step.params.0).unwrap_or_default();
            CsvRecord { fields: vec![CsvField { value: step.id.clone(), quoted: false }, CsvField { value: step.kind.clone(), quoted: false }, CsvField { value, quoted: true }] }
        })
        .collect();
    Ok(CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: false, records })
}

pub fn serialize_bytes(snapshot: &SequenceSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
