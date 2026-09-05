//! ↩️ Inverse (undo) construction for the `replace-quantity-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔢quantities` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceQuantityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.quantities.iter().find(|row| row.header.id == payload.quantity_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceQuantityRequirement(super::ReplaceQuantityRequirement { quantity_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
