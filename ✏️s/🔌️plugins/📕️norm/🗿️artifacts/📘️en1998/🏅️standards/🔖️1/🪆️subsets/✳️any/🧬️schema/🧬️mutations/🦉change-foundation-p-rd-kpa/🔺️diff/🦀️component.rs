//! 🔺️ `change-foundation-p-rd-kpa` sparse diff construction — writes only `En1998Diff.foundation_p_rd_kpa` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_foundation_p_rd_kpa::mutation::ChangeFoundationPRdKpa;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFoundationPRdKpa, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { foundation_p_rd_kpa: Some(payload.new_foundation_p_rd_kpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
