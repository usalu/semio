//! 🔺️ `change-moisture-mu-interior` — sparse diff construction.

use super::mutation::ChangeMoistureMuInterior;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeMoistureMuInterior, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { moisture_mu_interior: Some(payload.new_moisture_mu_interior), ..Default::default() }
}
//#endregion 🔖️Diff
