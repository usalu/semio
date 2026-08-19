//! 🔺️ `change-wall-r` sparse diff construction — writes only `En1998Diff.wall_r` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_wall_r::mutation::ChangeWallR;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeWallR, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_wall_r.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Wall behaviour factor r must be a finite number, got {}.", payload.new_wall_r), Vec::<String>::new());
    }
    if base.wall_r == payload.new_wall_r {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Wall behaviour factor r is already {}.", payload.new_wall_r));
    }
    protocol::MutationOutcome::new(En1998Diff { wall_r: Some(payload.new_wall_r.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
