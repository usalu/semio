//! 🌿️ vcs → csv — the snapshot as a one-record RFC 4180 table (header `VCS_RECORD_COLUMNS`, one value
//! row), written by stdio's own csv codec. `IoFidelity::Lossy` only in that a tag containing `;` splits.
use crate::standards::v1::subsets::any::io::{vcs_record, VCS_RECORD_COLUMNS};
use crate::VcsSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_csv::schema::snapshot::{CsvField, CsvRecord};
use semio_s_artifact_stdio_csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct VcsIntoCsv;

impl Serializer<VcsSnapshot> for VcsIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &VcsSnapshot) -> IoResult<IoPayload> {
        let record = |values: Vec<String>| CsvRecord { fields: values.into_iter().map(|value| CsvField { value, quoted: false }).collect() };
        let csv = CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records: vec![record(VCS_RECORD_COLUMNS.iter().map(|c| c.to_string()).collect()), record(vcs_record(from).to_vec())] };
        Ok(IoOutcome::clean(IoPayload::Binary(<CsvSnapshot as store::ArtifactPack>::encode_pack(&csv))))
    }
}
