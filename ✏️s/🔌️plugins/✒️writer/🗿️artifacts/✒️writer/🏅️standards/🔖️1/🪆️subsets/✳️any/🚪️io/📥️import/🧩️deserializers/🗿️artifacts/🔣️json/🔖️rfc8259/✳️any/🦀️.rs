//! 📥️ Foreign leaf — deserialize `WriterSnapshot` FROM `s.stdio.json@rfc8259/*`. JSON's native
//! `Text` `IoPayload` is its own DSL text (`JsonSnapshot::parse_dsl`); the resulting json VALUE
//! structurally IS a `WriterSnapshot` (json is the universal bridge dialect every domain artifact
//! in this repo imports from), so this is a direct value decode, not a text-content projection
//! like the prose formats.

use crate::WriterSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_json::JsonSnapshot;

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
        // 🧾️ `print_dsl` is the s.stdio.json ARTIFACT text — a `semio stdio.json.dsl v1` preamble line
        // followed by the body — so feeding it back to a JSON reader fails on its very first byte
        // (`unexpected byte 115 at offset 0`, the `s` of `semio`). The document this leaf imports is
        // the snapshot's VALUE, which `to_pack_value` hands over as first-party JSON.
        let snapshot: WriterSnapshot = dsl::os_pack::json::from_json_str(&dsl::os_pack::json::to_string(&json.to_pack_value())).map_err(|error| IoError { message: format!("JsonIntoWriter: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome { value: snapshot, diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Deserializer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
