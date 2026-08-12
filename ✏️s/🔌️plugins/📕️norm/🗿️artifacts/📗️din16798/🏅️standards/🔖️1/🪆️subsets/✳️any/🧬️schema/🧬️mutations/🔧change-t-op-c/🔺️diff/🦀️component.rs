//! 🔺️ `change-t-op-c` sparse diff construction — writes only `Din16798Diff.t_op_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_t_op_c::mutation::ChangeTOpC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTOpC, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { t_op_c: Some(payload.new_t_op_c.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
