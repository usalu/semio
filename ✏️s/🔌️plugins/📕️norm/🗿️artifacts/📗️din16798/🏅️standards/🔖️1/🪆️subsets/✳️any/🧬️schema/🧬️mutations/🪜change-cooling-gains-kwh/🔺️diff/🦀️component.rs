//! 🔺️ `change-cooling-gains-kwh` sparse diff construction — writes only `Din16798Diff.cooling_gains_kwh` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_cooling_gains_kwh::mutation::ChangeCoolingGainsKwh;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCoolingGainsKwh, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_cooling_gains_kwh.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cooling internal gains must be a finite number, got {}.", payload.new_cooling_gains_kwh), Vec::<String>::new());
    }
    if base.cooling_gains_kwh == payload.new_cooling_gains_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cooling internal gains is already {}.", payload.new_cooling_gains_kwh));
    }
    protocol::MutationOutcome::new(Din16798Diff { cooling_gains_kwh: Some(payload.new_cooling_gains_kwh.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
