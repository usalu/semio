//! 🔺️ Sparse diff construction for the `rename-equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛠️equipment` per Wave C.

use super::mutation::RenameEquipment;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramEquipmentDelta, ProgramEquipmentPatchEntry};
use crate::artifacts::program::registers::EquipmentPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameEquipment, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = EquipmentPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { equipment: Some(ProgramEquipmentDelta { patched: vec![ProgramEquipmentPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
