//! 🔺️ Sparse diff construction for the `flexibility` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateFlexibilityRequirement, DeleteFlexibilityRequirement, RenameFlexibilityRequirement, ReplaceFlexibilityRequirement};
use crate::artifacts::program::diff::{ProgramFlexibilityDelta, ProgramFlexibilityPatchEntry};
use crate::artifacts::program::registers::FlexibilityRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.flexibility` on apply.
pub fn diff_create(payload: &CreateFlexibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { added: vec![payload.flexibility_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteFlexibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameFlexibilityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = FlexibilityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { patched: vec![ProgramFlexibilityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceFlexibilityRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.flexibility.iter().find(|row| row.header.id == payload.flexibility_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.flexibility_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { patched: vec![ProgramFlexibilityPatchEntry { id: payload.flexibility_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
