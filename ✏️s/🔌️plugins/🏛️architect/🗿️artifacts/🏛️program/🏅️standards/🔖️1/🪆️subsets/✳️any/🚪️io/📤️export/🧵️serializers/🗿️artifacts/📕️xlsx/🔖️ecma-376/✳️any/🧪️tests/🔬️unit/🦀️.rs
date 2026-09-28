use super::*;
use semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx;
use semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx;

#[semio_framework_async_macros::async_test]
async fn exports_every_program_table_to_a_real_workbook() {
    let program = crate::sample_plugin();
    let snapshot = serialize(&program).expect("serialize program workbook");
    let workbook = snapshot.project_workbook().expect("project the program workbook");
    assert_eq!(workbook.sheets.len(), 70);
    assert!(workbook.sheets.iter().any(|sheet| sheet.name == "meta" && sheet.cells.iter().any(|cell| cell.row == 2 && cell.value == XlsxCellValue::InlineString("Sample Clinic".into()))));
    assert_eq!(workbook.sheets.iter().find(|sheet| sheet.name == "elements").expect("elements sheet").cells.iter().filter(|cell| cell.row > 1 && cell.col == 0).count(), 2);
    assert!(workbook.sheets.iter().any(|sheet| sheet.name == "risks" && sheet.cells.is_empty()));

    let raw = encode_xlsx(&snapshot).expect("encode real XLSX");
    let observed = decode_xlsx(&raw).expect("decode real XLSX");
    assert_eq!(observed.project_workbook().expect("project the decoded workbook"), workbook);
}
