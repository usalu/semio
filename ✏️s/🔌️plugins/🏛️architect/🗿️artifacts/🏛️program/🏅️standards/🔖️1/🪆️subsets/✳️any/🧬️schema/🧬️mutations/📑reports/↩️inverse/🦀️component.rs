//! ↩️ Inverse (undo) construction for the `reports` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateReportRecord, DeleteReportRecord, RenameReportRecord, ReplaceReportRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateReportRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteReportRecord(DeleteReportRecord { id: payload.report_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteReportRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.reports.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateReportRecord(CreateReportRecord { report_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameReportRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.reports.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameReportRecord(RenameReportRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceReportRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.reports.iter().find(|row| row.header.id == payload.report_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceReportRecord(ReplaceReportRecord { report_record: existing.clone() })],
        None => Vec::new(),
    }
}
