//! ↩️ `change-silo-shell-shear-stress` inverse: restores the previous `shell_shear_stress`.

use crate::change_silo_shell_shear_stress::ChangeSiloShellShearStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloShellShearStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_shell_shear_stress::ChangeSiloShellShearStress> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloShellShearStress { id: payload.id.clone(), new_shell_shear_stress: row.shell_shear_stress })],
        None => Vec::new(),
    })
}
