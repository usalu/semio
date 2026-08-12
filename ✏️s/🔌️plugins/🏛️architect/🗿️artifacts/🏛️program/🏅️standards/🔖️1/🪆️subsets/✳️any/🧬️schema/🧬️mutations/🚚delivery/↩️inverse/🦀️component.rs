//! ↩️ Inverse (undo) construction for the `delivery` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateDeliveryConstraint, DeleteDeliveryConstraint, RenameDeliveryConstraint, ReplaceDeliveryConstraint};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateDeliveryConstraint, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteDeliveryConstraint(DeleteDeliveryConstraint { id: payload.delivery_constraint.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteDeliveryConstraint, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.delivery.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateDeliveryConstraint(CreateDeliveryConstraint { delivery_constraint: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameDeliveryConstraint, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.delivery.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameDeliveryConstraint(RenameDeliveryConstraint { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceDeliveryConstraint, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.delivery.iter().find(|row| row.header.id == payload.delivery_constraint.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceDeliveryConstraint(ReplaceDeliveryConstraint { delivery_constraint: existing.clone() })],
        None => Vec::new(),
    }
}
