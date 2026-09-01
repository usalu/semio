//! ↩️ Inverse (undo) construction for the `replace-governance` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏛️update-governance` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

pub async fn inverse(_payload: &super::ReplaceGovernance, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::ReplaceGovernance(super::ReplaceGovernance { new_governance: base.governance.clone() })]
}
