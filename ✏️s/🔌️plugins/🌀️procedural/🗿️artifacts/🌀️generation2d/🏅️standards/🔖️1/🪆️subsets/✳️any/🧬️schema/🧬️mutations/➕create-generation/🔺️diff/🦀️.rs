//! 🔺️ Sparse diff for `CreateGeneration`, built directly from `(payload, base)`.
use super::CreateGeneration;
use crate::artifacts::generation2d::diff::diff_generation_from_ops;
use crate::artifacts::generation2d::{Generation2dDiff, Generation2dSnapshot};
use flow::playbook::GenerationMutation;

//#region 🔖️Diff
pub fn diff(payload: &CreateGeneration, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if base.generation.generations.iter().any(|entry| entry.id == payload.generation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A generation with id \"{}\" already exists.", payload.generation.id), [payload.generation.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, vec![GenerationMutation::Add { generation: payload.generation.clone() }]))
}
//#endregion 🔖️Diff
