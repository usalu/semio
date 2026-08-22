//! 🔺️ Sparse diff for `DeleteGeneration`, built directly from `(payload, base)`.
use super::mutation::DeleteGeneration;
use crate::artifacts::procedural2d::diff::diff_generation_from_ops;
use crate::artifacts::procedural2d::{Procedural2dDiff, Procedural2dSnapshot};
use flow::playbook::GenerationMutation;

//#region 🔖️Diff
pub fn diff(payload: &DeleteGeneration, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
    if !base.generation.generations.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, vec![GenerationMutation::Remove { id: payload.id.clone() }]))
}
//#endregion 🔖️Diff
