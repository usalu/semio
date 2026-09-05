//! ↩️ `change-solar-absorptance` — undo restores BASE's `solar_absorptance`.

use super::ChangeSolarAbsorptance;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSolarAbsorptance, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeSolarAbsorptance(ChangeSolarAbsorptance { new_solar_absorptance: base.solar_absorptance })]
}
//#endregion 🔖️Inverse
