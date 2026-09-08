//! ↩️ `create-generation` inverse — undo of a create is always a `delete-generation` by id.

use crate::standards::v1::subsets::any::schema::mutations::create_generation::CreateGeneration;
use crate::standards::v1::subsets::any::schema::mutations::delete_generation::DeleteGeneration;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

pub fn inverse(payload: &CreateGeneration, _base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    vec![Generation3dMutation::DeleteGeneration(DeleteGeneration { id: payload.generation.id.clone() })]
}
