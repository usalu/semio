//! ↩️ Inverse (undo) construction for the `create-stakeholder` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `👥stakeholders` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateStakeholder, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::mutation::DeleteStakeholder { id: payload.stakeholder.header.id.clone() })]
}
