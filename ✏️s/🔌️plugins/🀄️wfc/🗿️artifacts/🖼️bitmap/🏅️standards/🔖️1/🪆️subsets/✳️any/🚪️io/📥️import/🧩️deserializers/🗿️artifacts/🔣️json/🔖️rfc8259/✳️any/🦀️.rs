//! 🔣️ bitmap ← `s.stdio.json@rfc8259` — the exact inverse of the JSON export leaf.

use crate::BitmapSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 📖️ Typed decode of canonical JSON text into `BitmapSnapshot`.
pub fn deserialize(text: &str) -> Result<BitmapSnapshot, semio_framework_value::ValueError> {
    crate::standards::v1::subsets::any::io::text::bitmap_json_decode::<BitmapSnapshot>(text)
}

pub struct JsonIntoBitmap;

impl Deserializer<BitmapSnapshot> for JsonIntoBitmap {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<BitmapSnapshot> {
        let text = match payload {
            IoPayload::Text(text) => text.clone(),
            IoPayload::Binary(bytes) => String::from_utf8(bytes.clone()).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("bitmap←json: not valid utf-8: {error}"))))?,
        };
        deserialize(&text).map(IoOutcome::clean).map_err(IoError::from_value_error)
    }
}


//#region 🧪Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪Tests
