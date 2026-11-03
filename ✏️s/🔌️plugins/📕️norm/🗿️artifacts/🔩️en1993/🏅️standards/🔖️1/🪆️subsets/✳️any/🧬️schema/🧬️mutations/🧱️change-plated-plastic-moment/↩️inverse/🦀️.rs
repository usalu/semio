//! ↩️ `change-plated-plastic-moment` inverse: restores the previous `plastic_moment`.

use crate::change_plated_plastic_moment::ChangePlatedPlasticMoment;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedPlasticMoment, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_plastic_moment::ChangePlatedPlasticMoment> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedPlasticMoment { id: payload.id.clone(), new_plastic_moment: row.plastic_moment })],
        None => Vec::new(),
    })
}
