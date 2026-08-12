//! ↩️ Inverse (undo) construction for the `audit_events` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateAuditEvent, DeleteAuditEvent, RenameAuditEvent, ReplaceAuditEvent};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateAuditEvent, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteAuditEvent(DeleteAuditEvent { id: payload.audit_event.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteAuditEvent, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.audit_events.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateAuditEvent(CreateAuditEvent { audit_event: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameAuditEvent, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.audit_events.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameAuditEvent(RenameAuditEvent { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceAuditEvent, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.audit_events.iter().find(|row| row.header.id == payload.audit_event.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceAuditEvent(ReplaceAuditEvent { audit_event: existing.clone() })],
        None => Vec::new(),
    }
}
