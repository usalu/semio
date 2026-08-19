//! ↩️ Inverse (undo) construction for the `delete-stakeholder` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `👥stakeholders` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteStakeholder, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.stakeholders.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateStakeholder(super::super::create_stakeholder::mutation::CreateStakeholder { stakeholder: existing.clone() })],
        None => Vec::new(),
    }
}
