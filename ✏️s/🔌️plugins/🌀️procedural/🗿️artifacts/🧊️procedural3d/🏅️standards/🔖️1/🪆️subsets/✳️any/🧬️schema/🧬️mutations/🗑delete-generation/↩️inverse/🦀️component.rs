//! ↩️ `delete-generation` inverse — reconstructs a `create-generation` from BASE state; a
//! generation already absent from `base` has nothing to undo.

use crate::artifacts::procedural3d::mutations::create_generation::mutation::CreateGeneration;
use crate::artifacts::procedural3d::mutations::delete_generation::mutation::DeleteGeneration;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

pub fn inverse(payload: &DeleteGeneration, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    base.generation.generations.iter().find(|entry| entry.id == payload.id).map(|entry| vec![Procedural3dMutation::CreateGeneration(CreateGeneration { generation: entry.clone() })]).unwrap_or_default()
}
