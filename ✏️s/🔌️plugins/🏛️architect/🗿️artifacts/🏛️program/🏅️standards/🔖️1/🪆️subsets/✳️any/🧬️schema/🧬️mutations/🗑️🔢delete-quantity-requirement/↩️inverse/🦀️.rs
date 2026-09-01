//! ↩️ Inverse (undo) construction for the `delete-quantity-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔢quantities` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteQuantityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.quantities.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateQuantityRequirement(super::super::create_quantity_requirement::CreateQuantityRequirement { quantity_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
