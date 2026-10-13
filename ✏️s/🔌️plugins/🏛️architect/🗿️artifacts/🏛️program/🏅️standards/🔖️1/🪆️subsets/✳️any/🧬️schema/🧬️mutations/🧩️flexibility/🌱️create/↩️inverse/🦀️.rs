//! ↩️ Inverse (undo) construction for the `create-flexibility-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧩flexibility` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateFlexibilityRequirement, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteFlexibilityRequirement(super::super::delete_flexibility_requirement::DeleteFlexibilityRequirement { id: payload.flexibility_requirement.header.id.clone() })]

    })())
}
