//! 🔺️ Sparse diff builder for `DeleteMaterial`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error), then
//! `mutation.target-referenced` (Error) while any element or region still names this material.
use super::DeleteMaterial;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dMaterialsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dMaterialRemoval;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &DeleteMaterial, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(at) = base.materials.iter().position(|material| material.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::referenced("Material", "element or region", &payload.id, guards::material_referrers(base, &payload.id)) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { materials: Some(Fem2dMaterialsDelta { removed: vec![Fem2dMaterialRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
