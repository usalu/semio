//! 🔺️ `change-n-cycles` sparse diff construction — writes only `En1999Diff.n_cycles` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_n_cycles::mutation::ChangeNCycles;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeNCycles, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_n_cycles.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Number of fatigue cycles must be a finite number, got {}.", payload.new_n_cycles), Vec::<String>::new());
    }
    if base.n_cycles == payload.new_n_cycles {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Number of fatigue cycles is already {}.", payload.new_n_cycles));
    }
    protocol::MutationOutcome::new(En1999Diff { n_cycles: Some(payload.new_n_cycles.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
