//! ↩️ `change-moisture-mu-exterior` — undo restores BASE's `moisture_mu_exterior`.

use super::ChangeMoistureMuExterior;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMoistureMuExterior, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeMoistureMuExterior(ChangeMoistureMuExterior { new_moisture_mu_exterior: base.moisture_mu_exterior })]
}
//#endregion 🔖️Inverse
