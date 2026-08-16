//! 🔺️ Sparse diff construction for the `rename-audit-event` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📒audit-events` per Wave C.

use super::mutation::RenameAuditEvent;
use crate::artifacts::program::diff::{ProgramAuditEventsDelta, ProgramAuditEventsPatchEntry};
use crate::artifacts::program::registers::AuditEventPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameAuditEvent, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = AuditEventPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { patched: vec![ProgramAuditEventsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
