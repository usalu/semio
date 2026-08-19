//! 🔺️ `change-z-investigated-m` sparse diff construction — writes only `En1997Diff.z_investigated_m` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_z_investigated_m::mutation::ChangeZInvestigatedM;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeZInvestigatedM, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_z_investigated_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Investigated depth [m] must be a finite number, got {}.", payload.new_z_investigated_m), Vec::<String>::new());
    }
    if base.z_investigated_m == payload.new_z_investigated_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Investigated depth [m] is already {}.", payload.new_z_investigated_m));
    }
    protocol::MutationOutcome::new(En1997Diff { z_investigated_m: Some(payload.new_z_investigated_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
