//! ↩️ Inverse (undo) construction for the `delete-change-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📝changes` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteChangeRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.changes.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateChangeRecord(super::super::create_change_record::CreateChangeRecord { change_record: base.changes[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
