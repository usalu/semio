//! 🔺️ Sparse diff builder for `ReplaceMaterial`.
use super::ReplaceMaterial;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dMaterialPatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dMaterialsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dMaterialsModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::target_mismatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::invariant;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::material_breach;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceMaterial, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.materials.iter().find(|material| material.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_material.id != payload.id {
        return target_mismatch("Material", &payload.id, &payload.new_material.id);
    }
    if existing == &payload.new_material {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Material \"{}\" already has that value.", payload.id));
    }
    if let Some(breach) = material_breach(&payload.new_material) {
        return invariant(breach, vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta { modified: vec![Fem3dMaterialsModification { id: payload.id.clone(), patch: Fem3dMaterialPatch::replacement(&payload.new_material) }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
