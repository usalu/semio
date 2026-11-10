//! 🔺️ `change-zone-thermal-mass` sparse diff.

use crate::change_zone_thermal_mass::ChangeZoneThermalMass;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ZonesRows, Din18599ZonesPatch};

pub fn diff(payload: &ChangeZoneThermalMass, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_c_wirk_wh_m2k.is_finite() || payload.new_c_wirk_wh_m2k <= 0.0 || payload.new_c_wirk_wh_m2k > 1000.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "c-wirk-wh-m2k must lie above 0 up to 1000.", Vec::<String>::new());
    }
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone \"{}\" does not exist.", payload.zone_id), [payload.zone_id.clone()]);
    };
    let class = (row.thermal_mass_class != payload.new_thermal_mass_class).then_some(payload.new_thermal_mass_class);
    let c_wirk = (row.c_wirk_wh_m2k != payload.new_c_wirk_wh_m2k).then_some(payload.new_c_wirk_wh_m2k);
    if class.is_none() && c_wirk.is_none() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Zone thermal mass already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        zones: Some(Din18599ZonesRows::modification(&payload.zone_id, Din18599ZonesPatch { thermal_mass_class: class, c_wirk_wh_m2k: c_wirk, ..Default::default() })),
        ..Default::default()
    })
}
