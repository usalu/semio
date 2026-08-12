//! ↩️ Inverse (undo) construction for the `knowledge` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateKnowledgeRecord, DeleteKnowledgeRecord, RenameKnowledgeRecord, ReplaceKnowledgeRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateKnowledgeRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteKnowledgeRecord(DeleteKnowledgeRecord { id: payload.knowledge_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteKnowledgeRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.knowledge.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateKnowledgeRecord(CreateKnowledgeRecord { knowledge_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameKnowledgeRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.knowledge.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameKnowledgeRecord(RenameKnowledgeRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceKnowledgeRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.knowledge.iter().find(|row| row.header.id == payload.knowledge_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceKnowledgeRecord(ReplaceKnowledgeRecord { knowledge_record: existing.clone() })],
        None => Vec::new(),
    }
}
