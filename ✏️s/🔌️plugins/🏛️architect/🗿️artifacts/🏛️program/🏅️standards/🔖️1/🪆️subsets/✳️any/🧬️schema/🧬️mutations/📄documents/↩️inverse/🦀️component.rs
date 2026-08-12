//! ↩️ Inverse (undo) construction for the `documents` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateDocument, DeleteDocument, RenameDocument, ReplaceDocument};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateDocument, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteDocument(DeleteDocument { id: payload.document.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteDocument, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.artifacts.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateDocument(CreateDocument { document: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameDocument, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.artifacts.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameDocument(RenameDocument { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceDocument, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.artifacts.iter().find(|row| row.header.id == payload.document.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceDocument(ReplaceDocument { document: existing.clone() })],
        None => Vec::new(),
    }
}
