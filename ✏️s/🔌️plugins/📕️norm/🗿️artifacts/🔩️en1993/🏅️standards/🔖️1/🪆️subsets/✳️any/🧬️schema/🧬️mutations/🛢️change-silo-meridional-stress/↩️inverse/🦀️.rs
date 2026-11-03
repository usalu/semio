//! ↩️ `change-silo-meridional-stress` inverse: restores the previous `meridional_stress`.

use crate::change_silo_meridional_stress::ChangeSiloMeridionalStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloMeridionalStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_meridional_stress::ChangeSiloMeridionalStress> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloMeridionalStress { id: payload.id.clone(), new_meridional_stress: row.meridional_stress })],
        None => Vec::new(),
    })
}
