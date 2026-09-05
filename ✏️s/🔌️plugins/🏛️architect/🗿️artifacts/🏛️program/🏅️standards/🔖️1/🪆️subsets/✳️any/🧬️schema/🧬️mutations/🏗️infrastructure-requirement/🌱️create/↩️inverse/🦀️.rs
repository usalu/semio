//! ↩️ Inverse (undo) construction for the `create-infrastructure-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏗️infrastructure` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::CreateInfrastructureRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteInfrastructureRequirement(super::super::delete_infrastructure_requirement::DeleteInfrastructureRequirement { id: payload.infrastructure_requirement.header.id.clone() })]
}
