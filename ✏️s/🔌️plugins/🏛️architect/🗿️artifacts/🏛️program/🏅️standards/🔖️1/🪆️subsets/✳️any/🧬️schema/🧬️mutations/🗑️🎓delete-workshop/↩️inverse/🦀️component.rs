//! ↩️ Inverse (undo) construction for the `delete-workshop` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🎓workshops` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteWorkshop, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.workshops.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateWorkshop(super::super::create_workshop::mutation::CreateWorkshop { workshop: existing.clone() })],
        None => Vec::new(),
    }
}
