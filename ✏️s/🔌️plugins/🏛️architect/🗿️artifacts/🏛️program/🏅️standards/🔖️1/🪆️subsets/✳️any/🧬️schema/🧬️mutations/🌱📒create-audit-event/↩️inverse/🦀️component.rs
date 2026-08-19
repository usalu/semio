//! ↩️ Inverse (undo) construction for the `create-audit-event` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📒audit-events` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateAuditEvent, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteAuditEvent(super::super::delete_audit_event::mutation::DeleteAuditEvent { id: payload.audit_event.header.id.clone() })]
}
