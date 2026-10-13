//! ↩️ Inverse (undo) construction for the `create-information-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `ℹ️information` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateInformationRequirement, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteInformationRequirement(super::super::delete_information_requirement::DeleteInformationRequirement { id: payload.information_requirement.header.id.clone() })]

    })())
}
