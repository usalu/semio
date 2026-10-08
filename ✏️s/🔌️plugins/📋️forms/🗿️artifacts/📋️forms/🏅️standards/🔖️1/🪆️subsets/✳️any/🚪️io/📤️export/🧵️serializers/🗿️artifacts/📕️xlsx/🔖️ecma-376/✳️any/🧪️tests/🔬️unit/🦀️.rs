use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::onboarding_example_spec;

/// 🔮️ The third-party `calamine` reader (test-only) opens the exported workbook: one `Questions` sheet whose cells from
/// column A are exactly the csv export's question grid.
#[semio_framework_async_macros::async_test]
async fn xlsx_is_a_real_workbook_of_the_question_grid_a_third_party_reader_agrees() {
    use calamine::Reader;
    let spec = onboarding_example_spec();
    let IoOutcome { value: IoPayload::Binary(pack), .. } = FormsIntoXlsx::serialize(&spec, &semio_framework_os_kernel::io::io_mechanism::ArchiveChildren::empty()).await.expect("xlsx") else { panic!("binary") };
    let snapshot = <semio_s_artifact_stdio_xlsx::XlsxSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("xlsx pack");
    let bytes = semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx(&snapshot).expect("xlsx bytes");
    let mut reference: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(bytes)).expect("calamine opens the workbook");
    assert_eq!(reference.sheet_names(), ["Questions"]);
    let range = reference.worksheet_range("Questions").expect("the Questions sheet");
    let grid = question_grid(&spec);
    assert_eq!(grid[0], ["id", "stepId", "label", "kind", "required"]);
    for (row, values) in grid.iter().enumerate() {
        for (col, value) in values.iter().enumerate() {
            assert_eq!(&range.get_value((row as u32, col as u32)).map(|cell| cell.to_string()).unwrap_or_default(), value, "row {row} column {col}");
        }
    }
}
