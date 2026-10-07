//! 🚪️ presentation -> json — foreign `Serializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Direct `dsl::os_pack::json`
//! serialization of every field via stdio's own `JsonSnapshot::from_value`/`write_json_pretty`
//! text codec, so this hop is `IoFidelity::Exact`.

use crate::PresentationSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct PresentationIntoJson;

impl Serializer<PresentationSnapshot> for PresentationIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &PresentationSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(from));
        let json = JsonSnapshot::from_value(value);
        Ok(IoOutcome::clean(IoPayload::Binary(write_json_pretty(&json.value).into_bytes())))
    }
}
