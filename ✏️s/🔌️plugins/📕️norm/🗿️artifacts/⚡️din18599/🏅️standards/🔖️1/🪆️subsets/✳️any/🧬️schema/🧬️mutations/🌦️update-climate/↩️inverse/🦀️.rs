//! ↩️ `update-climate` inverse — restores the pre-change `climate` facet from BASE state; `update`
//! is its own inverse partner (per `📓️taxonomy.md`).

use crate::update_climate::UpdateClimate;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &UpdateClimate, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_climate::UpdateClimate> {
    Ok(vec![Op::from(UpdateClimate { new_climate: base.climate.clone() })])
}
//#endregion 🔖️Inverse
