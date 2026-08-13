//! wires <- csv
//! 🐛️ Same baseline-broken `headers`/`rows` mismatch as the export leaf (see its module doc) — stdio's
//! `CsvSnapshot` is `has_header`/`records` now, and `WiresSnapshot`'s derived JSON never had a
//! "headers"/"rows" shape to begin with. Left as an honest no-op (fresh empty document) pending a real
//! wires<->csv design.
use crate::artifacts::wires::schema::snapshot::WiresSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &CsvSnapshot) -> Result<WiresSnapshot, store::TextError> {
    let _ = (from, STDIO_CSV_DOCUMENT_SCHEMA);
    Ok(crate::artifacts::wires::empty_wires_snapshot())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<WiresSnapshot, store::TextError> {
    <WiresSnapshot as store::ArtifactPack>::decode_pack(bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}
