//! ↩️ `change-material-g-modulus` inverse: restores the previous `g_modulus`.

use crate::change_material_g_modulus::ChangeMaterialGModulus;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialGModulus, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_g_modulus::ChangeMaterialGModulus> {
    Ok(match base.materials.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMaterialGModulus { id: payload.id.clone(), new_g_modulus: row.g_modulus })],
        None => Vec::new(),
    })
}
