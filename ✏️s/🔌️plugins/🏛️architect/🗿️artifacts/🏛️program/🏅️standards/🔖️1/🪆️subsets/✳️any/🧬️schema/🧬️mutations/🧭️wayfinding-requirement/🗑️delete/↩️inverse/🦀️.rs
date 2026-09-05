//! ↩️ Inverse (undo) construction for the `delete-wayfinding-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧭wayfinding` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteWayfindingRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.wayfinding.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateWayfindingRequirement(super::super::create_wayfinding_requirement::CreateWayfindingRequirement { wayfinding_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
