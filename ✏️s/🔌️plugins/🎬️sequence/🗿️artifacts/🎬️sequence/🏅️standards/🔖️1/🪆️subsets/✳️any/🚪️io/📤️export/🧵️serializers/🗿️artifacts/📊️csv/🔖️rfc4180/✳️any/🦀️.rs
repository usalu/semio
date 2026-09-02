//! 🚪️ sequence -> csv — foreign `Serializer<SequenceSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Symmetric with the sibling
//! `Deserializer`'s row shape: id + kind + one JSON-encoded params column. `edges` are never
//! written (a flat grid has no edge concept), so this hop is `IoFidelity::Lossy`.

use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct SequenceIntoCsv;

impl Serializer<SequenceSnapshot> for SequenceIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &SequenceSnapshot) -> IoResult<IoPayload> {
        let fixture = from.try_to_fixture().map_err(|error| IoError { message: format!("SequenceIntoCsv: {error}"), diagnostics: Vec::new() })?;
        let records = fixture
            .steps
            .iter()
            .map(|step| {
                let value = serde_json::to_string(&step.params.0).unwrap_or_default();
                CsvRecord { fields: vec![CsvField { value: step.id.clone(), quoted: false }, CsvField { value: step.kind.clone(), quoted: false }, CsvField { value, quoted: true }] }
            })
            .collect();
        let csv = CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: false, records };
        Ok(IoOutcome::clean(IoPayload::Binary(<CsvSnapshot as store::ArtifactPack>::encode_pack(&csv))))
    }
}
