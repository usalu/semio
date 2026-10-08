//! 🚪️ sequence <- csv — foreign `Deserializer<SequenceSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Each row's first field
//! becomes a step id, the rest become one JSON-encoded `value` param, matching `stdio.csv`'s own
//! "one record = one row" grid shape as closely as a step-DAG import honestly can. `edges` are
//! never recoverable from a flat grid, so this hop is `IoFidelity::Lossy`.
//!
//! 🐛️ Fixes a pre-migration bug: the old `deserialize_bytes` decoded the incoming bytes as a
//! `SequenceSnapshot` pack directly instead of as a `CsvSnapshot` pack — this impl's `deserialize`
//! decodes the foreign `CsvSnapshot` first, as the coordinate (`CSV_DIALECT`) requires.

use crate::{SequenceHostSnapshot, SequenceSnapshot, SequenceStep, StepParams, SEQUENCE_DOCUMENT_SCHEMA};
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct CsvIntoSequence;

impl Deserializer<SequenceSnapshot> for CsvIntoSequence {
    const FROM: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<SequenceSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, "CsvIntoSequence: expected a binary csv payload".to_string())));
        };
        let _ = STDIO_CSV_DOCUMENT_SCHEMA;
        let csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError::from_value_error(match error.into_value_error() { Ok(error) => error.under("CsvIntoSequence"), Err(error) => ValueError::new(ValueRefusalKind::InvariantViolated, format!("CsvIntoSequence: in-memory decode reported a transport failure: {error}")) }))?;
        let steps = csv
            .records
            .iter()
            .enumerate()
            .map(|(index, record)| {
                let id = record.fields.first().map_or_else(|| format!("step-{index}"), |field| field.value.clone());
                let values: Vec<String> = record.fields.iter().skip(1).map(|field| field.value.clone()).collect();
                SequenceStep {
                    id,
                    kind: "computation.import".into(),
                    params: StepParams::new().insert("value", neural_engine::Value::Atom(neural_engine::Atom::String(semio_framework_pack_json::to_json_string(&values)))),
                    x: index as f64 * 280.0,
                    y: 0.0,
                    slot: None,
                    collapsed: false,
                }
            })
            .collect();
        Ok(IoOutcome::clean(SequenceSnapshot::from_host_snapshot(SequenceHostSnapshot { schema: SEQUENCE_DOCUMENT_SCHEMA.into(), steps, edges: Vec::new() })))
    }
}
