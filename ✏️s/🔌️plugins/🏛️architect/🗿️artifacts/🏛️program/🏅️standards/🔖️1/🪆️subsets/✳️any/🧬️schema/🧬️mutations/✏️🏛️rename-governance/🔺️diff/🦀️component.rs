//! 🔺️ Sparse diff construction for the `rename-governance` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏛️update-governance` per Wave C.

use super::mutation::RenameGovernance;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ New `Governance` with only `framework` changed.
pub fn diff(payload: &RenameGovernance, base: &ProgramSnapshot) -> ProgramDiff {
    let mut value = base.governance.clone();
    value.framework = payload.new_framework.clone();
    ProgramDiff { governance: Some(value), ..Default::default() }
}
