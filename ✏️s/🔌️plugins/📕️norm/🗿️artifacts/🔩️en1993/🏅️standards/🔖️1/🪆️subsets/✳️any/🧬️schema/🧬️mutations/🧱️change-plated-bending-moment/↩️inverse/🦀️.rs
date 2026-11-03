//! ↩️ `change-plated-bending-moment` inverse: restores the previous `bending_moment`.

use crate::change_plated_bending_moment::ChangePlatedBendingMoment;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedBendingMoment, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_bending_moment::ChangePlatedBendingMoment> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedBendingMoment { id: payload.id.clone(), new_bending_moment: row.bending_moment })],
        None => Vec::new(),
    })
}
