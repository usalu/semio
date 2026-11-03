//! ↩️ `change-fatigue-model-lane` inverse: restores the previous `lane`.

use crate::change_fatigue_model_lane::ChangeFatigueModelLane;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueModelLane, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_model_lane::ChangeFatigueModelLane> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFatigueModelLane, base: &En1991Snapshot) -> Option<ChangeFatigueModelLane> {
    let r0 = base.fatigue_models.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFatigueModelLane { id: payload.id.clone(), new_lane: r0.lane })
}
