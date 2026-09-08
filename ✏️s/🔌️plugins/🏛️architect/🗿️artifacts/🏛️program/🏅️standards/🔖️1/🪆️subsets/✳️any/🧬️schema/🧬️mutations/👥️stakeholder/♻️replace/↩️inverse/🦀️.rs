//! ↩️ Inverse (undo) construction for the `replace-stakeholder` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `👥stakeholders` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceStakeholder, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.stakeholders.iter().find(|row| row.header.id == payload.stakeholder.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceStakeholder(super::ReplaceStakeholder { stakeholder: existing.clone() })],
        None => Vec::new(),
    }
}
