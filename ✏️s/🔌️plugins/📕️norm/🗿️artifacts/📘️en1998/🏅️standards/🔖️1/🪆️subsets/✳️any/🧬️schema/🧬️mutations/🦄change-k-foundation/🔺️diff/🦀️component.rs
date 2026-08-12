//! 🔺️ `change-k-foundation` sparse diff construction — writes only `En1998Diff.k_foundation` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_k_foundation::mutation::ChangeKFoundation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeKFoundation, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { k_foundation: Some(payload.new_k_foundation.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
