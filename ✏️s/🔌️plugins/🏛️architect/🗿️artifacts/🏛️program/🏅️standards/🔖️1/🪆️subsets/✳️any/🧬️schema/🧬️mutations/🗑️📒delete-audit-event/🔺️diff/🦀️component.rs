//! 🔺️ Sparse diff construction for the `delete-audit-event` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📒audit-events` per Wave C.

use super::mutation::DeleteAuditEvent;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAuditEventsDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteAuditEvent, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { audit_events: Some(ProgramAuditEventsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
