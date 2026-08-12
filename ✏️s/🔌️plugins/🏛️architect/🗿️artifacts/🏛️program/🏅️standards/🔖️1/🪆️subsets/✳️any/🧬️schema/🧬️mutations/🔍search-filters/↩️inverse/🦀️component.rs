//! ↩️ Inverse (undo) construction for the `search_filters` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateSearchFilter, DeleteSearchFilter, RenameSearchFilter, ReplaceSearchFilter};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateSearchFilter, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteSearchFilter(DeleteSearchFilter { id: payload.search_filter.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteSearchFilter, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.search_filters.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateSearchFilter(CreateSearchFilter { search_filter: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameSearchFilter, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.search_filters.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameSearchFilter(RenameSearchFilter { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceSearchFilter, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.search_filters.iter().find(|row| row.header.id == payload.search_filter.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSearchFilter(ReplaceSearchFilter { search_filter: existing.clone() })],
        None => Vec::new(),
    }
}
