//! 🔺️ Sparse diff construction for the `rename-approval-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👍approvals` per Wave C.

use super::mutation::RenameApprovalRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramApprovalsDelta, ProgramApprovalsPatchEntry};
use crate::artifacts::program::registers::ApprovalRecordPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameApprovalRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ApprovalRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { approvals: Some(ProgramApprovalsDelta { patched: vec![ProgramApprovalsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
