//! 📦️ `wfc3d` artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::snapshot::Wfc3dSnapshot;
use store::{ErasedSnapshotRetirement, PackError, SnapshotRetirementStep};

/// 📦️ Encodes a `Wfc3dSnapshot` to its binary pack form.
pub fn encode(document: &Wfc3dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Wfc3dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Wfc3dSnapshot, PackError> {
    <Wfc3dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region ♻️Retirement
/// 🧮️ One retirement step's budget unit — the bytes one displaced collection is charged, so a
/// caller with a small `maximum_bytes` makes progress across several turns instead of stalling.
const WFC3D_RETIREMENT_STEP_BYTES: usize = 4_096;

/// ♻️ The four collections a displaced `Wfc3dSnapshot` releases, in release order. `tiles` holds the
/// media — including any `store::ArtifactChild` handle into the mesh store — so it is released
/// FIRST: a child handle outliving the document that named it is the one leak this artifact can
/// actually have.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Wfc3dRetirementStage {
    Tiles,
    Rules,
    Edges,
    Slots,
}

impl Wfc3dRetirementStage {
    const ORDER: [Self; 4] = [Self::Tiles, Self::Rules, Self::Edges, Self::Slots];
}

/// ♻️ A snapshot displaced by a decode-in-place, released in bounded steps rather than dropped in
/// one unbounded `Drop` — the discipline `store::ArtifactStore` asks of every artifact whose decode
/// can overwrite a live projection. `Drop` asserts the terminal state was actually reached, so a
/// caller that abandons a retirement mid-way fails loudly instead of leaking silently.
pub struct Wfc3dSnapshotRetirement {
    displaced: std::mem::ManuallyDrop<Wfc3dSnapshot>,
    stage: usize,
}

impl Wfc3dSnapshotRetirement {
    fn new(displaced: Wfc3dSnapshot) -> Self {
        Self { displaced: std::mem::ManuallyDrop::new(displaced), stage: 0 }
    }
}

impl ErasedSnapshotRetirement for Wfc3dSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
        if self.stage >= Wfc3dRetirementStage::ORDER.len() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes < WFC3D_RETIREMENT_STEP_BYTES {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let released = match Wfc3dRetirementStage::ORDER[self.stage] {
            Wfc3dRetirementStage::Tiles => std::mem::take(&mut self.displaced.tiles).len(),
            Wfc3dRetirementStage::Rules => std::mem::take(&mut self.displaced.rules).len(),
            Wfc3dRetirementStage::Edges => std::mem::take(&mut self.displaced.edges).len(),
            Wfc3dRetirementStage::Slots => std::mem::take(&mut self.displaced.slots).len(),
        };
        self.stage += 1;
        Ok(SnapshotRetirementStep::Pending { released_items: released.max(1).min(maximum_items), released_bytes: WFC3D_RETIREMENT_STEP_BYTES })
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage >= Wfc3dRetirementStage::ORDER.len()
    }
}

/// ⚠️ `ManuallyDrop` exists so the collections above are released by `close_step`, never by an
/// unbounded `Drop`; what reaches this impl is the emptied husk, and dropping it is O(1). The assert
/// is what turns an abandoned retirement into a loud failure instead of a silent leak.
impl Drop for Wfc3dSnapshotRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "wfc3d snapshot displacement reached Drop before terminal-empty close");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.displaced) };
    }
}

/// 📖️ Decodes into a LIVE projection and hands back the displaced document as a bounded retirement —
/// the only decode entry point a mounted store may call, because it never drops the old snapshot
/// inline.
pub fn decode_into(target: &mut Wfc3dSnapshot, bytes: &[u8]) -> Result<Box<dyn ErasedSnapshotRetirement>, PackError> {
    let next = decode(bytes)?;
    Ok(Box::new(Wfc3dSnapshotRetirement::new(std::mem::replace(target, next))))
}
//#endregion ♻️Retirement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `encode`/`decode` speak, named as the schema names the export.
pub type Wfc3dSnapshotBinary = Vec<u8>;
//#endregion 🚚️Carrier

mod native_codec {
use super::*;
use crate::schema::snapshot::{GraphRule, Slot3d, SlotEdge, Tile, TileMedia3d, Wfc3dSnapshot, WFC3D_DOCUMENT_SCHEMA};
pub(crate)use controlled_native::{decode_sqlite_snapshot_native,encode_sqlite_snapshot_native};
use crate::standards::v1::subsets::any::io::text::snapshot::{Wfc3dSnapshotDsl,wfc3d_document_to_dsl,wfc3d_document_from_dsl};

impl store::ArtifactPack for Wfc3dSnapshotDsl {
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

impl store::ArtifactPack for Wfc3dSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Wfc3dSnapshotDsl as store::ArtifactPack>::encode_pack_with(&wfc3d_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Wfc3dSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        wfc3d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Wfc3dSnapshotDsl as store::ArtifactPack>::record_spec()
    }
}
}
