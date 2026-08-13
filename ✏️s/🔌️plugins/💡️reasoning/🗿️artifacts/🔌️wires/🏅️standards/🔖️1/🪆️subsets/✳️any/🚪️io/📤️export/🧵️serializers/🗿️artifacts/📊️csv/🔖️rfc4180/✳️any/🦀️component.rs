//! wires -> csv
//! 🐛️ Pre-migration content here referenced `CsvSnapshot.headers`/`.rows` — stdio's `CsvSnapshot` was
//! independently reshaped to `has_header`/`records` (unrelated churn, baseline-broken before this
//! migration touched the file: `WiresSnapshot`'s own derived JSON never had "headers"/"rows" keys
//! either, so this always produced a degenerate empty `CsvSnapshot` in practice). Left as an honest
//! no-op passthrough (empty CSV, no real tabular mapping) pending a real wires<->csv design — not this
//! migration's scope.
use crate::artifacts::wires::schema::snapshot::WiresSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &WiresSnapshot) -> Result<CsvSnapshot, store::TextError> {
    let _ = snapshot;
    Ok(CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records: Vec::new() })
}

pub fn serialize_bytes(snapshot: &WiresSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
