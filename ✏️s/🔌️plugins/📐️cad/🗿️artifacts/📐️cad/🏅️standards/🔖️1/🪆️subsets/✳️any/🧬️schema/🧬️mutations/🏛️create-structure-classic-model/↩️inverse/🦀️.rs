//! ↩️ `create-structure-classic-model` — undo restores whichever handle occupied `structure_classic_model` BEFORE this create ran
//! (a real prior handle if the slot was occupied, or `delete-structure-classic-model` if it was empty) — never a
//! bare "delete", since `create-structure-classic-model` may have OVERWRITTEN an existing handle.

use super::CreateStructureClassicModel;
use crate::delete_structure_classic_model;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_structure_classic_model::CreateStructureClassicModel> + From<crate::delete_structure_classic_model::DeleteStructureClassicModel>>(_payload: &CreateStructureClassicModel, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match &base.structure_classic_model {
        Some(existing) => vec![Op::from(CreateStructureClassicModel { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => vec![Op::from(delete_structure_classic_model::DeleteStructureClassicModel {})],
    })
}
//#endregion 🔖️Inverse
