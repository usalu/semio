//! 🔺️ `change-zone-theta-i-cool-c` sparse diff.

use crate::change_zone_theta_i_cool_c::ChangeZoneThetaICoolC;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599ZonesRows, Din18599ZonesPatch};

pub fn diff(payload: &ChangeZoneThetaICoolC, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_theta_i_cool_c.is_finite() || payload.new_theta_i_cool_c < 18.0 || payload.new_theta_i_cool_c > 35.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "theta-i-cool-c must lie from 18 to 35.", Vec::<String>::new());
    }
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone \"{}\" does not exist.", payload.zone_id), [payload.zone_id.clone()]);
    };
    if row.theta_i_cool_c == payload.new_theta_i_cool_c {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Zone theta i cool c already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff {
        zones: Some(Din18599ZonesRows::modification(&payload.zone_id, Din18599ZonesPatch { theta_i_cool_c: Some(payload.new_theta_i_cool_c), ..Default::default() })),
        ..Default::default()
    })
}
