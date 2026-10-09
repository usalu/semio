//! 📤️ Foreign leaf — serialize `WriterSnapshot` INTO `s.stdio.json@rfc8259/*`. Full struct fidelity
//! via `serde_json`, then wire-encoded through `JsonSnapshot`'s own `ArtifactDsl` (json is the
//! universal bridge dialect every domain artifact in this repo exports to).

use crate::WriterSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use {semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

//#region 🔖️Serializer
pub struct WriterIntoJson;
impl Serializer<WriterSnapshot> for WriterIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    /// 🪧️ Exact — see the sibling deserializer's doc comment.
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &WriterSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let json = JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(from)));
        Ok(IoOutcome { value: IoPayload::Text(store::ArtifactDsl::print_dsl(&json)), diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
