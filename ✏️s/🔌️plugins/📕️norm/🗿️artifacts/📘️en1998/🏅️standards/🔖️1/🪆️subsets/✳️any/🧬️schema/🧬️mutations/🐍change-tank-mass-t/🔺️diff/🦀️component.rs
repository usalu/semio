//! 🔺️ `change-tank-mass-t` sparse diff construction — writes only `En1998Diff.tank_mass_t` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tank_mass_t::mutation::ChangeTankMassT;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTankMassT, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_tank_mass_t.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tank mass [t] must be a finite number, got {}.", payload.new_tank_mass_t), Vec::<String>::new());
    }
    if base.tank_mass_t == payload.new_tank_mass_t {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tank mass [t] is already {}.", payload.new_tank_mass_t));
    }
    protocol::MutationOutcome::new(En1998Diff { tank_mass_t: Some(payload.new_tank_mass_t.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
