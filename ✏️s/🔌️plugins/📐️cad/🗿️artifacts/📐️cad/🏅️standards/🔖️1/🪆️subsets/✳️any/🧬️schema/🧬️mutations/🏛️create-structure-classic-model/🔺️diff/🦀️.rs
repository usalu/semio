//! 🔺️ `️create-structure-classic-model ` — sparse diff construction from an exact composed model child handle.

use super::CreateStructureClassicModel;
use crate::diff::CadDiff;
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateStructureClassicModel, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let candidate = match crate::cad_model_child_from_uri(&payload.child_id, &payload.target) {
        Ok(candidate) => candidate,
        Err(reason) => return protocol::MutationOutcome::fatal("mutation.child-identity", reason, [payload.child_id.clone()]),
    };
    if base.structure_classic_model.as_ref() == Some(&candidate) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Structure-classic-model child is already {}.", payload.child_id));
    }
    protocol::MutationOutcome::new(CadDiff { structure_classic_model: Some(Some(candidate)), ..Default::default() })
}
//#endregion 🔖️Diff
