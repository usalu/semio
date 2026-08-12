//! ↩️ `change-sigma-ed-shell-mpa` inverse — restores the pre-change `sigma_ed_shell_mpa` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_sigma_ed_shell_mpa::mutation::ChangeSigmaEdShellMpa;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSigmaEdShellMpa, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeSigmaEdShellMpa(ChangeSigmaEdShellMpa { new_sigma_ed_shell_mpa: base.sigma_ed_shell_mpa.clone() })]
}
//#endregion 🔖️Inverse
