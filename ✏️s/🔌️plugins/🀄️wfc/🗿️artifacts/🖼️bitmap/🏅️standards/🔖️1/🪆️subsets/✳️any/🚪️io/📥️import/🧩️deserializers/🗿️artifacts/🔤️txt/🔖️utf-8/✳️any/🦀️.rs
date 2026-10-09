//! 🔤️ bitmap ← `s.stdio.txt@utf-8` — the exact inverse of the txt export leaf.

use crate::BitmapSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

/// 📖️ Typed decode of native `.wfcbitmap` DSL text into `BitmapSnapshot`.
pub fn deserialize(text: &str) -> Result<BitmapSnapshot, semio_framework_diagnostic::TextError> {
    <BitmapSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

pub struct TxtIntoBitmap;

impl Deserializer<BitmapSnapshot> for TxtIntoBitmap {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<BitmapSnapshot> {
        let text = match payload {
            IoPayload::Text(text) => text.clone(),
            IoPayload::Binary(bytes) => String::from_utf8(bytes.clone()).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("bitmap←txt: not valid utf-8: {error}"))))?,
        };
        deserialize(&text).map(IoOutcome::clean).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(error.kind, error.to_string())))
    }
}


//#region 🧪Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪Tests
