//! 🧪️ The reusable `wfc-graph` window — projection, ports and the copy contract `wfc3d` depends on.

use super::{definition, graph_records, render, GraphCamera, GraphEdgeView, GraphSlotView, SlotGraphView, WFC_GRAPH_BODY, WFC_GRAPH_PORT_IN, WFC_GRAPH_PORT_OUT, WFC_GRAPH_WINDOW};

/// 🧩️ A local view with no artifact behind it at all — the point of the trait is that this compiles.
struct TwoSlots;

impl SlotGraphView for TwoSlots {
    fn graph_slots(&self) -> Vec<GraphSlotView> {
        vec![
            GraphSlotView { id: "a".into(), x: 0.0, y: 0.0, width: 2.0, height: 2.0, pinned_tile_id: None },
            GraphSlotView { id: "b".into(), x: 4.0, y: 0.0, width: 2.0, height: 2.0, pinned_tile_id: Some("wall".into()) },
        ]
    }
    fn graph_edges(&self) -> Vec<GraphEdgeView> {
        vec![GraphEdgeView { id: "e".into(), from_slot_id: "a".into(), to_slot_id: "b".into(), relation: "adjacent".into() }]
    }
}

/// 🪪️ The window kind id and body key are the shared constants `wfc3d` copies verbatim.
#[test]
fn window_identity_is_the_shared_contract() {
    assert_eq!(WFC_GRAPH_WINDOW, "wfc-graph");
    assert_eq!(WFC_GRAPH_BODY, "wfc.graph");
    let definition = definition();
    assert_eq!(definition.id, WFC_GRAPH_WINDOW);
    assert_eq!(definition.body_key, WFC_GRAPH_BODY);
    assert_eq!(definition.actions.len(), 9);
}

/// 🚦️ Every verb is `Migrated`: a `BatchOnly` verb never reaches interactive dispatch and would be
/// dead in the pane.
#[test]
fn every_declared_verb_is_interactive() {
    for action in definition().actions {
        assert_eq!(action.semantics.execution.interactive_job, semio_framework::InteractiveJobClassification::Migrated, "{} is not interactive", action.id);
    }
}

/// 🕸️ One node per slot, one edge per adjacency, one in/out port pair each.
#[test]
fn projection_is_one_node_per_slot() {
    let (nodes, edges) = graph_records(&TwoSlots);
    assert_eq!(nodes.len(), 2);
    assert_eq!(edges.len(), 1);
    assert_eq!(nodes[0].inputs.len(), 1);
    assert_eq!(nodes[0].outputs.len(), 1);
    assert_eq!(nodes[0].inputs[0].id, format!("a@{WFC_GRAPH_PORT_IN}"));
    assert_eq!(edges[0].source_port_id, format!("a@{WFC_GRAPH_PORT_OUT}"));
    assert_eq!(edges[0].target_port_id, format!("b@{WFC_GRAPH_PORT_IN}"));
    assert_eq!(edges[0].label.as_deref(), Some("adjacent"));
    assert!(nodes[1].label.as_deref().is_some_and(|label| label.contains("wall")), "a pinned slot says so in its caption");
}

/// 🖼️ The surface renders non-empty for a real view.
#[test]
fn render_produces_a_surface() {
    render(&TwoSlots, GraphCamera::default(), &["a".into()]).expect("the graph window renders");
}
