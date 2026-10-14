//! ↩️ `create-generation` inverse — undo of a create is always a `delete-generation` by id.

use crate::create_generation::CreateGeneration;
use crate::delete_generation::DeleteGeneration;
use crate::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub fn inverse(payload: &CreateGeneration, _base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Generation3dMutation::DeleteGeneration(DeleteGeneration { id: payload.generation.id.clone() })]

    })())
}
