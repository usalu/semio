//! ↩️ `change-diaphragm-stiffness` inverse: restores the previous `diaphragm_stiffness_n_per_m`.

use crate::change_diaphragm_stiffness::ChangeDiaphragmStiffness;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeDiaphragmStiffness, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_stiffness::ChangeDiaphragmStiffness> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDiaphragmStiffness, base: &En1996Snapshot) -> Option<ChangeDiaphragmStiffness> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDiaphragmStiffness { id: payload.id.clone(), new_diaphragm_stiffness_n_per_m: r0.diaphragm_stiffness_n_per_m })
}
