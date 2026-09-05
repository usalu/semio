//! 🔺️ `change-irradiance-w-m2` — sparse diff construction.

use super::ChangeIrradianceWM2;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeIrradianceWM2, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_irradiance_w_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Irradiance wm2 must be a finite number.", Vec::<String>::new());
    }
    if base.irradiance_w_m2 == payload.new_irradiance_w_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Irradiance wm2 already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { irradiance_w_m2: Some(payload.new_irradiance_w_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
