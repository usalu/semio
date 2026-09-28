//! 🌿️ vcs → xlsx — the same one-record table as csv (header row 1, values row 2) on a `Vcs` worksheet
//! of inline strings, written by stdio's own xlsx codec.
use crate::standards::v1::subsets::any::io::{vcs_record, VCS_RECORD_COLUMNS};
use crate::VcsSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
use semio_s_artifact_stdio_xlsx::XlsxSnapshot;

pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct VcsIntoXlsx;

impl Serializer<VcsSnapshot> for VcsIntoXlsx {
    const INTO: Dialect = XLSX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &VcsSnapshot) -> IoResult<IoPayload> {
        let rows = [VCS_RECORD_COLUMNS.map(String::from), vcs_record(from)];
        let cells = rows.iter().enumerate().flat_map(|(row, values)| values.iter().enumerate().filter(|(_, value)| !value.is_empty()).map(move |(col, value)| XlsxCell { row: row as u32 + 1, col: col as u32 + 1, value: XlsxCellValue::InlineString(value.clone()) })).collect();
        let snapshot = XlsxSnapshot { workbook: XlsxWorkbook { sheets: vec![XlsxSheet { name: "Vcs".into(), cells }], shared_strings: Vec::new() }, ..Default::default() };
        Ok(IoOutcome::clean(IoPayload::Binary(<XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot))))
    }
}
