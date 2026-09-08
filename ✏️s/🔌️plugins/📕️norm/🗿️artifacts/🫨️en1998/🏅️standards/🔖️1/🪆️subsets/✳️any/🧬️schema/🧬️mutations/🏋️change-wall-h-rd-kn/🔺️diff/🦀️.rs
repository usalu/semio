//! 🔺️ `change-wall-h-rd-kn` sparse diff construction — writes only `En1998Diff.wall_h_rd_kn` from the payload.

use crate::diff::En1998Diff;
use crate::mutations::change_wall_h_rd_kn::ChangeWallHRdKn;
use crate::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWallHRdKn, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_wall_h_rd_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Wall horizontal resistance H_Rd [kN] must be a finite number, got {}.", payload.new_wall_h_rd_kn), Vec::<String>::new());
    }
    if base.wall_h_rd_kn == payload.new_wall_h_rd_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Wall horizontal resistance H_Rd [kN] is already {}.", payload.new_wall_h_rd_kn));
    }
    protocol::MutationOutcome::new(En1998Diff { wall_h_rd_kn: Some(payload.new_wall_h_rd_kn), ..Default::default() })
}
//#endregion 🔖️Diff
