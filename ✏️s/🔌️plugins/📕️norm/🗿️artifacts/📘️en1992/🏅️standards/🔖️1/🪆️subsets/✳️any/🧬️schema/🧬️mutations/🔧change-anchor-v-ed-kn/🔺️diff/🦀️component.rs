//! 🔺️ `change-anchor-v-ed-kn` sparse diff construction — writes only `En1992Diff.anchor_v_ed_kn` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_anchor_v_ed_kn::mutation::ChangeAnchorVEdKn;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnchorVEdKn, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { anchor_v_ed_kn: Some(payload.new_anchor_v_ed_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
