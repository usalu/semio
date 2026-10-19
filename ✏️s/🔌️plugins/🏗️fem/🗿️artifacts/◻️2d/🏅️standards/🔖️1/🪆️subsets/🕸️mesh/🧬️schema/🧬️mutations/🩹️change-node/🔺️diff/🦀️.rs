//! 🔺️ Validates supplied node fields on the current replay base.
use super::ChangeNode;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodesDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dNodesModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

pub fn diff(payload: &ChangeNode, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.nodes.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let candidate = payload.patch.candidate(existing);
    if let Some(rejection) = guards::node_geometry(&candidate) { return rejection; }
    if candidate == *existing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" already has the supplied properties.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { nodes: Some(Fem2dNodesDelta { modified: vec![Fem2dNodesModification { id: payload.id.clone(), patch: payload.patch.clone() }], ..Default::default() }), ..Default::default() })
}
