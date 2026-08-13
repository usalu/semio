//! playground -> csv
use crate::artifacts::playground::schema::snapshot::PlaygroundSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvField, CsvRecord, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

/// 📤️ `PlaygroundSnapshot` carries exactly one authored scalar (`schema`), so its CSV form is a
/// single-column table: one header record naming the column, one data record holding the value.
///
/// 🎯️ Rewritten for stdio's migrated `CsvSnapshot` (`{has_header, records}`, was `{headers, rows}`).
/// The previous version probed the JSON for `"headers"`/`"rows"` keys that a `PlaygroundSnapshot`
/// never had and fell through to `unwrap_or_default()`, so it emitted an EMPTY table and the
/// round trip through CSV silently lost the schema. This carries it.
pub fn serialize(snapshot: &PlaygroundSnapshot) -> Result<CsvSnapshot, store::TextError> {
    let field = |value: &str| CsvRecord { fields: vec![CsvField { value: value.to_string(), quoted: false }] };
    Ok(CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records: vec![field("schema"), field(&snapshot.schema)] })
}

pub fn serialize_bytes(snapshot: &PlaygroundSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
