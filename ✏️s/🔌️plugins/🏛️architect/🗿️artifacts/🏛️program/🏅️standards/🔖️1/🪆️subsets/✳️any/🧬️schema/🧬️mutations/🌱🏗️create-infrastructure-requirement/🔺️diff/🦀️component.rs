//! 🔺️ Sparse diff construction for the `create-infrastructure-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏗️infrastructure` per Wave C.

use super::mutation::CreateInfrastructureRequirement;
use crate::artifacts::program::diff::ProgramInfrastructureDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateInfrastructureRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.infrastructure_requirement.header.id.clone();
    if base.infrastructure.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An infrastructure requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta { added: vec![payload.infrastructure_requirement.clone()], ..Default::default() }), ..Default::default() })
}
