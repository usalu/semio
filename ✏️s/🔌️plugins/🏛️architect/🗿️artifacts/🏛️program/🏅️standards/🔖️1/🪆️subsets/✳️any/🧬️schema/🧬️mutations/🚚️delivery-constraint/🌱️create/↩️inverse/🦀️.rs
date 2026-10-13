//! ↩️ Inverse (undo) construction for the `create-delivery-constraint` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🚚delivery` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateDeliveryConstraint, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteDeliveryConstraint(super::super::delete_delivery_constraint::DeleteDeliveryConstraint { id: payload.delivery_constraint.header.id.clone() })]

    })())
}
