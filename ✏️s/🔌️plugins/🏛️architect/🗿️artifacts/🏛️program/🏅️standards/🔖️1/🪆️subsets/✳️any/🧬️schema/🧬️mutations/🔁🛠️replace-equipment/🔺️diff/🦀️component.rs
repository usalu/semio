//! 🔺️ Sparse diff construction for the `replace-equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛠️equipment` per Wave C.

use super::mutation::ReplaceEquipment;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramEquipmentDelta, ProgramEquipmentPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceEquipment, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.equipment.iter().find(|row| row.header.id == payload.equipment.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.equipment).expect("diff_patch always produces a full patch");
    ProgramDiff { equipment: Some(ProgramEquipmentDelta { patched: vec![ProgramEquipmentPatchEntry { id: payload.equipment.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
