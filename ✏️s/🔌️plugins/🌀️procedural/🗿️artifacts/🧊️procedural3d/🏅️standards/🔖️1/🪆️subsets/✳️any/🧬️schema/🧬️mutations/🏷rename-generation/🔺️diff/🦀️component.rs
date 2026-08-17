//! 🔺️ `rename-generation` sparse diff construction. `FormGeneration.name` is a plain display
//! label, not a key (`id` is the only key), so no name-collision Fatal check applies here.

use crate::artifacts::procedural3d::diff::{diff_generation_from_ops, Procedural3dDiff};
use crate::artifacts::procedural3d::mutations::rename_generation::mutation::RenameGeneration;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::playbook::GenerationMutation;

pub fn diff(payload: &RenameGeneration, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
    let Some(existing) = base.generation.generations.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::new(Procedural3dDiff::default()).warn("mutation.no-op", format!("Generation \"{}\" is already named \"{}\".", payload.id, payload.new_name));
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, vec![GenerationMutation::Rename { id: payload.id.clone(), name: payload.new_name.clone() }]))
}
