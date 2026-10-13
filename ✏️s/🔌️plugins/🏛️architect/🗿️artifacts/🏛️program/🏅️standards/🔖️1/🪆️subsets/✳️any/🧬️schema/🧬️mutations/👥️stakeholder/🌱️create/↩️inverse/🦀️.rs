//! ↩️ Inverse (undo) construction for the `create-stakeholder` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `👥stakeholders` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateStakeholder, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteStakeholder(super::super::delete_stakeholder::DeleteStakeholder { id: payload.stakeholder.header.id.clone() })]

    })())
}
