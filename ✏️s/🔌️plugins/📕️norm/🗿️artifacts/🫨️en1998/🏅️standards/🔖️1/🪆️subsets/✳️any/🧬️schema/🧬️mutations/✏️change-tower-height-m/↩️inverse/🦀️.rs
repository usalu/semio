//! ↩️ `change-tower-height-m` inverse: restores the previous `height_m`.

use crate::change_tower_height_m::ChangeTowerHeightM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerHeightM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_height_m::ChangeTowerHeightM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTowerHeightM, base: &En1998Snapshot) -> Option<ChangeTowerHeightM> {
    let r0 = base.towers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTowerHeightM { id: payload.id.clone(), new_height_m: r0.height_m })
}
