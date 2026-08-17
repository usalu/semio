//! 🔺️ `change-wall-phi-deg` sparse diff construction — writes only `En1998Diff.wall_phi_deg` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_wall_phi_deg::mutation::ChangeWallPhiDeg;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWallPhiDeg, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_wall_phi_deg.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Wall backfill friction angle [deg] must be a finite number, got {}.", payload.new_wall_phi_deg), Vec::<String>::new());
    }
    if base.wall_phi_deg == payload.new_wall_phi_deg {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Wall backfill friction angle [deg] is already {}.", payload.new_wall_phi_deg));
    }
    protocol::MutationOutcome::new(En1998Diff { wall_phi_deg: Some(payload.new_wall_phi_deg.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
