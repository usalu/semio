//! ↩️ Inverse (undo) construction for the `performance` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreatePerformanceCriterion, DeletePerformanceCriterion, RenamePerformanceCriterion, ReplacePerformanceCriterion};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreatePerformanceCriterion, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeletePerformanceCriterion(DeletePerformanceCriterion { id: payload.performance_criterion.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeletePerformanceCriterion, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.performance.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreatePerformanceCriterion(CreatePerformanceCriterion { performance_criterion: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenamePerformanceCriterion, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.performance.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenamePerformanceCriterion(RenamePerformanceCriterion { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplacePerformanceCriterion, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.performance.iter().find(|row| row.header.id == payload.performance_criterion.header.id) {
        Some(existing) => vec![ProgramMutation::ReplacePerformanceCriterion(ReplacePerformanceCriterion { performance_criterion: existing.clone() })],
        None => Vec::new(),
    }
}
