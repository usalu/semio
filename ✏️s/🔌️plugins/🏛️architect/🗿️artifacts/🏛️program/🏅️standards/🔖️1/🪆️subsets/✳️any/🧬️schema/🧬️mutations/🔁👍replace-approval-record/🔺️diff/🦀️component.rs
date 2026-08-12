//! 🔺️ Sparse diff construction for the `replace-approval-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👍approvals` per Wave C.

use super::mutation::ReplaceApprovalRecord;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramApprovalsDelta, ProgramApprovalsPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceApprovalRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.approvals.iter().find(|row| row.header.id == payload.approval_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.approval_record).expect("diff_patch always produces a full patch");
    ProgramDiff { approvals: Some(ProgramApprovalsDelta { patched: vec![ProgramApprovalsPatchEntry { id: payload.approval_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
