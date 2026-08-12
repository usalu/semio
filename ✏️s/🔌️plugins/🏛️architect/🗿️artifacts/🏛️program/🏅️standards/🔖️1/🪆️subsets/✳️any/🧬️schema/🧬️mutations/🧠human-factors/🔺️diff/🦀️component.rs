//! 🔺️ Sparse diff construction for the `human_factors` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateHumanFactorRequirement, DeleteHumanFactorRequirement, RenameHumanFactorRequirement, ReplaceHumanFactorRequirement};
use crate::artifacts::program::diff::{ProgramHumanFactorsDelta, ProgramHumanFactorsPatchEntry};
use crate::artifacts::program::registers::HumanFactorRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.human_factors` on apply.
pub fn diff_create(payload: &CreateHumanFactorRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta { added: vec![payload.human_factor_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteHumanFactorRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameHumanFactorRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = HumanFactorRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta { patched: vec![ProgramHumanFactorsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceHumanFactorRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.human_factors.iter().find(|row| row.header.id == payload.human_factor_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.human_factor_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { human_factors: Some(ProgramHumanFactorsDelta { patched: vec![ProgramHumanFactorsPatchEntry { id: payload.human_factor_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
