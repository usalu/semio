//! ↩️ `change-material-e-modulus` inverse: restores the previous `e_modulus`.

use crate::change_material_e_modulus::ChangeMaterialEModulus;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialEModulus, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_e_modulus::ChangeMaterialEModulus> {
    Ok(match base.materials.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMaterialEModulus { id: payload.id.clone(), new_e_modulus: row.e_modulus })],
        None => Vec::new(),
    })
}
