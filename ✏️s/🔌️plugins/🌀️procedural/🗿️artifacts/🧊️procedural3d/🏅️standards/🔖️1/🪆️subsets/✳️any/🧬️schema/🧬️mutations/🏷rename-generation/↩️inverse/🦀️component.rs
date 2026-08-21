//! ↩️ `rename-generation` inverse — old name looked up from BASE, never inverted structurally;
//! missing target ⇒ nothing to undo.

use crate::artifacts::procedural3d::mutations::rename_generation::mutation::RenameGeneration;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

pub async fn inverse(payload: &RenameGeneration, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    base.generation.generations.iter().find(|entry| entry.id == payload.id).map(|entry| vec![Procedural3dMutation::RenameGeneration(RenameGeneration { id: payload.id.clone(), new_name: entry.name.clone() })]).unwrap_or_default()
}
