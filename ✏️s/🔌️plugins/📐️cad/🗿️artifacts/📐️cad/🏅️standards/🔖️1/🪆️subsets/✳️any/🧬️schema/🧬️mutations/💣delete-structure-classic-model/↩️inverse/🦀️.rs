//! ↩️ `delete-structure-classic-model` — undo is `create-structure-classic-model` with the escrowed handle captured from BASE;
//! empty (`Vec::new()`) when the slot was already absent (nothing to undo).

use super::DeleteStructureClassicModel;
use crate::create_structure_classic_model;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_structure_classic_model::CreateStructureClassicModel>>(_payload: &DeleteStructureClassicModel, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match &base.structure_classic_model {
        Some(existing) => vec![Op::from(create_structure_classic_model::CreateStructureClassicModel { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
