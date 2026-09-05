//! ↩️ Inverse (undo) construction for the `replace-audit-event` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📒audit-events` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceAuditEvent, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.audit_events.iter().find(|row| row.header.id == payload.audit_event.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceAuditEvent(super::ReplaceAuditEvent { audit_event: existing.clone() })],
        None => Vec::new(),
    }
}
