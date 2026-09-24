//! 📋️ forms -> xlsx — the question grid (`question_grid`, the csv export's rows) as one worksheet of
//! inline-string cells, written by stdio's own xlsx codec.
//!
//! 🔖 `IoFidelity::Lossy`: the same projection as csv — there is no xlsx import.
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::csv::v_rfc4180::any::question_grid;
use crate::FormsSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
use semio_s_artifact_stdio_xlsx::XlsxSnapshot;

pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct FormsIntoXlsx;

impl Serializer<FormsSnapshot> for FormsIntoXlsx {
    const INTO: Dialect = XLSX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &FormsSnapshot) -> IoResult<IoPayload> {
        let cells = question_grid(from)
            .into_iter()
            .enumerate()
            .flat_map(|(row, values)| values.into_iter().enumerate().filter(|(_, value)| !value.is_empty()).map(move |(col, value)| XlsxCell { row: row as u32 + 1, col: col as u32 + 1, value: XlsxCellValue::InlineString(value) }))
            .collect();
        let snapshot = XlsxSnapshot { workbook: XlsxWorkbook { sheets: vec![XlsxSheet { name: "Questions".into(), cells }], shared_strings: Vec::new() }, ..Default::default() };
        Ok(IoOutcome::clean(IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot))))
    }
}
