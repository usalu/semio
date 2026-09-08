
use super::*;
use protocol::Inference;
use semio_framework_artifact_flow_flow::Widget;

fn chain_snapshot() -> FlowSnapshot {
    let mut fixture = FlowSnapshot::default().to_fixture();
    fixture.widgets = vec![Widget::InputSlider { id: "a".into(), label: "A".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 }, Widget::InputSlider { id: "b".into(), label: "B".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 }];
    fixture.synapses = vec![semio_framework_artifact_flow_flow::SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: String::new(), to_port: String::new() }];
    FlowSnapshot::from_fixture(fixture)
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = chain_snapshot();
    assert_eq!(FlowInference::infer(&snapshot), FlowInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(FlowInference::infer(&FlowSnapshot::default()), FlowInference::default());
}

#[semio_framework_async_macros::async_test]
async fn topology_counts_every_widget_exactly_once() {
    let snapshot = chain_snapshot();
    let inferred = FlowInference::infer(&snapshot);
    let widget_count = snapshot.to_fixture().widgets.len();
    assert_eq!(inferred.topology.node_count as usize, widget_count);
    assert_eq!(inferred.topology.topo_order.len(), widget_count);
    assert!(inferred.topology.cycle_free);
}
