//! ↩️ `change-material-ramberg-osgood-n` inverse: restores the previous `ramberg_osgood_n`.

use crate::change_material_ramberg_osgood_n::ChangeMaterialRambergOsgoodN;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialRambergOsgoodN, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_ramberg_osgood_n::ChangeMaterialRambergOsgoodN> {
    Ok(match base.materials.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMaterialRambergOsgoodN { id: payload.id.clone(), new_ramberg_osgood_n: row.ramberg_osgood_n })],
        None => Vec::new(),
    })
}
