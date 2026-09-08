
use super::*;

fn slider(id: &str) -> Widget {
    Widget::InputSlider { id: id.into(), label: id.into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 }
}

fn synapse(id: &str, from: &str, to: &str) -> SynapseSpec {
    SynapseSpec { id: id.into(), from: from.into(), to: to.into(), from_port: String::new(), to_port: String::new() }
}

#[semio_framework_async_macros::async_test]
async fn linear_chain_orders_roots_before_leaves_with_increasing_depth() {
    let widgets = vec![slider("a"), slider("b"), slider("c")];
    let synapses = vec![synapse("s1", "a", "b"), synapse("s2", "b", "c")];
    let topology = compute_flow_topology(&widgets, &synapses);
    assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert_eq!(topology.depth.get("c"), Some(&2));
    assert!(topology.cycle_free);
    assert_eq!(topology.node_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn a_cycle_is_reported_as_not_cycle_free_but_still_totals_every_widget() {
    let widgets = vec![slider("a"), slider("b")];
    let synapses = vec![synapse("s1", "a", "b"), synapse("s2", "b", "a")];
    let topology = compute_flow_topology(&widgets, &synapses);
    assert!(!topology.cycle_free);
    assert_eq!(topology.topo_order.len(), 2);
}
