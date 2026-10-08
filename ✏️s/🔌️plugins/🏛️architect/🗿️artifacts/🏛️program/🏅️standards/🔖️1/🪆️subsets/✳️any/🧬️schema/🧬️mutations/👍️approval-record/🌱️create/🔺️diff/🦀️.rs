//! 🔺️ Sparse diff construction for the `create-approval-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👍approvals` per Wave C.

use super::CreateApprovalRecord;
use crate::diff::ProgramApprovalsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateApprovalRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.approval_record.header.id;
    if base.approvals.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An approval record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.approvals.len());
    if at > base.approvals.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the approval record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { approvals: Some(ProgramApprovalsDelta::insertion(at, payload.approval_record.clone())), ..Default::default() })
}
