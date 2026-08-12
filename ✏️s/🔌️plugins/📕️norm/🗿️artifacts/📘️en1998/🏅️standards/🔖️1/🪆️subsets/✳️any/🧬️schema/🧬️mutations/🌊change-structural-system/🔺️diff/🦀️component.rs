//! 🔺️ `change-structural-system` sparse diff construction — writes only `En1998Diff.structural_system` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_structural_system::mutation::ChangeStructuralSystem;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStructuralSystem, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { structural_system: Some(payload.new_structural_system.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
