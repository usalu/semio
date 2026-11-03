//! ↩️ `change-pier-height-m` inverse: restores the previous `height_m`.

use crate::change_pier_height_m::ChangePierHeightM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierHeightM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_height_m::ChangePierHeightM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierHeightM, base: &En1998Snapshot) -> Option<ChangePierHeightM> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierHeightM { id: payload.id.clone(), new_height_m: r0.height_m })
}
