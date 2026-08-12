//! ↩️ `change-bed-joint-thickness-mm` inverse — restores the pre-change `bed_joint_thickness_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1996::mutations::change_bed_joint_thickness_mm::mutation::ChangeBedJointThicknessMm;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeBedJointThicknessMm, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeBedJointThicknessMm(ChangeBedJointThicknessMm { new_bed_joint_thickness_mm: base.bed_joint_thickness_mm.clone() })]
}
//#endregion 🔖️Inverse
