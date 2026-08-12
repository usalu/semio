//! 🔺️ Sparse diff construction for the `requirements` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateRequirement, DeleteRequirement, RenameRequirement, ReplaceRequirement};
use crate::artifacts::program::diff::{ProgramRequirementsDelta, ProgramRequirementsPatchEntry};
use crate::artifacts::program::registers::RequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.requirements` on apply.
pub fn diff_create(payload: &CreateRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { requirements: Some(ProgramRequirementsDelta { added: vec![payload.requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { requirements: Some(ProgramRequirementsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = RequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { requirements: Some(ProgramRequirementsDelta { patched: vec![ProgramRequirementsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.requirements.iter().find(|row| row.header.id == payload.requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { requirements: Some(ProgramRequirementsDelta { patched: vec![ProgramRequirementsPatchEntry { id: payload.requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
