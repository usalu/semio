//! 🔺️ `change-multiple-resisting-systems` sparse diff construction — writes only `En1998Diff.multiple_resisting_systems` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_multiple_resisting_systems::mutation::ChangeMultipleResistingSystems;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMultipleResistingSystems, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { multiple_resisting_systems: Some(payload.new_multiple_resisting_systems.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
