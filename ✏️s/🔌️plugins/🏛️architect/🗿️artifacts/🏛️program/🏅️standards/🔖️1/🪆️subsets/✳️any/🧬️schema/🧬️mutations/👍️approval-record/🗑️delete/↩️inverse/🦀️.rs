//! ↩️ Inverse (undo) construction for the `delete-approval-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `👍approvals` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteApprovalRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.approvals.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateApprovalRecord(super::super::create_approval_record::CreateApprovalRecord { approval_record: base.approvals[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
