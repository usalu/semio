//! ↩️ `change-silo-v-rd-n` inverse: restores the previous `v_rd_n`.

use crate::change_silo_v_rd_n::ChangeSiloVRdN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloVRdN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_v_rd_n::ChangeSiloVRdN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSiloVRdN, base: &En1998Snapshot) -> Option<ChangeSiloVRdN> {
    let r0 = base.silos.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSiloVRdN { id: payload.id.clone(), new_v_rd_n: r0.v_rd_n })
}
