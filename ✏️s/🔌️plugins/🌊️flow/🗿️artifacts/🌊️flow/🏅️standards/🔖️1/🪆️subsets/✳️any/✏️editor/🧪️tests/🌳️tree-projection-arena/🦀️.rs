//! 🌳️ The fixture tree projection law, SATURATING the process-global built-node page pool
//! `semio_framework_ui_contract` hands every UI build and asserting it drains to terminal-empty.
//! Its own test binary: the pool is process-wide, so in the lib binary any sibling law building a
//! tree read foreign pages into this probe and failed its own builds with `ui.fixed-capacity`
//! (tickets 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, 26/09/23 session 10 slice T7).
use serde_json::Value;

fn projection_fixture_node(value: &Value) -> semio_framework_plugin::BuiltNode {
    let mut node = semio_framework_plugin::BuiltNode::try_new(value["key"].as_str().unwrap(), serde_json::from_value(value["component"].clone()).unwrap()).unwrap();
    for child in value["children"].as_array().unwrap() {
        node.children.try_push(projection_fixture_node(child)).unwrap();
    }
    if value["rejected"].as_bool() == Some(true) {
        node.rejected_children.try_push(semio_framework_plugin::BuiltNode::try_new("rejected", serde_json::from_value(serde_json::json!({ "type": "text", "value": "rejected" })).unwrap()).unwrap()).unwrap();
    }
    node
}

fn saturated_rejected_fixture_tree(pages: usize, capacity: usize) -> semio_framework_plugin::ComponentTree {
    use semio_framework_ui_contract::{BuiltNode, Component, Label, TextProps};
    let text = |key: &str| BuiltNode::try_new(key, Component::Text(TextProps { value: Label::try_from("fixture").unwrap(), emphasize: None, data_attributes: None })).unwrap();
    let mut root = text("root");
    let mut full_pages = pages.checked_sub(capacity + 2).unwrap();
    for group in 0..capacity {
        let mut branch = text(&format!("group-{group}"));
        for row in 0..capacity {
            let mut node = text(&format!("row-{row}"));
            if full_pages > 0 {
                for leaf in 0..capacity {
                    node.children.try_push(text(&format!("leaf-{leaf}"))).unwrap();
                }
                full_pages -= 1;
            }
            branch.children.try_push(node).unwrap();
        }
        root.children.try_push(branch).unwrap();
    }
    assert_eq!(full_pages, 0);
    root.rejected_children.try_push(text("rejected")).unwrap();
    semio_framework_plugin::ComponentTree { root }
}

#[test]
fn flow_render_fixture_projection_retires_populated_and_rejected_pages() {
    use semio_framework_plugin::artifact_app_laws::{project_and_retire_fixture_tree, FIXTURE_TREE_MAX_DEPTH, FIXTURE_TREE_MAX_NODES, FIXTURE_TREE_RETIRE_STEPS};
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖼️tree-projection/🔣️.json")).unwrap();
    assert_eq!(fixture["contractId"], "semio.fixture.tree-projection/v1");
    assert_eq!(fixture["maximumDepth"], FIXTURE_TREE_MAX_DEPTH);
    assert_eq!(fixture["maximumNodes"], FIXTURE_TREE_MAX_NODES);
    assert_eq!(fixture["retirementSteps"], FIXTURE_TREE_RETIRE_STEPS);
    for row in fixture["cases"].as_array().unwrap() {
        let tree = semio_framework_plugin::ComponentTree { root: projection_fixture_node(&row["input"]) };
        match project_and_retire_fixture_tree(tree) {
            Ok(json) => assert_eq!(serde_json::from_str::<Value>(&json).unwrap(), row["expected"]),
            Err(error) => assert_eq!(Some(error), row["error"].as_str()),
        }
        assert!(semio_framework_ui_contract::close_built_node_page_one());
    }
    let probe = &fixture["retirementProbe"];
    let pages = probe["reservedPages"].as_u64().unwrap() as usize;
    let capacity = probe["childCapacity"].as_u64().unwrap() as usize;
    assert_eq!(pages, semio_framework_ui_contract::UI_BUILT_CHILD_RETIRE_SLOTS);
    assert_eq!(capacity, semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX);
    drop(saturated_rejected_fixture_tree(pages, capacity));
    let steps = (1..=FIXTURE_TREE_RETIRE_STEPS).find(|_| semio_framework_ui_contract::close_built_node_page_one()).expect("all reserved pages retire within the exact authority bound");
    assert_eq!(steps, probe["closeSteps"].as_u64().unwrap() as usize);
    assert_eq!(steps - pages, probe["ownedNodes"].as_u64().unwrap() as usize);
    assert!(steps > probe["supersededLimit"].as_u64().unwrap() as usize);
    let tree = saturated_rejected_fixture_tree(pages, capacity);
    assert_eq!(project_and_retire_fixture_tree(tree).unwrap_err(), probe["error"].as_str().unwrap());
    assert_eq!(semio_framework_ui_contract::close_built_node_page_one(), probe["terminalEmpty"].as_bool().unwrap());
    eprintln!("[DEBUG] retained Flow fixture tree observation: two positive trees, two structural denials and {pages} saturated pages retired in {steps} counted turns");
}
