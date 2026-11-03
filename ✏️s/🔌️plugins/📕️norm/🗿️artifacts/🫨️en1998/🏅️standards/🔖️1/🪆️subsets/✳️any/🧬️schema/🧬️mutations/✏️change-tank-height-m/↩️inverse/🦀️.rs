//! ↩️ `change-tank-height-m` inverse: restores the previous `height_m`.

use crate::change_tank_height_m::ChangeTankHeightM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTankHeightM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tank_height_m::ChangeTankHeightM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTankHeightM, base: &En1998Snapshot) -> Option<ChangeTankHeightM> {
    let r0 = base.tanks.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTankHeightM { id: payload.id.clone(), new_height_m: r0.height_m })
}
