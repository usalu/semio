//! 🔺️ `change-solar-gains-kwh` sparse diff construction — writes only `Din18599Diff.solar_gains_kwh` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_solar_gains_kwh::ChangeSolarGainsKwh;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSolarGainsKwh, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_solar_gains_kwh.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Solar gains kwh must be a finite number.", Vec::<String>::new());
    }
    if base.solar_gains_kwh == payload.new_solar_gains_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Solar gains kwh already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { solar_gains_kwh: Some(payload.new_solar_gains_kwh.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
