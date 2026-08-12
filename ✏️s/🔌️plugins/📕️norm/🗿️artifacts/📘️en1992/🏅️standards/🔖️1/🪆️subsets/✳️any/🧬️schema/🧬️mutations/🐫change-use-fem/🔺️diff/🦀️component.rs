//! 🔺️ `change-use-fem` sparse diff construction — writes only `En1992Diff.use_fem` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_use_fem::mutation::ChangeUseFem;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeUseFem, _base: &En1992Snapshot) -> En1992Diff {
    En1992Diff { use_fem: Some(payload.new_use_fem.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
