//! 🔺️ `rename-generation` sparse diff construction. `FormGeneration.name` is a plain display
//! label, not a key (`id` is the only key), so no name-collision Fatal check applies here.

use crate::standards::v1::subsets::any::schema::diff::{diff_generation_from_ops, Generation3dDiff};
use crate::standards::v1::subsets::any::schema::mutations::rename_generation::RenameGeneration;
use crate::Generation3dSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationMutation;

pub fn diff(payload: &RenameGeneration, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let Some(existing) = base.generation.generations.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warn("mutation.no-op", format!("Generation \"{}\" is already named \"{}\".", payload.id, payload.new_name));
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, &[GenerationMutation::Rename { id: payload.id.clone(), name: payload.new_name.clone() }]))
}
