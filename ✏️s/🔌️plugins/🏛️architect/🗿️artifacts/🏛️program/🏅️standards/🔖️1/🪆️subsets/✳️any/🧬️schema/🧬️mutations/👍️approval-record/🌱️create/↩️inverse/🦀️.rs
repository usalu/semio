//! ↩️ Inverse (undo) construction for the `create-approval-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `👍approvals` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateApprovalRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteApprovalRecord(super::super::delete_approval_record::DeleteApprovalRecord { id: payload.approval_record.header.id.clone() })]
}
