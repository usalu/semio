//! ↩️ `change-moisture-mu-interior` — undo restores BASE's `moisture_mu_interior`.

use super::mutation::ChangeMoistureMuInterior;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMoistureMuInterior, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeMoistureMuInterior(ChangeMoistureMuInterior { new_moisture_mu_interior: base.moisture_mu_interior })]
}
//#endregion 🔖️Inverse
