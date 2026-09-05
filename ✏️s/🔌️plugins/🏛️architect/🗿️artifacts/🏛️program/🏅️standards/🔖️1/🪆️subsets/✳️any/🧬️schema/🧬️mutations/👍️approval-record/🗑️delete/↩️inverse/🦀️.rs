//! ↩️ Inverse (undo) construction for the `delete-approval-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `👍approvals` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteApprovalRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.approvals.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateApprovalRecord(super::super::create_approval_record::CreateApprovalRecord { approval_record: existing.clone() })],
        None => Vec::new(),
    }
}
