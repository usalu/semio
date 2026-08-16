//! 🔺️ Sparse diff construction for the `delete-growth-plan` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📈growth` per Wave C.

use super::mutation::DeleteGrowthPlan;
use crate::artifacts::program::diff::ProgramGrowthDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteGrowthPlan, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { growth: Some(ProgramGrowthDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
