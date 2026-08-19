//! ↩️ Inverse (undo) construction for the `delete-service-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛎️services` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteServiceRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.services.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateServiceRequirement(super::super::create_service_requirement::mutation::CreateServiceRequirement { service_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
