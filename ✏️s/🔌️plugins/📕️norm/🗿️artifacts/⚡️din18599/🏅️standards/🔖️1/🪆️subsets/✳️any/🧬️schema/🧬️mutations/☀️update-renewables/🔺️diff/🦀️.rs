//! 🔺️ `update-renewables` sparse diff.

use crate::mutations::update_renewables::UpdateRenewables;
use crate::Din18599Snapshot;
use crate::diff::{Din18599Diff, Din18599RenewablesPatch};

pub fn diff(payload: &UpdateRenewables, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.renewables == payload.new_renewables {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "renewables already has this value.");
    }
    let (old, new) = (&base.renewables, &payload.new_renewables);
    protocol::MutationOutcome::new(Din18599Diff {
        renewables: Some(Din18599RenewablesPatch {
            pv_area_m2: (old.pv_area_m2 != new.pv_area_m2).then(|| new.pv_area_m2.clone()),
            pv_efficiency: (old.pv_efficiency != new.pv_efficiency).then(|| new.pv_efficiency.clone()),
            solar_thermal_kwh_a: (old.solar_thermal_kwh_a != new.solar_thermal_kwh_a).then(|| new.solar_thermal_kwh_a.clone()),
        }),
        ..Default::default()
    })
}
