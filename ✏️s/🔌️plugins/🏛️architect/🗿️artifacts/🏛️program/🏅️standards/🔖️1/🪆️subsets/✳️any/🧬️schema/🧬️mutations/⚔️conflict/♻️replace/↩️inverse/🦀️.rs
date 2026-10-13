//! ↩️ Inverse (undo) construction for the `replace-conflict` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚔️conflicts` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceConflict, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.conflicts.iter().find(|row| row.header.id == payload.conflict.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceConflict(super::ReplaceConflict { conflict: existing.clone() })],
        None => Vec::new(),
    }

    })())
}
