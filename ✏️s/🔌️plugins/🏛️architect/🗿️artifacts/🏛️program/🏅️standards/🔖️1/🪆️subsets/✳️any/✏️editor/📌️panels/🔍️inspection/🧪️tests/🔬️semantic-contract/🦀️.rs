use super::*;
use semio_framework_plugin::BuiltNode;

fn project(node: BuiltNode) -> serde_json::Value {
    let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
    serde_json::from_str(&text).expect("independent tree JSON oracle")
}

/// 🔎️ The first `setAdjacencyKind` cell of the matrix, wherever the window put it — the matrix nests
/// its pairs under windowed row groups now, so a fixed child index is no longer a stable address.
fn first_binding(node: &semio_framework_plugin::BuiltNode) -> Option<&semio_framework_plugin::ActionBinding> {
    if !node.bindings.is_empty() {
        return Some(&node.bindings[0]);
    }
    (0..node.children.len()).find_map(|index| first_binding(&node.children[index]))
}

#[test]
fn architect_semantic_panels_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️panels.json")).expect("neutral semantic vectors");
    let program = crate::sample_plugin();
    let tree = project(render(&program).expect("inspector"));
    let fields = tree["children"][0]["children"].as_array().expect("summary fields");
    assert_eq!(serde_json::Value::Array(fields.iter().map(|field| field["component"]["label"].clone()).collect()), vectors["summaryFields"]);
    let adjacency_cfg = crate::editor::architect::modes::edit::windows::adjacency::config::ArchitectAdjacencyWindowConfig::default();
    let node = crate::editor::architect::modes::edit::windows::adjacency::render(&program, &adjacency_cfg, &semio_framework_plugin::TreeWindows::unhosted()).expect("adjacency");
    let binding: serde_json::Value = serde_json::to_value(first_binding(&node).expect("the matrix materialises at least one cycling pair")).expect("independent adjacency binding oracle");
    project(node);
    assert_eq!(binding["args"]["cycle"], true);
    assert_eq!(serde_json::Value::Array(binding["args"].as_object().expect("arguments").keys().cloned().map(serde_json::Value::String).collect()), vectors["adjacencyArgs"]);
    let tree = project(crate::editor::architect::panels::catalogue::render(&semio_framework_plugin::TreeWindows::unhosted()).expect("catalogue"));
    let registers = tree["children"].as_array().expect("catalogue sections").iter().find(|section| section["key"] == "architect-catalogue.registers").expect("the one register section");
    assert_eq!(registers["component"]["window"]["total"].as_u64().expect("the register section stamps its extent"), vectors["registerCount"].as_u64().expect("register count"));
    let text = tree.to_string();
    for action in vectors["catalogueActions"].as_array().expect("actions") {
        assert!(text.contains(action.as_str().expect("id")));
    }
    let tree = project(crate::viewer::architect::modes::view::windows::register::render(&program, &semio_framework_plugin::TreeWindows::unhosted()).expect("viewer register"));
    for total in vectors["viewerTotals"].as_array().expect("totals") {
        assert!(tree.to_string().contains(total.as_str().expect("total")));
    }
    let graph_cfg = crate::editor::architect::modes::edit::windows::graph::config::ArchitectGraphWindowConfig::default();
    let register_cfg = crate::editor::architect::modes::edit::windows::register::config::ArchitectRegisterWindowConfig::default();
    let report_cfg = crate::editor::architect::modes::edit::windows::report::config::ArchitectReportWindowConfig::default();
    for node in [
        crate::editor::architect::modes::edit::windows::graph::render(&program, &graph_cfg).expect("graph"),
        crate::editor::architect::modes::edit::windows::register::render(&program, &register_cfg).expect("register"),
        crate::editor::architect::modes::edit::windows::report::render(&program, &report_cfg, &semio_framework_plugin::ViewModel::default(), &semio_framework_plugin::TreeWindows::unhosted()).expect("report placeholder"),
        crate::editor::architect::modes::edit::windows::trace::render(&program, &semio_framework_plugin::TreeWindows::unhosted()).expect("trace"),
        crate::editor::architect::panels::document::render(&program, &semio_framework_plugin::TreeWindows::unhosted()).expect("document"),
    ] {
        project(node);
    }
}
