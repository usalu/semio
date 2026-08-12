//! 🔺️ `change-bed-joint-thickness-mm` sparse diff construction — writes only `En1996Diff.bed_joint_thickness_mm` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_bed_joint_thickness_mm::mutation::ChangeBedJointThicknessMm;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBedJointThicknessMm, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { bed_joint_thickness_mm: Some(payload.new_bed_joint_thickness_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
