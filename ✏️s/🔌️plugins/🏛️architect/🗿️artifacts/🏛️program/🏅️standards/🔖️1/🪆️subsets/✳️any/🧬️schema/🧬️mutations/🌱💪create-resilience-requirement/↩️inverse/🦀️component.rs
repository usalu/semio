//! ↩️ Inverse (undo) construction for the `create-resilience-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💪resilience` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateResilienceRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteResilienceRequirement(super::super::delete_resilience_requirement::mutation::DeleteResilienceRequirement { id: payload.resilience_requirement.header.id.clone() })]
}
