//! 🏛️ program → csv — every register flattened to one RFC 4180 table
//! (`register,id,name,status,priority,tags,source`), written by stdio's own csv codec through
//! `export_registers_csv`, the same table the editor's "Export registers" action writes.
//!
//! 🔖 `IoFidelity::Lossy`: register rows only — relationships, adjacencies, quantities and project
//! metadata have no column in this table.
use crate::schema::snapshot::ProgramSnapshot;
use crate::kernel::PluginError;
use crate::standards::v1::subsets::any::io::tables::{collect_rows,RegisterCsvRow,REGISTER_ROW_COLUMNS};
use semio_s_artifact_stdio_csv as stdio_csv;

pub fn register() {}

pub fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    export_registers_csv(snapshot).map(String::into_bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("program→csv: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

fn csv_record(values: &[&str]) -> stdio_csv::schema::snapshot::CsvRecord {
    stdio_csv::schema::snapshot::CsvRecord { fields: values.iter().map(|v| stdio_csv::schema::snapshot::CsvField { value: (*v).to_string(), quoted: false }).collect() }
}

/// 📤️ Flattens all registers into a `CsvSnapshot`, encoded by stdio's real RFC 4180 codec.
pub fn export_registers_csv(program: &ProgramSnapshot) -> Result<String, PluginError> {
    Ok(stdio_csv::standards::v_rfc4180::subsets::any::io::text::snapshot::encode_csv(&rows_to_csv_snapshot(&collect_rows(program))))
}

/// ↔ Exports relationships as a CSV table preserving endpoints, encoded by stdio's real RFC 4180
/// codec.
pub fn export_relationships_csv(program: &ProgramSnapshot) -> Result<String, PluginError> {
    let mut records = vec![csv_record(&["id", "source_id", "target_id", "kind", "name"])];
    for rel in &program.relationships {
        records.push(csv_record(&[&rel.header.id.to_string(), &rel.source_id.to_string(), &rel.target_id.to_string(), &format!("{:?}", rel.kind), &rel.header.name]));
    }
    let snapshot = stdio_csv::CsvSnapshot { schema: stdio_csv::STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records };
    Ok(stdio_csv::standards::v_rfc4180::subsets::any::io::text::snapshot::encode_csv(&snapshot))
}

fn rows_to_csv_snapshot(rows: &[RegisterCsvRow]) -> stdio_csv::CsvSnapshot {
    let mut records = vec![csv_record(&REGISTER_ROW_COLUMNS)];
    records.extend(rows.iter().map(|row| {
        let cols = row.columns();
        csv_record(&[&cols[0], &cols[1], &cols[2], &cols[3], &cols[4], &cols[5], &cols[6]])
    }));
    stdio_csv::CsvSnapshot { schema: stdio_csv::STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records }
}
