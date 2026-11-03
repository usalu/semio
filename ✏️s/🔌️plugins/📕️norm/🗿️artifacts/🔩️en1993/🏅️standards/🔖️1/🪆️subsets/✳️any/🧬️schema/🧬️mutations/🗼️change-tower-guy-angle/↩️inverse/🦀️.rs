//! ↩️ `change-tower-guy-angle` inverse: restores the previous `guy_angle`.

use crate::change_tower_guy_angle::ChangeTowerGuyAngle;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerGuyAngle, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_guy_angle::ChangeTowerGuyAngle> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerGuyAngle { id: payload.id.clone(), new_guy_angle: row.guy_angle })],
        None => Vec::new(),
    })
}
