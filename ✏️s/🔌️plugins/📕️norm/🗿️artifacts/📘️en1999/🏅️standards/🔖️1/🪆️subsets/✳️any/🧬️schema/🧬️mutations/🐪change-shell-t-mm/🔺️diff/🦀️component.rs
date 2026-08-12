//! 🔺️ `change-shell-t-mm` sparse diff construction — writes only `En1999Diff.shell_t_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_shell_t_mm::mutation::ChangeShellTMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeShellTMm, _base: &En1999Snapshot) -> En1999Diff {
    En1999Diff { shell_t_mm: Some(payload.new_shell_t_mm.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
