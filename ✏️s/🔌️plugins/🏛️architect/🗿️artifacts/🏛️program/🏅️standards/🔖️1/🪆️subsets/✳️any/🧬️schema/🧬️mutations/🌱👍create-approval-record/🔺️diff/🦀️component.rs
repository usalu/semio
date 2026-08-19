//! 🔺️ Sparse diff construction for the `create-approval-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👍approvals` per Wave C.

use super::mutation::CreateApprovalRecord;
use crate::artifacts::program::diff::ProgramApprovalsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateApprovalRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.approval_record.header.id.clone();
    if base.approvals.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An approval record already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { approvals: Some(ProgramApprovalsDelta { added: vec![payload.approval_record.clone()], ..Default::default() }), ..Default::default() })
}
