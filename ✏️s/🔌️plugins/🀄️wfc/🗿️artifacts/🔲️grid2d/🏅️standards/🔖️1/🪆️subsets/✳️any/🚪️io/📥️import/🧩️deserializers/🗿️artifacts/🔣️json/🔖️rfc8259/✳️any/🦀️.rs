//! 🔣️ grid2d ← `s.stdio.json@rfc8259` — every field round-trips untouched, so this hop is
//! `IoFidelity::Exact`.

use crate::Grid2dSnapshot;
use semio_framework_value::FromValue;
use semio_framework::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 📖️ Typed decode of a `JsonSnapshot`'s free-form `value` into `Grid2dSnapshot`'s own field shape.
pub fn deserialize(from: &JsonSnapshot) -> Result<Grid2dSnapshot, semio_framework_diagnostic::TextError> {
    Grid2dSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&from.to_pack_value())).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, format!("grid2d<-json: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

pub struct JsonIntoGrid2d;

impl Deserializer<Grid2dSnapshot> for JsonIntoGrid2d {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<Grid2dSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "JsonIntoGrid2d: expected a binary json payload")));
        };
        let json = <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| match error { store::PackError::Refusal(store::PackRefusal::TextRefusal(error)) => text_refusal(error), error => IoError::from_value_error(match error.into_value_error() { Ok(error) => error.under("JsonIntoGrid2d: json decode"), Err(error) => semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, format!("JsonIntoGrid2d: json decode: in-memory decode reported a transport failure: {error}")) }) })?;
        let snapshot = deserialize(&json).map_err(text_refusal)?;
        Ok(IoOutcome::clean(snapshot))
    }
}

/// 📍️ Retains the complete canonical text refusal and source diagnostic.
fn text_refusal(error: semio_framework_diagnostic::TextError) -> IoError {
    let mut callback = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(isize::MAX as usize, &mut callback);
    IoError::from_text_error_controlled(error, &mut control).unwrap_or_else(IoError::from_value_error)
}
