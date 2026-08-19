//! ↩️ Inverse (undo) construction for the `rename-communication-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📡communication` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::RenameCommunicationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.communication.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameCommunicationRequirement(super::mutation::RenameCommunicationRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
