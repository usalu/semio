//! 🔺️ `change-n-cycles-bridge` sparse diff construction — writes only `En1995Diff.n_cycles_bridge` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_n_cycles_bridge::mutation::ChangeNCyclesBridge;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNCyclesBridge, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { n_cycles_bridge: Some(payload.new_n_cycles_bridge.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
