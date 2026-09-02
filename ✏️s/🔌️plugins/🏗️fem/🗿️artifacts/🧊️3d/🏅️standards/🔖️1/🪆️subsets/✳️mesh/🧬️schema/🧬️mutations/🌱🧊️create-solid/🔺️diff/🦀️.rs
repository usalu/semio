//! 🔺️ Sparse diff builder for `CreateSolid`.
use super::CreateSolid;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSolidsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSolid, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.solids.iter().any(|solid| solid.id == payload.solid.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A solid with id \"{}\" already exists.", payload.solid.id), [payload.solid.id.clone()]);
    }
    if !base.materials.iter().any(|material| material.id == payload.solid.material_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.solid.material_id), [payload.solid.material_id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { solids: Some(Fem3dSolidsDelta { added: vec![payload.solid.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
