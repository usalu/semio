//! ↩️ `change-plated-stiffener-flange-thk` inverse: restores the previous `stiffener_flange_thickness`.

use crate::change_plated_stiffener_flange_thk::ChangePlatedStiffenerFlangeThk;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStiffenerFlangeThk, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stiffener_flange_thk::ChangePlatedStiffenerFlangeThk> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStiffenerFlangeThk { id: payload.id.clone(), new_stiffener_flange_thickness: row.stiffener_flange_thickness })],
        None => Vec::new(),
    })
}
