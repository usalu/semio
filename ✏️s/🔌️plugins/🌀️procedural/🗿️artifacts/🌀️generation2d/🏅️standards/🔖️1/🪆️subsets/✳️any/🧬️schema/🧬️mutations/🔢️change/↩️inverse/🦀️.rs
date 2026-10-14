//! ↩️ Inverse for `ChangeGenerationValue`, reconstructed from BASE.
use super::ChangeGenerationValue;
use crate::change_generation_value;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeGenerationValue>>(payload: &ChangeGenerationValue, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.generation.generations.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![change_generation_value(payload.id.clone(), payload.question_id.clone(), entry.values.get(&payload.question_id).cloned().unwrap_or(semio_framework_value::DslValue::Null))],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
