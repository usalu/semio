//! 🔺️ `change-n-pile-ed-kn` sparse diff construction — writes only `En1997Diff.n_pile_ed_kn` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_n_pile_ed_kn::mutation::ChangeNPileEdKn;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNPileEdKn, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { n_pile_ed_kn: Some(payload.new_n_pile_ed_kn.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
