//! ↩️ `change-n-cycles-bridge` inverse — restores the pre-change `n_cycles_bridge` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_n_cycles_bridge::ChangeNCyclesBridge;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeNCyclesBridge, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeNCyclesBridge(ChangeNCyclesBridge { new_n_cycles_bridge: base.n_cycles_bridge.clone() })]
}
//#endregion 🔖️Inverse
