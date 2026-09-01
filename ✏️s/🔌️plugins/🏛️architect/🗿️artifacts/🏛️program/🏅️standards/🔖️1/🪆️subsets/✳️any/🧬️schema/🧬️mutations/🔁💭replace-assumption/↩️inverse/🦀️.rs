//! ↩️ Inverse (undo) construction for the `replace-assumption` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💭assumptions` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceAssumption, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.assumptions.iter().find(|row| row.header.id == payload.assumption.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceAssumption(super::ReplaceAssumption { assumption: existing.clone() })],
        None => Vec::new(),
    }
}
