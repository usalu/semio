//! 🔺️ `change-sigma-ed-shell-mpa` sparse diff construction — writes only `En1999Diff.sigma_ed_shell_mpa` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_sigma_ed_shell_mpa::mutation::ChangeSigmaEdShellMpa;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSigmaEdShellMpa, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { sigma_ed_shell_mpa: Some(payload.new_sigma_ed_shell_mpa.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
