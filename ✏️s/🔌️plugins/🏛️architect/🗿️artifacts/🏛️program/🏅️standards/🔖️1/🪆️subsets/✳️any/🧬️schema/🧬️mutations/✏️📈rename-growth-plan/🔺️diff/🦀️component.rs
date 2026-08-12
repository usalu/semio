//! 🔺️ Sparse diff construction for the `rename-growth-plan` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📈growth` per Wave C.

use super::mutation::RenameGrowthPlan;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramGrowthDelta, ProgramGrowthPatchEntry};
use crate::artifacts::program::registers::GrowthPlanPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameGrowthPlan, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = GrowthPlanPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { growth: Some(ProgramGrowthDelta { patched: vec![ProgramGrowthPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
