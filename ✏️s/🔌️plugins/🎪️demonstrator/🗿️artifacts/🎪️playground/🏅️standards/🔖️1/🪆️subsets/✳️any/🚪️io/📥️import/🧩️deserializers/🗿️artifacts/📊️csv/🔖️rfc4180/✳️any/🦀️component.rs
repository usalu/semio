//! playground <- csv
use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

/// 📥️ Inverse of the serializer's single-column table: takes the first data record's first field as
/// `schema`, skipping the header record when the table declares one. A table that carries no data
/// record degrades to `PlaygroundSnapshot::default()` rather than erroring — an empty CSV is a
/// legitimate input, not a malformed one.
///
/// 🎯️ Rewritten for stdio's migrated `CsvSnapshot` (`{has_header, records}`, was `{headers, rows}`).
pub async fn deserialize(from: &CsvSnapshot) -> Result<PlaygroundSnapshot, store::TextError> {
    let _ = STDIO_CSV_DOCUMENT_SCHEMA;
    let first_data_record = from.records.iter().skip(usize::from(from.has_header)).next();
    Ok(match first_data_record.and_then(|record| record.fields.first()) {
        Some(field) if !field.value.is_empty() => PlaygroundSnapshot { schema: field.value.clone() },
        _ => PlaygroundSnapshot::default(),
    })
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<PlaygroundSnapshot, store::TextError> {
    <PlaygroundSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}
