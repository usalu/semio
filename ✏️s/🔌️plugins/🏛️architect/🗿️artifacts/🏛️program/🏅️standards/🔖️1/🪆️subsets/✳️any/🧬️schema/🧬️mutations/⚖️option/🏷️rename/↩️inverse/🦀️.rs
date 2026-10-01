//! ↩️ Inverse (undo) construction for the `rename-option-evaluation` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚖️options` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::RenameOptionEvaluation, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.options.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameOptionEvaluation(super::RenameOptionEvaluation { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
