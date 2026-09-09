//! 🔺️ `create-shape-model ` — sparse diff construction from an exact composed model child handle.

use super::CreateShapeModel;
use crate::diff::CadDiff;
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateShapeModel, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let candidate = match crate::cad_model_child_from_uri(&payload.child_id, &payload.target) {
        Ok(candidate) => candidate,
        Err(reason) => return protocol::MutationOutcome::fatal("mutation.child-identity", reason, [payload.child_id.clone()]),
    };
    if base.shape_model.as_ref() == Some(&candidate) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shape-model child is already {}.", payload.child_id));
    }
    protocol::MutationOutcome::new(CadDiff { shape_model: Some(Some(candidate)), ..Default::default() })
}
//#endregion 🔖️Diff
