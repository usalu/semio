//! ↩️ `change-system-v-rd-n` inverse: restores the previous `base_shear_resistance_n`.

use crate::change_system_v_rd_n::ChangeSystemVRdN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSystemVRdN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_system_v_rd_n::ChangeSystemVRdN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSystemVRdN, base: &En1998Snapshot) -> Option<ChangeSystemVRdN> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSystemVRdN { building_id: payload.building_id.clone(), id: payload.id.clone(), new_base_shear_resistance_n: r1.base_shear_resistance_n })
}
