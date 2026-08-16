//! 🔺️ Sparse diff construction for the `delete-approval-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👍approvals` per Wave C.

use super::mutation::DeleteApprovalRecord;
use crate::artifacts::program::diff::ProgramApprovalsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteApprovalRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { approvals: Some(ProgramApprovalsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
