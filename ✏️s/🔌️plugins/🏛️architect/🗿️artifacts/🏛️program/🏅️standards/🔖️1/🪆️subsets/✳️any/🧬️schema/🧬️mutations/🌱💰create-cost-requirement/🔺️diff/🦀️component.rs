//! 🔺️ Sparse diff construction for the `create-cost-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💰costs` per Wave C.

use super::mutation::CreateCostRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramCostsDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.costs` on apply.
pub fn diff(payload: &CreateCostRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { costs: Some(ProgramCostsDelta { added: vec![payload.cost_requirement.clone()], ..Default::default() }), ..Default::default() }
}
