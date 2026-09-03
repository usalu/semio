//! 🚪️ equation <- csv — foreign `Deserializer<EquationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Each row becomes one graph
//! node (`id,label,x,y`), mirroring `equation_results_from_graph`'s own id/x/y table shape plus
//! a label column. `edges`, the geometry point cloud, and `equation` are never recoverable from a
//! flat grid, so this hop is `IoFidelity::Lossy`.

use crate::artifacts::equation::{equation_snapshot_with_state, EquationGeometry, EquationGraph, EquationNode, EquationSnapshot};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct CsvIntoEquation;

impl Deserializer<EquationSnapshot> for CsvIntoEquation {
    const FROM: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<EquationSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "CsvIntoEquation: expected a binary csv payload".to_string(), diagnostics: Vec::new() });
        };
        let _ = STDIO_CSV_DOCUMENT_SCHEMA;
        let csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("CsvIntoEquation: csv decode failed: {error}"), diagnostics: Vec::new() })?;
        let nodes: Vec<EquationNode> = csv
            .records
            .iter()
            .enumerate()
            .map(|(index, record)| {
                let id = record.fields.first().map(|field| field.value.clone()).unwrap_or_else(|| format!("node-{index}"));
                let label = record.fields.get(1).map(|field| field.value.clone()).unwrap_or_default();
                let x = record.fields.get(2).and_then(|field| field.value.parse().ok()).unwrap_or(0.0);
                let y = record.fields.get(3).and_then(|field| field.value.parse().ok()).unwrap_or(0.0);
                EquationNode { id, label, x, y }
            })
            .collect();
        let graph = EquationGraph { directed: true, nodes, edges: Vec::new(), algorithm: "topo".into(), algorithm_seed: None };
        Ok(IoOutcome::clean(equation_snapshot_with_state(graph, EquationGeometry { points: Vec::new() })))
    }
}
