//! ↩️ Inverse (undo) construction for the `options` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateOptionEvaluation, DeleteOptionEvaluation, RenameOptionEvaluation, ReplaceOptionEvaluation};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateOptionEvaluation, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteOptionEvaluation(DeleteOptionEvaluation { id: payload.option_evaluation.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteOptionEvaluation, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.options.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateOptionEvaluation(CreateOptionEvaluation { option_evaluation: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameOptionEvaluation, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.options.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameOptionEvaluation(RenameOptionEvaluation { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceOptionEvaluation, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.options.iter().find(|row| row.header.id == payload.option_evaluation.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceOptionEvaluation(ReplaceOptionEvaluation { option_evaluation: existing.clone() })],
        None => Vec::new(),
    }
}
