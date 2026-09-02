//! ↩️ Inverse for `ChangeGenerationValue`, reconstructed from BASE.
use super::ChangeGenerationValue;
use crate::artifacts::procedural2d::mutations::change_generation_value;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeGenerationValue, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    match base.generation.generations.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![change_generation_value(payload.id.clone(), payload.question_id.clone(), entry.values.get(&payload.question_id).cloned().unwrap_or(serde_json::Value::Null))],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
