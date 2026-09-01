//! 🔺️ Sparse diff construction for the `delete-audit-event` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📒audit-events` per Wave C.

use super::DeleteAuditEvent;
use crate::artifacts::program::diff::ProgramAuditEventsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteAuditEvent, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.audit_events.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No audit event exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
