//! ↩️ `change-plated-flange-moment` inverse: restores the previous `flange_moment`.

use crate::change_plated_flange_moment::ChangePlatedFlangeMoment;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedFlangeMoment, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_flange_moment::ChangePlatedFlangeMoment> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedFlangeMoment { id: payload.id.clone(), new_flange_moment: row.flange_moment })],
        None => Vec::new(),
    })
}
