//! 🚪️ present <- json — foreign `Deserializer<PresentSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Every `PresentSnapshot` field
//! round-trips through `serde_json` untouched (via stdio's own `JsonSnapshot::to_serde_value()`
//! bridge), so this hop is `IoFidelity::Exact`.

use crate::artifacts::present::{PresentSnapshot, PRESENT_DOCUMENT_SCHEMA};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoPresent;

impl Deserializer<PresentSnapshot> for JsonIntoPresent {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<PresentSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "JsonIntoPresent: expected a binary json payload".to_string(), diagnostics: Vec::new() });
        };
        let text = std::str::from_utf8(bytes).map_err(|error| IoError { message: format!("JsonIntoPresent: not valid utf-8: {error}"), diagnostics: Vec::new() })?;
        let value = parse_json_text(text).map_err(|error| IoError { message: format!("JsonIntoPresent: {error}"), diagnostics: Vec::new() })?;
        let json = JsonSnapshot::from_value(value);
        let mut out: PresentSnapshot = serde_json::from_value(json.to_serde_value()).map_err(|error| IoError { message: format!("JsonIntoPresent: {error}"), diagnostics: Vec::new() })?;
        if out.schema.is_empty() {
            out.schema = PRESENT_DOCUMENT_SCHEMA.into();
        }
        Ok(IoOutcome::clean(out).await)
    }
}
