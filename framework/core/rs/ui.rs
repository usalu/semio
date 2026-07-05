//! 🧩 Declarative UI graph types shared by kernel, plugins, and renderers.

use crate::layout::NamedLayout;
use crate::layout::WindowEngagement;
use crate::layout::WindowLayout;
use crate::layout::WindowMeasure;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖Command
pub use crate::layout::{CommandDescriptor, StyleSpec};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<HashMap<String, String>>,
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
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSelectNode {
    pub id: String,
    pub value: String,
    pub items: Vec<UiSelectItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiToggleNode {
    pub id: String,
    pub icon_id: String,
    pub pressed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiVec3Node {
    pub id: String,
    pub value: Option<[f64; 3]>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiKeyValueEntry {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiKeyValueNode {
    pub entries: Vec<UiKeyValueEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSliderNode {
    pub id: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiNumberStepperNode {
    pub id: String,
    pub value: f64,
    pub step: f64,
    pub uniform: bool,
    pub on_absolute: CommandDescriptor,
    pub on_delta: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiRingNode {
    pub id: String,
    pub orb_id: String,
    pub t: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiIconSelectNode {
    pub id: String,
    pub value: String,
    pub uniform: bool,
    pub classifier_kind: String,
    pub on_change: CommandDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UiControlNode {
    Input(UiInputNode),
    Select(UiSelectNode),
    Toggle(UiToggleNode),
    Vec3(UiVec3Node),
    Button(UiButtonNode),
    KeyValue(UiKeyValueNode),
    Slider(UiSliderNode),
    NumberStepper(UiNumberStepperNode),
    Ring(UiRingNode),
    IconSelect(UiIconSelectNode),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiFieldNode {
    pub id: String,
    pub label: String,
    pub child: UiControlNode,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSectionNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", alias = "title")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    pub children: Vec<UiNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeItemAction {
    pub icon_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub command: CommandDescriptor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reveal_on_hover: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeItemNode {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "icon")]
    pub icon_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "expanded")]
    pub default_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_command: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhover_command: Option<CommandDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<UiTreeItemAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_data: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UiTreeItemNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<UiControlNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

impl UiTreeItemNode {
    /** @emoji 🌳 Builds a tree item with optional extensions unset. */
    pub fn base(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            command: None,
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeSectionNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    pub items: Vec<UiTreeItemNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTreeNode {
    pub sections: Vec<UiTreeSectionNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_change: Option<CommandDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiInspectorFieldGroup {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    pub fields: Vec<UiNode>,
}

pub const UI_INSPECTOR_MIXED_PLACEHOLDER: &str = "Mixed";
//#endregion 🔖Primitives

//#region 🔖InspectorHelpers
pub fn ui_inspector_all_equal<T: PartialEq>(values: &[T]) -> bool {
    if values.len() <= 1 {
        return true;
    }
    values.windows(2).all(|pair| pair[0] == pair[1])
}

pub struct UiInspectorMixedText {
    pub value: String,
    pub placeholder: Option<String>,
}

pub fn ui_inspector_mixed_text(values: &[String]) -> UiInspectorMixedText {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedText {
        value: if uniform {
            values.first().cloned().unwrap_or_default()
        } else {
            String::new()
        },
        placeholder: if uniform {
            None
        } else {
            Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into())
        },
    }
}

pub struct UiInspectorMixedNumber {
    pub value: f64,
    pub uniform: bool,
}

pub fn ui_inspector_mixed_number(values: &[f64]) -> UiInspectorMixedNumber {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedNumber {
        value: if uniform {
            *values.first().unwrap_or(&0.0)
        } else {
            f64::NAN
        },
        uniform,
    }
}

pub fn ui_inspector_mixed_select(values: &[String]) -> UiInspectorMixedText {
    ui_inspector_mixed_text(values)
}

pub struct UiInspectorMixedToggle {
    pub pressed: bool,
    pub uniform: bool,
}

pub fn ui_inspector_mixed_toggle(values: &[bool]) -> UiInspectorMixedToggle {
    let uniform = ui_inspector_all_equal(values);
    UiInspectorMixedToggle {
        pressed: uniform && values.first().copied().unwrap_or(false),
        uniform,
    }
}

pub fn ui_inspector_mixed_slider(values: &[f64]) -> UiInspectorMixedNumber {
    ui_inspector_mixed_number(values)
}

pub struct UiInspectorMixedVec3 {
    pub value: Option<[f64; 3]>,
    pub uniform: bool,
}

pub fn ui_inspector_mixed_vec3(values: &[[f64; 3]]) -> UiInspectorMixedVec3 {
    let serialized: Vec<String> = values
        .iter()
        .map(|row| serde_json::to_string(row).unwrap_or_default())
        .collect();
    let uniform = ui_inspector_all_equal(&serialized);
    UiInspectorMixedVec3 {
        value: if uniform { values.first().copied() } else { None },
        uniform,
    }
}

pub fn ui_inspector_readonly_field(
    id: impl Into<String>,
    label: impl Into<String>,
    value: impl Into<String>,
) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id,
            input_kind: "text".into(),
            value: value.into(),
            placeholder: None,
            commit: None,
            on_change: CommandDescriptor {
                controller_id: String::new(),
                command: String::new(),
                args: None,
            },
        }),
    })
}

pub fn ui_inspector_groups_to_tree(groups: &[UiInspectorFieldGroup]) -> UiNode {
    let sections: Vec<UiSectionNode> = groups
        .iter()
        .filter(|group| !group.fields.is_empty())
        .map(|group| UiSectionNode {
            id: group.id.clone(),
            label: Some(group.label.clone()),
            default_open: Some(group.default_open.unwrap_or(true)),
            children: group.fields.clone(),
        })
        .collect();
    ui_declarative_sections_to_tree(&sections)
}

pub fn ui_declarative_sections_to_tree(sections: &[UiSectionNode]) -> UiNode {
    let tree_sections: Vec<UiTreeSectionNode> = sections
        .iter()
        .map(|section| UiTreeSectionNode {
            id: section.id.clone(),
            label: section.label.clone(),
            default_open: Some(section.default_open.unwrap_or(true)),
            items: section
                .children
                .iter()
                .enumerate()
                .map(|(index, child)| {
                    ui_declarative_child_to_tree_item(child, format!("{}.{}", section.id, index))
                })
                .collect(),
        })
        .collect();
    UiNode::Tree(if tree_sections.is_empty() {
        UiTreeNode {
            sections: vec![UiTreeSectionNode {
                id: "empty".into(),
                label: None,
                default_open: None,
                items: vec![UiTreeItemNode {
                    id: "empty".into(),
                    label: "—".into(),
                    description: None,
                    icon_id: None,
                    selected: None,
                    default_open: None,
                    command: None,
                    hover_command: None,
                    unhover_command: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: None,
                }],
            }],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
        }
    } else {
        UiTreeNode {
            sections: tree_sections,
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
        }
    })
}

fn ui_declarative_child_to_tree_item(node: &UiNode, fallback_id: String) -> UiTreeItemNode {
    match node {
        UiNode::Text(text) => UiTreeItemNode {
            id: format!("{}.text", fallback_id),
            label: text.value.clone(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            command: None,
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
        UiNode::Field(field) => {
            let description = if let UiControlNode::Input(input) = &field.child {
                input
                    .placeholder
                    .clone()
                    .or_else(|| if input.value.is_empty() { None } else { Some(input.value.clone()) })
            } else {
                None
            };
            UiTreeItemNode {
                id: field.id.clone(),
                label: field.label.clone(),
                description,
                icon_id: None,
                selected: None,
                default_open: None,
                command: None,
                hover_command: None,
                unhover_command: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: Some(field.child.clone()),
                is_hidden: None,
            }
        }
        UiNode::Button(button) => UiTreeItemNode {
            id: button.id.clone().unwrap_or(fallback_id),
            label: button.label.clone(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            command: None,
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: Some(UiControlNode::Button(button.clone())),
            is_hidden: None,
        },
        UiNode::Input(input) => tree_control_item(input.id.clone(), UiControlNode::Input(input.clone())),
        UiNode::Select(select) => tree_control_item(select.id.clone(), UiControlNode::Select(select.clone())),
        UiNode::Toggle(toggle) => tree_control_item(toggle.id.clone(), UiControlNode::Toggle(toggle.clone())),
        UiNode::Vec3(vec3) => tree_control_item(vec3.id.clone(), UiControlNode::Vec3(vec3.clone())),
        UiNode::KeyValue(key_value) => tree_control_item(fallback_id, UiControlNode::KeyValue(key_value.clone())),
        UiNode::Slider(slider) => tree_control_item(slider.id.clone(), UiControlNode::Slider(slider.clone())),
        UiNode::NumberStepper(stepper) => {
            tree_control_item(stepper.id.clone(), UiControlNode::NumberStepper(stepper.clone()))
        }
        UiNode::Ring(ring) => tree_control_item(ring.id.clone(), UiControlNode::Ring(ring.clone())),
        UiNode::IconSelect(icon_select) => {
            tree_control_item(icon_select.id.clone(), UiControlNode::IconSelect(icon_select.clone()))
        }
        UiNode::Separator(_) => UiTreeItemNode {
            id: format!("{}.sep", fallback_id),
            label: "—".into(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            command: None,
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
        other => UiTreeItemNode {
            id: fallback_id,
            label: format!("{other:?}"),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            command: None,
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        },
    }
}

fn tree_control_item(id: String, control: UiControlNode) -> UiTreeItemNode {
    UiTreeItemNode {
        id,
        label: String::new(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        command: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: Some(control),
        is_hidden: None,
    }
}
//#endregion 🔖InspectorHelpers

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
    #[serde(default = "world3d_default_meshes_json")]
    pub meshes_json: String,
    pub instances_json: String,
    #[serde(default = "world3d_default_selection_json")]
    pub selection_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vortices_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attractions_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_volumes_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brush_preview_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_json: Option<String>,
}

pub fn world3d_default_selection_json() -> String {
    r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#.into()
}

pub fn world3d_default_meshes_json() -> String {
    "[]".into()
}

pub fn world3d_camera_json(position: [f64; 3], target: [f64; 3], fov: f64) -> String {
    serde_json::json!({
        "position": position,
        "target": target,
        "up": [0.0, 0.0, 1.0],
        "fov": fov,
    })
    .to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphScene {
    pub nodes_json: String,
    pub edges_json: String,
    pub viewport_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operators_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_menu_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_items_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_off_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lod_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogue_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computing_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixture_json: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEditorScene {
    pub buffer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlays_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholders_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_carets_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_spans_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_json: Option<String>,
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
pub struct VirtualFileSystemScene {
    pub schema_json: String,
    pub rows_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_row_ids_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_row_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_drop_enabled: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_file_system: Option<VirtualFileSystemScene>,
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
    Vec3(UiVec3Node),
    KeyValue(UiKeyValueNode),
    Slider(UiSliderNode),
    NumberStepper(UiNumberStepperNode),
    Ring(UiRingNode),
    IconSelect(UiIconSelectNode),
    Field(UiFieldNode),
    Section(UiSectionNode),
    Tree(UiTreeNode),
    ComponentScene(UiComponentSceneNode),
}

impl NodeGraphScene {
    /** @emoji 🕸️ Builds a node-graph scene with optional extensions unset. */
    pub fn base(nodes_json: String, edges_json: String, viewport_json: String) -> Self {
        Self {
            nodes_json,
            edges_json,
            viewport_json,
            editable: None,
            operators_json: None,
            context_menu_json: None,
            find_items_json: None,
            selection_json: None,
            hover_json: None,
            preview_off_json: None,
            lod_json: None,
            catalogue_json: None,
            controls_json: None,
            clusters_json: None,
            computing_json: None,
            capabilities_json: None,
            fixture_json: None,
        }
    }
}

impl TextEditorScene {
    /** @emoji ✍️ Builds a text-editor scene with optional extensions unset. */
    pub fn base(buffer: String, language: Option<String>, selection_json: Option<String>) -> Self {
        Self {
            buffer,
            language,
            selection_json,
            tokens_json: None,
            diagnostics_json: None,
            completions_json: None,
            overlays_json: None,
            occurrences_json: None,
            placeholders_json: None,
            extra_carets_json: None,
            selectable_spans_json: None,
            settings_json: None,
            camera_json: None,
        }
    }
}

//#region 🔖SceneCommands
/** @emoji 🎮 Renderer-to-plugin command names for node-graph surfaces. */
pub mod node_graph_commands {
    pub const SELECT: &str = "nodeGraphSelect";
    pub const HOVER: &str = "nodeGraphHover";
    pub const EDIT: &str = "nodeGraphEdit";
    pub const VIEWPORT: &str = "nodeGraphViewport";
    pub const SPOTLIGHT_COMMIT: &str = "spotlightCommit";
}

/** @emoji ✍️ Renderer-to-plugin command names for text-editor surfaces. */
pub mod text_editor_commands {
    pub const EDIT: &str = "textEdit";
    pub const SELECT: &str = "textSelect";
    pub const HOVER: &str = "textHover";
    pub const REQUEST_COMPLETIONS: &str = "requestCompletions";
    pub const COMMIT_RENAME: &str = "commitRename";
    pub const FORMAT_DOCUMENT: &str = "formatDocument";
}
//#endregion 🔖SceneCommands

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
        data_attributes: None,
    })
}

fn component_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    component_kind: impl Into<String>,
    pane_id: Option<String>,
    binding_id: Option<String>,
    canvas_2d: Option<Canvas2dScene>,
    world_3d: Option<World3dScene>,
    node_graph: Option<NodeGraphScene>,
    text_editor: Option<TextEditorScene>,
    table: Option<TableScene>,
    raster: Option<RasterScene>,
    virtual_file_system: Option<VirtualFileSystemScene>,
) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
        component_kind: component_kind.into(),
        pane_id,
        binding_id,
        canvas_2d,
        world_3d,
        node_graph,
        text_editor,
        table,
        raster,
        virtual_file_system,
    })
}

pub fn build_canvas_2d_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: Canvas2dScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        "canvas-2d",
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

pub fn build_world_3d_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: World3dScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        "world-3d",
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
        None,
    )
}

pub fn build_node_graph_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: NodeGraphScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        "node-graph",
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
        None,
    )
}

pub fn build_text_editor_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: TextEditorScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        "text-editor",
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
        None,
    )
}

pub fn build_table_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: TableScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        "table",
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
        None,
    )
}

pub fn build_raster_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: RasterScene,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        "raster",
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
        None,
    )
}

pub fn build_virtual_file_system_scene(
    surface_id: impl Into<String>,
    controller_id: impl Into<String>,
    scene: VirtualFileSystemScene,
    pane_id: Option<String>,
    binding_id: Option<String>,
) -> UiNode {
    component_scene(
        surface_id,
        controller_id,
        "virtualFileSystem",
        pane_id,
        binding_id,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(scene),
    )
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<crate::tools::ToolNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowKindDefinition {
    pub id: String,
    pub label: String,
    pub body_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub measures: Vec<WindowMeasure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engagement: Option<WindowEngagement>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub named_layouts: Vec<NamedLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_layout: Option<WindowLayout>,
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
