//! ↩️ `change-plated-stiffener-shape` inverse: restores the previous `stiffener_shape`.

use crate::change_plated_stiffener_shape::ChangePlatedStiffenerShape;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStiffenerShape, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stiffener_shape::ChangePlatedStiffenerShape> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStiffenerShape { id: payload.id.clone(), new_stiffener_shape: row.stiffener_shape })],
        None => Vec::new(),
    })
}
