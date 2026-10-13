//! 🔺️ Sparse diff construction for the `rename-approval-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👍approvals` per Wave C.

use super::RenameApprovalRecord;
use crate::standards::v1::subsets::any::schema::diff::{ProgramApprovalsDelta, ProgramApprovalsPatchEntry};
use crate::standards::v1::subsets::any::schema::registers::ApprovalRecordPatch;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `modified = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameApprovalRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.approvals.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No approval record exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This approval record already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ApprovalRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { approvals: Some(ProgramApprovalsDelta { modified: vec![ProgramApprovalsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
