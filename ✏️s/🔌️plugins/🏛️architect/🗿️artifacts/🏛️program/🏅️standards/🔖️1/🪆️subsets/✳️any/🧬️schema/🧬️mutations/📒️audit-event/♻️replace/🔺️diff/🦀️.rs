//! 🔺️ Sparse diff construction for the `replace-audit-event` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📒audit-events` per Wave C.

use super::ReplaceAuditEvent;
use crate::artifacts::program::diff::{ProgramAuditEventsDelta, ProgramAuditEventsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceAuditEvent, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.audit_events.iter().find(|row| row.header.id == payload.audit_event.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No audit event exists with this id.", [payload.audit_event.header.id.0.clone()]);
    };
    if existing == &payload.audit_event {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This audit event already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.audit_event).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { patched: vec![ProgramAuditEventsPatchEntry { id: payload.audit_event.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
