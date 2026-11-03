//! ↩️ `change-member-wall-eff-height-m` inverse: restores the previous `wall_effective_height_m`.

use crate::change_member_wall_eff_height_m::ChangeMemberWallEffHeightM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberWallEffHeightM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_wall_eff_height_m::ChangeMemberWallEffHeightM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberWallEffHeightM, base: &En1998Snapshot) -> Option<ChangeMemberWallEffHeightM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberWallEffHeightM { building_id: payload.building_id.clone(), id: payload.id.clone(), new_wall_effective_height_m: r1.wall_effective_height_m })
}
