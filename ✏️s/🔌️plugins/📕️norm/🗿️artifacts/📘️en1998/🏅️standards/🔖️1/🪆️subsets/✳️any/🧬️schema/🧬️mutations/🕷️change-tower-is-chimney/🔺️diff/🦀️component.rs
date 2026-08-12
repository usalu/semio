//! 🔺️ `change-tower-is-chimney` sparse diff construction — writes only `En1998Diff.tower_is_chimney` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tower_is_chimney::mutation::ChangeTowerIsChimney;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTowerIsChimney, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { tower_is_chimney: Some(payload.new_tower_is_chimney.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
