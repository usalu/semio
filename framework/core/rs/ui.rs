//! 🧩 Declarative UI graph types shared by kernel, plugins, and renderers.

use serde::{Deserialize, Serialize};

//#region 🔖Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandDescriptor {
    pub controller_id: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<String>,
}
//#endregion 🔖Command

//#region 🔖Primitives
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiStackNode {
    pub direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    pub children: Vec<UiNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTextNode {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emphasize: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiButtonNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub icon_id: String,
    pub label: String,
    pub command: CommandDescriptor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSeparatorNode {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiInputNode {
    pub id: String,
    pub input_kind: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSelectItem {
    pub id: String,
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSelectNode {
    pub id: String,
    pub value: String,
    pub items: Vec<UiSelectItem>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiToggleNode {
    pub id: String,
    pub icon_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub pressed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSliderNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSectionNode {
    pub id: String,
    pub title: String,
    pub children: Vec<UiNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeItemNode {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    pub children: Vec<UiNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeNode {
    pub id: String,
    pub items: Vec<UiTreeItemNode>,
}
//#endregion 🔖Primitives

//#region 🔖ComponentScenes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canvas2dScene {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub layers_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World3dScene {
    pub camera_json: String,
    pub instances_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphScene {
    pub nodes_json: String,
    pub edges_json: String,
    pub viewport_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEditorScene {
    pub buffer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableScene {
    pub columns_json: String,
    pub rows_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RasterScene {
    pub width: u32,
    pub height: u32,
    pub pixels_base64: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiComponentSceneNode {
    pub surface_id: String,
    pub controller_id: String,
    pub component_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pane_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_2d: Option<Canvas2dScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_3d: Option<World3dScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_graph: Option<NodeGraphScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_editor: Option<TextEditorScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableScene>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raster: Option<RasterScene>,
}
//#endregion 🔖ComponentScenes

//#region 🔖UiNode
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UiNode {
    Stack(UiStackNode),
    Text(UiTextNode),
    Button(UiButtonNode),
    Separator(UiSeparatorNode),
    Input(UiInputNode),
    Select(UiSelectNode),
    Toggle(UiToggleNode),
    Slider(UiSliderNode),
    Section(UiSectionNode),
    Tree(UiTreeNode),
    ComponentScene(UiComponentSceneNode),
}

pub fn ui_stack_vertical(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("standard".into()),
        padding: None,
        children,
    })
}

pub fn ui_text(value: impl Into<String>) -> UiNode {
    UiNode::Text(UiTextNode {
        value: value.into(),
        emphasize: None,
    })
}

pub fn build_canvas_2d_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: Canvas2dScene,
) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
        component_kind: "canvas-2d".into(),
        pane_id: None,
        binding_id: None,
        canvas_2d: Some(scene),
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        raster: None,
    })
}

pub fn build_world_3d_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: World3dScene,
) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
        component_kind: "world-3d".into(),
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: Some(scene),
        node_graph: None,
        text_editor: None,
        table: None,
        raster: None,
    })
}

pub fn build_node_graph_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: NodeGraphScene,
) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
        component_kind: "node-graph".into(),
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: None,
        node_graph: Some(scene),
        text_editor: None,
        table: None,
        raster: None,
    })
}

pub fn build_text_editor_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: TextEditorScene,
) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
        component_kind: "text-editor".into(),
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: Some(scene),
        table: None,
        raster: None,
    })
}

pub fn build_table_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: TableScene,
) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
        component_kind: "table".into(),
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: Some(scene),
        raster: None,
    })
}
//#endregion 🔖UiNode

//#region 🔖Manifest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keybinding {
    pub keys: String,
    pub command: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModeDefinition {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowKindDefinition {
    pub id: String,
    pub label: String,
    pub body_key: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelTabDefinition {
    pub id: String,
    pub label: String,
    pub group: String,
    pub body_key: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppDefinition {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    pub controller_id: String,
    pub modes: Vec<ModeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode_id: Option<String>,
    pub window_kinds: Vec<WindowKindDefinition>,
    pub panel_tabs: Vec<PanelTabDefinition>,
    pub keybindings: Vec<Keybinding>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramDefinition {
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub yields: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleDefinition {
    pub id: String,
    pub label: String,
    pub document_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub plugin_id: String,
    pub label: String,
    pub version: String,
    pub apps: Vec<AppDefinition>,
    pub programs: Vec<ProgramDefinition>,
    pub examples: Vec<ExampleDefinition>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_mode_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_window_kind_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panel_json: Option<String>,
}
//#endregion 🔖Manifest
