//! ↩️ `change-silo-plan-perimeter` inverse: restores the previous `plan_perimeter`.

use crate::change_silo_plan_perimeter::ChangeSiloPlanPerimeter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloPlanPerimeter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_plan_perimeter::ChangeSiloPlanPerimeter> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloPlanPerimeter { id: payload.id.clone(), new_plan_perimeter: row.plan_perimeter })],
        None => Vec::new(),
    })
}
