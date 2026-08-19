//! 🔺️ `change-c-kpa` sparse diff construction — writes only `En1997Diff.c_kpa` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_c_kpa::mutation::ChangeCKpa;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeCKpa, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_c_kpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cohesion c [kPa] must be a finite number, got {}.", payload.new_c_kpa), Vec::<String>::new());
    }
    if base.c_kpa == payload.new_c_kpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cohesion c [kPa] is already {}.", payload.new_c_kpa));
    }
    protocol::MutationOutcome::new(En1997Diff { c_kpa: Some(payload.new_c_kpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
