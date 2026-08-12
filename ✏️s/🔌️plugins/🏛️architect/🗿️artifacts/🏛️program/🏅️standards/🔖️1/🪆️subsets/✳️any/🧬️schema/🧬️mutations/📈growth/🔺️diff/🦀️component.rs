//! 🔺️ Sparse diff construction for the `growth` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateGrowthPlan, DeleteGrowthPlan, RenameGrowthPlan, ReplaceGrowthPlan};
use crate::artifacts::program::diff::{ProgramGrowthDelta, ProgramGrowthPatchEntry};
use crate::artifacts::program::registers::GrowthPlanPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.growth` on apply.
pub fn diff_create(payload: &CreateGrowthPlan, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { growth: Some(ProgramGrowthDelta { added: vec![payload.growth_plan.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteGrowthPlan, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { growth: Some(ProgramGrowthDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameGrowthPlan, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = GrowthPlanPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { growth: Some(ProgramGrowthDelta { patched: vec![ProgramGrowthPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceGrowthPlan, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.growth.iter().find(|row| row.header.id == payload.growth_plan.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.growth_plan).expect("diff_patch always produces a full patch");
    ProgramDiff { growth: Some(ProgramGrowthDelta { patched: vec![ProgramGrowthPatchEntry { id: payload.growth_plan.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
