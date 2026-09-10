//! 📦️ Assembly artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::snapshot::AssemblySnapshot;
use store::{ErasedSnapshotRetirement, PackError, SnapshotRetirementStep};

/// 📦️ Encodes an `AssemblySnapshot` to its binary pack form.
pub fn encode(document: &AssemblySnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes an `AssemblySnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<AssemblySnapshot, PackError> {
    <AssemblySnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region ♻️Retirement
/// 🧮️ One retirement step's budget unit — the bytes one displaced collection is charged, so a
/// caller with a small `maximum_bytes` makes progress across several turns instead of stalling.
const ASSEMBLY_RETIREMENT_STEP_BYTES: usize = 4_096;

/// ♻️ The five collections a displaced `AssemblySnapshot` releases, in release order. `modules`
/// holds `store::ArtifactChild` handles into the kit store, so it is released FIRST: a child handle
/// outliving the document that named it is the one leak this artifact can actually have.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AssemblyRetirementStage {
    Modules,
    Rules,
    Weights,
    Edges,
    Slots,
}

impl AssemblyRetirementStage {
    const ORDER: [Self; 5] = [Self::Modules, Self::Rules, Self::Weights, Self::Edges, Self::Slots];
}

/// ♻️ A snapshot displaced by a decode-in-place, released in bounded steps rather than dropped in
/// one unbounded `Drop` — the discipline `store::ArtifactStore` asks of every artifact whose
/// decode can overwrite a live projection. `Drop` asserts the terminal state was actually reached,
/// so a caller that abandons a retirement mid-way fails loudly instead of leaking silently.
pub struct AssemblySnapshotRetirement {
    displaced: std::mem::ManuallyDrop<AssemblySnapshot>,
    stage: usize,
}

impl AssemblySnapshotRetirement {
    fn new(displaced: AssemblySnapshot) -> Self {
        Self { displaced: std::mem::ManuallyDrop::new(displaced), stage: 0 }
    }
}

impl ErasedSnapshotRetirement for AssemblySnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.stage >= AssemblyRetirementStage::ORDER.len() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes < ASSEMBLY_RETIREMENT_STEP_BYTES {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let released = match AssemblyRetirementStage::ORDER[self.stage] {
            AssemblyRetirementStage::Modules => std::mem::take(&mut self.displaced.modules).len(),
            AssemblyRetirementStage::Rules => std::mem::take(&mut self.displaced.rules).len(),
            AssemblyRetirementStage::Weights => std::mem::take(&mut self.displaced.weights).len(),
            AssemblyRetirementStage::Edges => std::mem::take(&mut self.displaced.edges).len(),
            AssemblyRetirementStage::Slots => std::mem::take(&mut self.displaced.slots).len(),
        };
        self.stage += 1;
        Ok(SnapshotRetirementStep::Pending { released_items: released.max(1).min(maximum_items), released_bytes: ASSEMBLY_RETIREMENT_STEP_BYTES })
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage >= AssemblyRetirementStage::ORDER.len()
    }
}

impl Drop for AssemblySnapshotRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Assembly snapshot displacement reached Drop before terminal-empty close");
        // ⚠️ `ManuallyDrop` exists so the collections above are released by `close_step`, never by an
        // unbounded `Drop`; what is left here is the emptied husk, and dropping it is O(1).
        unsafe { std::mem::ManuallyDrop::drop(&mut self.displaced) };
    }
}

/// 📖️ Decodes into a LIVE projection and hands back the displaced document as a bounded retirement
/// — the only decode entry point a mounted store may call, because it never drops the old snapshot
/// inline.
pub fn decode_into(target: &mut AssemblySnapshot, bytes: &[u8]) -> Result<Box<dyn ErasedSnapshotRetirement>, PackError> {
    let next = decode(bytes)?;
    Ok(Box::new(AssemblySnapshotRetirement::new(std::mem::replace(target, next))))
}
//#endregion ♻️Retirement

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `encode`/`decode` speak, named as the schema names the export.
pub type AssemblySnapshotBinary = Vec<u8>;
//#endregion 🚚️Carrier
