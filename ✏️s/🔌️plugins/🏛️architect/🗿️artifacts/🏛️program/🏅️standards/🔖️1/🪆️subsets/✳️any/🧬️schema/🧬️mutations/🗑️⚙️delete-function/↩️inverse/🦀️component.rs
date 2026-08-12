//! ↩️ Inverse (undo) construction for the `delete-function` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚙️functions` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteFunction, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.functions.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateFunction(super::super::create_function::mutation::CreateFunction { function: existing.clone() })],
        None => Vec::new(),
    }
}
