//! ↩️ `update-lighting` inverse.

use crate::update_lighting::UpdateLighting;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &UpdateLighting, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_lighting::UpdateLighting> {
    Ok((|| {
    vec![Op::from(UpdateLighting { new_lighting: base.lighting.clone() })]

    })())
}
