//! ↩️ Inverse (undo) construction for the `delete-safety-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🦺safety` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteSafetyRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.safety.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateSafetyRequirement(super::super::create_safety_requirement::mutation::CreateSafetyRequirement { safety_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
