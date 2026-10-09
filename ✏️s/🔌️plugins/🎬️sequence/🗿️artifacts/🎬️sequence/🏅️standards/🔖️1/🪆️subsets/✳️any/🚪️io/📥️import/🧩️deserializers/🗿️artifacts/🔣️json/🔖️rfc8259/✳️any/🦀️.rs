//! 🚪️ sequence <- json. The exact fixture rebuilds the composed content child and local owner.

use crate::{SequenceHostSnapshot, SequenceSnapshot};
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoSequence;

impl Deserializer<SequenceSnapshot> for JsonIntoSequence {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<SequenceSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, "JsonIntoSequence: expected a binary json payload".to_string())));
        };
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let text = std::str::from_utf8(bytes).map_err(|error| IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, format!("JsonIntoSequence: {error}"))))?;
        let fixture: SequenceHostSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| IoError::from_value_error(error.under("JsonIntoSequence")))?;
        Ok(IoOutcome::clean(SequenceSnapshot::from_host_snapshot(fixture)))
    }
}
