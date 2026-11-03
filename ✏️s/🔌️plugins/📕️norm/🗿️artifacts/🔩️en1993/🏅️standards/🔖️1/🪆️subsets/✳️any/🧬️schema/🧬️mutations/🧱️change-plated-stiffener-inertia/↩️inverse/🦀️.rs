//! ↩️ `change-plated-stiffener-inertia` inverse: restores the previous `stiffener_inertia`.

use crate::change_plated_stiffener_inertia::ChangePlatedStiffenerInertia;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStiffenerInertia, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stiffener_inertia::ChangePlatedStiffenerInertia> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStiffenerInertia { id: payload.id.clone(), new_stiffener_inertia: row.stiffener_inertia })],
        None => Vec::new(),
    })
}
