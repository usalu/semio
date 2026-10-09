//! 🧱️ The one place that names the typed table of the `s.stdio.csv` artifact: records of text fields in, the RFC 4180 document text out (CRLF line endings, fields quoted exactly when the standard requires it).
//! 📎 https://www.rfc-editor.org/rfc/rfc4180

use semio_s_artifact_stdio_csv::standards::v_rfc4180::subsets::any::io::text::snapshot::encode_csv_with;
use semio_s_artifact_stdio_csv::{CsvField, CsvRecord, CsvSnapshot};

/// 🧾️ One record of plain text fields.
pub fn record<S: Into<String>>(fields: impl IntoIterator<Item = S>) -> CsvRecord {
    CsvRecord { fields: fields.into_iter().map(|value| CsvField { value: value.into(), quoted: false }).collect() }
}

/// 📄️ The RFC 4180 text of the records, the first one being the header.
pub fn document_text(records: Vec<CsvRecord>) -> String {
    encode_csv_with(&CsvSnapshot { records, ..CsvSnapshot::default() }, "\r\n")
}

/// 📥️ The records of RFC 4180 text, as plain strings: the reading half the tests use to check the writer against its own decoder.
pub fn read_records(text: &str) -> Vec<Vec<String>> {
    semio_s_artifact_stdio_csv::standards::v_rfc4180::subsets::any::io::text::snapshot::decode_csv_with(text, true).records.into_iter().map(|record| record.fields.into_iter().map(|field| field.value).collect()).collect()
}
