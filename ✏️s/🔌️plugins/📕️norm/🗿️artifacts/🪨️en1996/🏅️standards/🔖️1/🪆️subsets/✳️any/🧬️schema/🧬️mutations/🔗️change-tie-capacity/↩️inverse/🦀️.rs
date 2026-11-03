//! ↩️ `change-tie-capacity` inverse: restores the previous `capacity_n`.

use crate::change_tie_capacity::ChangeTieCapacity;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeTieCapacity, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_capacity::ChangeTieCapacity> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieCapacity, base: &En1996Snapshot) -> Option<ChangeTieCapacity> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.ties.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieCapacity { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_capacity_n: r1.capacity_n })
}
