//! 🔺️ `change-foundation-h-ed-kn` sparse diff construction — writes only `En1998Diff.foundation_h_ed_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_foundation_h_ed_kn::mutation::ChangeFoundationHEdKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFoundationHEdKn, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { foundation_h_ed_kn: Some(payload.new_foundation_h_ed_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
