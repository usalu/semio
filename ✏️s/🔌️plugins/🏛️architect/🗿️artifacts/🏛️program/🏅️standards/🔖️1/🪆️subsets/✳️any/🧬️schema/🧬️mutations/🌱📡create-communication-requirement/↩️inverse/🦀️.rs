//! ↩️ Inverse (undo) construction for the `create-communication-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📡communication` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::CreateCommunicationRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteCommunicationRequirement(super::super::delete_communication_requirement::DeleteCommunicationRequirement { id: payload.communication_requirement.header.id.clone() })]
}
