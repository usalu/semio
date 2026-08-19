//! ↩️ Inverse (undo) construction for the `create-assumption` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💭assumptions` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateAssumption, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteAssumption(super::super::delete_assumption::mutation::DeleteAssumption { id: payload.assumption.header.id.clone() })]
}
