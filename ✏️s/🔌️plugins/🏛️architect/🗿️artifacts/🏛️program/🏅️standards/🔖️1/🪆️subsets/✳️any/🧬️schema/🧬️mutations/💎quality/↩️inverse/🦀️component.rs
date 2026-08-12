//! ↩️ Inverse (undo) construction for the `quality` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateQualityRecord, DeleteQualityRecord, RenameQualityRecord, ReplaceQualityRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateQualityRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteQualityRecord(DeleteQualityRecord { id: payload.quality_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteQualityRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.quality.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateQualityRecord(CreateQualityRecord { quality_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameQualityRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.quality.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameQualityRecord(RenameQualityRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceQualityRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.quality.iter().find(|row| row.header.id == payload.quality_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceQualityRecord(ReplaceQualityRecord { quality_record: existing.clone() })],
        None => Vec::new(),
    }
}
