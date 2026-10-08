//! ↩️ Inverse (undo) construction for the `delete-priority-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⭐priorities` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeletePriorityRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.priorities.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreatePriorityRecord(super::super::create_priority_record::CreatePriorityRecord { priority_record: base.priorities[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
