//! 🔺️ Sparse diff for `RenameGeneration`, built directly from `(payload, base)`.
use super::mutation::RenameGeneration;
use crate::artifacts::procedural2d::diff::diff_generation_from_ops;
use crate::artifacts::procedural2d::{Procedural2dDiff, Procedural2dSnapshot};
use flow::playbook::GenerationMutation;

//#region 🔖️Diff
pub fn diff(payload: &RenameGeneration, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
    let Some(entry) = base.generation.generations.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if entry.name == payload.name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Generation \"{}\" is already named \"{}\".", payload.id, payload.name));
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, vec![GenerationMutation::Rename { id: payload.id.clone(), name: payload.name.clone() }]))
}
//#endregion 🔖️Diff
