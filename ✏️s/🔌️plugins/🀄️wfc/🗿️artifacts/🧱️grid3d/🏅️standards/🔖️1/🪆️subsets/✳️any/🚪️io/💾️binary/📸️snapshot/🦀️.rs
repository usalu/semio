//! 📦️ `s.wfc.grid3d` snapshot — binary document surface + laws (constitutional: pack), plus the
//! bounded RETIREMENT a displaced snapshot is released through.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::snapshot::Grid3dSnapshot;
use store::{ErasedSnapshotRetirement, PackError, SnapshotRetirementStep};

/// 📦️ Encodes a `Grid3dSnapshot` to its binary pack form.
pub fn encode(document: &Grid3dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Grid3dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Grid3dSnapshot, PackError> {
    <Grid3dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region ♻️Retirement
/// 🧮️ One retirement step's budget unit — the bytes one displaced collection is charged, so a caller
/// with a small `maximum_bytes` makes progress across several turns instead of stalling.
const GRID3D_RETIREMENT_STEP_BYTES: usize = 4_096;

/// ♻️ The seven collections a displaced `Grid3dSnapshot` releases, in release order. `tiles` goes
/// FIRST because a tile's media may hold a `store::ArtifactChild` handle into the mesh store, and a
/// child handle outliving the document that named it is the one leak this artifact can have.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Grid3dRetirementStage {
    Tiles,
    Rules,
    Pinned,
    Masked,
    CellSizesX,
    CellSizesY,
    CellSizesZ,
}

impl Grid3dRetirementStage {
    const ORDER: [Self; 7] = [Self::Tiles, Self::Rules, Self::Pinned, Self::Masked, Self::CellSizesX, Self::CellSizesY, Self::CellSizesZ];
}

/// ♻️ A snapshot displaced by a decode-in-place, released in bounded steps rather than dropped in
/// one unbounded `Drop` — the discipline `store::ArtifactStore` asks of every artifact whose decode
/// can overwrite a live projection. `Drop` asserts the terminal state was actually reached, so a
/// caller that abandons a retirement mid-way fails loudly instead of leaking silently.
pub struct Grid3dSnapshotRetirement {
    displaced: std::mem::ManuallyDrop<Grid3dSnapshot>,
    stage: usize,
}

impl Grid3dSnapshotRetirement {
    fn new(displaced: Grid3dSnapshot) -> Self {
        Self { displaced: std::mem::ManuallyDrop::new(displaced), stage: 0 }
    }
}

impl ErasedSnapshotRetirement for Grid3dSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
        if self.stage >= Grid3dRetirementStage::ORDER.len() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes < GRID3D_RETIREMENT_STEP_BYTES {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let released = match Grid3dRetirementStage::ORDER[self.stage] {
            Grid3dRetirementStage::Tiles => std::mem::take(&mut self.displaced.tiles).len(),
            Grid3dRetirementStage::Rules => std::mem::take(&mut self.displaced.rules).len(),
            Grid3dRetirementStage::Pinned => std::mem::take(&mut self.displaced.pinned).len(),
            Grid3dRetirementStage::Masked => std::mem::take(&mut self.displaced.masked).len(),
            Grid3dRetirementStage::CellSizesX => std::mem::take(&mut self.displaced.cell_sizes_x).len(),
            Grid3dRetirementStage::CellSizesY => std::mem::take(&mut self.displaced.cell_sizes_y).len(),
            Grid3dRetirementStage::CellSizesZ => std::mem::take(&mut self.displaced.cell_sizes_z).len(),
        };
        self.stage += 1;
        Ok(SnapshotRetirementStep::Pending { released_items: released.max(1).min(maximum_items), released_bytes: GRID3D_RETIREMENT_STEP_BYTES })
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage >= Grid3dRetirementStage::ORDER.len()
    }
}

impl Drop for Grid3dSnapshotRetirement {
    /// ⚠️ `ManuallyDrop` exists so the members are released by `close_step`, never by an unbounded
    /// `Drop`; what reaches here is the emptied husk, and dropping it is O(1).
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "wfc grid3d snapshot displacement reached Drop before terminal-empty close");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.displaced) };
    }
}

/// 📖️ Decodes into a LIVE projection and hands back the displaced document as a bounded retirement
/// — the only decode entry point a mounted store may call, because it never drops the old snapshot
/// inline.
pub fn decode_into(target: &mut Grid3dSnapshot, bytes: &[u8]) -> Result<Box<dyn ErasedSnapshotRetirement>, PackError> {
    let next = decode(bytes)?;
    Ok(Box::new(Grid3dSnapshotRetirement::new(std::mem::replace(target, next))))
}
//#endregion ♻️Retirement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `encode`/`decode` speak.
pub type Grid3dSnapshotBinary = Vec<u8>;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

impl store::ArtifactPack for Grid3dSnapshot {
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
}
pub use snapshot_codec::*;
