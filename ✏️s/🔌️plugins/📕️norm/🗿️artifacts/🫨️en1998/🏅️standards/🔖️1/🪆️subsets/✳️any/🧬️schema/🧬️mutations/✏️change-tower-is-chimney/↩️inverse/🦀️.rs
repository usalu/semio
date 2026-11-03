//! ↩️ `change-tower-is-chimney` inverse: restores the previous `is_chimney`.

use crate::change_tower_is_chimney::ChangeTowerIsChimney;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerIsChimney, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_is_chimney::ChangeTowerIsChimney> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTowerIsChimney, base: &En1998Snapshot) -> Option<ChangeTowerIsChimney> {
    let r0 = base.towers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTowerIsChimney { id: payload.id.clone(), new_is_chimney: r0.is_chimney })
}
