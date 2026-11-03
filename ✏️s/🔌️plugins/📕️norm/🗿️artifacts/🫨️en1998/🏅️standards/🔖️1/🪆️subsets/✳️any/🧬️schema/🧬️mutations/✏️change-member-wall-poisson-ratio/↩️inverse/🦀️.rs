//! ↩️ `change-member-wall-poisson-ratio` inverse: restores the previous `wall_poisson_ratio`.

use crate::change_member_wall_poisson_ratio::ChangeMemberWallPoissonRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberWallPoissonRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_wall_poisson_ratio::ChangeMemberWallPoissonRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberWallPoissonRatio, base: &En1998Snapshot) -> Option<ChangeMemberWallPoissonRatio> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberWallPoissonRatio { building_id: payload.building_id.clone(), id: payload.id.clone(), new_wall_poisson_ratio: r1.wall_poisson_ratio })
}
