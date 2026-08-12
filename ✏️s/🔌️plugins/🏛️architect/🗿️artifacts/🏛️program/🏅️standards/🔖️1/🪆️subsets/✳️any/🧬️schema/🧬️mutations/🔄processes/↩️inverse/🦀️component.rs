//! ↩️ Inverse (undo) construction for the `processes` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateProcess, DeleteProcess, RenameProcess, ReplaceProcess};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateProcess, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteProcess(DeleteProcess { id: payload.process.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteProcess, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.processes.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateProcess(CreateProcess { process: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameProcess, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.processes.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameProcess(RenameProcess { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceProcess, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.processes.iter().find(|row| row.header.id == payload.process.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceProcess(ReplaceProcess { process: existing.clone() })],
        None => Vec::new(),
    }
}
