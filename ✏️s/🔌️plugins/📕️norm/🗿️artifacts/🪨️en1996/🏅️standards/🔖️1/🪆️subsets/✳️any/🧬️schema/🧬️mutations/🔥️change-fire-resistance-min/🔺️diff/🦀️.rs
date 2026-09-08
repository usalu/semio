//! 🔺️ `change-fire-resistance-min` sparse diff construction — writes only `En1996Diff.fire_resistance_min` from the payload.

use crate::diff::En1996Diff;
use crate::mutations::change_fire_resistance_min::ChangeFireResistanceMin;
use crate::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFireResistanceMin, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.fire_resistance_min == payload.new_fire_resistance_min {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fire resistance min already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { fire_resistance_min: Some(payload.new_fire_resistance_min), ..Default::default() })
}
//#endregion 🔖️Diff
