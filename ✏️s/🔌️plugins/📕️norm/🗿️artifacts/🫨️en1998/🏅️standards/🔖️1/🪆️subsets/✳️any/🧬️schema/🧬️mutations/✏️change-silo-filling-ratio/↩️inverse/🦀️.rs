//! ↩️ `change-silo-filling-ratio` inverse: restores the previous `filling_ratio`.

use crate::change_silo_filling_ratio::ChangeSiloFillingRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloFillingRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_filling_ratio::ChangeSiloFillingRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSiloFillingRatio, base: &En1998Snapshot) -> Option<ChangeSiloFillingRatio> {
    let r0 = base.silos.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSiloFillingRatio { id: payload.id.clone(), new_filling_ratio: r0.filling_ratio })
}
