//! ↩️ `change-diaphragm-shear-str-mpa` inverse: restores the previous `shear_strength_mpa`.

use crate::change_diaphragm_shear_str_mpa::ChangeDiaphragmShearStrMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeDiaphragmShearStrMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_shear_str_mpa::ChangeDiaphragmShearStrMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDiaphragmShearStrMpa, base: &En1998Snapshot) -> Option<ChangeDiaphragmShearStrMpa> {
    let r0 = base.diaphragms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDiaphragmShearStrMpa { id: payload.id.clone(), new_shear_strength_mpa: r0.shear_strength_mpa })
}
