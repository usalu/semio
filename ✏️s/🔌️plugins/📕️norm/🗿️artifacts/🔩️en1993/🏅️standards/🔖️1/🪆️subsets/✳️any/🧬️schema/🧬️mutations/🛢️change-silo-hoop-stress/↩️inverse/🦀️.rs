//! ↩️ `change-silo-hoop-stress` inverse: restores the previous `hoop_stress`.

use crate::change_silo_hoop_stress::ChangeSiloHoopStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloHoopStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_hoop_stress::ChangeSiloHoopStress> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloHoopStress { id: payload.id.clone(), new_hoop_stress: row.hoop_stress })],
        None => Vec::new(),
    })
}
