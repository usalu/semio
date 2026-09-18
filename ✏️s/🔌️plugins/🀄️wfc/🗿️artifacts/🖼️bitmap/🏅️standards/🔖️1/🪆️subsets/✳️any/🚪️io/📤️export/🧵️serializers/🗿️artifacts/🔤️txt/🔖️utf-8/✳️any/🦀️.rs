//! 🔤️ bitmap → `s.stdio.txt@utf-8` — the document's own `.wfcbitmap` DSL text. This artifact's
//! native serialization already IS utf-8 text, so the hop is exact, not lossy.

use crate::BitmapSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct BitmapIntoTxt;

impl Serializer<BitmapSnapshot> for BitmapIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &BitmapSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(<BitmapSnapshot as store::ArtifactDsl>::print_dsl(from))))
    }
}
