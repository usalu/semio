//! 🔺️ `change-tower-is-chimney` sparse diff construction — writes only `En1998Diff.tower_is_chimney` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tower_is_chimney::mutation::ChangeTowerIsChimney;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTowerIsChimney, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.tower_is_chimney == payload.new_tower_is_chimney {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tower-is-chimney flag is already {}.", payload.new_tower_is_chimney));
    }
    protocol::MutationOutcome::new(En1998Diff { tower_is_chimney: Some(payload.new_tower_is_chimney.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
