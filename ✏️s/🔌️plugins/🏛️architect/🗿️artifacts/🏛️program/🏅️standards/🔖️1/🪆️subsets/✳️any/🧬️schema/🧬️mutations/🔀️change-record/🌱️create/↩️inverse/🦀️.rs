//! ↩️ Inverse (undo) construction for the `create-change-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📝changes` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateChangeRecord, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteChangeRecord(super::super::delete_change_record::DeleteChangeRecord { id: payload.change_record.header.id.clone() })]

    })())
}
