//! 🚪️ sequence <- json — foreign `Deserializer<SequenceSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Every `SequenceSnapshot`
//! field round-trips through `serde_json` untouched, so this hop is `IoFidelity::Exact`.

use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoSequence;

impl Deserializer<SequenceSnapshot> for JsonIntoSequence {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<SequenceSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "JsonIntoSequence: expected a binary json payload".to_string(), diagnostics: Vec::new() });
        };
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let text = std::str::from_utf8(bytes).map_err(|error| IoError { message: format!("JsonIntoSequence: not valid utf-8: {error}"), diagnostics: Vec::new() })?;
        let value: serde_json::Value = serde_json::from_str(text).map_err(|error| IoError { message: format!("JsonIntoSequence: not valid json: {error}"), diagnostics: Vec::new() })?;
        let json = JsonSnapshot::from_value(value);
        let snapshot: SequenceSnapshot = serde_json::from_value(json.to_serde_value()).map_err(|error| IoError { message: format!("JsonIntoSequence: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
