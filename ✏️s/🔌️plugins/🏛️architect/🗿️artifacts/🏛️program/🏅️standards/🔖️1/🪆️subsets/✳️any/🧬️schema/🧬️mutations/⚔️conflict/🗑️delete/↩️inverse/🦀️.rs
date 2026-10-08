//! ↩️ Inverse (undo) construction for the `delete-conflict` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚔️conflicts` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteConflict, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.conflicts.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateConflict(super::super::create_conflict::CreateConflict { conflict: base.conflicts[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
