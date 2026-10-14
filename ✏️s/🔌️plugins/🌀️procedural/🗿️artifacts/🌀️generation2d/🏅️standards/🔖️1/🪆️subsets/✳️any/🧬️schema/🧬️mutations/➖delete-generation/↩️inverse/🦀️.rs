//! ↩️ Inverse for `DeleteGeneration`, reconstructed from BASE.
use super::DeleteGeneration;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_generation::CreateGeneration> + From<crate::select_generation::SelectGeneration>>(payload: &DeleteGeneration, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.generation.generations.iter().position(|entry| entry.id == payload.id) {
        Some(at) => {
            let mut steps = Vec::new();
            if base.generation.selected_generation_id.as_deref() != Some(payload.id.as_str()) {
                steps.push(crate::select_generation::select_generation(base.generation.selected_generation_id.clone()));
            }
            steps.push(Op::from(crate::create_generation::CreateGeneration { generation: base.generation.generations[at].clone(), index: Some(at) }));
            steps
        }
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
