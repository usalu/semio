//! ↩️ `change-silo-plan-area` inverse: restores the previous `plan_area`.

use crate::change_silo_plan_area::ChangeSiloPlanArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloPlanArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_plan_area::ChangeSiloPlanArea> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloPlanArea { id: payload.id.clone(), new_plan_area: row.plan_area })],
        None => Vec::new(),
    })
}
