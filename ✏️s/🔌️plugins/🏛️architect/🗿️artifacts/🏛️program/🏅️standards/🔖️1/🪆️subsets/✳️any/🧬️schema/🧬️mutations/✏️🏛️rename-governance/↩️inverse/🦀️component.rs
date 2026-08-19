//! ↩️ Inverse (undo) construction for the `rename-governance` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏛️update-governance` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

pub async fn inverse(_payload: &super::mutation::RenameGovernance, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::RenameGovernance(super::mutation::RenameGovernance { new_framework: base.governance.framework.clone() })]
}
