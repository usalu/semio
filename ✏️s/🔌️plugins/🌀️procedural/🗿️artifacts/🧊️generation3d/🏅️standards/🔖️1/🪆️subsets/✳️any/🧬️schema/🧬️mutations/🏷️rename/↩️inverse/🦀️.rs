//! ↩️ `rename-generation` inverse — old name looked up from BASE, never inverted structurally;
//! missing target ⇒ nothing to undo.

use crate::standards::v1::subsets::any::schema::mutations::rename_generation::RenameGeneration;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

pub fn inverse(payload: &RenameGeneration, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.generation.generations.iter().find(|entry| entry.id == payload.id).map(|entry| vec![Generation3dMutation::RenameGeneration(RenameGeneration { id: payload.id.clone(), new_name: entry.name.clone() })]).unwrap_or_default()

    })())
}
