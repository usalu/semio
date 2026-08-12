//! ↩️ `update-climate` inverse — restores the pre-change `climate` facet from BASE state; `update`
//! is its own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din18599::mutations::update_climate::mutation::UpdateClimate;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateClimate, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::UpdateClimate(UpdateClimate { new_climate: base.climate.clone() })]
}
//#endregion 🔖️Inverse
