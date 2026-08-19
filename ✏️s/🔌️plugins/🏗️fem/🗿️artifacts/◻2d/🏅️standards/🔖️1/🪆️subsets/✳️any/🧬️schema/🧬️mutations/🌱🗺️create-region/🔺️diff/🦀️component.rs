//! 🔺️ Sparse diff builder for `CreateRegion`.
use super::mutation::CreateRegion;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dRegionsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &CreateRegion, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.regions.iter().any(|region| region.id == payload.region.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A region with id \"{}\" already exists.", payload.region.id), [payload.region.id.clone()]);
    }
    if !base.materials.iter().any(|material| material.id == payload.region.material_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.region.material_id), [payload.region.material_id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { regions: Some(Fem2dRegionsDelta { added: vec![payload.region.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
