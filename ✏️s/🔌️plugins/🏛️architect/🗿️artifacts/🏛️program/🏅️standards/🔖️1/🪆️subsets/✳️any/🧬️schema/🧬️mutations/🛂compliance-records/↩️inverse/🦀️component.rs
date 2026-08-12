//! ↩️ Inverse (undo) construction for the `compliance_records` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateComplianceRecord, DeleteComplianceRecord, RenameComplianceRecord, ReplaceComplianceRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateComplianceRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteComplianceRecord(DeleteComplianceRecord { id: payload.compliance_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteComplianceRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.compliance_records.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateComplianceRecord(CreateComplianceRecord { compliance_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameComplianceRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.compliance_records.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameComplianceRecord(RenameComplianceRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceComplianceRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.compliance_records.iter().find(|row| row.header.id == payload.compliance_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceComplianceRecord(ReplaceComplianceRecord { compliance_record: existing.clone() })],
        None => Vec::new(),
    }
}
