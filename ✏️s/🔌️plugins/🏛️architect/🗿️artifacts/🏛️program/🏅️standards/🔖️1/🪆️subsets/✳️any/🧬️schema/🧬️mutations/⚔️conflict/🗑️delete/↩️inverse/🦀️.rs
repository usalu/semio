//! ↩️ Inverse (undo) construction for the `delete-conflict` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚔️conflicts` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteConflict, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.conflicts.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateConflict(super::super::create_conflict::CreateConflict { conflict: existing.clone() })],
        None => Vec::new(),
    }
}
