//! ↩️ Inverse (undo) construction for the `create-program-element` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧱elements` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateProgramElement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteProgramElement(super::super::delete_program_element::mutation::DeleteProgramElement { id: payload.program_element.header.id.clone() })]
}
