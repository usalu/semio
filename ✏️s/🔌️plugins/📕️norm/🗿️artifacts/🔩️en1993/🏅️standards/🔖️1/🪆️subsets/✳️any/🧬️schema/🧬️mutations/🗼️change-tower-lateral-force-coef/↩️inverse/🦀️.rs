//! ↩️ `change-tower-lateral-force-coef` inverse: restores the previous `lateral_force_coefficient`.

use crate::change_tower_lateral_force_coef::ChangeTowerLateralForceCoef;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerLateralForceCoef, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_lateral_force_coef::ChangeTowerLateralForceCoef> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerLateralForceCoef { id: payload.id.clone(), new_lateral_force_coefficient: row.lateral_force_coefficient })],
        None => Vec::new(),
    })
}
