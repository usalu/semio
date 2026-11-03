//! ↩️ `change-concentrated-position` inverse: restores the previous `position_m`.

use crate::change_concentrated_position::ChangeConcentratedPosition;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeConcentratedPosition, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_concentrated_position::ChangeConcentratedPosition> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeConcentratedPosition, base: &En1996Snapshot) -> Option<ChangeConcentratedPosition> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.load_case_id)?;
    let r2 = r1.concentrated.iter().find(|row| row.id == payload.id)?;
    Some(ChangeConcentratedPosition { wall_id: payload.wall_id.clone(), load_case_id: payload.load_case_id.clone(), id: payload.id.clone(), new_position_m: r2.position_m })
}
