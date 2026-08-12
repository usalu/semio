//! ↩️ Inverse (undo) construction for the `create-approval-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `👍approvals` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateApprovalRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteApprovalRecord(super::super::delete_approval_record::mutation::DeleteApprovalRecord { id: payload.approval_record.header.id.clone() })]
}
