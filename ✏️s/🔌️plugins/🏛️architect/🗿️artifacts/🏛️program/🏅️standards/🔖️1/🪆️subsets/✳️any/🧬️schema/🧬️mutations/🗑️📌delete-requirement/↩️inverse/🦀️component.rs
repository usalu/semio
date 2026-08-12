//! ↩️ Inverse (undo) construction for the `delete-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📌requirements` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.requirements.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateRequirement(super::super::create_requirement::mutation::CreateRequirement { requirement: existing.clone() })],
        None => Vec::new(),
    }
}
