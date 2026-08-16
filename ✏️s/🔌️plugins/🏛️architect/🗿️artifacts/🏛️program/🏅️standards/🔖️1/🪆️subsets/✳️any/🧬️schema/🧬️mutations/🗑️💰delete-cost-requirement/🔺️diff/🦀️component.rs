//! 🔺️ Sparse diff construction for the `delete-cost-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💰costs` per Wave C.

use super::mutation::DeleteCostRequirement;
use crate::artifacts::program::diff::ProgramCostsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteCostRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { costs: Some(ProgramCostsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
