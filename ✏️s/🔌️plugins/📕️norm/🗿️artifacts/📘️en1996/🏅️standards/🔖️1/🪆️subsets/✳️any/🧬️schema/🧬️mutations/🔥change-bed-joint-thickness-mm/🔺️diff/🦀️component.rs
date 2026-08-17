//! 🔺️ `change-bed-joint-thickness-mm` sparse diff construction — writes only `En1996Diff.bed_joint_thickness_mm` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_bed_joint_thickness_mm::mutation::ChangeBedJointThicknessMm;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBedJointThicknessMm, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_bed_joint_thickness_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Bed joint thickness mm must be a finite number.", Vec::<String>::new());
    }
    if base.bed_joint_thickness_mm == payload.new_bed_joint_thickness_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Bed joint thickness mm already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { bed_joint_thickness_mm: Some(payload.new_bed_joint_thickness_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
