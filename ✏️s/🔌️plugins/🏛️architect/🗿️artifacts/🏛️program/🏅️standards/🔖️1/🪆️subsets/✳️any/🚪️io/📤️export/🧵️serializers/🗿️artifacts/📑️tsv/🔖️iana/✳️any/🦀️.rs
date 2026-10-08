//! 📑️ Native IANA TSV register exchange.
use crate::{ProgramSnapshot,kernel::PluginError};
use crate::standards::v1::subsets::any::io::tables::{collect_rows,RegisterCsvRow,REGISTER_ROW_COLUMNS};
use semio_s_artifact_stdio_tsv as stdio_tsv;
use semio_s_artifact_stdio_tsv::standards::iana::subsets::any::schema::snapshot as stdio_tsv_line_ending;

/// 📤️ Flattens all registers into a `TsvSnapshot`, encoded by stdio's real IANA TSV codec.
pub fn export_registers_tsv(program: &ProgramSnapshot) -> Result<String, PluginError> {
    Ok(stdio_tsv::standards::iana::subsets::any::io::text::snapshot::encode_tsv(&rows_to_tsv_snapshot(&collect_rows(program))))
}

fn rows_to_tsv_snapshot(rows: &[RegisterCsvRow]) -> stdio_tsv::TsvSnapshot {
    let mut records: Vec<Vec<String>> = vec![REGISTER_ROW_COLUMNS.iter().map(|c| c.to_string()).collect()];
    records.extend(rows.iter().map(|row| row.columns().to_vec()));
    stdio_tsv::TsvSnapshot { schema: stdio_tsv::STDIO_TSV_DOCUMENT_SCHEMA.into(), records, trailing_newline: true, line_ending: stdio_tsv_line_ending::LineEnding::Lf }
}
