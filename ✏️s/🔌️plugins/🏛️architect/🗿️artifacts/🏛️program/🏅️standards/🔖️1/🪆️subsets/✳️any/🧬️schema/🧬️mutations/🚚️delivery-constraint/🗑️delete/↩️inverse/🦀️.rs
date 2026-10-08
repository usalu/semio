//! ↩️ Inverse (undo) construction for the `delete-delivery-constraint` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🚚delivery` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteDeliveryConstraint, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.delivery.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateDeliveryConstraint(super::super::create_delivery_constraint::CreateDeliveryConstraint { delivery_constraint: base.delivery[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
