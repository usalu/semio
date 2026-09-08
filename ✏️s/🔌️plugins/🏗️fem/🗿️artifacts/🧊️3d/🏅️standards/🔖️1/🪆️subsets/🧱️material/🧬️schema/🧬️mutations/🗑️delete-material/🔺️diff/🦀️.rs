//! 🔺️ Sparse diff builder for `DeleteMaterial`.
use super::DeleteMaterial;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dMaterialsDelta};
use crate::standards::v1::subsets::any::schema::mutations::{material_referrers, target_referenced};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteMaterial, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.materials.iter().any(|material| material.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let referrers = material_referrers(base, &payload.id);
    if !referrers.is_empty() {
        return target_referenced("Material", &payload.id, referrers);
    }
    protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
