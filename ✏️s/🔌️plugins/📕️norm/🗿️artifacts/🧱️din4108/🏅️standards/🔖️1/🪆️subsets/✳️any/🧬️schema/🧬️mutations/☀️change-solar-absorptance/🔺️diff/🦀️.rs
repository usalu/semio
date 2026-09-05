//! 🔺️ `change-solar-absorptance` — sparse diff construction.

use super::ChangeSolarAbsorptance;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSolarAbsorptance, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_solar_absorptance.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Solar absorptance must be a finite number.", Vec::<String>::new());
    }
    if base.solar_absorptance == payload.new_solar_absorptance {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Solar absorptance already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { solar_absorptance: Some(payload.new_solar_absorptance.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
