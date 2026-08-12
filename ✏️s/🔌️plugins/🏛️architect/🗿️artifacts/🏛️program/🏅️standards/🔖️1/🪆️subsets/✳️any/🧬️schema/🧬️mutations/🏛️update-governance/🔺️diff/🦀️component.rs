//! 🔺️ Sparse diff construction for the `update_governance` mutation leaf.

use super::mutation::{RenameGovernance, ReplaceGovernance};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ New `Governance` with only `framework` changed.
pub fn diff_rename(payload: &RenameGovernance, base: &ProgramSnapshot) -> ProgramDiff {
    let mut value = base.governance.clone();
    value.framework = payload.new_framework.clone();
    ProgramDiff { governance: Some(value), ..Default::default() }
}

/// 🔁️ New `Governance` wholesale.
pub fn diff_replace(payload: &ReplaceGovernance, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { governance: Some(payload.new_governance.clone()), ..Default::default() }
}
