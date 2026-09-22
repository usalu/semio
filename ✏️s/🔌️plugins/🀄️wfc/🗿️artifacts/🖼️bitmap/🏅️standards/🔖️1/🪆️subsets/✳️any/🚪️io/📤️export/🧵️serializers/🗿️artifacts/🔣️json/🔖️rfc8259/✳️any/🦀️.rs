//! 🔣️ bitmap → `s.stdio.json@rfc8259` — the snapshot in its own canonical JSON projection, the same
//! one the committed fixtures are written in, so a round trip through JSON is exact.

use crate::BitmapSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct BitmapIntoJson;

/// 🖨️ Typed encode of `BitmapSnapshot` into its canonical JSON text.
pub fn serialize(from: &BitmapSnapshot) -> String {
    dsl::json::to_json_string(from)
}

impl Serializer<BitmapSnapshot> for BitmapIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &BitmapSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(serialize(from))))
    }
}


//#region 🧪Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪Tests
