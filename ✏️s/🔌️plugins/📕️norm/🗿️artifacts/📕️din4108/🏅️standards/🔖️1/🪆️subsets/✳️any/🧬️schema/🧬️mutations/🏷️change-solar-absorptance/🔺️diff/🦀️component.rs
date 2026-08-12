//! 🔺️ `change-solar-absorptance` — sparse diff construction.

use super::mutation::ChangeSolarAbsorptance;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSolarAbsorptance, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { solar_absorptance: Some(payload.new_solar_absorptance), ..Default::default() }
}
//#endregion 🔖️Diff
