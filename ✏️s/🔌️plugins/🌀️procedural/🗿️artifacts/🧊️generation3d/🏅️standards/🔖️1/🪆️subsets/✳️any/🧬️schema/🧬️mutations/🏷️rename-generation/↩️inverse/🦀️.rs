//! ↩️ `rename-generation` inverse — old name looked up from BASE, never inverted structurally;
//! missing target ⇒ nothing to undo.

use crate::mutations::rename_generation::RenameGeneration;
use crate::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

pub fn inverse(payload: &RenameGeneration, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    base.generation.generations.iter().find(|entry| entry.id == payload.id).map(|entry| vec![Generation3dMutation::RenameGeneration(RenameGeneration { id: payload.id.clone(), new_name: entry.name.clone() })]).unwrap_or_default()
}
