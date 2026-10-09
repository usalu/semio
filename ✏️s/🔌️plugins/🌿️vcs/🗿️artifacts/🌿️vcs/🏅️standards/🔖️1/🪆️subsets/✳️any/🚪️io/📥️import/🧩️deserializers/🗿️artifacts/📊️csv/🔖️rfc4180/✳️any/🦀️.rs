//! 🌿️ vcs ← csv — a table whose header names `VCS_RECORD_COLUMNS` (any order, any producer); its first
//! value row becomes the snapshot. The inverse of the sibling export.
use crate::standards::v1::subsets::any::io::vcs_from_record;
use crate::VcsSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_csv::CsvSnapshot;

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct CsvIntoVcs;

impl Deserializer<VcsSnapshot> for CsvIntoVcs {
    const FROM: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<VcsSnapshot> {
        let error = |message: String| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("CsvIntoVcs: {message}")));
        let IoPayload::Binary(bytes) = payload else {
            return Err(error("expected a binary csv payload".into()));
        };
        let csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| error(e.to_string()))?;
        let row = |at: usize| csv.records.get(at).map(|record| record.fields.iter().map(|field| field.value.clone()).collect::<Vec<_>>()).ok_or_else(|| error(format!("the table has no row {at}")));
        Ok(IoOutcome::clean(vcs_from_record(&row(0)?, &row(1)?).map_err(error)?))
    }
}
