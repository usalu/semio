//! ↩️ Inverse (undo) construction for the `delete-environmental-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🌿environmental` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteEnvironmentalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.environmental.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateEnvironmentalRequirement(super::super::create_environmental_requirement::mutation::CreateEnvironmentalRequirement { environmental_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
