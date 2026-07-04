//! 🗺️ GIS 2D plugin — GIS map play app bundled as a hot-swappable WASM component.

use gis_2d::{
    empty_gis_map_projection, GisMapDocument, GisMapEnvelope, GisMapOp, GisMapStore, GIS_MAP_SCHEMA,
};
use semio_framework_plugin::{
    build_canvas_2d_scene, create_default_layout, ui_inspector_groups_to_tree, ui_inspector_mixed_toggle,
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, Canvas2dScene, CommandDescriptor, PluginApp,
    PluginBundle, UiControlNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiToggleNode, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::LazyLock;
use vcs::{create_document_vcs_envelope, materialize_document_projection, DocumentVcsCommand};

//#region 🔖Constants
const GIS2D_PLAY_APP_ID: &str = "gis2d-play";
const GIS2D_PLAY_SURFACE: &str = "gis2d.play.composite";
const GIS2D_PLAY_BODY_COMPOSITE: &str = "gis2d.play.composite";
const GIS2D_PLAY_BODY_HIERARCHY: &str = "gis2d.play.hierarchy";
const GIS2D_PLAY_BODY_CATALOGUE: &str = "gis2d.play.catalogue";
const GIS2D_PLAY_BODY_INSPECTION: &str = "gis2d.play.inspection";
const GIS2D_PLAY_WINDOW_MAIN: &str = "gis2d-main";

const GIS_MAP_LAYER_IDS: &[(&str, &str, &str)] = &[
    ("raster", "Raster", "map"),
    ("water", "Water", "droplets"),
    ("land", "Land", "mountain"),
    ("roads", "Roads", "route"),
    ("buildings", "Buildings", "building"),
    ("borders", "Borders", "square-dashed"),
    ("labels", "Labels", "type"),
    ("positions", "Positions", "map-pin"),
    ("positionLabels", "Position Labels", "tag"),
    ("routes", "Routes", "git-branch"),
    ("regions", "Regions", "layers"),
];
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Gis2dPlayRuntime {
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    layer_visibility: HashMap<String, bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Gis2dPlayEnvelope {
    envelope: GisMapEnvelope,
    #[serde(default)]
    applied_edit_ids: Vec<String>,
    #[serde(default)]
    redo_edit_ids: Vec<String>,
    #[serde(default)]
    runtime: Gis2dPlayRuntime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GisMapCanvasLayer {
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
fn default_layer_visibility() -> HashMap<String, bool> {
    GIS_MAP_LAYER_IDS.iter().map(|(id, _, _)| ((*id).into(), true)).collect()
}

fn default_envelope() -> Gis2dPlayEnvelope {
    let mut runtime = Gis2dPlayRuntime::default();
    runtime.layer_visibility = default_layer_visibility();
    Gis2dPlayEnvelope {
        envelope: create_document_vcs_envelope(GIS_MAP_SCHEMA, "gis", empty_gis_map_projection(), None),
        applied_edit_ids: Vec::new(),
        redo_edit_ids: Vec::new(),
        runtime,
    }
}

fn parse_envelope(document_json: &str) -> Gis2dPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &Gis2dPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn gis2d_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: GIS2D_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn store_from_envelope(play: &Gis2dPlayEnvelope) -> GisMapStore {
    let mut store = GisMapStore::new(play.envelope.clone());
    store.set_envelope(play.envelope.clone(), play.applied_edit_ids.clone());
    store
}

fn sync_store_to_envelope(store: &GisMapStore, runtime: &Gis2dPlayRuntime, redo_edit_ids: &[String]) -> Gis2dPlayEnvelope {
    Gis2dPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        redo_edit_ids: redo_edit_ids.to_vec(),
        runtime: runtime.clone(),
    }
}

fn materialized_projection(play: &Gis2dPlayEnvelope) -> GisMapDocument {
    materialize_document_projection(&play.envelope, &play.applied_edit_ids)
        .unwrap_or_else(|_| play.envelope.vcs.initial_projection.clone())
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default()
}

fn layer_visible(runtime: &Gis2dPlayRuntime, layer_id: &str) -> bool {
    runtime.layer_visibility.get(layer_id).copied().unwrap_or(true)
}

fn default_map_layers() -> Vec<Value> {
    GIS_MAP_LAYER_IDS
        .iter()
        .enumerate()
        .map(|(index, (id, label, _))| {
            json!({
                "id": id,
                "kind": "map-layer",
                "name": label,
                "x": (index % 4) as f64 * 140.0,
                "y": (index / 4) as f64 * 90.0,
                "width": 120.0,
                "height": 72.0,
            })
        })
        .collect()
}

fn canvas_layers(play: &Gis2dPlayEnvelope) -> String {
    let projection = materialized_projection(play);
    let source = if projection.layers.is_empty() {
        default_map_layers()
    } else {
        projection.layers.clone()
    };
    let layers: Vec<GisMapCanvasLayer> = source
        .iter()
        .filter_map(|layer| {
            let id = layer.get("id").and_then(|value| value.as_str())?;
            if !layer_visible(&play.runtime, id) {
                return None;
            }
            Some(GisMapCanvasLayer {
                id: id.into(),
                kind: layer.get("kind").and_then(|value| value.as_str()).unwrap_or("map-layer").into(),
                name: layer.get("name").and_then(|value| value.as_str()).unwrap_or(id).into(),
                x: layer.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0),
                y: layer.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0),
                width: layer.get("width").and_then(|value| value.as_f64()).unwrap_or(120.0),
                height: layer.get("height").and_then(|value| value.as_f64()).unwrap_or(72.0),
            })
        })
        .collect();
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖DocumentHelpers

//#region 🔖Panels
fn tree_item(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    icon_id: Option<String>,
    command: Option<CommandDescriptor>,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id,
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

fn build_hierarchy_tree(play: &Gis2dPlayEnvelope) -> UiNode {
    let projection = materialized_projection(play);
    let layer_items: Vec<UiTreeItemNode> = if projection.layers.is_empty() {
        GIS_MAP_LAYER_IDS
            .iter()
            .map(|(id, label, icon)| {
                tree_item(
                    format!("gis2d-play-hierarchy.layer.{id}"),
                    *label,
                    Some((*id).into()),
                    Some((*icon).into()),
                    Some(gis2d_cmd("setSelection", Some(json!({ "ids": [id] })))),
                )
            })
            .collect()
    } else {
        projection
            .layers
            .iter()
            .filter_map(|layer| {
                let id = layer.get("id").and_then(|value| value.as_str())?;
                let label = layer.get("name").and_then(|value| value.as_str()).unwrap_or(id);
                Some(tree_item(
                    format!("gis2d-play-hierarchy.layer.{id}"),
                    label,
                    Some(id.into()),
                    Some("layers".into()),
                    Some(gis2d_cmd("setSelection", Some(json!({ "ids": [id] })))),
                ))
            })
            .collect()
    };
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "gis2d-play-hierarchy.layers".into(),
            label: Some(FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL.into()),
            default_open: Some(true),
            items: layer_items,
        }],
        selected_ids: Some(
            play.runtime
                .selected_ids
                .iter()
                .map(|id| format!("gis2d-play-hierarchy.layer.{id}"))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: Some(gis2d_cmd("setSelection", None)),
    })
}

