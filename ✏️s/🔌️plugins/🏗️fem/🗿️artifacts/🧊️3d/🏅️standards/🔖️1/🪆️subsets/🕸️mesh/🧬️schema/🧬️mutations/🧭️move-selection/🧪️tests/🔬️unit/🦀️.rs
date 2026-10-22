//! 🧪️ Shared transform vectors preserve concurrent geometry and inspector field intent.
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::{ColdMutation, Mutation};

use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;

fn fixture() -> DslValue {
    semio_framework_pack_json::from_json_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧭️intent/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("transform intent vector")
}

fn decode<T: FromValue>(fixture: &DslValue, key: &str) -> T {
    T::from_value(fixture.get(key).expect("fixture field").clone()).expect("typed fixture")
}

fn apply(base: &Fem3dSnapshotValue, mutation: &Fem3dMutation) -> Fem3dSnapshotValue {
    crate::vcs::apply_mutation(base, mutation).expect("valid mutation").0
}

#[test]
fn transform_authors_only_changed_geometry_and_preserves_peer_fields() {
    let vector = fixture();
    let base: Fem3dSnapshotValue = decode(&vector, "base");
    let motion: Fem3dMutation = decode(&vector, "move");
    let peers: Vec<Fem3dMutation> = decode(&vector, "peer");
    let forward = motion.diff(&base);
    assert_eq!(forward.diff().nodes.as_ref().unwrap().modified[0].patch.to_value(), *vector.get("nodePatch").unwrap());
    assert_eq!(forward.diff().solids.as_ref().unwrap().modified[0].patch.to_value(), *vector.get("solidPatch").unwrap());
    let peer = peers.iter().fold(base.clone(), |state, event| apply(&state, event));
    let combined = protocol::apply_diff_cold(forward.diff(), &peer).unwrap();
    assert_eq!(combined.nodes[0].to_value(), *vector.get("combinedNode").unwrap());
    assert_eq!(combined.solids[0].to_value(), *vector.get("combinedSolid").unwrap());
    let reverse = peers.iter().fold(apply(&base, &motion), |state, event| apply(&state, event));
    assert_eq!(combined, reverse, "same-base independent geometry and inspector edits converge");
    assert_eq!(apply(&peer, &motion), combined, "semantic replay recalculates against peer geometry");
    let inverse = motion.inverse(&base).unwrap();
    assert_eq!(inverse.to_value(), *vector.get("inverse").unwrap());
    let undone = inverse.iter().rev().fold(combined.clone(), |state, event| apply(&state, event));
    assert_eq!(undone.nodes[0].to_value(), *vector.get("undoneNode").unwrap());
    assert_eq!(undone.solids[0].to_value(), *vector.get("undoneSolid").unwrap());
    assert_eq!(undone, peer, "inverse restores only authored geometry");
    assert_eq!(apply(&undone, &motion), combined, "redo retains peer fields");
    assert_eq!(protocol::apply_diff_cold(&protocol::DiffAlgebra::inverse(forward.diff(), &base), &combined).unwrap(), peer);
    eprintln!("[DEBUG] FEM3d sparse transform diff, inverse, concurrent inspector replay and redo match the shared vector");
}

#[test]
fn finite_transform_overflow_is_refused_atomically_without_an_inverse() {
    let vector = fixture();
    let mut base: Fem3dSnapshotValue = decode(&vector, "base");
    base.nodes = vec![decode(&vector, "overflowNode")];
    let Fem3dMutation::MoveSelection(mut motion) = decode(&vector, "move") else { panic!("move vector") };
    motion.dx = decode(&vector, "overflowOffset");
    let motion = Fem3dMutation::MoveSelection(motion);
    let outcome = motion.diff(&base);
    assert_eq!(outcome.diff(), &Default::default(), "overflow carries no partial patch");
    assert_eq!(outcome.messages()[0].code.0, vector.get("overflowCode").unwrap().as_str().unwrap());
    assert!(outcome.messages()[0].level >= semio_framework_diagnostic::Severity::Error);
    assert_eq!(apply(&base, &motion), base);
    assert!(motion.inverse(&base).unwrap().is_empty());
    eprintln!("[DEBUG] FEM3d finite transform overflow refuses publication atomically");
}

#[test]
fn rotation_and_scaling_inverse_leave_unmoved_peer_coordinates_intact() {
    let vector = fixture();
    let base: Fem3dSnapshotValue = decode(&vector, "base");
    for scenario in vector.get("motions").unwrap().as_array().unwrap() {
        let Fem3dMutation::MoveSelection(mut motion) = decode(&vector, "move") else { panic!("move vector") };
        motion.solid_ids.clear();
        motion.dx = decode(scenario, "dx");
        motion.axis_x = decode(scenario, "axisX");
        motion.axis_z = decode(scenario, "axisZ");
        motion.angle = decode(scenario, "angle");
        motion.sx = decode(scenario, "sx");
        let operation = Fem3dMutation::MoveSelection(motion);
        let moved = apply(&base, &operation);
        let position: [f64; 3] = decode(scenario, "position");
        for (actual, expected) in [moved.nodes[0].x, moved.nodes[0].y, moved.nodes[0].z].into_iter().zip(position) { assert!((actual - expected).abs() < 1e-12); }
        let diff = operation.diff(&base);
        let fields: Vec<_> = diff.diff().nodes.as_ref().unwrap().modified[0].patch.to_value().as_object().unwrap().iter().map(|(key, _)| key.clone()).collect();
        assert_eq!(fields, decode::<Vec<String>>(scenario, "authored"));
        let peer = Fem3dMutation::ChangeNode(crate::standards::v1::subsets::any::schema::mutations::change_node::ChangeNode { id: "n1".into(), patch: decode(scenario, "peerPatch") });
        let combined = apply(&moved, &peer);
        let restored = operation.inverse(&base).unwrap().iter().rev().fold(combined, |state, event| apply(&state, event));
        assert_eq!(restored, apply(&base, &peer));
    }
    eprintln!("[DEBUG] FEM3d rotation and scale preserve unowned peer coordinates through exact undo");
}
