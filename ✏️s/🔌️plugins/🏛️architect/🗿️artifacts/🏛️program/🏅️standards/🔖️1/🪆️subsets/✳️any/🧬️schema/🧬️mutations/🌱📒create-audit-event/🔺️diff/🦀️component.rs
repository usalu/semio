//! 🔺️ Sparse diff construction for the `create-audit-event` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📒audit-events` per Wave C.

use super::mutation::CreateAuditEvent;
use crate::artifacts::program::diff::ProgramAuditEventsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.audit_events` on apply.
pub fn diff(payload: &CreateAuditEvent, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { added: vec![payload.audit_event.clone()], ..Default::default() }), ..Default::default() }
}
