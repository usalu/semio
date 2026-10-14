//! ↩️ Inverse for `RenameGeneration`, reconstructed from BASE.
use super::RenameGeneration;
use crate::rename_generation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<RenameGeneration>>(payload: &RenameGeneration, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.generation.generations.iter().find(|entry| entry.id == payload.id) {
        Some(entry) => vec![rename_generation(payload.id.clone(), entry.name.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
