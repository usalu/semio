//! 🔺️ Sparse diff for `DeleteGeneration`, built directly from `(payload, base)`.
use super::DeleteGeneration;
use crate::diff::diff_generation_from_ops;
use crate::{Generation2dDiff, Generation2dSnapshot};
use semio_framework_artifact_playbook_playbook::GenerationMutation;

//#region 🔖️Diff
pub fn diff(payload: &DeleteGeneration, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if !base.generation.generations.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, vec![GenerationMutation::Remove { id: payload.id.clone() }]))
}
//#endregion 🔖️Diff
