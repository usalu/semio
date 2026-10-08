//! 🔺️ Sparse diff construction for the `create-service-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛎️services` per Wave C.

use super::CreateServiceRequirement;
use crate::diff::ProgramServicesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateServiceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.service_requirement.header.id;
    if base.services.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A service requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.services.len());
    if at > base.services.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the service requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { services: Some(ProgramServicesDelta::insertion(at, payload.service_requirement.clone())), ..Default::default() })
}
