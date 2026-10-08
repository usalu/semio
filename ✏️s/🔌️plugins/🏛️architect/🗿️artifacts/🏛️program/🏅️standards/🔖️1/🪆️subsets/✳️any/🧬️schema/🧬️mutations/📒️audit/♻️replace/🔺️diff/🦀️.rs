//! 🔺️ Sparse diff construction for the `replace-audit-event` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📒audit-events` per Wave C.

use super::ReplaceAuditEvent;
use crate::diff::ProgramAuditEventsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceAuditEvent, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.audit_event.header.id;
    let Some(position) = base.audit_events.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No audit event exists with this id.", [id.0.clone()]);
    };
    if base.audit_events[position] == payload.audit_event {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This audit event already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramAuditEventsDelta::removal(&base.audit_events, position);
    delta.absorb(ProgramAuditEventsDelta::insertion(position, payload.audit_event.clone()));
    protocol::MutationOutcome::new(ProgramDiff { audit_events: Some(delta), ..Default::default() })
}
