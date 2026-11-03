//! ↩️ `change-building-joint-gap-m` inverse: restores the previous `joint_gap_m`.

use crate::change_building_joint_gap_m::ChangeBuildingJointGapM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBuildingJointGapM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_joint_gap_m::ChangeBuildingJointGapM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBuildingJointGapM, base: &En1998Snapshot) -> Option<ChangeBuildingJointGapM> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBuildingJointGapM { id: payload.id.clone(), new_joint_gap_m: r0.joint_gap_m })
}
