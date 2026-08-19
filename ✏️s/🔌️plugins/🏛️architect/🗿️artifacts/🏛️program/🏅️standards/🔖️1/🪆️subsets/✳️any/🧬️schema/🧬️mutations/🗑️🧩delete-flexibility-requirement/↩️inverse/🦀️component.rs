//! ↩️ Inverse (undo) construction for the `delete-flexibility-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧩flexibility` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteFlexibilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.flexibility.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateFlexibilityRequirement(super::super::create_flexibility_requirement::mutation::CreateFlexibilityRequirement { flexibility_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
