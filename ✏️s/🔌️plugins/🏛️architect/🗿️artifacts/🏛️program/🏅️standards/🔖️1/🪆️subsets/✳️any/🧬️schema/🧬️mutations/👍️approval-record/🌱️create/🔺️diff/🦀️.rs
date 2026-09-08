//! 🔺️ Sparse diff construction for the `create-approval-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👍approvals` per Wave C.

use super::CreateApprovalRecord;
use crate::diff::ProgramApprovalsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateApprovalRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.approval_record.header.id.clone();
    if base.approvals.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An approval record already exists with this id.", [id.0]);
    }
    protocol::MutationOutcome::new(ProgramDiff { approvals: Some(ProgramApprovalsDelta { added: vec![payload.approval_record.clone()], ..Default::default() }), ..Default::default() })
}
