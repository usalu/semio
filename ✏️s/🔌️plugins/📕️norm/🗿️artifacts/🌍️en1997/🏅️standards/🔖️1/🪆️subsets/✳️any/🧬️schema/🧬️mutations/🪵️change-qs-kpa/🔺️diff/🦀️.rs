//! 🔺️ `change-qs-kpa` sparse diff construction — writes only `En1997Diff.q_s_kpa` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_q_s_kpa::ChangeQSKpa;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeQSKpa, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_q_s_kpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Shaft resistance q_s [kPa] must be a finite number, got {}.", payload.new_q_s_kpa), Vec::<String>::new());
    }
    if base.q_s_kpa == payload.new_q_s_kpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shaft resistance q_s [kPa] is already {}.", payload.new_q_s_kpa));
    }
    protocol::MutationOutcome::new(En1997Diff { q_s_kpa: Some(payload.new_q_s_kpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
