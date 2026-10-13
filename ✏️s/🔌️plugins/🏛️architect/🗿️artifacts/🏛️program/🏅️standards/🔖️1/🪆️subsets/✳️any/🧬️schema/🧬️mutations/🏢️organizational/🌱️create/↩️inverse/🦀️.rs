//! ↩️ Inverse (undo) construction for the `create-organizational-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏢organizational` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateOrganizationalRequirement, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteOrganizationalRequirement(super::super::delete_organizational_requirement::DeleteOrganizationalRequirement { id: payload.organizational_requirement.header.id.clone() })]

    })())
}
