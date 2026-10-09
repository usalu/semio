//! 🔣️ grid2d → `s.stdio.json@rfc8259` — direct typed encode of every field, so this hop is
//! `IoFidelity::Exact`.

use crate::Grid2dSnapshot;
use semio_framework_value::ToValue;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🖨️ Typed encode of `Grid2dSnapshot` into a `JsonSnapshot`'s free-form `value`.
pub fn serialize(from: &Grid2dSnapshot) -> JsonSnapshot {
    JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&from.to_value()))
}

pub struct Grid2dIntoJson;

impl Serializer<Grid2dSnapshot> for Grid2dIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Grid2dSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let json = serialize(from);
        let bytes = <JsonSnapshot as store::ArtifactPack>::encode_pack(&json);
        if bytes.is_empty() {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Grid2dIntoJson: empty json pack")));
        }
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
