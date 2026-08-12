//! 🔺️ Sparse diff construction for the `performance` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreatePerformanceCriterion, DeletePerformanceCriterion, RenamePerformanceCriterion, ReplacePerformanceCriterion};
use crate::artifacts::program::diff::{ProgramPerformanceDelta, ProgramPerformancePatchEntry};
use crate::artifacts::program::registers::PerformanceCriterionPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.performance` on apply.
pub fn diff_create(payload: &CreatePerformanceCriterion, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { performance: Some(ProgramPerformanceDelta { added: vec![payload.performance_criterion.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeletePerformanceCriterion, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { performance: Some(ProgramPerformanceDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenamePerformanceCriterion, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = PerformanceCriterionPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { performance: Some(ProgramPerformanceDelta { patched: vec![ProgramPerformancePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplacePerformanceCriterion, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.performance.iter().find(|row| row.header.id == payload.performance_criterion.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.performance_criterion).expect("diff_patch always produces a full patch");
    ProgramDiff { performance: Some(ProgramPerformanceDelta { patched: vec![ProgramPerformancePatchEntry { id: payload.performance_criterion.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
