//! ↩️ Inverse (undo) construction for the `update_governance` mutation leaf.

use super::mutation::{RenameGovernance, ReplaceGovernance};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

pub fn inverse_rename(_payload: &RenameGovernance, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::RenameGovernance(RenameGovernance { new_framework: base.governance.framework.clone() })]
}

pub fn inverse_replace(_payload: &ReplaceGovernance, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::ReplaceGovernance(ReplaceGovernance { new_governance: base.governance.clone() })]
}
