use super::*;
use crate::editor::generation3d::unit_tests::context::{self, app_with_registry, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use crate::standards::v1::subsets::any::schema::{example_snapshot, PROCEDURAL_EXAMPLE_HEX_COLUMN};

/// 🧫️ The ONE language-agnostic statement of graph keyboard traversal, answered here by the Rust
/// implementation and independently by `🧪️tests/🔬️unit/🟦️.ts`.
const GRAPH_KEYBOARD_FIXTURE_JSON: &str = include_str!("../../../../../🧫️fixtures/🧭️graph-keyboard-navigation.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(GRAPH_KEYBOARD_FIXTURE_JSON).expect("graph keyboard fixture")
}

fn hex_column() -> Generation3dSnapshotRead {
    Generation3dSnapshotRead::new(example_snapshot(PROCEDURAL_EXAMPLE_HEX_COLUMN).expect("the bundled hexagonal mushroom column example"))
}

fn step_of(name: &str) -> FlowGraphStep {
    match name {
        "next" => FlowGraphStep::Next,
        "previous" => FlowGraphStep::Previous,
        "upstream" => FlowGraphStep::Upstream,
        "downstream" => FlowGraphStep::Downstream,
        other => panic!("{other} is not a declared keyboard step"),
    }
}

fn strings(value: &serde_json::Value) -> Vec<String> {
    value.as_array().expect("id list").iter().map(|id| id.as_str().expect("id").to_string()).collect()
}

/// ⚖️ LAW: the fixture's `graph` is the bundled `hexagonal-mushroom-column` example, node for node,
/// coordinate for coordinate and wire for wire.
///
/// Without this the twin below would be answering a graph nobody opens: the fixture could drift from
/// the document, both halves would keep agreeing with each other, and the app would traverse
/// something else entirely. It is the hinge that makes a language-neutral fixture a statement about
/// THIS artifact rather than about an invented one.
#[test]
fn the_shared_fixture_restates_the_bundled_hex_column_graph() {
    let fixture = fixture();
    let document = hex_column();
    let graph = &document.host_document;
    assert_eq!(fixture["exampleId"].as_str(), Some(PROCEDURAL_EXAMPLE_HEX_COLUMN), "the fixture names the example it restates");
    let declared = fixture["graph"]["nodes"].as_array().expect("nodes");
    assert_eq!(declared.len(), graph.widgets.len(), "the fixture states every widget of the example and no other");
    for node in declared {
        let id = node["id"].as_str().expect("node id");
        let widget = graph.widgets.iter().find(|widget| crate::widget_id(widget) == id).unwrap_or_else(|| panic!("{id} is not a widget of the example"));
        let layout = graph.layout.get(crate::widget_id(widget)).unwrap_or_else(|| panic!("{id} carries no layout"));
        assert_eq!(layout.x, node["x"].as_f64().expect("x"), "{id}: layout x");
        assert_eq!(layout.y, node["y"].as_f64().expect("y"), "{id}: layout y");
    }
    let wires = fixture["graph"]["wires"].as_array().expect("wires");
    assert_eq!(wires.len(), graph.synapses.len(), "the fixture states every wire of the example and no other");
    for wire in wires {
        let id = wire["id"].as_str().expect("wire id");
        let synapse = graph.synapses.iter().find(|synapse| synapse.id == id).unwrap_or_else(|| panic!("{id} is not a wire of the example"));
        assert_eq!(synapse.from, wire["from"].as_str().expect("from"), "{id}: source");
        assert_eq!(synapse.to, wire["to"].as_str().expect("to"), "{id}: target");
    }
}

/// ⚖️ LAW: the reading order the traversal walks is exactly the one the fixture declares — ascending
/// layout x, then ascending layout y, then the document's own widget order.
#[test]
fn the_keyboard_order_is_the_reading_order_the_fixture_declares() {
    let fixture = fixture();
    let document = hex_column();
    assert_eq!(document.host_document.keyboard_order(), strings(&fixture["keyboardOrder"]).iter().map(String::as_str).collect::<Vec<_>>(), "reading order");
}

/// ⚖️ LAW: every anchor rule the fixture states holds — a port steps from its owning node, a
/// multi-selection from its first member in reading order, a stale id from nowhere at all.
#[test]
fn every_anchor_rule_the_fixture_states_holds() {
    let fixture = fixture();
    let document = hex_column();
    for row in fixture["anchors"].as_array().expect("anchors") {
        let selection = strings(&row["selection"]);
        let expected = row["anchor"].as_str();
        assert_eq!(anchor_node(&document.host_document, &selection), expected, "{:?}: {}", selection, row["why"].as_str().unwrap_or_default());
    }
}

/// ⚖️ LAW: every arrow sequence the fixture states lands on the ids it states, step by step —
/// including the two steps that must NOT move (a source node asked for its upstream, a sink asked for
/// its downstream), which the fixture writes as a repeated row.
#[test]
fn every_arrow_walk_lands_where_the_fixture_says() {
    let fixture = fixture();
    let document = hex_column();
    let mut walked = 0usize;
    for walk in fixture["walks"].as_array().expect("walks") {
        let name = walk["name"].as_str().expect("walk name");
        let mut selection = strings(&walk["start"]);
        for step in walk["steps"].as_array().expect("steps") {
            let named = step["step"].as_str().expect("step");
            if let Some(target) = step_target(&document.host_document, &selection, step_of(named)) {
                selection = vec![target.to_string()];
            }
            assert_eq!(selection, strings(&step["expected"]), "{name}: after {named}");
            walked += 1;
        }
    }
    assert!(walked >= 20, "the fixture must carry a real walk, not a token one: {walked} steps");
    eprintln!("[DEBUG] graph keyboard walks asserted {walked} steps");
}

/// ⚖️ LAW: a traversal step publishes the framework's selection lane and NOTHING else. A keyboard
/// step is not an edit: an artifact or config op here would put arrow keys into undo history and make
/// the document dirty for moving a highlight.
#[test]
fn a_keyboard_step_authors_no_document_and_no_config_operation() {
    let document = hex_column();
    let ports = std::collections::BTreeMap::from([("extrude".to_string(), vec!["extrude@wire".to_string(), "extrude@vector".to_string()])]);
    for step in [FlowGraphStep::Next, FlowGraphStep::Previous, FlowGraphStep::Upstream, FlowGraphStep::Downstream] {
        let target = step_target(&document.host_document, &["extrude".to_string()], step);
        assert!(target.is_some(), "the hex column answers every direction from `extrude`");
    }
    assert_eq!(activate_ports(&document.host_document, &ports, &["extrude".to_string()]).map(Vec::as_slice), Some(["extrude@wire".to_string(), "extrude@vector".to_string()].as_slice()), "activate opens the anchor's ports");
    assert_eq!(activate_ports(&document.host_document, &ports, &["extrude@wire".to_string(), "extrude@vector".to_string()]), None, "an already-open node opens no further");
    assert_eq!(activate_ports(&document.host_document, &ports, &["height".to_string()]), None, "a node with no published port opens nothing");
}

/// ⚖️ LAW: the verbs the fixture names are the verbs the app declares, and the arrow chords reach
/// them — stated here rather than only in `⌨️keyboard-reachability.json` so the traversal fixture is
/// self-contained about which action each step is.
#[test]
fn every_step_the_fixture_names_is_a_declared_arg_free_verb() {
    let fixture = fixture();
    let definition = crate::editor::generation3d::create_generation3d_app();
    for (_, verb) in fixture["verbs"].as_object().expect("verbs") {
        let id = verb.as_str().expect("verb id");
        let action = definition
            .window_kinds
            .iter()
            .flat_map(|window| window.actions.iter())
            .find(|action| action.id == id)
            .unwrap_or_else(|| panic!("{id} is declared by no generation3d editor window"));
        assert!(!action.args.iter().any(|arg| arg.required), "{id} must be arg-free: a chord carries a key and an action id and nothing else");
    }
}

/// 🕹️ LAW (end to end): an arrow verb moves the SAME `graph` selection a pointer pick moves — the
/// whole point of routing through `Emit.interaction_writes` instead of an app-private cursor. Read
/// off `protocol::InteractionState` after the retained job has actually published, never off the emit.
#[semio_framework_async_macros::async_test]
async fn an_arrow_verb_moves_the_framework_owned_graph_selection() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    assert!(context::snapshot(&app).host_document.widgets.iter().any(|widget| crate::widget_id(widget) == "height"), "the fixture app opens the hex column");
    context::select_graph(&mut app, "node", &["height"]).await;
    dispatch(&mut app, Generation3dCommand::SelectDownstreamNode(select_downstream_node::SelectDownstreamNode {})).await;
    assert_eq!(
        app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.clone()),
        Some(vec!["extrusion-axis".to_string()]),
        "arrow-right followed the wire out of `height`"
    );
    dispatch(&mut app, Generation3dCommand::SelectUpstreamNode(select_upstream_node::SelectUpstreamNode {})).await;
    assert_eq!(app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.clone()), Some(vec!["height".to_string()]), "arrow-left walked back");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// ⏎️ LAW (end to end): activate opens the selected node's ports, and an arrow afterwards closes the
/// node again by stepping from the port's owner — the two halves of `anchor_node`'s port rule, proven
/// against the live interaction store rather than against the reducer alone.
#[semio_framework_async_macros::async_test]
async fn activate_opens_the_selected_nodes_ports_and_an_arrow_closes_them() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    context::select_graph(&mut app, "node", &["extrude"]).await;
    dispatch(&mut app, Generation3dCommand::ActivateSelection(activate_selection::ActivateSelection {})).await;
    let opened = app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.clone()).unwrap_or_default();
    assert!(!opened.is_empty() && opened.iter().all(|id| id.starts_with("extrude@")), "activate selects the node's own ports: {opened:?}");
    eprintln!("[DEBUG] activateSelection opened {} ports of `extrude`: {opened:?}", opened.len());
    dispatch(&mut app, Generation3dCommand::SelectDownstreamNode(select_downstream_node::SelectDownstreamNode {})).await;
    assert_eq!(
        app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.clone()),
        Some(vec!["column-preview".to_string()]),
        "an arrow from an opened node steps from the node that owns the ports"
    );
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}