fn build_catalogue_tree(play: &Gis2dPlayEnvelope) -> UiNode {
    let items: Vec<UiTreeItemNode> = GIS_MAP_LAYER_IDS
        .iter()
        .map(|(id, label, icon)| {
            tree_item(
                format!("gis2d-play-catalogue.layer.{id}"),
                *label,
                None,
                Some((*icon).into()),
                Some(gis2d_cmd("toggleLayerVisibility", Some(json!({ "layerId": id })))),
            )
        })
        .collect();
    let _ = play;
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "gis2d-play-catalogue.layers".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(play: &Gis2dPlayEnvelope) -> UiNode {
    if play.runtime.selected_ids.is_empty() {
        let visible_count = GIS_MAP_LAYER_IDS
            .iter()
            .filter(|(id, _, _)| layer_visible(&play.runtime, id))
            .count();
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", GIS_MAP_SCHEMA)),
            ui_text(format!("Layers visible: {visible_count}/{}", GIS_MAP_LAYER_IDS.len())),
            ui_text("Select a map layer in the hierarchy."),
        ]);
    }
    let layer_id = &play.runtime.selected_ids[0];
    let label = GIS_MAP_LAYER_IDS
        .iter()
        .find(|(id, _, _)| *id == layer_id.as_str())
        .map(|(_, label, _)| *label)
        .unwrap_or(layer_id.as_str());
    let visible = layer_visible(&play.runtime, layer_id);
    let mixed = ui_inspector_mixed_toggle(&[visible]);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "gis2d-play-inspector.layer".into(),
        label: "Map Layer".into(),
        default_open: Some(true),
        fields: vec![
            ui_inspector_readonly_field("gis2d-play-inspector.id", "Id", layer_id.clone()),
            ui_inspector_readonly_field("gis2d-play-inspector.label", "Label", label.to_string()),
            UiNode::Field(UiFieldNode {
                id: "gis2d-play-inspector.visible".into(),
                label: "Visible".into(),
                child: UiControlNode::Toggle(UiToggleNode {
                    id: "gis2d-play-inspector.visible.toggle".into(),
                    icon_id: "eye".into(),
                    pressed: mixed.uniform && mixed.pressed,
                    text: None,
                    on_change: gis2d_cmd("toggleLayerVisibility", Some(json!({ "layerId": layer_id }))),
                }),
            }),
        ],
    }])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_canvas(play: &Gis2dPlayEnvelope) -> UiNode {
    build_canvas_2d_scene(
        GIS2D_PLAY_SURFACE,
        GIS2D_PLAY_APP_ID,
        Canvas2dScene {
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 1.0,
            layers_json: canvas_layers(play),
        },
    )
}
//#endregion 🔖Render

