//! ↩️ Inverse (undo) construction for the `delete-audit-event` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📒audit-events` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteAuditEvent, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.audit_events.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateAuditEvent(super::super::create_audit_event::CreateAuditEvent { audit_event: existing.clone() })],
        None => Vec::new(),
    }
}
