//! 🚪️ vcs <- json — foreign `Deserializer<VcsSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-
//! SUBSET-MECHANISM design.md §3). Bridges via json's own text codec (`parse_json_text`) then a
//! genuine `pack::JsonValue -> DslValue -> VcsSnapshot` structural deserialize (ticket
//! 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS — no `serde_json` left in
//! this hop), so this hop is `IoFidelity::Exact`.

use crate::VcsSnapshot;
use semio_framework_value::FromValue;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub fn deserialize(from: &JsonSnapshot) -> Result<VcsSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    VcsSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&from.to_pack_value())).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, format!("vcs<-json: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<VcsSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}

pub struct JsonIntoVcs;

impl Deserializer<VcsSnapshot> for JsonIntoVcs {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<VcsSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "JsonIntoVcs: expected a binary json payload".to_string())));
        };
        deserialize_bytes(bytes).map(IoOutcome::clean).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("JsonIntoVcs: {error}"))))
    }
}
