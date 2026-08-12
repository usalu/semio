//! 🔺️ `change-pile-n-profiles` sparse diff construction — writes only `En1997Diff.pile_n_profiles` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_pile_n_profiles::mutation::ChangePileNProfiles;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePileNProfiles, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { pile_n_profiles: Some(payload.new_pile_n_profiles.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
