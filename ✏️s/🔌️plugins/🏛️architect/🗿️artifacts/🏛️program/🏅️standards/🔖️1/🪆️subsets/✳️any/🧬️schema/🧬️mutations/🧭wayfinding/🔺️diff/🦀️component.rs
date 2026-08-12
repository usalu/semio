//! 🔺️ Sparse diff construction for the `wayfinding` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateWayfindingRequirement, DeleteWayfindingRequirement, RenameWayfindingRequirement, ReplaceWayfindingRequirement};
use crate::artifacts::program::diff::{ProgramWayfindingDelta, ProgramWayfindingPatchEntry};
use crate::artifacts::program::registers::WayfindingRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.wayfinding` on apply.
pub fn diff_create(payload: &CreateWayfindingRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { wayfinding: Some(ProgramWayfindingDelta { added: vec![payload.wayfinding_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteWayfindingRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { wayfinding: Some(ProgramWayfindingDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameWayfindingRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = WayfindingRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { wayfinding: Some(ProgramWayfindingDelta { patched: vec![ProgramWayfindingPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceWayfindingRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.wayfinding.iter().find(|row| row.header.id == payload.wayfinding_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.wayfinding_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { wayfinding: Some(ProgramWayfindingDelta { patched: vec![ProgramWayfindingPatchEntry { id: payload.wayfinding_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
