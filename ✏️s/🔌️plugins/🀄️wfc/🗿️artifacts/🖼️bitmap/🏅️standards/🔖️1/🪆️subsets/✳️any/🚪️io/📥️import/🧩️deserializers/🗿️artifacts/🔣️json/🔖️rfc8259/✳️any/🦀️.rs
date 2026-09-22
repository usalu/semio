//! 🔣️ bitmap ← `s.stdio.json@rfc8259` — the exact inverse of the JSON export leaf.

use crate::BitmapSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 📖️ Typed decode of canonical JSON text into `BitmapSnapshot`.
pub fn deserialize(text: &str) -> Result<BitmapSnapshot, String> {
    dsl::json::from_json_str::<BitmapSnapshot>(text).map_err(|error| error.to_string())
}

pub struct JsonIntoBitmap;

impl Deserializer<BitmapSnapshot> for JsonIntoBitmap {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<BitmapSnapshot> {
        let text = match payload {
            IoPayload::Text(text) => text.clone(),
            IoPayload::Binary(bytes) => String::from_utf8(bytes.clone()).map_err(|error| IoError { message: format!("bitmap←json: not valid utf-8: {error}"), diagnostics: Vec::new() })?,
        };
        deserialize(&text).map(IoOutcome::clean).map_err(|error| IoError { message: error, diagnostics: Vec::new() })
    }
}


//#region 🧪Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪Tests
