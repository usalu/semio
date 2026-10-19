//! 🔺️ Validates supplied solid fields on the current replay base.
use super::ChangeSolid;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSolidsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSolidsModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::invariant;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::solid_breach;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

pub fn diff(payload: &ChangeSolid, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.solids.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solid \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let candidate = payload.patch.candidate(existing);
    if !base.materials.iter().any(|row| row.id == candidate.material_id) { return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", candidate.material_id), [candidate.material_id.clone()]); }
    if let Some(breach) = solid_breach(&candidate) { return invariant(breach, vec![payload.id.clone()]); }
    if candidate == *existing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Solid \"{}\" already has the supplied properties.", payload.id));
    }
    protocol::MutationOutcome::new(Fem3dDiff { solids: Some(Fem3dSolidsDelta { modified: vec![Fem3dSolidsModification { id: payload.id.clone(), patch: payload.patch.clone() }], ..Default::default() }), ..Default::default() })
}
