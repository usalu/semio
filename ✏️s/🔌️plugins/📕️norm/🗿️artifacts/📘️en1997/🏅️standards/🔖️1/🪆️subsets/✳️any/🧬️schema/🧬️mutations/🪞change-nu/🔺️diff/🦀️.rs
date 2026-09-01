//! 🔺️ `change-nu` sparse diff construction — writes only `En1997Diff.nu` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_nu::ChangeNu;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNu, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_nu.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Poisson's ratio nu must be a finite number, got {}.", payload.new_nu), Vec::<String>::new());
    }
    if base.nu == payload.new_nu {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Poisson's ratio nu is already {}.", payload.new_nu));
    }
    protocol::MutationOutcome::new(En1997Diff { nu: Some(payload.new_nu.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
