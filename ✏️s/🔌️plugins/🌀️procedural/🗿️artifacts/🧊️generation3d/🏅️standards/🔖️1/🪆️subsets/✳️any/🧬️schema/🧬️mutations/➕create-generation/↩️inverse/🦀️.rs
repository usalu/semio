//! ↩️ `create-generation` inverse — undo of a create is always a `delete-generation` by id.

use crate::create_generation::CreateGeneration;
use crate::delete_generation::DeleteGeneration;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub fn inverse<Op: From<crate::delete_generation::DeleteGeneration>>(payload: &CreateGeneration, _base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(DeleteGeneration { id: payload.generation.id.clone() })]

    })())
}
