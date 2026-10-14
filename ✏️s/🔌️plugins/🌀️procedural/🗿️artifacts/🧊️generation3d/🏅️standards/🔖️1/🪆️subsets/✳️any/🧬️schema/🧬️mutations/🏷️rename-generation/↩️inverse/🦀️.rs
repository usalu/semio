//! ↩️ `rename-generation` inverse — old name looked up from BASE, never inverted structurally;
//! missing target ⇒ nothing to undo.

use crate::rename_generation::RenameGeneration;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub fn inverse<Op: From<RenameGeneration>>(payload: &RenameGeneration, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.generation.generations.iter().find(|entry| entry.id == payload.id).map(|entry| vec![Op::from(RenameGeneration { id: payload.id.clone(), new_name: entry.name.clone() })]).unwrap_or_default()

    })())
}
