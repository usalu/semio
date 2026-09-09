//! 🔺️ `create-building-model ` — sparse diff construction from an exact composed model child handle.

use super::CreateBuildingModel;
use crate::diff::CadDiff;
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateBuildingModel, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let candidate = match crate::cad_model_child_from_uri(&payload.child_id, &payload.target) {
        Ok(candidate) => candidate,
        Err(reason) => return protocol::MutationOutcome::fatal("mutation.child-identity", reason, [payload.child_id.clone()]),
    };
    if base.building_model.as_ref() == Some(&candidate) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Building-model child is already {}.", payload.child_id));
    }
    protocol::MutationOutcome::new(CadDiff { building_model: Some(Some(candidate)), ..Default::default() })
}
//#endregion 🔖️Diff
