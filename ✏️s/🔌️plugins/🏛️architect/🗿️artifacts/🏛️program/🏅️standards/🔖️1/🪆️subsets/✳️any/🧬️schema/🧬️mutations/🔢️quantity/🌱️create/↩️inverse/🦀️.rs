//! ↩️ Inverse (undo) construction for the `create-quantity-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔢quantities` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateQuantityRequirement, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteQuantityRequirement(super::super::delete_quantity_requirement::DeleteQuantityRequirement { id: payload.quantity_requirement.header.id.clone() })]

    })())
}
