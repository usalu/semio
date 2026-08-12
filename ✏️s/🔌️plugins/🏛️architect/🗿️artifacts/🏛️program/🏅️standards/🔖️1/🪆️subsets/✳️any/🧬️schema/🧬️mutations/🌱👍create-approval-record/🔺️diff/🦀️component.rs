//! 🔺️ Sparse diff construction for the `create-approval-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👍approvals` per Wave C.

use super::mutation::CreateApprovalRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramApprovalsDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.approvals` on apply.
pub fn diff(payload: &CreateApprovalRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { approvals: Some(ProgramApprovalsDelta { added: vec![payload.approval_record.clone()], ..Default::default() }), ..Default::default() }
}
