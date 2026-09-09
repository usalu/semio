use super::*;
use protocol::Inference;
use semio_framework_artifact_flow_flow::{FlowFixture, SynapseSpec, Widget};

//#region 🧸️Fixtures
fn sample_snapshot() -> Generation3dSnapshot {
    let mut snapshot = Generation3dSnapshot::default();
    snapshot.fixture = FlowFixture {
        schema: "flow.fixture".into(),
        camera: semio_framework_artifact_flow_flow::CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![
            Widget::InputSlider { id: "a".into(), label: "A".into(), value: 1.0, min: 0.0, max: 10.0, step: 1.0 },
            Widget::Neuron { id: "b".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: false },
            Widget::OutputPreview { id: "c".into(), preview: Default::default(), expanded: Default::default() },
        ],
        synapses: vec![
            SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "value".into(), to_port: "a".into() },
            SynapseSpec { id: "s2".into(), from: "b".into(), to: "c".into(), from_port: "sum".into(), to_port: String::new() },
        ],
        layout: Default::default(),
    };
    snapshot
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    let snapshot = sample_snapshot();
    assert_eq!(Generation3dInference::infer(&snapshot), Generation3dInference::infer(&snapshot));
}

/// 💡️ LAW (totality on the identity element): the EMPTY document infers an empty topology — no
/// nodes, no edges, an empty order, zero depth, and trivially cycle-free.
///
/// This is deliberately not `infer(Default::default()) == Inference::default()`: neither side of
/// that equation holds. `Generation3dSnapshot::default()` is the three-widget demo graph
/// (`default_generation3d_snapshot`'s docstring), and `Generation3dTopology::default()` is the
/// DERIVED zero value, whose `cycle_free: false` contradicts the empty graph's real semantics
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn inference_of_the_empty_document_is_empty_and_trivially_cycle_free() {
    let empty = crate::standards::v1::subsets::any::schema::empty_generation3d_snapshot();
    assert_eq!(
        Generation3dInference::infer(&empty),
        Generation3dInference { topology: Generation3dTopology { node_count: 0, edge_count: 0, topo_order: Vec::new(), depth: 0, cycle_free: true } }
    );
    empty.retire_cold();
}

#[test]
fn topology_matches_the_linear_chain() {
    let snapshot = sample_snapshot();
    let inferred = Generation3dInference::infer(&snapshot);
    assert_eq!(inferred.topology.node_count, 3);
    assert_eq!(inferred.topology.edge_count, 2);
    assert!(inferred.topology.cycle_free);
    assert_eq!(inferred.topology.depth, 2);
    assert_eq!(inferred.topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}
//#endregion 🧪️InferenceLaws
