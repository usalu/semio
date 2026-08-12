//! ↩️ `create-generation` inverse — undo of a create is always a `delete-generation` by id.

use crate::artifacts::procedural3d::mutations::create_generation::mutation::CreateGeneration;
use crate::artifacts::procedural3d::mutations::delete_generation::mutation::DeleteGeneration;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

pub fn inverse(payload: &CreateGeneration, _base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    vec![Procedural3dMutation::DeleteGeneration(DeleteGeneration { id: payload.generation.id.clone() })]
}
