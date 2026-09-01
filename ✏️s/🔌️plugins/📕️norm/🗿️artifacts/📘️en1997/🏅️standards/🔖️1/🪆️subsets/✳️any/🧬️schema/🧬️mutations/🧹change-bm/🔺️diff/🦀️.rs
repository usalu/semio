//! 🔺️ `change-bm` sparse diff construction — writes only `En1997Diff.b_m` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_b_m::ChangeBM;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBM, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_b_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Footing width B [m] must be a finite number, got {}.", payload.new_b_m), Vec::<String>::new());
    }
    if base.b_m == payload.new_b_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Footing width B [m] is already {}.", payload.new_b_m));
    }
    protocol::MutationOutcome::new(En1997Diff { b_m: Some(payload.new_b_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
