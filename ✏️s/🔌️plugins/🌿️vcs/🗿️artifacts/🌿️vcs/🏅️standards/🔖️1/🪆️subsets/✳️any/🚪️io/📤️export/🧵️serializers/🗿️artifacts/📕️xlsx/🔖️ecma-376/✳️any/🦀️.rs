//! 🌿️ vcs → xlsx — the same one-record table as csv (header row 1, values row 2, from column A) on a `Vcs` worksheet
//! of inline strings: stdio's own minimal package builder turns the workbook into the authoritative OPC XML parts
//! (`build_minimal_xlsx`), and stdio's xlsx codec writes them.
use crate::standards::v1::subsets::any::io::{vcs_record, VCS_RECORD_COLUMNS};
use crate::VcsSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
use semio_s_artifact_stdio_xlsx::schema::construction::build_minimal_xlsx;
use semio_s_artifact_stdio_xlsx::XlsxSnapshot;

pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct VcsIntoXlsx;

impl Serializer<VcsSnapshot> for VcsIntoXlsx {
    const INTO: Dialect = XLSX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &VcsSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let rows = [VCS_RECORD_COLUMNS.map(String::from), vcs_record(from)];
        let cells = rows.iter().enumerate().flat_map(|(row, values)| values.iter().enumerate().filter(|(_, value)| !value.is_empty()).map(move |(col, value)| XlsxCell { row: row as u32 + 1, col: col as u32, value: XlsxCellValue::InlineString(value.clone()) })).collect();
        let snapshot = build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Vcs".into(), cells }], shared_strings: Vec::new() });
        Ok(IoOutcome::clean(IoPayload::Binary(<XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot))))
    }
}
