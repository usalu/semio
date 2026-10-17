//! 🔤️ bitmap → `framework.io.text@utf-8` — the document's own `.wfcbitmap` DSL text. This artifact's
//! native serialization already IS utf-8 text, so the hop is exact, not lossy.

use crate::BitmapSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};

pub const TXT_DIALECT: Dialect = semio_framework::io_schema::CARRIER_TEXT;

pub struct BitmapIntoTxt;

/// 🖨️ Typed encode of `BitmapSnapshot` into its native `.wfcbitmap` DSL text.
pub fn serialize(from: &BitmapSnapshot) -> String {
    <BitmapSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(from)
}

impl Serializer<BitmapSnapshot> for BitmapIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &BitmapSnapshot, _: &ArchiveChildren, _control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(serialize(from))))
    }
}


//#region 🧪Tests

//#endregion 🧪Tests
