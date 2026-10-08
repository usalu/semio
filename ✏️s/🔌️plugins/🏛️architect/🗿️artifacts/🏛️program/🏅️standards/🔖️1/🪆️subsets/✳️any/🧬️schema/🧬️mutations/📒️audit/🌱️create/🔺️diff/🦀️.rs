//! 🔺️ Sparse diff construction for the `create-audit-event` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📒audit-events` per Wave C.

use super::CreateAuditEvent;
use crate::diff::ProgramAuditEventsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateAuditEvent, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.audit_event.header.id;
    if base.audit_events.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An audit event already exists with this id.", [id.0.clone()]);
    }
    let length = base.audit_events.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the audit event list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.audit_events.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { added: vec![payload.audit_event.clone()], reordered, ..Default::default() }), ..Default::default() })
}
