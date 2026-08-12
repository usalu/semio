//! ↩️ Inverse (undo) construction for the `analyses` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateAnalysisRecord, DeleteAnalysisRecord, RenameAnalysisRecord, ReplaceAnalysisRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateAnalysisRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteAnalysisRecord(DeleteAnalysisRecord { id: payload.analysis_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteAnalysisRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.analyses.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateAnalysisRecord(CreateAnalysisRecord { analysis_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameAnalysisRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.analyses.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameAnalysisRecord(RenameAnalysisRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceAnalysisRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.analyses.iter().find(|row| row.header.id == payload.analysis_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceAnalysisRecord(ReplaceAnalysisRecord { analysis_record: existing.clone() })],
        None => Vec::new(),
    }
}
