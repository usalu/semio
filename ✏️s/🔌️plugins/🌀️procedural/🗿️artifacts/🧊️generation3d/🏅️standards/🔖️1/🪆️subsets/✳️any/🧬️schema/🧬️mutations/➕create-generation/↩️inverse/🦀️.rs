//! ↩️ `create-generation` inverse — undo of a create is always a `delete-generation` by id.

use crate::artifacts::generation3d::mutations::create_generation::CreateGeneration;
use crate::artifacts::generation3d::mutations::delete_generation::DeleteGeneration;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;

pub fn inverse(payload: &CreateGeneration, _base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    vec![Generation3dMutation::DeleteGeneration(DeleteGeneration { id: payload.generation.id.clone() })]
}
