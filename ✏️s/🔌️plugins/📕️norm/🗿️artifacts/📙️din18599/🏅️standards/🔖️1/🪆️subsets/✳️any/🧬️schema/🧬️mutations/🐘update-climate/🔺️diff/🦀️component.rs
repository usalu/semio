//! 🔺️ `update-climate` sparse diff construction — mints a fresh content-addressed child handle
//! from the payload's literal `MonthlyClimate` (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM
//! round 2; the payload itself is unchanged — it still carries the real climate data, never a
//! handle).

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::update_climate::mutation::UpdateClimate;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &UpdateClimate, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { climate: Some(crate::artifacts::din18599::din18599_climate_child_from_data(&payload.new_climate)), ..Default::default() }
}
//#endregion 🔖️Diff
