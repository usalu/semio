//! ↩️ Inverse (undo) construction for the `delete-option-evaluation` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚖️options` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteOptionEvaluation, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.options.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateOptionEvaluation(super::super::create_option_evaluation::mutation::CreateOptionEvaluation { option_evaluation: existing.clone() })],
        None => Vec::new(),
    }
}
