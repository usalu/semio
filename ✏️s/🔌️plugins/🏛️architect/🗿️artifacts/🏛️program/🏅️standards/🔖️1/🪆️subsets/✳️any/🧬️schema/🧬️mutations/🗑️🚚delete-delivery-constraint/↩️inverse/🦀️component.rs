//! ↩️ Inverse (undo) construction for the `delete-delivery-constraint` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🚚delivery` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteDeliveryConstraint, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.delivery.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateDeliveryConstraint(super::super::create_delivery_constraint::mutation::CreateDeliveryConstraint { delivery_constraint: existing.clone() })],
        None => Vec::new(),
    }
}
