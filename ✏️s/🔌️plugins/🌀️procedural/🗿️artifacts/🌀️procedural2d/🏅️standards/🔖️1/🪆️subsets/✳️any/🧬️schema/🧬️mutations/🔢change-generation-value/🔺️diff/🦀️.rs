//! 🔺️ Sparse diff for `ChangeGenerationValue`, built directly from `(payload, base)`.
use super::ChangeGenerationValue;
use crate::artifacts::procedural2d::diff::diff_generation_from_ops;
use crate::artifacts::procedural2d::{Procedural2dDiff, Procedural2dSnapshot};
use flow::playbook::GenerationMutation;

//#region 🔖️Diff
pub fn diff(payload: &ChangeGenerationValue, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
    let Some(entry) = base.generation.generations.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if entry.values.get(&payload.question_id) == Some(&payload.value) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Generation \"{}\" question \"{}\" already has this value.", payload.id, payload.question_id));
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, vec![GenerationMutation::UpdateValues { id: payload.id.clone(), question_id: payload.question_id.clone(), value: payload.value.clone() }]))
}
//#endregion 🔖️Diff
