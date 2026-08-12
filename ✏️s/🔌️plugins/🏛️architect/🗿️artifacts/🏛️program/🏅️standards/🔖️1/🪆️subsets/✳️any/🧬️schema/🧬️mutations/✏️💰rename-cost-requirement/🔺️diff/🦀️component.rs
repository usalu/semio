//! 🔺️ Sparse diff construction for the `rename-cost-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💰costs` per Wave C.

use super::mutation::RenameCostRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramCostsDelta, ProgramCostsPatchEntry};
use crate::artifacts::program::registers::CostRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameCostRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = CostRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { costs: Some(ProgramCostsDelta { patched: vec![ProgramCostsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
