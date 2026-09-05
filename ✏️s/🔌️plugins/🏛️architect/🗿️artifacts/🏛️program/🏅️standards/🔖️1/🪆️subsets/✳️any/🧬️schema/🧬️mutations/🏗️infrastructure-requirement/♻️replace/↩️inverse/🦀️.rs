//! ↩️ Inverse (undo) construction for the `replace-infrastructure-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏗️infrastructure` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceInfrastructureRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.infrastructure.iter().find(|row| row.header.id == payload.infrastructure_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceInfrastructureRequirement(super::ReplaceInfrastructureRequirement { infrastructure_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
