//! ↩️ `change-tank-v-rd-n` inverse: restores the previous `v_rd_n`.

use crate::change_tank_v_rd_n::ChangeTankVRdN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTankVRdN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tank_v_rd_n::ChangeTankVRdN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTankVRdN, base: &En1998Snapshot) -> Option<ChangeTankVRdN> {
    let r0 = base.tanks.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTankVRdN { id: payload.id.clone(), new_v_rd_n: r0.v_rd_n })
}
