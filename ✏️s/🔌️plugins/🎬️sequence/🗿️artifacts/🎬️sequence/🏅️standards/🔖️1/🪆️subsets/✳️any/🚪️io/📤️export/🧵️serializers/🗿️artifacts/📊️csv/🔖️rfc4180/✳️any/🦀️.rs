//! 🚪️ sequence -> csv — foreign `Serializer<SequenceSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Symmetric with the sibling
//! `Deserializer`'s row shape: id + kind + one JSON-encoded params column. `edges` are never
//! written (a flat grid has no edge concept), so this hop is `IoFidelity::Lossy`.

use crate::SequenceSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_csv::schema::snapshot::{CsvField, CsvRecord};
use semio_s_artifact_stdio_csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct SequenceIntoCsv;

impl Serializer<SequenceSnapshot> for SequenceIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &SequenceSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let fixture = from.try_to_host_snapshot().map_err(|error| IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner, format!("SequenceIntoCsv: {error}"))))?;
        let records = fixture
            .steps
            .iter()
            .map(|step| {
                let value = semio_framework_pack_json::to_json_string(&step.params.0);
                CsvRecord { fields: vec![CsvField { value: step.id.clone(), quoted: false }, CsvField { value: step.kind.clone(), quoted: false }, CsvField { value, quoted: true }] }
            })
            .collect();
        let csv = CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: false, records };
        Ok(IoOutcome::clean(IoPayload::Binary(<CsvSnapshot as store::ArtifactPack>::encode_pack(&csv))))
    }
}
