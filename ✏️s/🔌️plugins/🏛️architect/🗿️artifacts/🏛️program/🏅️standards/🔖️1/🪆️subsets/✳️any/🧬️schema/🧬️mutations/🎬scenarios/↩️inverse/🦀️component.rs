//! ↩️ Inverse (undo) construction for the `scenarios` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateScenario, DeleteScenario, RenameScenario, ReplaceScenario};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateScenario, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteScenario(DeleteScenario { id: payload.scenario.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteScenario, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.scenarios.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateScenario(CreateScenario { scenario: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameScenario, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.scenarios.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameScenario(RenameScenario { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceScenario, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.scenarios.iter().find(|row| row.header.id == payload.scenario.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceScenario(ReplaceScenario { scenario: existing.clone() })],
        None => Vec::new(),
    }
}
