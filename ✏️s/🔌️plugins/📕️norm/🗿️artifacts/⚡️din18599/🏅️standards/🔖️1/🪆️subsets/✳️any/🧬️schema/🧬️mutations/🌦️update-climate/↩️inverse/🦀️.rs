//! ↩️ `update-climate` inverse — restores the pre-change `climate` facet from BASE state; `update`
//! is its own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::update_climate::UpdateClimate;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateClimate, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::UpdateClimate(UpdateClimate { new_climate: crate::din18599_climate(base) })]
}
//#endregion 🔖️Inverse
