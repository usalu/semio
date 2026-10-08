//! 🔺️ Sparse diff construction for the `replace-approval-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👍approvals` per Wave C.

use super::ReplaceApprovalRecord;
use crate::diff::ProgramApprovalsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceApprovalRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.approval_record.header.id;
    let Some(position) = base.approvals.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No approval record exists with this id.", [id.0.clone()]);
    };
    if base.approvals[position] == payload.approval_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This approval record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramApprovalsDelta::removal(&base.approvals, position);
    delta.absorb(ProgramApprovalsDelta::insertion(position, payload.approval_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { approvals: Some(delta), ..Default::default() })
}
