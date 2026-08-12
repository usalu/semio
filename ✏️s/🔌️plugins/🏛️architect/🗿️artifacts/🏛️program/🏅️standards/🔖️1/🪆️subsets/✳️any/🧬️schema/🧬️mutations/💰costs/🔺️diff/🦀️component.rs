//! 🔺️ Sparse diff construction for the `costs` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateCostRequirement, DeleteCostRequirement, RenameCostRequirement, ReplaceCostRequirement};
use crate::artifacts::program::diff::{ProgramCostsDelta, ProgramCostsPatchEntry};
use crate::artifacts::program::registers::CostRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.costs` on apply.
pub fn diff_create(payload: &CreateCostRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { costs: Some(ProgramCostsDelta { added: vec![payload.cost_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteCostRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { costs: Some(ProgramCostsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameCostRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = CostRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { costs: Some(ProgramCostsDelta { patched: vec![ProgramCostsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceCostRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.costs.iter().find(|row| row.header.id == payload.cost_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.cost_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { costs: Some(ProgramCostsDelta { patched: vec![ProgramCostsPatchEntry { id: payload.cost_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
