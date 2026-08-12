//! program <- csv. `stdio.csv`'s real `CsvSnapshot` shape (`has_header` + `records`) landed
//! after this leaf was first written; the old `headers`/`rows` fields it read never existed on
//! `ProgramSnapshot` either (this always failed deserialization) — lagging call site fixed to
//! match (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W5a
//! closer), same honest no-mapping-exists semantics fem's csv import leaf documents: no CSV
//! grid can reconstruct a ~78-register program artifact, so this returns a structurally valid
//! empty snapshot rather than fabricating one.
use crate::artifacts::program::schema::snapshot::ProgramSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &CsvSnapshot) -> Result<ProgramSnapshot, store::TextError> {
    let _ = (STDIO_CSV_DOCUMENT_SCHEMA, from);
    Ok(ProgramSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ProgramSnapshot, store::TextError> {
    let _ = bytes;
    Ok(ProgramSnapshot::default())
}
