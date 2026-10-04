//! program -> xlsx — one worksheet per program table, header row 1 and one row per record from column A; stdio's minimal
//! package builder turns the workbook into the authoritative OPC XML parts (`build_minimal_xlsx`).
use crate::ProgramSnapshot;
pub use semio_s_artifact_stdio_xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
pub use semio_s_artifact_stdio_xlsx::XlsxSnapshot;
use semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx;
use std::collections::BTreeSet;

pub fn register() {}

fn export_error(message: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

fn cell_value(value: &semio_framework_value::DslValue) -> Result<XlsxCellValue, semio_framework_diagnostic::TextError> {
    match value {
        semio_framework_value::DslValue::Null => Ok(XlsxCellValue::Empty),
        semio_framework_value::DslValue::Bool(flag) => Ok(XlsxCellValue::Boolean(*flag)),
        semio_framework_value::DslValue::Number(_) => value.as_f64().map(XlsxCellValue::Number).ok_or_else(|| export_error(format!("program->xlsx: number {value:?} is not representable as f64"))),
        semio_framework_value::DslValue::String(text) => Ok(XlsxCellValue::InlineString(text.clone())),
        semio_framework_value::DslValue::Bytes(_) | semio_framework_value::DslValue::Array(_) | semio_framework_value::DslValue::Object(_) => Ok(XlsxCellValue::InlineString(semio_framework_pack_json::to_json_string(value))),
    }
}

pub fn serialize(snapshot: &ProgramSnapshot) -> Result<XlsxSnapshot, semio_framework_diagnostic::TextError> {
    let tables = crate::io::program_export_tables(snapshot).map_err(export_error)?;
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
    Ok(build_minimal_xlsx(XlsxWorkbook { sheets, shared_strings: Vec::new() }))
}

pub fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(<XlsxSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}

pub fn serialize_raw_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    let workbook = serialize(snapshot)?;
    semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx(&workbook).map_err(|error| export_error(format!("program->xlsx: {error}")))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
