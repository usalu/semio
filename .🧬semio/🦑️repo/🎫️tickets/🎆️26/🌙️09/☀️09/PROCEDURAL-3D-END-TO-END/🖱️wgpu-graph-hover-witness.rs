use serde::{Deserialize, Serialize};

__CANONICAL_HOVER__

struct GraphScene { hover: Option<NodeGraphHover> }
struct Scene { node_graph: Option<GraphScene> }
enum UiNode { ComponentScene(Scene), Text }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpNode {
    path: String,
    __DUMP_FIELD__
}
fn dump(ui_node: &UiNode) -> DumpNode {
    __PROJECT_HOVER__
    DumpNode { path: "graph".into(), __SET_FIELD__ }
}

#[test]
fn structural_dump_has_exact_hover_and_omits_cleared_or_non_graph_hover() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(__FIXTURE__)).unwrap();
    for row in fixture["surfaces"].as_array().unwrap() {
        let graph = UiNode::ComponentScene(Scene { node_graph: Some(GraphScene { hover: serde_json::from_value(row["hover"].clone()).unwrap() }) });
        let actual = serde_json::to_value(dump(&graph)).unwrap();
        assert_eq!(actual["graphHover"], row["hover"], "{}", row["name"]);
        assert_eq!(actual.get("graphHover").is_some(), !row["hover"].is_null());
    }
    let actual = serde_json::to_value(dump(&UiNode::Text)).unwrap();
    assert!(actual.get("graphHover").is_none());
}
