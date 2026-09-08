//! 🔺️ `delete-generation` sparse diff construction. `FormGeneration` carries no widget/synapse
//! reference, so removing a generation never cascades into the fixture.

use crate::standards::v1::subsets::any::schema::diff::{diff_generation_from_ops, Generation3dDiff};
use crate::standards::v1::subsets::any::schema::mutations::delete_generation::DeleteGeneration;
use crate::Generation3dSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationMutation;

pub fn diff(payload: &DeleteGeneration, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if !base.generation.generations.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, vec![GenerationMutation::Remove { id: payload.id.clone() }]))
}
