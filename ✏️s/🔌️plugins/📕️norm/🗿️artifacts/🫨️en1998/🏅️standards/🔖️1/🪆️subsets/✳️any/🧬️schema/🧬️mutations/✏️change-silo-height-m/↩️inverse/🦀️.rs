//! ↩️ `change-silo-height-m` inverse: restores the previous `height_m`.

use crate::change_silo_height_m::ChangeSiloHeightM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloHeightM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_height_m::ChangeSiloHeightM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSiloHeightM, base: &En1998Snapshot) -> Option<ChangeSiloHeightM> {
    let r0 = base.silos.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSiloHeightM { id: payload.id.clone(), new_height_m: r0.height_m })
}
