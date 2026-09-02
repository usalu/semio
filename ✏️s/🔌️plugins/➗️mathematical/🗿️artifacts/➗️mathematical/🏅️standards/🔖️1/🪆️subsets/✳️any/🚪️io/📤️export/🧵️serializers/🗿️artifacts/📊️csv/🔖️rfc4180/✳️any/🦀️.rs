//! 🚪️ mathematical -> csv — foreign `Serializer<MathematicalSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Symmetric with the sibling
//! `Deserializer`'s row shape: one row per node (`id,label,x,y`). `edges`, geometry, and `equation`
//! are never written (a flat grid has no edge/point-cloud/expression-tree concept), so this hop is
//! `IoFidelity::Lossy`.

use crate::artifacts::mathematical::{require_mathematical_scene, MathematicalSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct MathematicalIntoCsv;

impl Serializer<MathematicalSnapshot> for MathematicalIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &MathematicalSnapshot) -> IoResult<IoPayload> {
        let scene = require_mathematical_scene(from).map_err(|error| IoError { message: format!("MathematicalIntoCsv: {error}"), diagnostics: Vec::new() })?;
        let graph = &scene.graph;
        let records = graph
            .nodes
            .iter()
            .map(|node| CsvRecord {
                fields: vec![
                    CsvField { value: node.id.clone(), quoted: false },
                    CsvField { value: node.label.clone(), quoted: true },
                    CsvField { value: format!("{}", node.x), quoted: false },
                    CsvField { value: format!("{}", node.y), quoted: false },
                ],
            })
            .collect();
        let csv = CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: false, records };
        Ok(IoOutcome::clean(IoPayload::Binary(<CsvSnapshot as store::ArtifactPack>::encode_pack(&csv))))
    }
}
