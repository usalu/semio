//! 🔤️ bitmap ← `s.stdio.txt@utf-8` — the exact inverse of the txt export leaf.

use crate::BitmapSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

/// 📖️ Typed decode of native `.wfcbitmap` DSL text into `BitmapSnapshot`.
pub fn deserialize(text: &str) -> Result<BitmapSnapshot, String> {
    <BitmapSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

pub struct TxtIntoBitmap;

impl Deserializer<BitmapSnapshot> for TxtIntoBitmap {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<BitmapSnapshot> {
        let text = match payload {
            IoPayload::Text(text) => text.clone(),
            IoPayload::Binary(bytes) => String::from_utf8(bytes.clone()).map_err(|error| IoError { message: format!("bitmap←txt: not valid utf-8: {error}"), diagnostics: Vec::new() })?,
        };
        deserialize(&text).map(IoOutcome::clean).map_err(|error| IoError { message: error, diagnostics: Vec::new() })
    }
}


//#region 🧪Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪Tests
