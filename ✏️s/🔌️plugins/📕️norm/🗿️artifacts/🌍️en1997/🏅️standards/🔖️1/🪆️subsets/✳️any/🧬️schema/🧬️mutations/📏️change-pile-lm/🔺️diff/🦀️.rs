//! 🔺️ `change-pile-lm` sparse diff construction — writes only `En1997Diff.pile_l_m` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_pile_l_m::ChangePileLM;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePileLM, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_pile_l_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Pile length [m] must be a finite number, got {}.", payload.new_pile_l_m), Vec::<String>::new());
    }
    if base.pile_l_m == payload.new_pile_l_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Pile length [m] is already {}.", payload.new_pile_l_m));
    }
    protocol::MutationOutcome::new(En1997Diff { pile_l_m: Some(payload.new_pile_l_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
