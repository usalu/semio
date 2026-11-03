//! ↩️ `change-plan-y` inverse: restores the previous `plan_y_m`.

use crate::change_plan_y::ChangePlanY;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangePlanY, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plan_y::ChangePlanY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePlanY, base: &En1996Snapshot) -> Option<ChangePlanY> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangePlanY { id: payload.id.clone(), new_plan_y_m: r0.plan_y_m })
}
