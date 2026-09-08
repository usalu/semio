
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

#[test]
fn inference_default_law() {
    assert_eq!(Generation3dInference::infer(&Generation3dSnapshot::default()), Generation3dInference::default());
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
