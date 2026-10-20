//! 🔺️ `️create-structure-classic-model ` — sparse diff construction from an exact composed model child handle.

use super::CreateStructureClassicModel;
use semio_s_artifact_cad_cad_model_diff::{CadDiff, CadModelSlot};
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateStructureClassicModel, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let candidate = match semio_s_artifact_cad_cad_model::snapshot::cad_model_child(&payload.child_id, &payload.target) {
        Ok(candidate) => candidate,
        Err(reason) => return protocol::MutationOutcome::fatal("mutation.invariant", reason, [payload.child_id.clone()]),
    };
    if base.structure_classic_model.as_ref() == Some(&candidate) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Structure-classic-model child is already {}.", payload.child_id));
    }
    protocol::MutationOutcome::new(CadDiff { structure_classic_model: Some(CadModelSlot { child: Some(candidate) }), ..Default::default() })
}
//#endregion 🔖️Diff
