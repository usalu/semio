//! ↩️ `change-shell-t-mm` inverse — restores the pre-change `shell_t_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_shell_t_mm::ChangeShellTMm;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeShellTMm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeShellTMm(ChangeShellTMm { new_shell_t_mm: base.shell_t_mm.clone() })]
}
//#endregion 🔖️Inverse
