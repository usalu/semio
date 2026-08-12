//! 🔺️ Sparse diff construction for the `equipment` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateEquipment, DeleteEquipment, RenameEquipment, ReplaceEquipment};
use crate::artifacts::program::diff::{ProgramEquipmentDelta, ProgramEquipmentPatchEntry};
use crate::artifacts::program::registers::EquipmentPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.equipment` on apply.
pub fn diff_create(payload: &CreateEquipment, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { equipment: Some(ProgramEquipmentDelta { added: vec![payload.equipment.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteEquipment, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { equipment: Some(ProgramEquipmentDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameEquipment, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = EquipmentPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { equipment: Some(ProgramEquipmentDelta { patched: vec![ProgramEquipmentPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceEquipment, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.equipment.iter().find(|row| row.header.id == payload.equipment.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.equipment).expect("diff_patch always produces a full patch");
    ProgramDiff { equipment: Some(ProgramEquipmentDelta { patched: vec![ProgramEquipmentPatchEntry { id: payload.equipment.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
