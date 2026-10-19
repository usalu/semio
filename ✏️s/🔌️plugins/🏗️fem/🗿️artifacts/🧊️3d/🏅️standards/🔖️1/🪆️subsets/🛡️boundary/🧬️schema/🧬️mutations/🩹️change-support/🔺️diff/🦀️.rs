//! 🔺️ Validates supplied support fields on the current replay base.
use super::ChangeSupport;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSupportsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSupportsModification;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

pub fn diff(payload: &ChangeSupport, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.supports.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let candidate = payload.patch.candidate(existing);
    if !payload.patch.valid() || !semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemDof::canonical(&existing.fixed) { return protocol::MutationOutcome::fatal("mutation.invariant", "Support sets must be canonical and must not accompany membership setters.", [payload.id.clone()]); }
    if !base.nodes.iter().any(|node| node.id == candidate.node_id) { return protocol::MutationOutcome::error("mutation.target-missing", format!("Node {} does not exist.", candidate.node_id), [candidate.node_id.clone()]); }
    if candidate == *existing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Support \"{}\" already has the supplied properties.", payload.id));
    }
    protocol::MutationOutcome::new(Fem3dDiff { supports: Some(Fem3dSupportsDelta { modified: vec![Fem3dSupportsModification { id: payload.id.clone(), patch: payload.patch.clone() }], ..Default::default() }), ..Default::default() })
}
