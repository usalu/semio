//! 🎨️ Architect play app — the presentation factories every window and panel builds its `UiNode`
//! tree from: tree sections/items, inspector fields, and the empty component-scene shell.
//!
//! These live at app level (not in the artifact engine) because they produce framework UI types and
//! app-addressed `ActionDescriptor`s — an artifact must never depend on an app.

use crate::apps::architect::architect_action;
use crate::apps::architect::ARCHITECT_APP_ID;
use crate::artifacts::program::registers::AdjacencyKind;
use crate::artifacts::program::{EntityId, ProgramSnapshot};
use semio_framework_plugin::{
    ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_mixed_toggle, ActionDescriptor, Label, SurfaceKind, UiComponentSceneNode, UiFieldNode, UiInputNode, UiNode, UiNumberStepperNode, UiPresence, UiStackNode,
    UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
};
use serde::Serialize;
use serde_json::{json, Value};

//#region 🔖️Tree
pub fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode::base(id, Label::data(label.into()))
}

pub fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { description, action: Some(action), menu: None, ..UiTreeItemNode::base(id, Label::data(label.into())) }
}

pub fn tree_section(id: impl Into<String>, label: Option<String>, items: Vec<UiTreeItemNode>) -> UiTreeSectionNode {
    UiTreeSectionNode { id: id.into(), label: label.map(Label::data), default_open: Some(true), presence: UiPresence::default(), items }
}

/// 🌳️ A plain, non-selectable tree — every remaining caller (catalogue/report/adjacency/trace) has
/// no interaction domain to bind (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); the
/// one tree that IS selectable (the document panel's element list) is built directly via the SDK's
/// `PanelTreeBuilder` instead, so this helper no longer takes a `selected_ids` param.
pub fn tree_node(sections: Vec<UiTreeSectionNode>) -> UiNode {
    UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), interaction_domain: None, drop_action: None, menu: None })
}

/// 🧱️ A horizontal stack — the adjacency matrix's glyph-strip + pair-tree pairing.
pub fn stack_row(id: impl Into<String>, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "row".into(), gap: Some("0.5rem".into()), padding: None, id: Some(id.into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}
//#endregion 🔖️Tree

//#region 🔖️Labels
pub fn element_label(program: &ProgramSnapshot, id: &EntityId) -> String {
    program.elements.iter().find(|element| &element.header.id == id).map_or_else(|| id.to_string(), |element| element.header.name.clone())
}

pub fn adjacency_kind_label(kind: &AdjacencyKind) -> &'static str {
    match kind {
        AdjacencyKind::Required => "Required",
        AdjacencyKind::Preferred => "Preferred",
        AdjacencyKind::Optional => "Optional",
        AdjacencyKind::Prohibited => "Prohibited",
    }
}

pub fn entity_to_json<T: Serialize>(entity: &T) -> Value {
    serde_json::to_value(entity).unwrap_or(Value::Null)
}

pub fn entity_id_from_json(value: &Value) -> Option<String> {
    value.get("id").and_then(|id| id.as_str()).map(str::to_string).or_else(|| value.get("header").and_then(|header| header.get("id")).and_then(|id| id.as_str()).map(str::to_string))
}

pub fn entity_name_from_json(value: &Value) -> String {
    value.get("name").and_then(|name| name.as_str()).map(str::to_string).or_else(|| value.get("header").and_then(|header| header.get("name")).and_then(|name| name.as_str()).map(str::to_string)).unwrap_or_else(|| "Untitled".into())
}
//#endregion 🔖️Labels

//#region 🔖️Inspector
pub fn inspector_patch_action(register_id: &str, entity_id: &str, patch: &Value) -> ActionDescriptor {
    architect_action("patchRegisterItem", Some(json!({ "registerId": register_id, "entityId": entity_id, "patch": patch })))
}

pub fn inspector_text_field(register_id: &str, entity_id: &str, field_id: &str, label: &str, values: &[String], key: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    let patch_value = mixed.value.clone();
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: Label::data(label),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder.map(Label::data),
            commit: Some("blur".into()),
            on_change: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
            min: None,
            max: None,
            step: None,
            accept: None,
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

pub fn inspector_number_field(register_id: &str, entity_id: &str, field_id: &str, label: &str, values: &[f64], key: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    let patch_value = mixed.value;
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: Label::data(label),
        child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
            id: format!("{field_id}.stepper"),
            value: mixed.value,
            step: 0.1,
            uniform: mixed.uniform,
            on_absolute: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
            on_delta: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

pub fn inspector_toggle_field(register_id: &str, entity_id: &str, field_id: &str, label: &str, values: &[bool], key: &str) -> UiNode {
    let mixed = ui_inspector_mixed_toggle(values);
    let patch_value = mixed.pressed;
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: Label::data(label),
        child: Box::new(UiNode::Toggle(UiToggleNode {
            id: format!("{field_id}.toggle"),
            icon_id: "check".into(),
            text: Some(Label::data(if mixed.pressed { "Yes" } else { "No" })),
            on_change: inspector_patch_action(register_id, entity_id, &json!({ key: patch_value })),
            presence: UiPresence::selected(mixed.pressed),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}
//#endregion 🔖️Inspector

//#region 🔖️Scene
pub fn empty_component_scene(surface_id: &str, component_kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: ARCHITECT_APP_ID.into(),
        component_kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    }
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[test]
    fn element_label_falls_back_to_the_raw_id() {
        let program = sample_plugin();
        assert_eq!(element_label(&program, &EntityId("nope".into())), "nope");
        assert_eq!(element_label(&program, &program.elements[0].header.id), program.elements[0].header.name);
    }

    #[test]
    fn entity_json_readers_accept_both_flat_and_header_shapes() {
        assert_eq!(entity_id_from_json(&json!({ "id": "a" })).as_deref(), Some("a"));
        assert_eq!(entity_id_from_json(&json!({ "header": { "id": "b" } })).as_deref(), Some("b"));
        assert_eq!(entity_name_from_json(&json!({ "header": { "name": "N" } })), "N");
        assert_eq!(entity_name_from_json(&json!({})), "Untitled");
    }

    #[test]
    fn every_adjacency_kind_has_a_label() {
        for kind in [AdjacencyKind::Required, AdjacencyKind::Preferred, AdjacencyKind::Optional, AdjacencyKind::Prohibited] {
            assert!(!adjacency_kind_label(&kind).is_empty());
        }
    }
}
//#endregion 🧪️Tests
