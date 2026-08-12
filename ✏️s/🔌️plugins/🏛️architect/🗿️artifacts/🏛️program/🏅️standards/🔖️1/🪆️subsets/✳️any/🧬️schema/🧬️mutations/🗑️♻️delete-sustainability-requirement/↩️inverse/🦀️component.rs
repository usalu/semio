//! ↩️ Inverse (undo) construction for the `delete-sustainability-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `♻️sustainability` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteSustainabilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.sustainability.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateSustainabilityRequirement(super::super::create_sustainability_requirement::mutation::CreateSustainabilityRequirement { sustainability_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
