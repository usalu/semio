//! 🔺️ `change-co2-ppm` sparse diff construction — writes only `Din16798Diff.co2_ppm` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_co2_ppm::mutation::ChangeCo2Ppm;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeCo2Ppm, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_co2_ppm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("CO2 concentration must be a finite number, got {}.", payload.new_co2_ppm), Vec::<String>::new());
    }
    if base.co2_ppm == payload.new_co2_ppm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("CO2 concentration is already {}.", payload.new_co2_ppm));
    }
    protocol::MutationOutcome::new(Din16798Diff { co2_ppm: Some(payload.new_co2_ppm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
