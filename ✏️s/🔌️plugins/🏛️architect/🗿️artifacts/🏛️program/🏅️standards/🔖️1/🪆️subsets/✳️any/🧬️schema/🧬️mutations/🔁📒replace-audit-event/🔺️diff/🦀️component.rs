//! 🔺️ Sparse diff construction for the `replace-audit-event` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📒audit-events` per Wave C.

use super::mutation::ReplaceAuditEvent;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAuditEventsDelta, ProgramAuditEventsPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceAuditEvent, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.audit_events.iter().find(|row| row.header.id == payload.audit_event.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.audit_event).expect("diff_patch always produces a full patch");
    ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { patched: vec![ProgramAuditEventsPatchEntry { id: payload.audit_event.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
