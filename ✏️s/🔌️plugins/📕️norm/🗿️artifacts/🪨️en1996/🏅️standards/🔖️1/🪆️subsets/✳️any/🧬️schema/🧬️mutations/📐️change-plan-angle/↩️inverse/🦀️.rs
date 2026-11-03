//! ↩️ `change-plan-angle` inverse: restores the previous `plan_angle_deg`.

use crate::change_plan_angle::ChangePlanAngle;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangePlanAngle, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plan_angle::ChangePlanAngle> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePlanAngle, base: &En1996Snapshot) -> Option<ChangePlanAngle> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangePlanAngle { id: payload.id.clone(), new_plan_angle_deg: r0.plan_angle_deg })
}
