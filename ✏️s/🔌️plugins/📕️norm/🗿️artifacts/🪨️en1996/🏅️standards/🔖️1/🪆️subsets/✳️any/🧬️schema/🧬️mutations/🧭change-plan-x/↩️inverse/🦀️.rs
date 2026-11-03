//! ↩️ `change-plan-x` inverse: restores the previous `plan_x_m`.

use crate::change_plan_x::ChangePlanX;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangePlanX, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plan_x::ChangePlanX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePlanX, base: &En1996Snapshot) -> Option<ChangePlanX> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangePlanX { id: payload.id.clone(), new_plan_x_m: r0.plan_x_m })
}
