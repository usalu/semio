//! ↩️ `change-plated-stiffener-eccentricity` inverse: restores the previous `stiffener_eccentricity`.

use crate::change_plated_stiffener_eccentricity::ChangePlatedStiffenerEccentricity;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStiffenerEccentricity, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stiffener_eccentricity::ChangePlatedStiffenerEccentricity> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStiffenerEccentricity { id: payload.id.clone(), new_stiffener_eccentricity: row.stiffener_eccentricity })],
        None => Vec::new(),
    })
}
