//! ↩️ `change-fatigue-model-model` inverse: restores the previous `model`.

use crate::change_fatigue_model_model::ChangeFatigueModelModel;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueModelModel, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_model_model::ChangeFatigueModelModel> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFatigueModelModel, base: &En1991Snapshot) -> Option<ChangeFatigueModelModel> {
    let r0 = base.fatigue_models.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFatigueModelModel { id: payload.id.clone(), new_model: r0.model.clone() })
}
