//! 📥️ Foreign leaf — deserialize `WriterSnapshot` FROM `s.stdio.json@rfc8259/*`. JSON's native
//! `Text` `IoPayload` is its own DSL text (`JsonSnapshot::parse_dsl`); the resulting `serde_json`
//! value structurally IS a `WriterSnapshot` (json is the universal bridge dialect every domain
//! artifact in this repo imports from), so this is a direct `serde_json::from_value`, not a
//! text-content projection like the prose formats.

use crate::artifacts::writer::WriterSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

//#region 🔖️Deserializer
pub struct JsonIntoWriter;
impl Deserializer<WriterSnapshot> for JsonIntoWriter {
    const FROM: Dialect = JSON_DIALECT;
    /// 🪧️ Exact: `serde_json` round-trips every `WriterSnapshot` field, including the `document`
    /// child handle — the same fidelity tier the native pack/dsl codecs themselves declare (neither
    /// restores the ephemeral working-scene text cache; that is a documented, orthogonal gap, not a
    /// json-specific loss).
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<WriterSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "JsonIntoWriter: expected a text payload".to_string(), diagnostics: Vec::new() });
        };
        let json = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| IoError { message: format!("JsonIntoWriter: {error}"), diagnostics: Vec::new() })?;
        let snapshot: WriterSnapshot = serde_json::from_value(json.to_serde_value()).map_err(|error| IoError { message: format!("JsonIntoWriter: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome { value: snapshot, diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Deserializer

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn json_into_writer_round_trips_a_real_snapshot() {
        let original = crate::artifacts::writer::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello");
        let text = serde_json::to_string(&original).expect("serialize");
        let outcome = JsonIntoWriter::deserialize(&IoPayload::Text(text)).expect("deserialize");
        assert_eq!(outcome.value, original);
    }
}
