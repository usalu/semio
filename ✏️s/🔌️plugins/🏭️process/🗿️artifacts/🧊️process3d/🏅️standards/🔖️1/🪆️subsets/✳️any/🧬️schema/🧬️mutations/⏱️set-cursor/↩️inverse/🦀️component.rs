//! ↩️ `change-cursor` inverse — reconstructs the pre-move cursor value from BASE state; `change` is
//! its own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::process3d::mutations::set_cursor::mutation::ChangeCursor;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeCursor, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
    vec![Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: base.resolved_up_to })]
}
//#endregion 🔖️Inverse
