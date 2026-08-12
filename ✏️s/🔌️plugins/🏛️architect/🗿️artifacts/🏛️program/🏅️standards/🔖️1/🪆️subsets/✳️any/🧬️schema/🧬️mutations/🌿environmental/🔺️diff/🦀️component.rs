//! 🔺️ Sparse diff construction for the `environmental` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateEnvironmentalRequirement, DeleteEnvironmentalRequirement, RenameEnvironmentalRequirement, ReplaceEnvironmentalRequirement};
use crate::artifacts::program::diff::{ProgramEnvironmentalDelta, ProgramEnvironmentalPatchEntry};
use crate::artifacts::program::registers::EnvironmentalRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.environmental` on apply.
pub fn diff_create(payload: &CreateEnvironmentalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { added: vec![payload.environmental_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteEnvironmentalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameEnvironmentalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = EnvironmentalRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { patched: vec![ProgramEnvironmentalPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceEnvironmentalRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.environmental.iter().find(|row| row.header.id == payload.environmental_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.environmental_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { patched: vec![ProgramEnvironmentalPatchEntry { id: payload.environmental_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
