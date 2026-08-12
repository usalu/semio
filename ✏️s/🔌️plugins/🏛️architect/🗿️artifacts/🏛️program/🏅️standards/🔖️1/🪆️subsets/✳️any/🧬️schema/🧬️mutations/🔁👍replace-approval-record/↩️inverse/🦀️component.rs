//! ↩️ Inverse (undo) construction for the `replace-approval-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `👍approvals` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceApprovalRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.approvals.iter().find(|row| row.header.id == payload.approval_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceApprovalRecord(super::mutation::ReplaceApprovalRecord { approval_record: existing.clone() })],
        None => Vec::new(),
    }
}
