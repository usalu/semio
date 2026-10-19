//! 🔺️ Sparse diff builder for `ReplaceSolid`.
use super::ReplaceSolid;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSolidsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSolidsModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::target_mismatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::invariant;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::solid_breach;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSolid, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.solids.iter().find(|solid| solid.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solid \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_solid.id != payload.id {
        return target_mismatch("Solid", &payload.id, &payload.new_solid.id);
    }
    if existing == &payload.new_solid {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Solid \"{}\" already has that value.", payload.id));
    }
    if !base.materials.iter().any(|material| material.id == payload.new_solid.material_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \"{}\" does not exist.", payload.new_solid.material_id), [payload.new_solid.material_id.clone()]);
    }
    if let Some(breach) = solid_breach(&payload.new_solid) {
        return invariant(breach, vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { solids: Some(Fem3dSolidsDelta { modified: vec![Fem3dSolidsModification { id: payload.id.clone(), patch: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSolidPatch::replacement(&payload.new_solid) }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
