//! ↩️ `change-silo-n-rd-n` inverse: restores the previous `n_rd_n`.

use crate::change_silo_n_rd_n::ChangeSiloNRdN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloNRdN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_n_rd_n::ChangeSiloNRdN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSiloNRdN, base: &En1998Snapshot) -> Option<ChangeSiloNRdN> {
    let r0 = base.silos.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSiloNRdN { id: payload.id.clone(), new_n_rd_n: r0.n_rd_n })
}
