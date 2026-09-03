//! 🚪️ vcs <- json — foreign `Deserializer<VcsSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-
//! SUBSET-MECHANISM design.md §3). Bridges via json's own text codec (`parse_json_text`) then a
//! genuine `pack::JsonValue -> DslValue -> VcsSnapshot` structural deserialize (ticket
//! 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS — no `serde_json` left in
//! this hop), so this hop is `IoFidelity::Exact`.

use crate::artifacts::vcs::VcsSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub fn deserialize(from: &JsonSnapshot) -> Result<VcsSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    VcsSnapshot::from_value(dsl::json::to_dsl_value(&from.to_pack_value())).map_err(|error| store::TextError::new(format!("vcs<-json: {error}"), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<VcsSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}

pub struct JsonIntoVcs;

impl Deserializer<VcsSnapshot> for JsonIntoVcs {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<VcsSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "JsonIntoVcs: expected a binary json payload".to_string(), diagnostics: Vec::new() });
        };
        deserialize_bytes(bytes).map(IoOutcome::clean).map_err(|error| IoError { message: format!("JsonIntoVcs: {error}"), diagnostics: Vec::new() })
    }
}
