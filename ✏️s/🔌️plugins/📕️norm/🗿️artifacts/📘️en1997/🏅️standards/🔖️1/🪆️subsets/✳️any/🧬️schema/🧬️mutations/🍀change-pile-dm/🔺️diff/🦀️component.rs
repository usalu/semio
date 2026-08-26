//! 🔺️ `change-pile-dm` sparse diff construction — writes only `En1997Diff.pile_d_m` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_pile_d_m::mutation::ChangePileDM;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePileDM, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_pile_d_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Pile diameter [m] must be a finite number, got {}.", payload.new_pile_d_m), Vec::<String>::new());
    }
    if base.pile_d_m == payload.new_pile_d_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Pile diameter [m] is already {}.", payload.new_pile_d_m));
    }
    protocol::MutationOutcome::new(En1997Diff { pile_d_m: Some(payload.new_pile_d_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
