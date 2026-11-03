//! ↩️ `change-concentrated-distance-a1` inverse: restores the previous `distance_a1_m`.

use crate::change_concentrated_distance_a1::ChangeConcentratedDistanceA1;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeConcentratedDistanceA1, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_concentrated_distance_a1::ChangeConcentratedDistanceA1> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeConcentratedDistanceA1, base: &En1996Snapshot) -> Option<ChangeConcentratedDistanceA1> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.load_case_id)?;
    let r2 = r1.concentrated.iter().find(|row| row.id == payload.id)?;
    Some(ChangeConcentratedDistanceA1 { wall_id: payload.wall_id.clone(), load_case_id: payload.load_case_id.clone(), id: payload.id.clone(), new_distance_a1_m: r2.distance_a1_m })
}
