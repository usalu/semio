//! ↩️ Inverse (undo) construction for the `decisions` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateDecision, DeleteDecision, RenameDecision, ReplaceDecision};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateDecision, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteDecision(DeleteDecision { id: payload.decision.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteDecision, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.decisions.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateDecision(CreateDecision { decision: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameDecision, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.decisions.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameDecision(RenameDecision { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceDecision, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.decisions.iter().find(|row| row.header.id == payload.decision.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceDecision(ReplaceDecision { decision: existing.clone() })],
        None => Vec::new(),
    }
}
