//! ↩️ Inverse for `DeleteGeneration`, reconstructed from BASE.
use super::DeleteGeneration;
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteGeneration, base: &Generation2dSnapshot) -> Result<Vec<Generation2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.generation.generations.iter().position(|entry| entry.id == payload.id) {
        Some(at) => vec![Generation2dMutation::CreateGeneration(crate::standards::v1::subsets::any::schema::mutations::create_generation::CreateGeneration { generation: base.generation.generations[at].clone(), index: Some(at) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
