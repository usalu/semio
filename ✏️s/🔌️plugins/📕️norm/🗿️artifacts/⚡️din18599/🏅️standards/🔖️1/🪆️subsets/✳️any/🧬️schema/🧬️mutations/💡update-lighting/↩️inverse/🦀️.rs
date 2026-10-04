//! ↩️ `update-lighting` inverse.

use crate::mutations::update_lighting::UpdateLighting;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &UpdateLighting, base: &Din18599Snapshot) -> Result<Vec<Din18599Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din18599Mutation::UpdateLighting(UpdateLighting { new_lighting: base.lighting.clone() })]

    })())
}
