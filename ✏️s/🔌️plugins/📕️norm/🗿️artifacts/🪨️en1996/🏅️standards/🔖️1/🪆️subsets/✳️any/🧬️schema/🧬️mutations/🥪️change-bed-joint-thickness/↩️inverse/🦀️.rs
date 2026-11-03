//! ↩️ `change-bed-joint-thickness` inverse: restores the previous `bed_joint_thickness_m`.

use crate::change_bed_joint_thickness::ChangeBedJointThickness;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeBedJointThickness, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bed_joint_thickness::ChangeBedJointThickness> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBedJointThickness, base: &En1996Snapshot) -> Option<ChangeBedJointThickness> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBedJointThickness { id: payload.id.clone(), new_bed_joint_thickness_m: r0.bed_joint_thickness_m })
}
