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
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
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
