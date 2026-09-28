//! 🌿️ vcs ← xlsx — the first worksheet read as a table: row 1 names `VCS_RECORD_COLUMNS`, row 2 holds
//! the values (empty cells read as empty strings). The inverse of the sibling export.
use crate::standards::v1::subsets::any::io::vcs_from_record;
use crate::VcsSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_xlsx::schema::snapshot::XlsxCellValue;
use semio_s_artifact_stdio_xlsx::XlsxSnapshot;

pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct XlsxIntoVcs;

impl Deserializer<VcsSnapshot> for XlsxIntoVcs {
    const FROM: Dialect = XLSX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<VcsSnapshot> {
        let error = |message: String| IoError { message: format!("XlsxIntoVcs: {message}"), diagnostics: Vec::new() };
        let IoPayload::Binary(bytes) = payload else {
            return Err(error("expected a binary xlsx payload".into()));
        };
        let xlsx = <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| error(e.to_string()))?;
        let sheet = xlsx.workbook.sheets.first().ok_or_else(|| error("the workbook has no worksheet".into()))?;
        let text = |value: &XlsxCellValue| match value {
            XlsxCellValue::InlineString(text) => Some(text.clone()),
            XlsxCellValue::SharedString(index) => xlsx.workbook.shared_strings.get(*index).cloned(),
            XlsxCellValue::Number(number) => Some(number.to_string()),
            XlsxCellValue::Boolean(value) => Some(value.to_string()),
            _ => None,
        };
        let row = |at: u32| {
            let width = sheet.cells.iter().map(|cell| cell.col).max().unwrap_or(0);
            (1..=width).map(|col| sheet.cells.iter().find(|cell| cell.row == at && cell.col == col).and_then(|cell| text(&cell.value)).unwrap_or_default()).collect::<Vec<_>>()
        };
        Ok(IoOutcome::clean(vcs_from_record(&row(1), &row(2)).map_err(error)?))
    }
}
