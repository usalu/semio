//! ↩️ `change-crane-hoist-rated-load` inverse: restores the previous `rated_load`.

use crate::change_crane_hoist_rated_load::ChangeCraneHoistRatedLoad;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneHoistRatedLoad, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_hoist_rated_load::ChangeCraneHoistRatedLoad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeCraneHoistRatedLoad, base: &En1991Snapshot) -> Option<ChangeCraneHoistRatedLoad> {
    let r0 = base.crane_hoists.iter().find(|row| row.id == payload.id)?;
    Some(ChangeCraneHoistRatedLoad { id: payload.id.clone(), new_rated_load: r0.rated_load })
}
