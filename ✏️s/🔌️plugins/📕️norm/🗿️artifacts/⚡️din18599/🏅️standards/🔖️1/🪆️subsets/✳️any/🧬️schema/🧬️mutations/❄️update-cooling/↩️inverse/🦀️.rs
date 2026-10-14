//! ↩️ `update-cooling` inverse.

use crate::update_cooling::UpdateCooling;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &UpdateCooling, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_cooling::UpdateCooling> {
    Ok((|| {
    vec![Op::from(UpdateCooling { new_cooling: base.cooling.clone() })]

    })())
}
