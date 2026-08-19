//! ↩️ Inverse (undo) construction for the `replace-workshop` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🎓workshops` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceWorkshop, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.workshops.iter().find(|row| row.header.id == payload.workshop.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceWorkshop(super::mutation::ReplaceWorkshop { workshop: existing.clone() })],
        None => Vec::new(),
    }
}
