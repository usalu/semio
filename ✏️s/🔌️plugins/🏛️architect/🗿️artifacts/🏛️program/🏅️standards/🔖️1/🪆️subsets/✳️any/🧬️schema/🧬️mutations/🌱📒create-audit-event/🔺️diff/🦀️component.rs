//! 🔺️ Sparse diff construction for the `create-audit-event` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📒audit-events` per Wave C.

use super::mutation::CreateAuditEvent;
use crate::artifacts::program::diff::ProgramAuditEventsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateAuditEvent, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.audit_event.header.id.clone();
    if base.audit_events.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An audit event already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { added: vec![payload.audit_event.clone()], ..Default::default() }), ..Default::default() })
}
