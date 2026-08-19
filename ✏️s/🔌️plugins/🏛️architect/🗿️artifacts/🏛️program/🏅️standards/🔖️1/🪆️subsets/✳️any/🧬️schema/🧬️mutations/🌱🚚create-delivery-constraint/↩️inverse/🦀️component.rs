//! ↩️ Inverse (undo) construction for the `create-delivery-constraint` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🚚delivery` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateDeliveryConstraint, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteDeliveryConstraint(super::super::delete_delivery_constraint::mutation::DeleteDeliveryConstraint { id: payload.delivery_constraint.header.id.clone() })]
}
