//! ↩️ `change-material-ductility-ultimate-strain` inverse: restores the previous `ultimate_strain`.

use crate::change_material_ductility_ultimate_strain::ChangeMaterialDuctilityUltimateStrain;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMaterialDuctilityUltimateStrain, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_material_ductility_ultimate_strain::ChangeMaterialDuctilityUltimateStrain> {
    Ok(match base.material_ductility.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMaterialDuctilityUltimateStrain { id: payload.id.clone(), new_ultimate_strain: row.ultimate_strain })],
        None => Vec::new(),
    })
}
