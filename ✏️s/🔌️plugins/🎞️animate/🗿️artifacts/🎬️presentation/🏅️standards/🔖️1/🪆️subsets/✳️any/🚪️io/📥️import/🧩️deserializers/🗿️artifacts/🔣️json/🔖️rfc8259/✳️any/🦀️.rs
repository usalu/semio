//! 🚪️ presentation <- json — foreign `Deserializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Every `PresentationSnapshot` field
//! round-trips through `serde_json` untouched (via stdio's own `JsonSnapshot::to_serde_value()`
//! bridge), so this hop is `IoFidelity::Exact`.

use crate::{PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoPresentation;

impl Deserializer<PresentationSnapshot> for JsonIntoPresentation {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<PresentationSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"JsonIntoPresentation: expected a binary json payload")));
        };
        let text = std::str::from_utf8(bytes).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("JsonIntoPresentation: not valid utf-8: {error}"))))?;
        let value = parse_json_text(text).map_err(|mut error| { error.message=format!("JsonIntoPresentation: {}",error.message);IoError::from_text_error_controlled(error,&mut semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut |_|true)).unwrap_or_else(IoError::from_value_error) })?;
        let json = JsonSnapshot::from_value(value);
        let dsl_value: semio_framework_value::DslValue = json.to_serde_value().into();
        let mut out: PresentationSnapshot = semio_framework_value::FromValue::from_value(dsl_value).map_err(|error| { let mut cause=error;cause.message=format!("JsonIntoPresentation: {}",cause.message);IoError::from_value_error(cause) })?;
        if out.schema.is_empty() {
            out.schema = PRESENTATION_DOCUMENT_SCHEMA.into();
        }
        Ok(IoOutcome::clean(out))
    }
}
