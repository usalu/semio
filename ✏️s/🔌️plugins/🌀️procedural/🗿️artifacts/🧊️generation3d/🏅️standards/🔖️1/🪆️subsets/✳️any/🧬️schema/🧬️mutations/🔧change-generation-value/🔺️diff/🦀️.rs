//! 🔺️ `change-generation-value` sparse diff construction.

use crate::artifacts::generation3d::diff::{diff_generation_from_ops, Generation3dDiff};
use crate::artifacts::generation3d::mutations::change_generation_value::ChangeGenerationValue;
use crate::artifacts::generation3d::Generation3dSnapshot;
use flow::playbook::GenerationMutation;

pub fn diff(payload: &ChangeGenerationValue, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let Some(existing) = base.generation.generations.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Generation \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.values.get(&payload.question_id) == Some(&payload.new_value) {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warn("mutation.no-op", format!("Generation \"{}\" question \"{}\" is already \"{}\".", payload.id, payload.question_id, payload.new_value));
    }
    protocol::MutationOutcome::new(diff_generation_from_ops(base, vec![GenerationMutation::UpdateValues { id: payload.id.clone(), question_id: payload.question_id.clone(), value: payload.new_value.clone() }]))
}
