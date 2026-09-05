//! 🔺️ `change-f-vk-mpa` sparse diff construction — writes only `En1996Diff.f_vk_mpa` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_f_vk_mpa::ChangeFVkMpa;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFVkMpa, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_f_vk_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "F vk mpa must be a finite number.", Vec::<String>::new());
    }
    if base.f_vk_mpa == payload.new_f_vk_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "F vk mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { f_vk_mpa: Some(payload.new_f_vk_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
