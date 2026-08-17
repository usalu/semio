//! 🔺️ `change-qb-kpa` sparse diff construction — writes only `En1997Diff.q_b_kpa` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_q_b_kpa::mutation::ChangeQBKpa;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeQBKpa, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_q_b_kpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Base resistance q_b [kPa] must be a finite number, got {}.", payload.new_q_b_kpa), Vec::<String>::new());
    }
    if base.q_b_kpa == payload.new_q_b_kpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Base resistance q_b [kPa] is already {}.", payload.new_q_b_kpa));
    }
    protocol::MutationOutcome::new(En1997Diff { q_b_kpa: Some(payload.new_q_b_kpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
