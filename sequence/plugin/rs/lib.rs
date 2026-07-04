//! 🔗 Sequence plugin — declarative sequence play app bundled as a hot-swappable WASM component.

use mathematical_graph_port_directed_dag::DagFixture;
use sequence_core::{default_fixture, SequenceFixture, SequenceHost, SequenceStep};
use semio_framework_plugin::{
    build_node_graph_scene, build_text_editor_scene, create_default_layout, ui_declarative_sections_to_tree,
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, App, CommandDescriptor, NodeGraphScene,
    PluginApp, PluginBundle, TextEditorScene, UiControlNode, UiFieldNode, UiInputNode, UiInspectorFieldGroup,
    UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;

//#region 🔖Constants
const SEQUENCE_PLAY_APP_ID: &str = "sequence-play";
const SEQUENCE_PLAY_SURFACE_MAIN: &str = "sequence.play.main";
const SEQUENCE_PLAY_SURFACE_SCRIPT: &str = "sequence.play.script";
const SEQUENCE_PLAY_SURFACE_COMPILED: &str = "sequence.play.compiled-dag";
const SEQUENCE_PLAY_BODY_MAIN: &str = "sequence.play.main";
const SEQUENCE_PLAY_BODY_SCRIPT: &str = "sequence.play.script";
const SEQUENCE_PLAY_BODY_COMPILED: &str = "sequence.play.compiled-dag";
const SEQUENCE_PLAY_BODY_HIERARCHY: &str = "sequence.play.hierarchy";
const SEQUENCE_PLAY_BODY_CATALOGUE: &str = "sequence.play.catalogue";
const SEQUENCE_PLAY_BODY_INSPECTOR: &str = "sequence.play.inspection";
const SEQUENCE_PLAY_WINDOW_MAIN: &str = "sequence-main";
const SEQUENCE_PLAY_WINDOW_SCRIPT: &str = "sequence-script";
const SEQUENCE_PLAY_WINDOW_COMPILED: &str = "sequence-compiled-dag";
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SequencePlayRuntime {
    #[serde(default)]
    selected_step_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SequencePlayEnvelope {
    fixture: SequenceFixture,
    #[serde(default)]
    runtime: SequencePlayRuntime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphPortRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphNodeRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    inputs: Vec<MediaGraphPortRecord>,
    outputs: Vec<MediaGraphPortRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphEdgeRecord {
    id: String,
    source_node_id: String,
    source_port_id: String,
    target_node_id: String,
    target_port_id: String,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
fn default_envelope() -> SequencePlayEnvelope {
    SequencePlayEnvelope {
        fixture: default_fixture(),
        runtime: SequencePlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> SequencePlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &SequencePlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn sequence_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: SEQUENCE_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn host_from_envelope(envelope: &SequencePlayEnvelope) -> SequenceHost {
    SequenceHost::from_fixture(envelope.fixture.clone())
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint
        .split_once(':')
        .map(|(node, port)| (node.to_string(), port.to_string()))
        .unwrap_or_else(|| (endpoint.to_string(), "next".into()))
}

fn fixture_to_media_graph(fixture: &DagFixture) -> (String, String) {
    let nodes: Vec<MediaGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| MediaGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node
                .inputs()
                .iter()
                .filter(|port| port.visible)
                .map(|port| MediaGraphPortRecord {
                    id: format!("{}:{}", node.id, port.id),
                    label: Some(port.label.clone()),
                })
                .collect(),
            outputs: node
                .outputs()
                .iter()
                .filter(|port| port.visible)
                .map(|port| MediaGraphPortRecord {
                    id: format!("{}:{}", node.id, port.id),
                    label: Some(port.label.clone()),
                })
                .collect(),
        })
        .collect();
    let edges: Vec<MediaGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            MediaGraphEdgeRecord {
                id: edge.id.clone(),
                source_node_id,
                source_port_id,
                target_node_id,
                target_port_id,
            }
        })
        .collect();
    (
        serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
    )
}
//#endregion 🔖DocumentHelpers

//#region 🔖TreeHelpers
fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        command: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn tree_item_with_command(id: impl Into<String>, label: impl Into<String>, description: Option<String>, command: CommandDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
        selected: None,
        default_open: None,
        command: Some(command),
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}
//#endregion 🔖TreeHelpers

//#region 🔖Panels
fn build_hierarchy_tree(fixture: &SequenceFixture, selected: &[String]) -> UiNode {
    let step_items: Vec<UiTreeItemNode> = fixture
        .steps
        .iter()
        .map(|step| {
            tree_item_with_command(
                format!("sequence-play-hierarchy.step.{}", step.id),
                format!("{} ({})", step.id, step.kind),
                Some(step.kind.clone()),
                sequence_cmd("setSelection", Some(json!({ "ids": [step.id.clone()] }))),
            )
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture
        .edges
        .iter()
        .map(|edge| {
            UiTreeItemNode {
                id: format!("sequence-play-hierarchy.edge.{}", edge.id),
                label: format!("{} → {}", edge.from, edge.to),
                description: Some(edge.id.clone()),
                icon_id: None,
                selected: None,
                default_open: None,
                command: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: None,
            }
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "sequence-play-hierarchy.steps".into(),
                label: Some("Steps".into()),
                default_open: Some(true),
                items: if step_items.is_empty() {
                    vec![tree_item("sequence-play-hierarchy.steps.empty", "(none)")]
                } else {
                    step_items
                },
            },
            UiTreeSectionNode {
                id: "sequence-play-hierarchy.edges".into(),
                label: Some("Flow edges".into()),
                default_open: Some(false),
                items: if edge_items.is_empty() {
                    vec![tree_item("sequence-play-hierarchy.edges.empty", "(none)")]
                } else {
                    edge_items
                },
            },
        ],
        selected_ids: Some(selected.iter().map(|id| format!("sequence-play-hierarchy.step.{id}")).collect()),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    let actions = [
        ("state.set", "Set state"),
        ("log.print", "Print log"),
        ("control.if", "If"),
        ("control.while", "While"),
        ("math.add", "Add"),
    ];
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "sequence-play-catalogue.actions".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items: actions
                .iter()
                .map(|(kind, label)| {
                    tree_item_with_command(
                        format!("sequence-play-catalogue.action.{kind}"),
                        *label,
                        Some((*kind).into()),
                        sequence_cmd("addStep", Some(json!({ "kind": kind }))),
                    )
                })
                .collect(),
        }],
        selected_ids: Some(vec![]),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(fixture: &SequenceFixture, selected: &[String]) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "sequence-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Select a step in the canvas or hierarchy.")],
        }]);
    }
    let steps: Vec<&SequenceStep> = selected
        .iter()
        .filter_map(|id| fixture.steps.iter().find(|step| &step.id == id))
        .collect();
    if steps.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "sequence-play-inspector.missing".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Step not found")],
        }]);
    }
    let step_ids: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
    let mut fields = vec![
        ui_inspector_readonly_field("sequence-play-inspector.kind", "Kind", steps[0].kind.clone()),
        ui_inspector_readonly_field(
            "sequence-play-inspector.params",
            "Params",
            serde_json::to_string(&steps[0].params).unwrap_or_else(|_| "{}".into()),
        ),
    ];
    if step_ids.len() == 1 {
        fields.insert(
            0,
            UiNode::Field(UiFieldNode {
                id: "sequence-play-inspector.id".into(),
                label: "Id".into(),
                child: UiControlNode::Input(UiInputNode {
                    id: "sequence-play-inspector.id.input".into(),
                    input_kind: "text".into(),
                    value: step_ids[0].clone(),
                    placeholder: None,
                    commit: None,
                    on_change: sequence_cmd("noop", None),
                }),
            }),
        );
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "sequence-play-inspector.step".into(),
        label: "Step".into(),
        default_open: None,
        fields,
    }])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_graph(envelope: &SequencePlayEnvelope) -> UiNode {
    let host = host_from_envelope(envelope);
    let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
    let viewport_json = serde_json::to_string(&envelope.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    build_node_graph_scene(
        SEQUENCE_PLAY_SURFACE_MAIN,
        SEQUENCE_PLAY_APP_ID,
        NodeGraphScene {
            nodes_json,
            edges_json,
            viewport_json,
        },
    )
}

fn render_script(envelope: &SequencePlayEnvelope) -> UiNode {
    let host = host_from_envelope(envelope);
    build_text_editor_scene(
        SEQUENCE_PLAY_SURFACE_SCRIPT,
        SEQUENCE_PLAY_APP_ID,
        TextEditorScene {
            buffer: host.compile_text(),
            language: Some("imperative".into()),
            selection_json: None,
        },
    )
}

fn render_compiled_dag(envelope: &SequencePlayEnvelope) -> UiNode {
    let host = host_from_envelope(envelope);
    build_text_editor_scene(
        SEQUENCE_PLAY_SURFACE_COMPILED,
        SEQUENCE_PLAY_APP_ID,
        TextEditorScene {
            buffer: host.compiled_wire_literal(),
            language: Some("wire".into()),
            selection_json: None,
        },
    )
}
//#endregion 🔖Render

//#region 🔖SequencePlayApp
struct SequencePlayApp;

impl PluginApp for SequencePlayApp {
    fn app_id(&self) -> &str {
        SEQUENCE_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("sequence envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        let mut host = host_from_envelope(&envelope);
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" | "selectNode" => {
                envelope.runtime.selected_step_ids = selection_ids(args);
                return vec![set_document_op(&envelope)];
            }
            "graphPointerDown" => {
                envelope.runtime.selected_step_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str());
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    if let Some(step) = envelope.fixture.steps.iter_mut().find(|step| step.id == node_id) {
                        step.x = x;
                        step.y = y;
                        host.replace_fixture(envelope.fixture.clone()).ok();
                        envelope.fixture = host.fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "connectMediaPorts" => {
                let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str());
                let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str());
                if let (Some(from), Some(to)) = (from, to) {
                    if host.connect_steps(from, to).is_ok() {
                        envelope.fixture = host.fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "addStep" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("log.print");
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let id = host.add_step(kind, x, y);
                envelope.fixture = host.fixture;
                envelope.runtime.selected_step_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "run" => {
                let _result = host.run();
                return vec![set_document_op(&envelope)];
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            SEQUENCE_PLAY_BODY_MAIN => render_main_graph(&envelope),
            SEQUENCE_PLAY_BODY_SCRIPT => render_script(&envelope),
            SEQUENCE_PLAY_BODY_COMPILED => render_compiled_dag(&envelope),
            SEQUENCE_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&envelope.fixture, &envelope.runtime.selected_step_ids),
            SEQUENCE_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            SEQUENCE_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope.fixture, &envelope.runtime.selected_step_ids),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .or_else(|| {
            args.and_then(|value| value.get("nodeId"))
                .and_then(|value| value.as_str())
                .map(|id| vec![id.to_string()])
        })
        .unwrap_or_default()
}
//#endregion 🔖SequencePlayApp

//#region 🔖Manifest
fn create_sequence_app() -> App {
    App::from_builder(
        App::builder(SEQUENCE_PLAY_APP_ID, "Sequence")
            .icon_id("sequence")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(SEQUENCE_PLAY_WINDOW_MAIN, "Sequence", SEQUENCE_PLAY_BODY_MAIN)
            .window_kind(SEQUENCE_PLAY_WINDOW_SCRIPT, "Script", SEQUENCE_PLAY_BODY_SCRIPT)
            .window_kind(SEQUENCE_PLAY_WINDOW_COMPILED, "DSL", SEQUENCE_PLAY_BODY_COMPILED)
            .default_layout(create_default_layout(
                &[
                    SEQUENCE_PLAY_WINDOW_MAIN.into(),
                    SEQUENCE_PLAY_WINDOW_SCRIPT.into(),
                    SEQUENCE_PLAY_WINDOW_COMPILED.into(),
                ],
                "row",
                Some(&[50.0, 25.0, 25.0]),
                Some(&["Sequence".into(), "Script".into(), "DSL".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                SEQUENCE_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                SEQUENCE_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                SEQUENCE_PLAY_BODY_INSPECTOR,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("demo", "Demo", serde_json::to_string(&default_envelope()).unwrap())
    .program("sequence", "Sequence", "graph")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("sequence", "Sequence", "0.1.0").register_app(create_sequence_app(), || Box::new(SequencePlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖Manifest

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_node_graph_scene() {
        let app = SequencePlayApp;
        let document = app.initial_document_json();
        let node = app.render(SEQUENCE_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_script_editor() {
        let app = SequencePlayApp;
        let document = app.initial_document_json();
        let node = app.render(SEQUENCE_PLAY_BODY_SCRIPT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn default_fixture_has_steps() {
        let envelope = default_envelope();
        assert_eq!(envelope.fixture.steps.len(), 2);
    }

    #[test]
    fn add_step_command_appends_step() {
        let mut app = SequencePlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("addStep", Some(&json!({ "kind": "log.print" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated_op: Value = serde_json::from_str(&ops[0]).unwrap();
        let updated: SequencePlayEnvelope = serde_json::from_value(updated_op["document"].clone()).unwrap();
        assert!(updated.fixture.steps.len() > 2);
    }
}
