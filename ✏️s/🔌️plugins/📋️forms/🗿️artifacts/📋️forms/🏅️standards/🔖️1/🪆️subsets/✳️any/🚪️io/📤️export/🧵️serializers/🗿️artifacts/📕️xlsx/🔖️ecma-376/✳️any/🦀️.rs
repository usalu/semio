//! 📋️ forms -> xlsx — the question grid (`question_grid`, the csv export's rows) as one worksheet of
//! inline-string cells from column A; stdio's minimal package builder turns the workbook into the authoritative OPC XML
//! parts (`build_minimal_xlsx`) and stdio's xlsx codec writes them.
//!
//! 🔖 `IoFidelity::Lossy`: the same projection as csv — there is no xlsx import.
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::csv::v_rfc4180::any::question_grid;
use crate::FormsSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
use semio_s_artifact_stdio_xlsx::schema::construction::build_minimal_xlsx;

pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct FormsIntoXlsx;

impl Serializer<FormsSnapshot> for FormsIntoXlsx {
    const INTO: Dialect = XLSX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &FormsSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let cells = question_grid(from)
            .into_iter()
            .enumerate()
            .flat_map(|(row, values)| values.into_iter().enumerate().filter(|(_, value)| !value.is_empty()).map(move |(col, value)| XlsxCell { row: row as u32 + 1, col: col as u32, value: XlsxCellValue::InlineString(value) }))
            .collect();
        let snapshot = build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Questions".into(), cells }], shared_strings: Vec::new() });
        Ok(IoOutcome::clean(IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot))))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
