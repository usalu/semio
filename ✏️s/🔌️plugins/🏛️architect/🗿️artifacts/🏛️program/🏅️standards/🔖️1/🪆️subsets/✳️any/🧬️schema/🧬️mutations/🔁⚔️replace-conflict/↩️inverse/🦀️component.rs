//! ↩️ Inverse (undo) construction for the `replace-conflict` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚔️conflicts` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceConflict, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.conflicts.iter().find(|row| row.header.id == payload.conflict.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceConflict(super::mutation::ReplaceConflict { conflict: existing.clone() })],
        None => Vec::new(),
    }
}
