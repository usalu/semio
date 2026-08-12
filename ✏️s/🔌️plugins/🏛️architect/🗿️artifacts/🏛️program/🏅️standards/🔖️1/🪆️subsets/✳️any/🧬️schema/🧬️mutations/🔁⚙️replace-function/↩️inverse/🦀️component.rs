//! ↩️ Inverse (undo) construction for the `replace-function` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚙️functions` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceFunction, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.functions.iter().find(|row| row.header.id == payload.function.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceFunction(super::mutation::ReplaceFunction { function: existing.clone() })],
        None => Vec::new(),
    }
}
