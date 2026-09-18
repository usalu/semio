//! 📦️ Bitmap artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::snapshot::BitmapSnapshot;
use store::{ErasedSnapshotRetirement, PackError, SnapshotRetirementStep};

/// 📦️ Encodes a `BitmapSnapshot` to its binary pack form.
pub fn encode(document: &BitmapSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `BitmapSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<BitmapSnapshot, PackError> {
    <BitmapSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region ♻️Retirement
/// 🧮️ One retirement step's budget unit — the bytes one displaced member is charged, so a caller
/// with a small `maximum_bytes` makes progress across several turns instead of stalling.
const BITMAP_RETIREMENT_STEP_BYTES: usize = 4_096;

/// ♻️ The three heap-owning members a displaced `BitmapSnapshot` releases, in release order. The
/// base64 index buffer is the largest by orders of magnitude, so it goes first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BitmapRetirementStage {
    Pixels,
    Palette,
    Pinned,
}

impl BitmapRetirementStage {
    const ORDER: [Self; 3] = [Self::Pixels, Self::Palette, Self::Pinned];
}

/// ♻️ A snapshot displaced by a decode-in-place, released in bounded steps rather than dropped in
/// one unbounded `Drop` — the discipline `store::ArtifactStore` asks of every artifact whose decode
/// can overwrite a live projection. `Drop` asserts the terminal state was actually reached, so a
/// caller that abandons a retirement mid-way fails loudly instead of leaking silently.
pub struct BitmapSnapshotRetirement {
    displaced: std::mem::ManuallyDrop<BitmapSnapshot>,
    stage: usize,
}

impl BitmapSnapshotRetirement {
    fn new(displaced: BitmapSnapshot) -> Self {
        Self { displaced: std::mem::ManuallyDrop::new(displaced), stage: 0 }
    }
}

impl ErasedSnapshotRetirement for BitmapSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.stage >= BitmapRetirementStage::ORDER.len() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes < BITMAP_RETIREMENT_STEP_BYTES {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let released = match BitmapRetirementStage::ORDER[self.stage] {
            BitmapRetirementStage::Pixels => std::mem::take(&mut self.displaced.input.pixels).len(),
            BitmapRetirementStage::Palette => std::mem::take(&mut self.displaced.input.palette).len(),
            BitmapRetirementStage::Pinned => std::mem::take(&mut self.displaced.pinned).len(),
        };
        self.stage += 1;
        Ok(SnapshotRetirementStep::Pending { released_items: released.max(1).min(maximum_items), released_bytes: BITMAP_RETIREMENT_STEP_BYTES })
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage >= BitmapRetirementStage::ORDER.len()
    }
}

impl Drop for BitmapSnapshotRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Bitmap snapshot displacement reached Drop before terminal-empty close");
        // ⚠️ `ManuallyDrop` exists so the members above are released by `close_step`, never by an
        // unbounded `Drop`; what is left here is the emptied husk, and dropping it is O(1).
        unsafe { std::mem::ManuallyDrop::drop(&mut self.displaced) };
    }
}

/// 📖️ Decodes into a LIVE projection and hands back the displaced document as a bounded retirement
/// — the only decode entry point a mounted store may call, because it never drops the old snapshot
/// inline.
pub fn decode_into(target: &mut BitmapSnapshot, bytes: &[u8]) -> Result<Box<dyn ErasedSnapshotRetirement>, PackError> {
    let next = decode(bytes)?;
    Ok(Box::new(BitmapSnapshotRetirement::new(std::mem::replace(target, next))))
}
//#endregion ♻️Retirement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `encode`/`decode` speak, named as the schema names the export.
pub type BitmapSnapshotBinary = Vec<u8>;
//#endregion 🚚️Carrier
