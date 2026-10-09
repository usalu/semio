//! 🚪️ forms -> json — foreign `Serializer<FormsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Direct `serde_json`
//! serialization of every field, so this hop is `IoFidelity::Exact`.

use crate::FormsSnapshot;
use semio_framework_value::ToValue;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct FormsIntoJson;

impl Serializer<FormsSnapshot> for FormsIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &FormsSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let text = semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&from.to_value()));
        Ok(IoOutcome::clean(IoPayload::Binary(text.into_bytes())))
    }
}
