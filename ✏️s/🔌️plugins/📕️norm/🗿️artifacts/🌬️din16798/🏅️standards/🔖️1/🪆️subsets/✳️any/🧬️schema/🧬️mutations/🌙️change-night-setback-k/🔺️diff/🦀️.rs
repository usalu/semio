//! 🔺️ `change-night-setback-k` sparse diff construction — writes only `Din16798Diff.night_setback_k` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_night_setback_k::ChangeNightSetbackK;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNightSetbackK, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_night_setback_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Night setback temperature must be a finite number, got {}.", payload.new_night_setback_k), Vec::<String>::new());
    }
    if base.night_setback_k == payload.new_night_setback_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Night setback temperature is already {}.", payload.new_night_setback_k));
    }
    protocol::MutationOutcome::new(Din16798Diff { night_setback_k: Some(payload.new_night_setback_k.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
