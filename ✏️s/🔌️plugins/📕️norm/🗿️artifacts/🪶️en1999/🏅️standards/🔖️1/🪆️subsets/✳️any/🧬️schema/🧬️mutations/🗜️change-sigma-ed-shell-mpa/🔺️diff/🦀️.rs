//! 🔺️ `change-sigma-ed-shell-mpa` sparse diff construction — writes only `En1999Diff.sigma_ed_shell_mpa` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_sigma_ed_shell_mpa::ChangeSigmaEdShellMpa;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSigmaEdShellMpa, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_sigma_ed_shell_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Shell design stress [MPa] must be a finite number, got {}.", payload.new_sigma_ed_shell_mpa), Vec::<String>::new());
    }
    if base.sigma_ed_shell_mpa == payload.new_sigma_ed_shell_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shell design stress [MPa] is already {}.", payload.new_sigma_ed_shell_mpa));
    }
    protocol::MutationOutcome::new(En1999Diff { sigma_ed_shell_mpa: Some(payload.new_sigma_ed_shell_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
