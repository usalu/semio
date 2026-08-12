//! ↩️ Inverse (undo) construction for the `create-workshop` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🎓workshops` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateWorkshop, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteWorkshop(super::super::delete_workshop::mutation::DeleteWorkshop { id: payload.workshop.header.id.clone() })]
}
