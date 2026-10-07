//! 🚪️ wires -> json — foreign `Serializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Symmetric with the sibling
//! `Deserializer`: emits `WiresSnapshot`'s own canonical JSON shape verbatim, so `IoFidelity::Exact`.

use crate::WiresSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct WiresIntoJson;

impl Serializer<WiresSnapshot> for WiresIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &WiresSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(from));
        let text = semio_framework_pack_json::to_string_pretty(&value);
        Ok(IoOutcome::clean(IoPayload::Text(text)))
    }
}
