//! 🔺️ `change-n-cycles-stud` — sparse diff construction.

use super::mutation::ChangeNCyclesStud;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeNCyclesStud, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_n_cycles_stud.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "N cycles stud must be a finite number.", Vec::<String>::new());
    }
    if base.n_cycles_stud == payload.new_n_cycles_stud {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "N cycles stud already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { n_cycles_stud: Some(payload.new_n_cycles_stud.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
