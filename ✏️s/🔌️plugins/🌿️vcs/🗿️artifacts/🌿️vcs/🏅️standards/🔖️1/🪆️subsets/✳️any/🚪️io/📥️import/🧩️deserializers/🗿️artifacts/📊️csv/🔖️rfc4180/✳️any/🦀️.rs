//! 🌿️ vcs ← csv — a table whose header names `VCS_RECORD_COLUMNS` (any order, any producer); its first
//! value row becomes the snapshot. The inverse of the sibling export.
use crate::standards::v1::subsets::any::io::vcs_from_record;
use crate::VcsSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_csv::CsvSnapshot;

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct CsvIntoVcs;

impl Deserializer<VcsSnapshot> for CsvIntoVcs {
    const FROM: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<VcsSnapshot> {
        let error = |message: String| IoError { message: format!("CsvIntoVcs: {message}"), diagnostics: Vec::new() };
        let IoPayload::Binary(bytes) = payload else {
            return Err(error("expected a binary csv payload".into()));
        };
        let csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| error(e.to_string()))?;
        let row = |at: usize| csv.records.get(at).map(|record| record.fields.iter().map(|field| field.value.clone()).collect::<Vec<_>>()).ok_or_else(|| error(format!("the table has no row {at}")));
        Ok(IoOutcome::clean(vcs_from_record(&row(0)?, &row(1)?).map_err(error)?))
    }
}
