//! ↩️ Inverse for `CreateGeneration`, reconstructed from BASE.
use super::CreateGeneration;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::delete_generation::DeleteGeneration>>(payload: &CreateGeneration, _base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::delete_generation::delete_generation(payload.generation.id.clone())]

    })())
}
//#endregion 🔖️Inverse
