//! ↩️ Inverse (undo) construction for the `create-sustainability-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `♻️sustainability` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateSustainabilityRequirement, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteSustainabilityRequirement(super::super::delete_sustainability_requirement::DeleteSustainabilityRequirement { id: payload.sustainability_requirement.header.id.clone() })]

    })())
}
