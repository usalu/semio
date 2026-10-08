//! ↩️ Inverse (undo) construction for the `delete-audit-event` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📒audit-events` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteAuditEvent, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.audit_events.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateAuditEvent(super::super::create_audit_event::CreateAuditEvent { audit_event: base.audit_events[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
