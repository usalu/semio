//! 🔺️ Sparse diff builder for `DeleteMaterial`.
use super::DeleteMaterial;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dMaterialsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dMaterialRemoval;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::material_referrers;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::target_referenced;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &DeleteMaterial, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(at) = base.materials.iter().position(|material| material.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let referrers = material_referrers(base, &payload.id);
    if !referrers.is_empty() {
        return target_referenced("Material", &payload.id, referrers);
    }
    protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta { removed: vec![Fem3dMaterialRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
