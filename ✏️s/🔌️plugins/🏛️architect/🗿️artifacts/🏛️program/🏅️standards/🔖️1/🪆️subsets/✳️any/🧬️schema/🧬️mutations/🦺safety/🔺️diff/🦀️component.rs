//! 🔺️ Sparse diff construction for the `safety` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateSafetyRequirement, DeleteSafetyRequirement, RenameSafetyRequirement, ReplaceSafetyRequirement};
use crate::artifacts::program::diff::{ProgramSafetyDelta, ProgramSafetyPatchEntry};
use crate::artifacts::program::registers::SafetyRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.safety` on apply.
pub fn diff_create(payload: &CreateSafetyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { safety: Some(ProgramSafetyDelta { added: vec![payload.safety_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteSafetyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { safety: Some(ProgramSafetyDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameSafetyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = SafetyRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { safety: Some(ProgramSafetyDelta { patched: vec![ProgramSafetyPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceSafetyRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.safety.iter().find(|row| row.header.id == payload.safety_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.safety_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { safety: Some(ProgramSafetyDelta { patched: vec![ProgramSafetyPatchEntry { id: payload.safety_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
