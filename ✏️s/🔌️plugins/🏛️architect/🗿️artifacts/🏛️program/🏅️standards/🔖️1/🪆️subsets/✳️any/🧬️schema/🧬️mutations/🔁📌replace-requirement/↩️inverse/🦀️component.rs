//! ↩️ Inverse (undo) construction for the `replace-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📌requirements` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.requirements.iter().find(|row| row.header.id == payload.requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceRequirement(super::mutation::ReplaceRequirement { requirement: existing.clone() })],
        None => Vec::new(),
    }
}
