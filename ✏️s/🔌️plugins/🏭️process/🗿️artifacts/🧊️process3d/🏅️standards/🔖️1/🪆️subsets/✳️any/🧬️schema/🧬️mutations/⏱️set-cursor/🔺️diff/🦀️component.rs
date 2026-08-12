//! 🔺️ `change-cursor` sparse diff construction.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::set_cursor::mutation::ChangeCursor;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCursor, _base: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff { resolved_up_to: Some(payload.new_resolved_up_to), ..Default::default() }
}
//#endregion 🔖️Diff
