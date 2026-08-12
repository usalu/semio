//! ↩️ Inverse (undo) construction for the `delete-infrastructure-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏗️infrastructure` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteInfrastructureRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.infrastructure.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateInfrastructureRequirement(super::super::create_infrastructure_requirement::mutation::CreateInfrastructureRequirement { infrastructure_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
