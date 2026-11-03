//! ↩️ `change-fatigue-model-assumed-axle-load` inverse: restores the previous `assumed_axle_load`.

use crate::change_fatigue_model_assumed_axle_load::ChangeFatigueModelAssumedAxleLoad;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueModelAssumedAxleLoad, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_model_assumed_axle_load::ChangeFatigueModelAssumedAxleLoad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFatigueModelAssumedAxleLoad, base: &En1991Snapshot) -> Option<ChangeFatigueModelAssumedAxleLoad> {
    let r0 = base.fatigue_models.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFatigueModelAssumedAxleLoad { id: payload.id.clone(), new_assumed_axle_load: r0.assumed_axle_load })
}
