//! ↩️ Inverse (undo) construction for the `create-service-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛎️services` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateServiceRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteServiceRequirement(super::super::delete_service_requirement::mutation::DeleteServiceRequirement { id: payload.service_requirement.header.id.clone() })]
}
