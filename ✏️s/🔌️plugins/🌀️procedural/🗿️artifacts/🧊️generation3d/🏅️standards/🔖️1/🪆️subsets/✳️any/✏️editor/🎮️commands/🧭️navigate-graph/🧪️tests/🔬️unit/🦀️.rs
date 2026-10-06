use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use crate::standards::v1::subsets::any::schema::{PROCEDURAL_EXAMPLE_HEX_COLUMN};
use crate::standards::v1::subsets::any::io::text::snapshot::{example_snapshot};

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

#[test]
fn every_anchor_rule_the_fixture_states_holds() {
    let fixture = fixture();
    let document = hex_column();
    for row in fixture["anchors"].as_array().expect("anchors") {
        let selection = strings(&row["selection"]);
        let expected = row["anchor"].as_str();
        assert_eq!(anchor_node(&document.host_snapshot, &selection), expected, "{:?}: {}", selection, row["why"].as_str().unwrap_or_default());
    }
}

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
            if let Some(target) = step_target(&document.host_snapshot, &selection, step_of(named)) {
                selection = vec![target.to_string()];
            }
            assert_eq!(selection, strings(&step["expected"]), "{name}: after {named}");
            walked += 1;
        }
    }
    assert!(walked >= 20, "the fixture must carry a real walk, not a token one: {walked} steps");
}

#[test]
fn a_keyboard_step_authors_no_document_and_no_config_operation() {
    let document = hex_column();
    let ports = std::collections::BTreeMap::from([("extrude".to_string(), vec!["extrude@wire".to_string(), "extrude@vector".to_string()])]);
    for step in [FlowGraphStep::Next, FlowGraphStep::Previous, FlowGraphStep::Upstream, FlowGraphStep::Downstream] {
        let target = step_target(&document.host_snapshot, &["extrude".to_string()], step);
        assert!(target.is_some(), "the hex column answers every direction from `extrude`");
    }
    assert_eq!(activate_ports(&document.host_snapshot, &ports, &["extrude".to_string()]).map(Vec::as_slice), Some(["extrude@wire".to_string(), "extrude@vector".to_string()].as_slice()), "activate opens the anchor's ports");
    assert_eq!(activate_ports(&document.host_snapshot, &ports, &["extrude@wire".to_string(), "extrude@vector".to_string()]), None, "an already-open node opens no further");
    assert_eq!(activate_ports(&document.host_snapshot, &ports, &["height".to_string()]), None, "a node with no published port opens nothing");
}
