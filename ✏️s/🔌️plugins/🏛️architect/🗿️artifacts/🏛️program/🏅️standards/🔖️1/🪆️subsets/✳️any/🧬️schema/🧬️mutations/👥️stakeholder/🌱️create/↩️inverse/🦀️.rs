//! ↩️ Inverse (undo) construction for the `create-stakeholder` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `👥stakeholders` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateStakeholder, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::DeleteStakeholder { id: payload.stakeholder.header.id.clone() })]
}
