//! ↩️ Inverse (undo) construction for the `delete-operational-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📋operations` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteOperationalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.operations.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateOperationalRequirement(super::super::create_operational_requirement::CreateOperationalRequirement { operational_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
