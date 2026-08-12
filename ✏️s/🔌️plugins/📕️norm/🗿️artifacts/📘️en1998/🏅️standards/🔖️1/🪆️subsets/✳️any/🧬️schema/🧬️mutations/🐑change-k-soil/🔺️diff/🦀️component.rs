//! 🔺️ `change-k-soil` sparse diff construction — writes only `En1998Diff.k_soil` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_k_soil::mutation::ChangeKSoil;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeKSoil, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { k_soil: Some(payload.new_k_soil.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
