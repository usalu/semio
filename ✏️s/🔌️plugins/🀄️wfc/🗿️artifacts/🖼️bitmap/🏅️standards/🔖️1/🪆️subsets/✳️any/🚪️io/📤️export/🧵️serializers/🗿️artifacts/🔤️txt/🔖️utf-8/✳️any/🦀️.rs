//! 🔤️ bitmap → `s.stdio.txt@utf-8` — the document's own `.wfcbitmap` DSL text. This artifact's
//! native serialization already IS utf-8 text, so the hop is exact, not lossy.

use crate::BitmapSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct BitmapIntoTxt;

/// 🖨️ Typed encode of `BitmapSnapshot` into its native `.wfcbitmap` DSL text.
pub fn serialize(from: &BitmapSnapshot) -> String {
    <BitmapSnapshot as store::ArtifactDsl>::print_dsl(from)
}

impl Serializer<BitmapSnapshot> for BitmapIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &BitmapSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(serialize(from))))
    }
}


//#region 🧪Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪Tests
