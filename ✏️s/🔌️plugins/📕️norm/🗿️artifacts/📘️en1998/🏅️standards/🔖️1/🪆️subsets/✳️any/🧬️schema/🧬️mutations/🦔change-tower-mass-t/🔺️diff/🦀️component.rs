//! 🔺️ `change-tower-mass-t` sparse diff construction — writes only `En1998Diff.tower_mass_t` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tower_mass_t::mutation::ChangeTowerMassT;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeTowerMassT, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_tower_mass_t.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tower mass [t] must be a finite number, got {}.", payload.new_tower_mass_t), Vec::<String>::new());
    }
    if base.tower_mass_t == payload.new_tower_mass_t {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tower mass [t] is already {}.", payload.new_tower_mass_t));
    }
    protocol::MutationOutcome::new(En1998Diff { tower_mass_t: Some(payload.new_tower_mass_t.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
