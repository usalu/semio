//! 🔺️ `change-beta-w` sparse diff construction — writes only `En1999Diff.beta_w` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_beta_w::mutation::ChangeBetaW;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBetaW, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { beta_w: Some(payload.new_beta_w.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
