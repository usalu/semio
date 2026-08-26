//! 🔺️ `change-silo-q-nominal` sparse diff construction — writes only `En1998Diff.silo_q_nominal` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_silo_q_nominal::mutation::ChangeSiloQNominal;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloQNominal, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_silo_q_nominal.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Silo nominal behaviour factor q must be a finite number, got {}.", payload.new_silo_q_nominal), Vec::<String>::new());
    }
    if base.silo_q_nominal == payload.new_silo_q_nominal {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Silo nominal behaviour factor q is already {}.", payload.new_silo_q_nominal));
    }
    protocol::MutationOutcome::new(En1998Diff { silo_q_nominal: Some(payload.new_silo_q_nominal.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
