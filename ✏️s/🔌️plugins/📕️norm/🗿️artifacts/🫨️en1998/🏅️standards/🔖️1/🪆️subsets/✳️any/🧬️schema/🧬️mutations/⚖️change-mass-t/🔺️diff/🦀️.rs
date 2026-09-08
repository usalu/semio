//! 🔺️ `change-mass-t` sparse diff construction — writes only `En1998Diff.mass_t` from the payload.

use crate::diff::En1998Diff;
use crate::mutations::change_mass_t::ChangeMassT;
use crate::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMassT, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_mass_t.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Seismic mass [t] must be a finite number, got {}.", payload.new_mass_t), Vec::<String>::new());
    }
    if base.mass_t == payload.new_mass_t {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Seismic mass [t] is already {}.", payload.new_mass_t));
    }
    protocol::MutationOutcome::new(En1998Diff { mass_t: Some(payload.new_mass_t), ..Default::default() })
}
//#endregion 🔖️Diff
