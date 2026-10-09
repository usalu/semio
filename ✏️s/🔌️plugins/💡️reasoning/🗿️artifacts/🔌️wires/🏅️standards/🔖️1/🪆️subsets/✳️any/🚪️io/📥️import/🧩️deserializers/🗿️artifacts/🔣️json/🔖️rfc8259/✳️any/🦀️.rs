//! 🚪️ wires <- json — foreign `Deserializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). `json` is this repo's universal
//! bridge dialect: the payload is `WiresSnapshot`'s OWN canonical JSON shape (its
//! `dsl::ToValue`/`dsl::FromValue`), not a lossy foreign-format transform — every field round
//! trips, so `IoFidelity::Exact`.

use crate::{WiresSnapshot, MINDMAP_WIRES_SCHEMA};
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoWires;

impl Deserializer<WiresSnapshot> for JsonIntoWires {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<WiresSnapshot> {
        let _ = MINDMAP_WIRES_SCHEMA;
        let text = match payload {
            IoPayload::Text(text) => text.as_str(),
            IoPayload::Binary(bytes) => std::str::from_utf8(bytes).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("JsonIntoWires: invalid utf-8: {error}"))))?,
        };
        let snapshot: WiresSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(IoError::from_value_error)?;
        Ok(IoOutcome::clean(snapshot))
    }
}
