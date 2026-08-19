//! 🔺️ `change-tower-q-nominal` sparse diff construction — writes only `En1998Diff.tower_q_nominal` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tower_q_nominal::mutation::ChangeTowerQNominal;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeTowerQNominal, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_tower_q_nominal.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tower nominal behaviour factor q must be a finite number, got {}.", payload.new_tower_q_nominal), Vec::<String>::new());
    }
    if base.tower_q_nominal == payload.new_tower_q_nominal {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tower nominal behaviour factor q is already {}.", payload.new_tower_q_nominal));
    }
    protocol::MutationOutcome::new(En1998Diff { tower_q_nominal: Some(payload.new_tower_q_nominal.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
