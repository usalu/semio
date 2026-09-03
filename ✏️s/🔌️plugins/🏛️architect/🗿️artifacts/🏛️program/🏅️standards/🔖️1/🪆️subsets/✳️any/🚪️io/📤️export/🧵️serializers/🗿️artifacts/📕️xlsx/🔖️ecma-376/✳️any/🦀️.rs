//! program -> xlsx
use crate::artifacts::program::ProgramSnapshot;
pub use semio_s_plugin_stdio::artifacts::xlsx::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxSnapshot, XlsxWorkbook};
use std::collections::BTreeSet;

pub async fn register() {}

fn export_error(message: impl Into<String>) -> store::TextError {
    store::TextError::new(message.into(), dsl::TextSpan::at(1, 1))
}

fn cell_value(value: &dsl::DslValue) -> Result<XlsxCellValue, store::TextError> {
    match value {
        dsl::DslValue::Null => Ok(XlsxCellValue::Empty),
        dsl::DslValue::Bool(flag) => Ok(XlsxCellValue::Boolean(*flag)),
        dsl::DslValue::Number(_) => value.as_f64().map(XlsxCellValue::Number).ok_or_else(|| export_error(format!("program->xlsx: number {value:?} is not representable as f64"))),
        dsl::DslValue::String(text) => Ok(XlsxCellValue::InlineString(text.clone())),
        dsl::DslValue::Array(_) | dsl::DslValue::Object(_) => Ok(XlsxCellValue::InlineString(dsl::json::to_json_string(value))),
    }
}

pub async fn serialize(snapshot: &ProgramSnapshot) -> Result<XlsxSnapshot, store::TextError> {
    let tables = crate::artifacts::program::io::program_export_tables(snapshot).await.map_err(export_error)?;
    let mut sheets = Vec::with_capacity(tables.len());
    for table in tables {
        let columns: Vec<String> = table.rows.iter().flat_map(|row| row.iter().map(|(key, _)| key.clone())).collect::<BTreeSet<_>>().into_iter().collect();
        let mut cells = Vec::with_capacity(columns.len().saturating_mul(table.rows.len().saturating_add(1)));
        for (col, name) in columns.iter().enumerate() {
            cells.push(XlsxCell { row: 1, col: u32::try_from(col).map_err(|_| export_error("program->xlsx: too many columns"))?, value: XlsxCellValue::InlineString(name.clone()) });
        }
        for (row_index, row) in table.rows.iter().enumerate() {
            let row_number = u32::try_from(row_index + 2).map_err(|_| export_error("program->xlsx: too many rows"))?;
            for (col, name) in columns.iter().enumerate() {
                let value = row.iter().find(|(key, _)| key == name).map(|(_, value)| cell_value(value)).transpose()?.unwrap_or(XlsxCellValue::Empty);
                cells.push(XlsxCell { row: row_number, col: u32::try_from(col).map_err(|_| export_error("program->xlsx: too many columns"))?, value });
            }
        }
        sheets.push(XlsxSheet { name: table.name.into(), cells });
    }
    Ok(XlsxSnapshot::from_parts(Default::default(), XlsxWorkbook { sheets, shared_strings: Vec::new() }))
}

pub async fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<XlsxSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot).await?))
}

pub async fn serialize_raw_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, store::TextError> {
    let workbook = serialize(snapshot).await?;
    semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx(&workbook).map_err(|error| export_error(format!("program->xlsx: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx;
    use semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx;

    #[semio_framework_async_macros::async_test]
    async fn exports_every_program_table_to_a_real_workbook() {
        let program = crate::artifacts::program::sample_plugin().await;
        let workbook = serialize(&program).await.expect("serialize program workbook");
        assert_eq!(workbook.workbook.sheets.len(), 70);
        assert!(workbook.workbook.sheets.iter().any(|sheet| sheet.name == "meta" && sheet.cells.iter().any(|cell| cell.row == 2 && cell.value == semio_s_plugin_stdio::artifacts::xlsx::XlsxCellValue::InlineString("Sample Clinic".into()))));
        assert_eq!(workbook.workbook.sheets.iter().find(|sheet| sheet.name == "elements").expect("elements sheet").cells.iter().filter(|cell| cell.row > 1 && cell.col == 0).count(), 2);
        assert!(workbook.workbook.sheets.iter().any(|sheet| sheet.name == "risks" && sheet.cells.is_empty()));

        let raw = encode_xlsx(&workbook).expect("encode real XLSX");
        let observed = decode_xlsx(&raw).expect("decode real XLSX");
        assert_eq!(observed.workbook, workbook.workbook);
    }
}
