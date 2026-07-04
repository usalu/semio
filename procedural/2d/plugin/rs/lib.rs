//! 🎲 Procedural 2D plugin — procedural revision play app bundled as a hot-swappable WASM component.

use procedural_2d::{
    empty_procedural2d_projection, Procedural2dDocument, Procedural2dEnvelope, Procedural2dOp, Procedural2dStore,
    PROCEDURAL_2D_SCHEMA,
};
use semio_framework_plugin::{
    build_canvas_2d_scene, create_default_layout, ui_inspector_groups_to_tree, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, App, Canvas2dScene, CommandDescriptor, PluginApp, PluginBundle, UiInspectorFieldGroup,
    UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;
use vcs::{create_document_vcs_envelope, materialize_document_projection, DocumentVcsCommand};

//#region 🔖Constants
const PROCEDURAL2D_PLAY_APP_ID: &str = "procedural2d-play";
const PROCEDURAL2D_PLAY_SURFACE_MAIN: &str = "procedural2d.play.main";
const PROCEDURAL2D_PLAY_SURFACE_PREVIEW: &str = "procedural2d.play.preview";
const PROCEDURAL2D_PLAY_BODY_MAIN: &str = "procedural2d.play.main";
const PROCEDURAL2D_PLAY_BODY_PREVIEW: &str = "procedural2d.play.preview";
const PROCEDURAL2D_PLAY_BODY_HIERARCHY: &str = "procedural2d.play.hierarchy";
const PROCEDURAL2D_PLAY_BODY_CATALOGUE: &str = "procedural2d.play.catalogue";
const PROCEDURAL2D_PLAY_BODY_INSPECTION: &str = "procedural2d.play.inspection";
const PROCEDURAL2D_PLAY_WINDOW_MAIN: &str = "procedural2d-main";
const PROCEDURAL2D_PLAY_WINDOW_PREVIEW: &str = "procedural2d-preview";
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Procedural2dPlayRuntime {
    #[serde(default)]
    selected_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Procedural2dPlayEnvelope {
    envelope: Procedural2dEnvelope,
    #[serde(default)]
    applied_edit_ids: Vec<String>,
    #[serde(default)]
    redo_edit_ids: Vec<String>,
    #[serde(default)]
    runtime: Procedural2dPlayRuntime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Procedural2dCanvasLayer {
    id: String,
    kind: String,
    name: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
fn default_envelope() -> Procedural2dPlayEnvelope {
    Procedural2dPlayEnvelope {
        envelope: create_document_vcs_envelope(
            PROCEDURAL_2D_SCHEMA,
            "procedural2d",
            empty_procedural2d_projection(),
            None,
        ),
        applied_edit_ids: Vec::new(),
        redo_edit_ids: Vec::new(),
        runtime: Procedural2dPlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> Procedural2dPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &Procedural2dPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn procedural2d_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: PROCEDURAL2D_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn store_from_envelope(play: &Procedural2dPlayEnvelope) -> Procedural2dStore {
    let mut store = Procedural2dStore::new(play.envelope.clone());
    store.set_envelope(play.envelope.clone(), play.applied_edit_ids.clone());
    store
}

fn sync_store_to_envelope(
    store: &Procedural2dStore,
    runtime: &Procedural2dPlayRuntime,
    redo_edit_ids: &[String],
) -> Procedural2dPlayEnvelope {
    Procedural2dPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        redo_edit_ids: redo_edit_ids.to_vec(),
        runtime: runtime.clone(),
    }
}

fn materialized_projection(play: &Procedural2dPlayEnvelope) -> Procedural2dDocument {
    materialize_document_projection(&play.envelope, &play.applied_edit_ids)
        .unwrap_or_else(|_| play.envelope.vcs.initial_projection.clone())
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default()
}

fn canvas_layers(document: &Procedural2dDocument, preview: bool) -> String {
    let revision = document.revision;
    let offset = if preview { 240.0 } else { 0.0 };
    let layers = vec![
        Procedural2dCanvasLayer {
            id: if preview {
                "procedural2d-preview.revision".into()
            } else {
                "procedural2d-main.revision".into()
            },
            kind: "rect".into(),
            name: format!("Revision {revision}"),
            x: offset,
            y: 0.0,
            width: 180.0 + (revision as f64 * 12.0).min(120.0),
            height: 72.0,
        },
        Procedural2dCanvasLayer {
            id: if preview {
                "procedural2d-preview.tile".into()
            } else {
                "procedural2d-main.tile".into()
            },
            kind: "rect".into(),
            name: if preview { "Preview Tile".into() } else { "Main Tile".into() },
            x: offset + 24.0,
            y: 96.0,
            width: 96.0,
            height: 96.0,
        },
    ];
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖DocumentHelpers

//#region 🔖Panels
fn tree_item(id: impl Into<String>, label: impl Into<String>, command: Option<CommandDescriptor>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        command,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn build_hierarchy_tree(play: &Procedural2dPlayEnvelope) -> UiNode {
    let projection = materialized_projection(play);
    let revision_row = tree_item(
        "procedural2d-play-hierarchy.revision",
        format!("Revision {}", projection.revision),
        Some(procedural2d_cmd("setSelection", Some(json!({ "ids": ["revision"] })))),
    );
    let checkpoint_items: Vec<UiTreeItemNode> = play
        .envelope
        .vcs
        .checkpoints
        .iter()
        .map(|checkpoint| {
            tree_item(
                format!("procedural2d-play-hierarchy.checkpoint.{}", checkpoint.id),
                checkpoint.message.clone().unwrap_or_else(|| checkpoint.id.clone()),
                None,
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "procedural2d-play-hierarchy.document".into(),
                label: Some(FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL.into()),
                default_open: Some(true),
                items: vec![revision_row],
            },
            UiTreeSectionNode {
                id: "procedural2d-play-hierarchy.checkpoints".into(),
                label: Some("Checkpoints".into()),
                default_open: Some(false),
                items: if checkpoint_items.is_empty() {
                    vec![tree_item("procedural2d-play-hierarchy.checkpoints.empty", "(none)", None)]
                } else {
                    checkpoint_items
                },
            },
        ],
        selected_ids: Some(
            play.runtime
                .selected_ids
                .iter()
                .map(|id| format!("procedural2d-play-hierarchy.{id}"))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: Some(procedural2d_cmd("setSelection", None)),
    })
}

fn build_catalogue_tree() -> UiNode {
    let revision_items = [1_i64, 2, 3, 5, 8]
        .into_iter()
        .map(|revision| {
            tree_item(
                format!("procedural2d-play-catalogue.revision.{revision}"),
                format!("Revision {revision}"),
                Some(procedural2d_cmd("setRevision", Some(json!({ "revision": revision })))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "procedural2d-play-catalogue.revisions".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items: revision_items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(play: &Procedural2dPlayEnvelope) -> UiNode {
    let projection = materialized_projection(play);
    if play.runtime.selected_ids.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", PROCEDURAL_2D_SCHEMA)),
            ui_text(format!("Revision: {}", projection.revision)),
            ui_text(format!("Edits applied: {}", play.applied_edit_ids.len())),
        ]);
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "procedural2d-play-inspector.revision".into(),
        label: "Revision".into(),
        default_open: Some(true),
        fields: vec![
            ui_inspector_readonly_field(
                "procedural2d-play-inspector.revision-value",
                "Value",
                projection.revision.to_string(),
            ),
            ui_inspector_readonly_field(
                "procedural2d-play-inspector.selection",
                "Selection",
                play.runtime.selected_ids.join(", "),
            ),
        ],
    }])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_canvas(play: &Procedural2dPlayEnvelope) -> UiNode {
    let projection = materialized_projection(play);
    build_canvas_2d_scene(
        PROCEDURAL2D_PLAY_SURFACE_MAIN,
        PROCEDURAL2D_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 1.0,
            layers_json: canvas_layers(&projection, false),
        },
    )
}

fn render_preview_canvas(play: &Procedural2dPlayEnvelope) -> UiNode {
    let projection = materialized_projection(play);
    build_canvas_2d_scene(
        PROCEDURAL2D_PLAY_SURFACE_PREVIEW,
        PROCEDURAL2D_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 1.0,
            layers_json: canvas_layers(&projection, true),
        },
    )
}
//#endregion 🔖Render

//#region 🔖Procedural2dPlayApp
struct Procedural2dPlayApp;

impl PluginApp for Procedural2dPlayApp {
    fn app_id(&self) -> &str {
        PROCEDURAL2D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("procedural2d envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        let mut store = store_from_envelope(&play);
        match command {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<Procedural2dPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                play.runtime.selected_ids = selection_ids(args);
                return vec![set_document_op(&play)];
            }
            "setRevision" => {
                let revision = args
                    .and_then(|value| value.get("revision"))
                    .and_then(|value| value.as_i64())
                    .unwrap_or(0);
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![Procedural2dOp::SetRevision { revision }],
                    description: None,
                });
                play.redo_edit_ids.clear();
                return vec![set_document_op(&sync_store_to_envelope(&store, &play.runtime, &play.redo_edit_ids))];
            }
            "undo" => {
                if let Some(last) = play.applied_edit_ids.pop() {
                    play.redo_edit_ids.push(last);
                    return vec![set_document_op(&play)];
                }
            }
            "redo" => {
                if let Some(next) = play.redo_edit_ids.pop() {
                    play.applied_edit_ids.push(next);
                    return vec![set_document_op(&play)];
                }
            }
            "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" | "canvasWheel" => {}
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        match body_key {
            PROCEDURAL2D_PLAY_BODY_MAIN => render_main_canvas(&play),
            PROCEDURAL2D_PLAY_BODY_PREVIEW => render_preview_canvas(&play),
            PROCEDURAL2D_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&play),
            PROCEDURAL2D_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            PROCEDURAL2D_PLAY_BODY_INSPECTION => build_inspector_tree(&play),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖Procedural2dPlayApp

//#region 🔖AppFactory
fn create_procedural2d_app() -> App {
    App::from_builder(
        App::builder(PROCEDURAL2D_PLAY_APP_ID, "Procedural 2D")
            .icon_id("procedural2d")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_MAIN, "Main", PROCEDURAL2D_PLAY_BODY_MAIN)
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_PREVIEW, "Preview", PROCEDURAL2D_PLAY_BODY_PREVIEW)
            .default_layout(create_default_layout(
                &[PROCEDURAL2D_PLAY_WINDOW_MAIN.into(), PROCEDURAL2D_PLAY_WINDOW_PREVIEW.into()],
                "row",
                Some(&[55.0, 45.0]),
                Some(&["Main".into(), "Preview".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                PROCEDURAL2D_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                PROCEDURAL2D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                PROCEDURAL2D_PLAY_BODY_INSPECTION,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .program("procedural2d", "Procedural 2D", "layout")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("procedural2d", "Procedural 2D", "0.1.0")
        .register_app(create_procedural2d_app(), || Box::new(Procedural2dPlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_main_canvas_scene() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn renders_preview_canvas_scene() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_PREVIEW, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn hierarchy_lists_revision() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("procedural2d-play-hierarchy.revision"));
    }

    #[test]
    fn catalogue_lists_revision_presets() {
        let app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL2D_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("procedural2d-play-catalogue.revision.1"));
    }

    #[test]
    fn set_revision_command_updates_projection() {
        let mut app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("setRevision", Some(&json!({ "revision": 5 })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Procedural2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(materialized_projection(&next).revision, 5);
    }

    #[test]
    fn set_selection_updates_runtime() {
        let mut app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "setSelection",
            Some(&json!({ "ids": ["revision"] })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Procedural2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.selected_ids, vec!["revision".to_string()]);
    }

    #[test]
    fn undo_redo_round_trip_revision() {
        let mut app = Procedural2dPlayApp;
        let document = app.initial_document_json();
        let applied = app
            .handle_command("setRevision", Some(&json!({ "revision": 3 })), &document, &ViewState::default());
        let document = serde_json::from_str::<Value>(&applied[0]).unwrap()["document"].to_string();
        let undone = app.handle_command("undo", None, &document, &ViewState::default());
        let undone_doc: Procedural2dPlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&undone[0]).unwrap()["document"].clone()).unwrap();
        assert_eq!(materialized_projection(&undone_doc).revision, 0);
        let undone_json = serde_json::from_str::<Value>(&undone[0]).unwrap()["document"].to_string();
        let redone = app.handle_command("redo", None, &undone_json, &ViewState::default());
        let redone_doc: Procedural2dPlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&redone[0]).unwrap()["document"].clone()).unwrap();
        assert_eq!(materialized_projection(&redone_doc).revision, 3);
    }
}
//#endregion 🧪Tests
