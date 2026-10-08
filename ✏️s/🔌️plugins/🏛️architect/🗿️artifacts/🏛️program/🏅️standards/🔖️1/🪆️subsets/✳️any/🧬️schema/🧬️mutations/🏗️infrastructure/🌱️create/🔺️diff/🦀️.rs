//! 🔺️ Sparse diff construction for the `create-infrastructure-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏗️infrastructure` per Wave C.

use super::CreateInfrastructureRequirement;
use crate::diff::ProgramInfrastructureDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateInfrastructureRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.infrastructure_requirement.header.id;
    if base.infrastructure.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An infrastructure requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.infrastructure.len());
    if at > base.infrastructure.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the infrastructure requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta::insertion(at, payload.infrastructure_requirement.clone())), ..Default::default() })
}
