//! ↩️ Inverse (undo) construction for the `approvals` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateApprovalRecord, DeleteApprovalRecord, RenameApprovalRecord, ReplaceApprovalRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateApprovalRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteApprovalRecord(DeleteApprovalRecord { id: payload.approval_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteApprovalRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.approvals.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateApprovalRecord(CreateApprovalRecord { approval_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameApprovalRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.approvals.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameApprovalRecord(RenameApprovalRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceApprovalRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.approvals.iter().find(|row| row.header.id == payload.approval_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceApprovalRecord(ReplaceApprovalRecord { approval_record: existing.clone() })],
        None => Vec::new(),
    }
}
