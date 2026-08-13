//! sequence <- csv
//!
//! 🐛️ Pre-migration content here read `from.headers`/`from.rows`, fields `CsvSnapshot` no longer
//! has since ticket `26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` dissolved it onto
//! `{schema, has_header, records: Vec<CsvRecord{fields: Vec<CsvField{value,quoted}>}>}` — a
//! pre-existing (unrelated to composition) bug this pass fixes outright: each row's first field
//! becomes a step id, the rest become one JSON-encoded `value` param, matching `stdio.csv`'s own
//! "one record = one row" grid shape as closely as a step-DAG import honestly can.
use crate::artifacts::sequence::schema::snapshot::SequenceSnapshot;
use crate::artifacts::sequence::{SequenceFixture, SequenceStep, StepParams, SEQUENCE_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &CsvSnapshot) -> Result<SequenceSnapshot, store::TextError> {
    let _ = STDIO_CSV_DOCUMENT_SCHEMA;
    let steps = from
        .records
        .iter()
        .enumerate()
        .map(|(index, record)| {
            let id = record.fields.first().map(|field| field.value.clone()).unwrap_or_else(|| format!("step-{index}"));
            let values: Vec<String> = record.fields.iter().skip(1).map(|field| field.value.clone()).collect();
            SequenceStep {
                id,
                kind: "computation.import".into(),
                params: StepParams::new().insert("value", neural_engine::Value::Atom(neural_engine::Atom::String(serde_json::to_string(&values).unwrap_or_default()))),
                x: index as f64 * 280.0,
                y: 0.0,
                slot: None,
                collapsed: false,
            }
        })
        .collect();
    Ok(SequenceSnapshot::from_fixture(SequenceFixture { schema: SEQUENCE_DOCUMENT_SCHEMA.into(), steps, edges: Vec::new() }))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<SequenceSnapshot, store::TextError> {
    <SequenceSnapshot as store::ArtifactPack>::decode_pack(bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}
