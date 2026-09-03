//! ↩️ `change-generation-value` inverse — old value looked up from BASE (defaulting to `Value::Null`
//! for a question with no prior answer, matching `flow::playbook::invert_generation_operation`'s
//! own `UpdateValues` rule); missing generation ⇒ nothing to undo.

use crate::artifacts::generation3d::mutations::change_generation_value::ChangeGenerationValue;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use serde_json::Value;

pub fn inverse(payload: &ChangeGenerationValue, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    base.generation
        .generations
        .iter()
        .find(|entry| entry.id == payload.id)
        .map(|entry| vec![Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue { id: payload.id.clone(), question_id: payload.question_id.clone(), new_value: entry.values.get(&payload.question_id).cloned().unwrap_or(Value::Null) })])
        .unwrap_or_default()
}
