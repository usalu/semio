//! ↩️ Inverse (undo) construction for the `replace-option-evaluation` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚖️options` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceOptionEvaluation, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.options.iter().find(|row| row.header.id == payload.option_evaluation.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceOptionEvaluation(super::ReplaceOptionEvaluation { option_evaluation: existing.clone() })],
        None => Vec::new(),
    }
}
