//! 🔣️ bitmap → `s.stdio.json@rfc8259` — the snapshot in its own canonical JSON projection, the same
//! one the committed fixtures are written in, so a round trip through JSON is exact.

use crate::BitmapSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct BitmapIntoJson;

/// 🖨️ Typed encode of `BitmapSnapshot` into its canonical JSON text.
pub fn serialize(from: &BitmapSnapshot) -> String {
    crate::standards::v1::subsets::any::io::text::bitmap_json_encode(from)
}

impl Serializer<BitmapSnapshot> for BitmapIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &BitmapSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(serialize(from))))
    }
}


//#region 🧪Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪Tests
