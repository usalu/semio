//! ↩️ `change-shell-r-mm` inverse — restores the pre-change `shell_r_mm` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_shell_r_mm::ChangeShellRMm;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeShellRMm, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeShellRMm(ChangeShellRMm { new_shell_r_mm: base.shell_r_mm.clone() })]
}
//#endregion 🔖️Inverse
