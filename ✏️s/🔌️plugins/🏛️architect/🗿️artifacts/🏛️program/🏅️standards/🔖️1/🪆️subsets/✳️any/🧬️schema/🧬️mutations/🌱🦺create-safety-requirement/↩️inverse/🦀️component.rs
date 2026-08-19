//! ↩️ Inverse (undo) construction for the `create-safety-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🦺safety` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateSafetyRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteSafetyRequirement(super::super::delete_safety_requirement::mutation::DeleteSafetyRequirement { id: payload.safety_requirement.header.id.clone() })]
}
