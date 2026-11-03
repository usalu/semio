//! ↩️ `change-plated-longitudinal-inertia` inverse: restores the previous `longitudinal_inertia`.

use crate::change_plated_longitudinal_inertia::ChangePlatedLongitudinalInertia;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedLongitudinalInertia, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_longitudinal_inertia::ChangePlatedLongitudinalInertia> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedLongitudinalInertia { id: payload.id.clone(), new_longitudinal_inertia: row.longitudinal_inertia })],
        None => Vec::new(),
    })
}
