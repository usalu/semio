//! ↩️ `change-bed-joint-thickness-mm` inverse — restores the pre-change `bed_joint_thickness_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_bed_joint_thickness_mm::ChangeBedJointThicknessMm;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBedJointThicknessMm, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeBedJointThicknessMm(ChangeBedJointThicknessMm { new_bed_joint_thickness_mm: base.bed_joint_thickness_mm })]
}
//#endregion 🔖️Inverse
