//! 🔺️ Sparse diff construction for the `create-growth-plan` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📈growth` per Wave C.

use super::mutation::CreateGrowthPlan;
use crate::artifacts::program::diff::ProgramGrowthDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.growth` on apply.
pub fn diff(payload: &CreateGrowthPlan, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { growth: Some(ProgramGrowthDelta { added: vec![payload.growth_plan.clone()], ..Default::default() }), ..Default::default() }
}
