use super::ChangeBedJointThickness;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeBedJointThickness, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![En1996Mutation::ChangeBedJointThickness(ChangeBedJointThickness { index: payload.index, new_bed_joint_thickness_m: base.walls[payload.index].bed_joint_thickness_m })] }
}
