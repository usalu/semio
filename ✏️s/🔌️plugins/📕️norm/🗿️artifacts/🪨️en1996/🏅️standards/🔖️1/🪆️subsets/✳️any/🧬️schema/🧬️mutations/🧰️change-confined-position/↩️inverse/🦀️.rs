//! ↩️ `change-confined-position` inverse: restores the previous `position_m`.

use crate::change_confined_position::ChangeConfinedPosition;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeConfinedPosition, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_confined_position::ChangeConfinedPosition> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeConfinedPosition, base: &En1996Snapshot) -> Option<ChangeConfinedPosition> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.confined.iter().find(|row| row.id == payload.id)?;
    Some(ChangeConfinedPosition { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_position_m: r1.position_m })
}
