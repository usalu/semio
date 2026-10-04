//! ↩️ Inverse (undo) construction for the `create-assumption` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💭assumptions` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateAssumption, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteAssumption(super::super::delete_assumption::DeleteAssumption { id: payload.assumption.header.id.clone() })]

    })())
}
