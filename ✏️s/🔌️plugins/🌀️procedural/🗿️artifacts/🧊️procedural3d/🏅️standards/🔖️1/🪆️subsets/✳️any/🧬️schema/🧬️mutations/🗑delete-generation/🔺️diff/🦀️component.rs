//! 🔺️ `delete-generation` sparse diff construction. `FormGeneration` carries no widget/synapse
//! reference, so removing a generation never cascades into the fixture.

use crate::artifacts::procedural3d::diff::{diff_generation_from_ops, Procedural3dDiff};
use crate::artifacts::procedural3d::mutations::delete_generation::mutation::DeleteGeneration;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::playbook::GenerationMutation;

pub fn diff(payload: &DeleteGeneration, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
    if !base.generation.generations.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, vec![GenerationMutation::Remove { id: payload.id.clone() }]))
}
