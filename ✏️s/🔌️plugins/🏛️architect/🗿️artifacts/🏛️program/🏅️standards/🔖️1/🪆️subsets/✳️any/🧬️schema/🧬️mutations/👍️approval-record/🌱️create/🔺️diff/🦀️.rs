//! 🔺️ Sparse diff construction for the `create-approval-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👍approvals` per Wave C.

use super::CreateApprovalRecord;
use crate::diff::ProgramApprovalsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateApprovalRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.approval_record.header.id;
    if base.approvals.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An approval record already exists with this id.", [id.0.clone()]);
    }
    let length = base.approvals.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the approval record list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.approvals.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { approvals: Some(ProgramApprovalsDelta { added: vec![payload.approval_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
