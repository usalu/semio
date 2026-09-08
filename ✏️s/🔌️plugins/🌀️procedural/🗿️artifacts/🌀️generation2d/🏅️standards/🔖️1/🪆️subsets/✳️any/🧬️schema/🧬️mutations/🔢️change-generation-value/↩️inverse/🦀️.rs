//! ↩️ Inverse for `ChangeGenerationValue`, reconstructed from BASE.
use super::ChangeGenerationValue;
use crate::standards::v1::subsets::any::schema::mutations::change_generation_value;
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeGenerationValue, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match base.generation.generations.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![change_generation_value(payload.id.clone(), payload.question_id.clone(), entry.values.get(&payload.question_id).cloned().unwrap_or(dsl::DslValue::Null))],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
