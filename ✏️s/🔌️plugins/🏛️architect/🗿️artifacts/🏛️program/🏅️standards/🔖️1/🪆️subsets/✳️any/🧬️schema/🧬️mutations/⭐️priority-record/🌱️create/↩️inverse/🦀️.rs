//! ↩️ Inverse (undo) construction for the `create-priority-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⭐priorities` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreatePriorityRecord, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeletePriorityRecord(super::super::delete_priority_record::DeletePriorityRecord { id: payload.priority_record.header.id.clone() })]

    })())
}
