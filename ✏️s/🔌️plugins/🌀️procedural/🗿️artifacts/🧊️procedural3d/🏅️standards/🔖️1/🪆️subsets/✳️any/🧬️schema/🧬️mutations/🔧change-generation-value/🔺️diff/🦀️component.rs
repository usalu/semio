//! 🔺️ `change-generation-value` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_generation_from_ops, Procedural3dDiff};
use crate::artifacts::procedural3d::mutations::change_generation_value::mutation::ChangeGenerationValue;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::playbook::GenerationMutation;

pub fn diff(payload: &ChangeGenerationValue, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_generation_from_ops(base, vec![GenerationMutation::UpdateValues { id: payload.id.clone(), question_id: payload.question_id.clone(), value: payload.new_value.clone() }])
}
