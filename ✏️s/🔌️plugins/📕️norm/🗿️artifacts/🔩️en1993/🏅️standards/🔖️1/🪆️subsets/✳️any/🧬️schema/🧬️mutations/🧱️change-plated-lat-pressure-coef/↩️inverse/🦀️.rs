//! ↩️ `change-plated-lat-pressure-coef` inverse: restores the previous `lateral_pressure_coefficient`.

use crate::change_plated_lat_pressure_coef::ChangePlatedLatPressureCoef;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedLatPressureCoef, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_lat_pressure_coef::ChangePlatedLatPressureCoef> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedLatPressureCoef { id: payload.id.clone(), new_lateral_pressure_coefficient: row.lateral_pressure_coefficient })],
        None => Vec::new(),
    })
}
