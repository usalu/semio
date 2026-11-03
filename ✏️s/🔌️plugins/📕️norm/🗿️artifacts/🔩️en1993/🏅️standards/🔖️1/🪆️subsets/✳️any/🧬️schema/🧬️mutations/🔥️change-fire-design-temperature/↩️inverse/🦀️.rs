//! ↩️ `change-fire-design-temperature` inverse: restores the previous `design_temperature`.

use crate::change_fire_design_temperature::ChangeFireDesignTemperature;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFireDesignTemperature, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_design_temperature::ChangeFireDesignTemperature> {
    Ok(match base.fire_exposures.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFireDesignTemperature { id: payload.id.clone(), new_design_temperature: row.design_temperature })],
        None => Vec::new(),
    })
}
