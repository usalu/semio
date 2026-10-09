//! 🚪️ forms <- json — foreign `Deserializer<FormsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Every `FormsSnapshot` field
//! (including its composed `structure`/`results` child handles) round-trips through `serde_json`
//! untouched — the same guarantee the native codec itself gives — so this hop is `IoFidelity::Exact`.

use crate::FormsSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoForms;

impl Deserializer<FormsSnapshot> for JsonIntoForms {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<FormsSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"JsonIntoForms: expected a binary json payload")));
        };
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let text = std::str::from_utf8(bytes).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("JsonIntoForms: not valid utf-8: {error}"))))?;
        let snapshot = semio_framework_pack_json::from_json_str::<FormsSnapshot>(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(IoError::from_value_error)?;
        Ok(IoOutcome::clean(snapshot))
    }
}
