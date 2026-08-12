//! ↩️ Inverse (undo) construction for the `site_context` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateSiteContext, DeleteSiteContext, RenameSiteContext, ReplaceSiteContext};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateSiteContext, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteSiteContext(DeleteSiteContext { id: payload.site_context.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteSiteContext, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.site_context.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateSiteContext(CreateSiteContext { site_context: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameSiteContext, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.site_context.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameSiteContext(RenameSiteContext { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceSiteContext, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.site_context.iter().find(|row| row.header.id == payload.site_context.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSiteContext(ReplaceSiteContext { site_context: existing.clone() })],
        None => Vec::new(),
    }
}
