//! ↩️ `delete-generation` inverse — reconstructs a `create-generation` from BASE state; a
//! generation already absent from `base` has nothing to undo.

use crate::artifacts::generation3d::mutations::create_generation::CreateGeneration;
use crate::artifacts::generation3d::mutations::delete_generation::DeleteGeneration;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;

pub fn inverse(payload: &DeleteGeneration, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    base.generation.generations.iter().find(|entry| entry.id == payload.id).map(|entry| vec![Generation3dMutation::CreateGeneration(CreateGeneration { generation: entry.clone() })]).unwrap_or_default()
}
