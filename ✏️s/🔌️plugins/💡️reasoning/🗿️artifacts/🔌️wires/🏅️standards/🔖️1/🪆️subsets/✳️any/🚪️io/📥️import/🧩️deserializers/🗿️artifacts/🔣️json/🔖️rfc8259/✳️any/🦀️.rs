//! 🚪️ wires <- json — foreign `Deserializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). `json` is this repo's universal
//! bridge dialect: the payload is `WiresSnapshot`'s OWN canonical JSON shape (its
//! `#[derive(Serialize, Deserialize)]`), not a lossy foreign-format transform — every field round
//! trips, so `IoFidelity::Exact`.

use crate::artifacts::wires::{WiresSnapshot, MINDMAP_WIRES_SCHEMA};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoWires;

impl Deserializer<WiresSnapshot> for JsonIntoWires {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    fn deserialize(payload: &IoPayload) -> IoResult<WiresSnapshot> {
        let _ = MINDMAP_WIRES_SCHEMA;
        let text = match payload {
            IoPayload::Text(text) => text.clone(),
            IoPayload::Binary(bytes) => std::str::from_utf8(bytes).map_err(|error| IoError { message: format!("JsonIntoWires: invalid utf-8: {error}"), diagnostics: Vec::new() })?.to_string(),
        };
        let value: serde_json::Value = serde_json::from_str(&text).map_err(|error| IoError { message: format!("JsonIntoWires: json parse failed: {error}"), diagnostics: Vec::new() })?;
        let snapshot: WiresSnapshot = serde_json::from_value(JsonSnapshot::from_value(value).to_serde_value()).map_err(|error| IoError { message: format!("JsonIntoWires: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
