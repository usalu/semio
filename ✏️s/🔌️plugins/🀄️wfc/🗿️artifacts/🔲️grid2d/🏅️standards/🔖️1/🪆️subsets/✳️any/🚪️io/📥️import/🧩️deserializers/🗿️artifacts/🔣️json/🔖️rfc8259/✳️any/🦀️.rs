//! 🔣️ grid2d ← `s.stdio.json@rfc8259` — every field round-trips untouched, so this hop is
//! `IoFidelity::Exact`.

use crate::Grid2dSnapshot;
use dsl::FromValue;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 📖️ Typed decode of a `JsonSnapshot`'s free-form `value` into `Grid2dSnapshot`'s own field shape.
pub fn deserialize(from: &JsonSnapshot) -> Result<Grid2dSnapshot, store::TextError> {
    Grid2dSnapshot::from_value(dsl::json::to_dsl_value(&from.to_pack_value())).map_err(|error| store::TextError::new(format!("grid2d<-json: {error}"), dsl::TextSpan::at(1, 1)))
}

pub struct JsonIntoGrid2d;

impl Deserializer<Grid2dSnapshot> for JsonIntoGrid2d {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<Grid2dSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "JsonIntoGrid2d: expected a binary json payload".to_string(), diagnostics: Vec::new() });
        };
        let json = <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("JsonIntoGrid2d: json decode failed: {error}"), diagnostics: Vec::new() })?;
        let snapshot = deserialize(&json).map_err(|error| IoError { message: format!("JsonIntoGrid2d: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
