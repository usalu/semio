//! ↩️ Inverse (undo) construction for the `create-status-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📶status-records` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateStatusRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteStatusRecord(super::super::delete_status_record::mutation::DeleteStatusRecord { id: payload.status_record.header.id.clone() })]
}
