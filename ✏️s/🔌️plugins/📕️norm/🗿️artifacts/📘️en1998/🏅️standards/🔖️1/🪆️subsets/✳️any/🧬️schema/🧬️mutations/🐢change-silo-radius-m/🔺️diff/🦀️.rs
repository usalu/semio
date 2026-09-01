//! 🔺️ `change-silo-radius-m` sparse diff construction — writes only `En1998Diff.silo_radius_m` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_silo_radius_m::ChangeSiloRadiusM;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloRadiusM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_silo_radius_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Silo radius [m] must be a finite number, got {}.", payload.new_silo_radius_m), Vec::<String>::new());
    }
    if base.silo_radius_m == payload.new_silo_radius_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Silo radius [m] is already {}.", payload.new_silo_radius_m));
    }
    protocol::MutationOutcome::new(En1998Diff { silo_radius_m: Some(payload.new_silo_radius_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
