//! 🔣️ grid2d → `s.stdio.json@rfc8259` — direct typed encode of every field, so this hop is
//! `IoFidelity::Exact`.

use crate::Grid2dSnapshot;
use dsl::ToValue;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🖨️ Typed encode of `Grid2dSnapshot` into a `JsonSnapshot`'s free-form `value`.
pub fn serialize(from: &Grid2dSnapshot) -> JsonSnapshot {
    JsonSnapshot::from_value(dsl::json::from_dsl_value(&from.to_value()))
}

pub struct Grid2dIntoJson;

impl Serializer<Grid2dSnapshot> for Grid2dIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Grid2dSnapshot) -> IoResult<IoPayload> {
        let json = serialize(from);
        let bytes = <JsonSnapshot as store::ArtifactPack>::encode_pack(&json);
        if bytes.is_empty() {
            return Err(IoError { message: "Grid2dIntoJson: empty json pack".to_string(), diagnostics: Vec::new() });
        }
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
