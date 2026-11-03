//! ↩️ `change-falling-stopping-distance` inverse: restores the previous `stopping_distance`.

use crate::change_falling_stopping_distance::ChangeFallingStoppingDistance;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFallingStoppingDistance, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_falling_stopping_distance::ChangeFallingStoppingDistance> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFallingStoppingDistance, base: &En1991Snapshot) -> Option<ChangeFallingStoppingDistance> {
    let r0 = base.falling_loads.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFallingStoppingDistance { id: payload.id.clone(), new_stopping_distance: r0.stopping_distance })
}
