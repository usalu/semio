//! 🔺️ `update-climate` sparse diff construction — mints a fresh content-addressed child handle
//! from the payload's literal `MonthlyClimate` (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM
//! round 2; the payload itself is unchanged — it still carries the real climate data, never a
//! handle).

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::update_climate::UpdateClimate;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &UpdateClimate, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if payload.new_climate.theta_e_c.iter().any(|v| !v.is_finite()) || payload.new_climate.g_h_w_m2.iter().any(|v| !v.is_finite() || *v < 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Monthly climate values must be finite, and irradiance must be non-negative.", Vec::<String>::new());
    }
    if crate::artifacts::din18599::din18599_climate(base) == payload.new_climate {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Climate profile is already this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { climate: Some(crate::artifacts::din18599::din18599_climate_child_from_data(&payload.new_climate)), ..Default::default() })
}
//#endregion 🔖️Diff
