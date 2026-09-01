//! ↩️ Inverse (undo) construction for the `rename-environmental-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🌿environmental` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::RenameEnvironmentalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.environmental.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameEnvironmentalRequirement(super::RenameEnvironmentalRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
