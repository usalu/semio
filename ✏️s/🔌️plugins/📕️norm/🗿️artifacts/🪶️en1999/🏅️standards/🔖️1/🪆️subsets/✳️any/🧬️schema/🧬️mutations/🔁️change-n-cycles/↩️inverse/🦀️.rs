//! ↩️ `change-n-cycles` inverse — restores the pre-change `n_cycles` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_n_cycles::ChangeNCycles;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeNCycles, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeNCycles(ChangeNCycles { new_n_cycles: base.n_cycles.clone() })]
}
//#endregion 🔖️Inverse
