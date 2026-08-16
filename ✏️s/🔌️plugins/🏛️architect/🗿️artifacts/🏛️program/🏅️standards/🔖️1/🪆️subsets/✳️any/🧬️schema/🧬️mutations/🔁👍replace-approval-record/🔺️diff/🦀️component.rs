//! 🔺️ Sparse diff construction for the `replace-approval-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👍approvals` per Wave C.

use super::mutation::ReplaceApprovalRecord;
use crate::artifacts::program::diff::{ProgramApprovalsDelta, ProgramApprovalsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub fn diff(payload: &ReplaceApprovalRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.approvals.iter().find(|row| row.header.id == payload.approval_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No approval record exists with this id.", [payload.approval_record.header.id.0.clone()]);
    };
    if existing == &payload.approval_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This approval record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.approval_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { approvals: Some(ProgramApprovalsDelta { patched: vec![ProgramApprovalsPatchEntry { id: payload.approval_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
