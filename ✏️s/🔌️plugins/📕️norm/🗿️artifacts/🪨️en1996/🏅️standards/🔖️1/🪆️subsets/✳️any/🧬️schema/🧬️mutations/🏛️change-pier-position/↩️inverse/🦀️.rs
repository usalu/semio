//! ↩️ `change-pier-position` inverse: restores the previous `position_m`.

use crate::change_pier_position::ChangePierPosition;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangePierPosition, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_position::ChangePierPosition> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierPosition, base: &En1996Snapshot) -> Option<ChangePierPosition> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierPosition { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_position_m: r1.position_m })
}
