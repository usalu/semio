//! 🚪️ mathematical <- csv — foreign `Deserializer<MathematicalSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Each row becomes one graph
//! node (`id,label,x,y`), mirroring `mathematical_results_from_graph`'s own id/x/y table shape plus
//! a label column. `edges`, the geometry point cloud, and `equation` are never recoverable from a
//! flat grid, so this hop is `IoFidelity::Lossy`.

use crate::artifacts::mathematical::{mathematical_snapshot_with_state, MathematicalGeometry, MathematicalGraph, MathematicalNode, MathematicalSnapshot};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct CsvIntoMathematical;

impl Deserializer<MathematicalSnapshot> for CsvIntoMathematical {
    const FROM: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<MathematicalSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "CsvIntoMathematical: expected a binary csv payload".to_string(), diagnostics: Vec::new() });
        };
        let _ = STDIO_CSV_DOCUMENT_SCHEMA;
        let csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("CsvIntoMathematical: csv decode failed: {error}"), diagnostics: Vec::new() })?;
        let nodes: Vec<MathematicalNode> = csv
            .records
            .iter()
            .enumerate()
            .map(|(index, record)| {
                let id = record.fields.first().map(|field| field.value.clone()).unwrap_or_else(|| format!("node-{index}"));
                let label = record.fields.get(1).map(|field| field.value.clone()).unwrap_or_default();
                let x = record.fields.get(2).and_then(|field| field.value.parse().ok()).unwrap_or(0.0);
                let y = record.fields.get(3).and_then(|field| field.value.parse().ok()).unwrap_or(0.0);
                MathematicalNode { id, label, x, y }
            })
            .collect();
        let graph = MathematicalGraph { directed: true, nodes, edges: Vec::new(), algorithm: "topo".into(), algorithm_seed: None };
        Ok(IoOutcome::clean(mathematical_snapshot_with_state(graph, MathematicalGeometry { points: Vec::new() })))
    }
}
