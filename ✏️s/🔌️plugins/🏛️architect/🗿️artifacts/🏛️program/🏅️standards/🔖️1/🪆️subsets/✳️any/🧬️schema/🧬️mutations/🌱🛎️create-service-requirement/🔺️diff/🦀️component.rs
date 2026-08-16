//! 🔺️ Sparse diff construction for the `create-service-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛎️services` per Wave C.

use super::mutation::CreateServiceRequirement;
use crate::artifacts::program::diff::ProgramServicesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateServiceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.service_requirement.header.id.clone();
    if base.services.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A service requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { services: Some(ProgramServicesDelta { added: vec![payload.service_requirement.clone()], ..Default::default() }), ..Default::default() })
}
