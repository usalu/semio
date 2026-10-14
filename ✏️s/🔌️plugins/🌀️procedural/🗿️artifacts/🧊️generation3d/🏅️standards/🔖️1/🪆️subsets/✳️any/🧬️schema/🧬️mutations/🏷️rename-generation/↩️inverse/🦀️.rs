//! ↩️ `rename-generation` inverse — old name looked up from BASE, never inverted structurally;
//! missing target ⇒ nothing to undo.

use crate::rename_generation::RenameGeneration;
use crate::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub fn inverse(payload: &RenameGeneration, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.generation.generations.iter().find(|entry| entry.id == payload.id).map(|entry| vec![Generation3dMutation::RenameGeneration(RenameGeneration { id: payload.id.clone(), new_name: entry.name.clone() })]).unwrap_or_default()

    })())
}
