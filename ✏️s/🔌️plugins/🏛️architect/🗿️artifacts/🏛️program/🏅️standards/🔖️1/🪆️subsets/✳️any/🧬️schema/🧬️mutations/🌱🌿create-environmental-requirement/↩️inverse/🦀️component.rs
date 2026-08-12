//! ↩️ Inverse (undo) construction for the `create-environmental-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🌿environmental` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateEnvironmentalRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteEnvironmentalRequirement(super::super::delete_environmental_requirement::mutation::DeleteEnvironmentalRequirement { id: payload.environmental_requirement.header.id.clone() })]
}
