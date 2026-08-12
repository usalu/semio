//! 🔺️ `change-f-vk-mpa` sparse diff construction — writes only `En1996Diff.f_vk_mpa` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_f_vk_mpa::mutation::ChangeFVkMpa;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFVkMpa, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { f_vk_mpa: Some(payload.new_f_vk_mpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
