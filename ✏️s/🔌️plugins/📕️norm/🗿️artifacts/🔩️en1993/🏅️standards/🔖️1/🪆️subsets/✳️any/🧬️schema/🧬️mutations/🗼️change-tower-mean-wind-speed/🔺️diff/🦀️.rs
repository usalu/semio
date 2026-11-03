//! 🔺️ `change-tower-mean-wind-speed` diff: patches `mean_wind_speed` of the addressed tower leg row.

use crate::change_tower_mean_wind_speed::ChangeTowerMeanWindSpeed;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993TowerLegDelta, En1993TowerLegPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeTowerMeanWindSpeed, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_mean_wind_speed.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.tower_legs.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tower", Vec::<String>::new());
    };
    if row.mean_wind_speed == payload.new_mean_wind_speed {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tower leg already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { tower_legs: En1993TowerLegDelta::modification(payload.id.clone(), En1993TowerLegPatch { mean_wind_speed: Some(payload.new_mean_wind_speed), ..Default::default() }), ..Default::default() })
}
