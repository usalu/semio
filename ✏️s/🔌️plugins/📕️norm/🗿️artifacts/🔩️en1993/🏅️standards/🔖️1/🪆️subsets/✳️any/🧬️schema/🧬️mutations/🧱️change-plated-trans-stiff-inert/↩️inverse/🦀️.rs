//! ↩️ `change-plated-trans-stiff-inert` inverse: restores the previous `transverse_stiffener_inertia`.

use crate::change_plated_trans_stiff_inert::ChangePlatedTransStiffInert;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedTransStiffInert, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_trans_stiff_inert::ChangePlatedTransStiffInert> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedTransStiffInert { id: payload.id.clone(), new_transverse_stiffener_inertia: row.transverse_stiffener_inertia })],
        None => Vec::new(),
    })
}
