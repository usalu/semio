//! ↩️ `change-tension-rod-stress-area` inverse: restores the previous `rod_stress_area`.

use crate::change_tension_rod_stress_area::ChangeTensionRodStressArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionRodStressArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_rod_stress_area::ChangeTensionRodStressArea> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionRodStressArea { id: payload.id.clone(), new_rod_stress_area: row.rod_stress_area })],
        None => Vec::new(),
    })
}
