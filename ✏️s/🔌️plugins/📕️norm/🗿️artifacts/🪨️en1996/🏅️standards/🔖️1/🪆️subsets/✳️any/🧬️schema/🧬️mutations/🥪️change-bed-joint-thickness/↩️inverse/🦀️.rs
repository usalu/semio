use crate::change_bed_joint_thickness::ChangeBedJointThickness;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeBedJointThickness, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bed_joint_thickness::ChangeBedJointThickness> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeBedJointThickness { index: payload.index, new_bed_joint_thickness_m: base.walls[payload.index].bed_joint_thickness_m })] }

    })())
}
