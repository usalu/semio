//! ↩️ Inverse (undo) construction for the `delete-assumption` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💭assumptions` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteAssumption, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.assumptions.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateAssumption(super::super::create_assumption::mutation::CreateAssumption { assumption: existing.clone() })],
        None => Vec::new(),
    }
}
