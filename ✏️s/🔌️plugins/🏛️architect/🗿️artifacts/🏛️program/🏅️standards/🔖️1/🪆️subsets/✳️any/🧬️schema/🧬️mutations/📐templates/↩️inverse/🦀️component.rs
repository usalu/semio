//! ↩️ Inverse (undo) construction for the `templates` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateTemplateRecord, DeleteTemplateRecord, RenameTemplateRecord, ReplaceTemplateRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateTemplateRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteTemplateRecord(DeleteTemplateRecord { id: payload.template_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteTemplateRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.templates.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateTemplateRecord(CreateTemplateRecord { template_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameTemplateRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.templates.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameTemplateRecord(RenameTemplateRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceTemplateRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.templates.iter().find(|row| row.header.id == payload.template_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceTemplateRecord(ReplaceTemplateRecord { template_record: existing.clone() })],
        None => Vec::new(),
    }
}
