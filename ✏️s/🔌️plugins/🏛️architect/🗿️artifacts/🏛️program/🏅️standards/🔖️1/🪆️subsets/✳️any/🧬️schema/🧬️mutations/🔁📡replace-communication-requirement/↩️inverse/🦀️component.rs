//! ↩️ Inverse (undo) construction for the `replace-communication-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📡communication` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceCommunicationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.communication.iter().find(|row| row.header.id == payload.communication_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceCommunicationRequirement(super::mutation::ReplaceCommunicationRequirement { communication_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