//#region 🔖Gis2dPlayApp
struct Gis2dPlayApp;

impl PluginApp for Gis2dPlayApp {
    fn app_id(&self) -> &str {
        GIS2D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("gis2d envelope json")
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
                    if let Ok(parsed) = serde_json::from_value::<Gis2dPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                play.runtime.selected_ids = selection_ids(args);
                return vec![set_document_op(&play)];
            }
            "toggleLayerVisibility" => {
                if let Some(layer_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) {
                    let visible = !layer_visible(&play.runtime, layer_id);
                    play.runtime.layer_visibility.insert(layer_id.into(), visible);
                    return vec![set_document_op(&play)];
                }
            }
            "setLayers" => {
                if let Some(layers) = args.and_then(|value| value.get("layers")) {
                    if let Ok(parsed) = serde_json::from_value(layers.clone()) {
                        let _ = store.dispatch(DocumentVcsCommand::Apply {
                            operations: vec![GisMapOp::SetLayers { layers: parsed }],
                            description: None,
                        });
                        play.redo_edit_ids.clear();
                        return vec![set_document_op(&sync_store_to_envelope(&store, &play.runtime, &play.redo_edit_ids))];
                    }
                }
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
            GIS2D_PLAY_BODY_COMPOSITE => render_canvas(&play),
            GIS2D_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&play),
            GIS2D_PLAY_BODY_CATALOGUE => build_catalogue_tree(&play),
            GIS2D_PLAY_BODY_INSPECTION => build_inspector_tree(&play),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖Gis2dPlayApp

//#region 🔖AppFactory
fn create_gis2d_app() -> App {
    App::from_builder(
        App::builder(GIS2D_PLAY_APP_ID, "GIS 2D")
            .icon_id("gis2d")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(GIS2D_PLAY_WINDOW_MAIN, "Map", GIS2D_PLAY_BODY_COMPOSITE)
            .default_layout(create_default_layout(
                &[GIS2D_PLAY_WINDOW_MAIN.into()],
                "row",
                Some(&[100.0]),
                Some(&["Map".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                GIS2D_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                GIS2D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                GIS2D_PLAY_BODY_INSPECTION,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .program("gis2d", "GIS 2D", "map")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("gis2d", "GIS 2D", "0.1.0").register_app(create_gis2d_app(), || Box::new(Gis2dPlayApp))
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
    fn renders_canvas_scene() {
        let app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(GIS2D_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    #[test]
    fn hierarchy_lists_map_layers() {
        let app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(GIS2D_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("gis2d-play-hierarchy.layer.raster"));
    }

    #[test]
    fn catalogue_lists_layer_toggles() {
        let app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(GIS2D_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("gis2d-play-catalogue.layer.water"));
    }

    #[test]
    fn set_selection_updates_runtime() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "setSelection",
            Some(&json!({ "ids": ["roads"] })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(next.runtime.selected_ids, vec!["roads".to_string()]);
    }

    #[test]
    fn set_layers_command_persists_projection() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let layers = vec![json!({ "id": "custom", "name": "Custom", "x": 0.0, "y": 0.0, "width": 80.0, "height": 40.0 })];
        let ops = app.handle_command("setLayers", Some(&json!({ "layers": layers })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(materialized_projection(&next).layers.len(), 1);
    }

    #[test]
    fn toggle_layer_visibility_hides_layer() {
        let mut app = Gis2dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "toggleLayerVisibility",
            Some(&json!({ "layerId": "raster" })),
            &document,
            &ViewState::default(),
        );
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: Gis2dPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert!(!layer_visible(&next.runtime, "raster"));
    }
}
//#endregion 🧪Tests
