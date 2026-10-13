//! ↩️ Inverse (undo) construction for the `replace-priority-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⭐priorities` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplacePriorityRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.priorities.iter().find(|row| row.header.id == payload.priority_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplacePriorityRecord(super::ReplacePriorityRecord { priority_record: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
