//! ↩️ Inverse (undo) construction for the `delete-communication-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📡communication` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteCommunicationRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.communication.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateCommunicationRequirement(super::super::create_communication_requirement::CreateCommunicationRequirement { communication_requirement: base.communication[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
