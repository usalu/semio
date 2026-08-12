//! ↩️ Inverse (undo) construction for the `issues` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateIssue, DeleteIssue, RenameIssue, ReplaceIssue};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateIssue, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteIssue(DeleteIssue { id: payload.issue.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteIssue, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.issues.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateIssue(CreateIssue { issue: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameIssue, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.issues.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameIssue(RenameIssue { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceIssue, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.issues.iter().find(|row| row.header.id == payload.issue.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceIssue(ReplaceIssue { issue: existing.clone() })],
        None => Vec::new(),
    }
}
