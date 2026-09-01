//! ↩️ Inverse (undo) construction for the `create-option-evaluation` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚖️options` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::CreateOptionEvaluation, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteOptionEvaluation(super::super::delete_option_evaluation::DeleteOptionEvaluation { id: payload.option_evaluation.header.id.clone() })]
}
