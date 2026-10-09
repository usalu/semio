//! 🌿️ vcs ← xlsx — the first worksheet of the workbook the package's authoritative XML parts project
//! (`XlsxSnapshot::project_workbook`), read as a table: row 1 names `VCS_RECORD_COLUMNS`, row 2 holds the values from
//! column A on (empty cells read as empty strings). The inverse of the sibling export.
use crate::standards::v1::subsets::any::io::vcs_from_record;
use crate::VcsSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_xlsx::schema::snapshot::XlsxCellValue;
use semio_s_artifact_stdio_xlsx::XlsxSnapshot;

pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct XlsxIntoVcs;

impl Deserializer<VcsSnapshot> for XlsxIntoVcs {
    const FROM: Dialect = XLSX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<VcsSnapshot> {
        let error = |message: String| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("XlsxIntoVcs: {message}")));
        let IoPayload::Binary(bytes) = payload else {
            return Err(error("expected a binary xlsx payload".into()));
        };
        let xlsx = <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| error(e.to_string()))?;
        let workbook = xlsx.project_workbook().map_err(|e| error(e.to_string()))?;
        let sheet = workbook.sheets.first().ok_or_else(|| error("the workbook has no worksheet".into()))?;
        let text = |value: &XlsxCellValue| match value {
            XlsxCellValue::InlineString(text) => Some(text.clone()),
            XlsxCellValue::SharedString(index) => workbook.shared_strings.get(*index).cloned(),
            XlsxCellValue::Number(number) => Some(number.to_string()),
            XlsxCellValue::Boolean(value) => Some(value.to_string()),
            _ => None,
        };
        let width = sheet.cells.iter().map(|cell| cell.col + 1).max().unwrap_or(0);
        let row = |at: u32| (0..width).map(|col| sheet.cells.iter().find(|cell| cell.row == at && cell.col == col).and_then(|cell| text(&cell.value)).unwrap_or_default()).collect::<Vec<_>>();
        Ok(IoOutcome::clean(vcs_from_record(&row(1), &row(2)).map_err(error)?))
    }
}
