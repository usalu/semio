//! ↩️ `change-local-fire-horizontal-distance` inverse: restores the previous `horizontal_distance`.

use crate::change_local_fire_horizontal_distance::ChangeLocalFireHorizontalDistance;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeLocalFireHorizontalDistance, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_local_fire_horizontal_distance::ChangeLocalFireHorizontalDistance> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeLocalFireHorizontalDistance, base: &En1991Snapshot) -> Option<ChangeLocalFireHorizontalDistance> {
    let r0 = base.local_fires.iter().find(|row| row.id == payload.id)?;
    Some(ChangeLocalFireHorizontalDistance { id: payload.id.clone(), new_horizontal_distance: r0.horizontal_distance })
}
