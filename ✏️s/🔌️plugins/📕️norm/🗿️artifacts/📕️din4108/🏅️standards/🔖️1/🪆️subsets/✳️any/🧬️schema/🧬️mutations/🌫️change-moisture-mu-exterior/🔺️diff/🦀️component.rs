//! 🔺️ `change-moisture-mu-exterior` — sparse diff construction.

use super::mutation::ChangeMoistureMuExterior;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeMoistureMuExterior, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { moisture_mu_exterior: Some(payload.new_moisture_mu_exterior), ..Default::default() }
}
//#endregion 🔖️Diff
