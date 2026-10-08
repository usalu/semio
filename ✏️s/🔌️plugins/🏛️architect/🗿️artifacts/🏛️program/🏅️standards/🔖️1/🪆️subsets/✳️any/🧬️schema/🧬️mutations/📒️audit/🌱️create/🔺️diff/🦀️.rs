//! 🔺️ Sparse diff construction for the `create-audit-event` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📒audit-events` per Wave C.

use super::CreateAuditEvent;
use crate::diff::ProgramAuditEventsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateAuditEvent, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.audit_event.header.id;
    if base.audit_events.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An audit event already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.audit_events.len());
    if at > base.audit_events.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the audit event list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { audit_events: Some(ProgramAuditEventsDelta::insertion(at, payload.audit_event.clone())), ..Default::default() })
}
