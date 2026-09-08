//! ↩️ Inverse (undo) construction for the `replace-delivery-constraint` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🚚delivery` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceDeliveryConstraint, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.delivery.iter().find(|row| row.header.id == payload.delivery_constraint.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceDeliveryConstraint(super::ReplaceDeliveryConstraint { delivery_constraint: existing.clone() })],
        None => Vec::new(),
    }
}
