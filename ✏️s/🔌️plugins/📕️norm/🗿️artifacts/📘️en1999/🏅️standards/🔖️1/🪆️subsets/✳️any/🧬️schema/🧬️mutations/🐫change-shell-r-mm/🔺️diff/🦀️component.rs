//! 🔺️ `change-shell-r-mm` sparse diff construction — writes only `En1999Diff.shell_r_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_shell_r_mm::mutation::ChangeShellRMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeShellRMm, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_shell_r_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Shell radius r [mm] must be a finite number, got {}.", payload.new_shell_r_mm), Vec::<String>::new());
    }
    if base.shell_r_mm == payload.new_shell_r_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shell radius r [mm] is already {}.", payload.new_shell_r_mm));
    }
    protocol::MutationOutcome::new(En1999Diff { shell_r_mm: Some(payload.new_shell_r_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
