//! 🔺️ Sparse diff construction for the `replace-governance` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏛️update-governance` per Wave C.

use super::mutation::ReplaceGovernance;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🔁️ New `Governance` wholesale.
pub fn diff(payload: &ReplaceGovernance, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { governance: Some(payload.new_governance.clone()), ..Default::default() }
}
