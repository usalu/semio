//! 🩻️ Block 2D app — DocumentApp impl, render, manifest (constitutional: ui).

use block_2d::{Block2dDefinition, Block2dHandleKind, Block2dHandleTemplate, BLOCK_2D_SCHEMA};
use block_2d_op::Block2dOperation;
use block_shared::BlockCompatibilityRule;
use semio_framework_plugin::{
    is_de_locale, localized_label_map, resolve_labels, selection_ids, tree_item_with_action, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionDescriptor, ActionEmit, App,
    AppLabelsOverlay, AppLabelsOverlayExt, ArtifactKindSpec, DocumentApp, DocumentView, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder, SurfaceKind, UiFieldNode,
    UiInputNode, UiNode, UiPresence, UiTreeItemNode, ViewState,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const BLOCK2D_PLAY_APP_ID: &str = "block2d-play";
const BLOCK2D_BODY_BOARD: &str = "block2d.play.board";
const BLOCK2D_BODY_DOCUMENT: &str = "block2d.play.document";
const BLOCK2D_BODY_KINDS: &str = "block2d.play.kinds";
const BLOCK2D_BODY_INSPECTOR: &str = "block2d.play.inspector";
const BLOCK2D_WINDOW_BOARD: &str = "block2d-board";
const BLOCK2D_EXAMPLE_LEFT: &str = "hexagonal-cut-concrete-forest-left";
const BLOCK2D_EXAMPLE_RIGHT: &str = "hexagonal-cut-concrete-forest-right";
//#endregion 🔖️Constants

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the block-2d app.
    struct Block2dLabels {
        window_board: &'static str = en: "Node Kind", de: "Knotenart";
        name: &'static str = en: "Name", de: "Name";
        label: &'static str = en: "Label", de: "Bezeichnung";
        variant: &'static str = en: "Variant", de: "Variante";
        description: &'static str = en: "Description", de: "Beschreibung";
        handle_kinds: &'static str = en: "Handle Kinds", de: "Griffarten";
        handles: &'static str = en: "Handles", de: "Griffe";
        no_handle_kinds: &'static str = en: "(no handle kinds)", de: "(keine Griffarten)";
        no_handles: &'static str = en: "(no handles)", de: "(keine Griffe)";
        summary: &'static str = en: "Node kind", de: "Knotenart";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
fn block2d_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("patchNodeKind", "Patch Node Kind", "Knotenart bearbeiten"),
        ("addHandleKind", "Add Handle Kind", "Griffart hinzufügen"),
        ("removeHandleKind", "Remove Handle Kind", "Griffart entfernen"),
        ("addHandle", "Add Handle", "Griff hinzufügen"),
        ("removeHandle", "Remove Handle", "Griff entfernen"),
        ("edit", "Edit", "Bearbeiten"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
    ];
    localized_label_map(is_de, ENTRIES)
}
//#endregion 🔖️CommandLabels

//#region 🔖️DocumentHelpers
fn block2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: BLOCK2D_PLAY_APP_ID.into(), action: action.into(), args }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Panels
fn build_document_tree(definition: &Block2dDefinition, selected: &[String], labels: &Block2dLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("block2d-play-document");
    let handle_kind_items: Vec<UiTreeItemNode> = definition
        .handle_kinds
        .iter()
        .map(|kind| {
            UiTreeItemNode {
                icon_id: Some("circle".into()),
                ..tree_item_with_action(builder.item_id("handle-kind", &kind.id), kind.label.clone(), Some(kind.color.clone()), block2d_action("setSelection", None))
            }
        })
        .collect();
    let handle_items: Vec<UiTreeItemNode> = definition
        .handles
        .iter()
        .map(|handle| {
            UiTreeItemNode {
                icon_id: Some("circle-dot".into()),
                ..tree_item_with_action(builder.item_id("handle", &handle.id), handle.handle_kind.clone(), Some(format!("{:.2}", handle.angle)), block2d_action("setSelection", None))
            }
        })
        .collect();
    let selected_ids: Vec<String> = selected.to_vec();
    builder
        .section_or_placeholder("block2d-play-document.handle-kinds", Some(labels.handle_kinds.into()), true, handle_kind_items, labels.no_handle_kinds)
        .section_or_placeholder("block2d-play-document.handles", Some(labels.handles.into()), true, handle_items, labels.no_handles)
        .selected(selected_ids)
        .selection_change(block2d_action("setSelection", None))
        .build()
}

fn text_field(id: &str, label: &str, value: &str, field: &str) -> UiNode {
    UiNode::Field(UiFieldNode {
        presence: UiPresence::default(),
        id: id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: value.into(),
            placeholder: None,
            commit: Some("blur".into()),
            on_change: block2d_action("patchNodeKind", Some(json!({ "field": field }))),
            min: None,
            max: None,
            step: None,
            accept: None,
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        menu: None,
    })
}

fn build_inspection_tree(definition: &Block2dDefinition, labels: &Block2dLabels) -> UiNode {
    ui_stack_vertical(vec![
        text_field("block2d-play-inspector.name", labels.name, &definition.node_kind.name, "name"),
        text_field("block2d-play-inspector.label", labels.label, &definition.node_kind.label, "label"),
        text_field("block2d-play-inspector.variant", labels.variant, definition.node_kind.variant.as_deref().unwrap_or(""), "variant"),
        text_field("block2d-play-inspector.description", labels.description, &definition.node_kind.description, "description"),
        ui_inspector_readonly_field("block2d-play-inspector.handle-count", labels.handles, definition.handles.len().to_string()),
    ])
}

fn render_board(definition: &Block2dDefinition, labels: &Block2dLabels) -> UiNode {
    ui_stack_vertical(vec![
        ui_text(format!("{}: {}", labels.summary, if definition.node_kind.label.is_empty() { "—" } else { &definition.node_kind.label })),
        ui_text(format!("{} {}, {} {}", definition.handle_kinds.len(), labels.handle_kinds, definition.handles.len(), labels.handles)),
    ])
}
//#endregion 🔖️Panels

use std::cell::RefCell;
/// 🎛️ Ephemeral view state: the multi-selected row ids in the document tree.
pub struct Block2dPlayApp {
    selected_ids: RefCell<Vec<String>>,
}

impl Default for Block2dPlayApp {
    fn default() -> Self {
        Self { selected_ids: RefCell::new(Vec::new()) }
    }
}

impl DocumentApp for Block2dPlayApp {
    type Projection = Block2dDefinition;
    type Operation = Block2dOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        BLOCK2D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        BLOCK_2D_SCHEMA
    }

    fn initial_projection(&self) -> Block2dDefinition {
        block_2d_engine::empty_block2d_definition()
    }

    fn handle_action(
        &self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, Block2dDefinition>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        _view_state: &ViewState,
    ) -> ActionEmit<Block2dOperation> {
        match action {
            "setSelection" => {
                *self.selected_ids.borrow_mut() = selection_ids(args);
                ActionEmit::default()
            }
            "patchNodeKind" => {
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                let mut node_kind = doc.projection.node_kind.clone();
                match field {
                    "name" => node_kind.name = value.to_string(),
                    "label" => node_kind.label = value.to_string(),
                    "variant" => node_kind.variant = if value.is_empty() { None } else { Some(value.to_string()) },
                    "description" => node_kind.description = value.to_string(),
                    _ => return ActionEmit::default(),
                }
                ActionEmit::operations(vec![Block2dOperation::SetNodeKind { node_kind }])
            }
            "addHandleKind" => {
                let id = block_2d_engine::next_id(doc.projection.handle_kinds.iter().map(|kind| kind.id.as_str()), "handle-kind-");
                let handle_kind = Block2dHandleKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_wire_kind: "cable.link".into() };
                ActionEmit::operations(vec![Block2dOperation::SetHandleKind { index: doc.projection.handle_kinds.len(), handle_kind }])
            }
            "removeHandleKind" => {
                let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) else { return ActionEmit::default() };
                ActionEmit::operations(vec![Block2dOperation::RemoveHandleKind { id: id.to_string() }])
            }
            "addHandle" => {
                let Some(handle_kind_id) = doc.projection.handle_kinds.first().map(|kind| kind.id.clone()) else { return ActionEmit::default() };
                let id = block_2d_engine::next_id(doc.projection.handles.iter().map(|handle| handle.id.as_str()), "handle-");
                let handle = Block2dHandleTemplate { id, handle_kind: handle_kind_id, angle: 0.0, radius: 0.36 };
                ActionEmit::operations(vec![Block2dOperation::SetHandle { index: doc.projection.handles.len(), handle }])
            }
            "removeHandle" => {
                let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) else { return ActionEmit::default() };
                ActionEmit::operations(vec![Block2dOperation::RemoveHandle { id: id.to_string() }])
            }
            "addCompatibilityRule" => {
                let source = args.and_then(|value| value.get("source")).and_then(|value| value.as_str()).unwrap_or_default();
                let target = args.and_then(|value| value.get("target")).and_then(|value| value.as_str()).unwrap_or_default();
                if source.is_empty() || target.is_empty() {
                    return ActionEmit::default();
                }
                let id = block_2d_engine::next_id(doc.projection.compatibility.iter().map(|rule| rule.id.as_str()), "compat-");
                let rule = BlockCompatibilityRule { id, source: source.to_string(), target: target.to_string(), bidirectional: true };
                ActionEmit::operations(vec![Block2dOperation::SetCompatibilityRule { index: doc.projection.compatibility.len(), rule }])
            }
            "removeCompatibilityRule" => {
                let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) else { return ActionEmit::default() };
                ActionEmit::operations(vec![Block2dOperation::RemoveCompatibilityRule { id: id.to_string() }])
            }
            "setActiveExample" => {
                let example = match args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    Some(BLOCK2D_EXAMPLE_LEFT) => block_2d_dsl::parse_dsl(block_2d_dsl::BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
                    Some(BLOCK2D_EXAMPLE_RIGHT) => block_2d_dsl::parse_dsl(block_2d_dsl::BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT).ok(),
                    _ => None,
                };
                match example {
                    Some(document) => ActionEmit::operations(vec![Block2dOperation::SetDocument { document }]),
                    None => ActionEmit::default(),
                }
            }
            "edit" | "textEdit" => {
                let Some(text) = args.and_then(|value| value.get("text")).and_then(|value| value.as_str()) else { return ActionEmit::default() };
                match serde_json::from_str::<Block2dDefinition>(text) {
                    Ok(document) if &document != doc.projection => ActionEmit::operations(vec![Block2dOperation::SetDocument { document }]),
                    _ => ActionEmit::default(),
                }
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(
        &self,
        body_key: &str,
        doc: &DocumentView<'_, Block2dDefinition>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
    ) -> UiNode {
        let labels = resolve_labels::<Block2dLabels>(view_state);
        match body_key {
            BLOCK2D_BODY_BOARD => render_board(doc.projection, labels),
            BLOCK2D_BODY_DOCUMENT | BLOCK2D_BODY_KINDS => build_document_tree(doc.projection, &*self.selected_ids.borrow(), labels),
            BLOCK2D_BODY_INSPECTOR => build_inspection_tree(doc.projection, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<Block2dLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default().window_kind_label(BLOCK2D_WINDOW_BOARD, labels.window_board).action_labels(block2d_action_labels(is_de))
    }
}
//#endregion 🔖️Block2dPlayApp

//#region 🔖️Manifest
pub fn create_block2d_app() -> App {
    App::from_builder(
        App::builder(BLOCK2D_PLAY_APP_ID, "Block 2D")
            .document(["semio", "block", "2d"])
            .artifact_kind(ArtifactKindSpec {
                id: "2d.block".into(),
                name: "Node Kind".into(),
                source_format: BLOCK_2D_SCHEMA.into(),
                component_kind: "block2d".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: BLOCK_2D_SCHEMA.into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("layout-grid")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(BLOCK2D_WINDOW_BOARD, "Node Kind", BLOCK2D_BODY_BOARD, SurfaceKind::Board2d, "layout-grid")
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, BLOCK2D_BODY_DOCUMENT)
            .panel_tab("framework.panel.inspection", "Inspection", PanelGroup::Details, BLOCK2D_BODY_INSPECTOR)
            .operation("patchNodeKind", "Patch Node Kind")
            .operation("addHandleKind", "Add Handle Kind")
            .operation("removeHandleKind", "Remove Handle Kind")
            .operation("addHandle", "Add Handle")
            .operation("removeHandle", "Remove Handle")
            .operation("addCompatibilityRule", "Add Compatibility Rule")
            .operation("removeCompatibilityRule", "Remove Compatibility Rule")
            .operation("setActiveExample", "Set Active Example")
            .operation("edit", "Edit")
            .view_action("setSelection", "Set Selection"),
    )
    .example(BLOCK2D_EXAMPLE_LEFT, "Hexagonal Cut Concrete Forest Left", serde_json::to_string(&block_2d_dsl::parse_dsl(block_2d_dsl::BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default())
    .example(BLOCK2D_EXAMPLE_RIGHT, "Hexagonal Cut Concrete Forest Right", serde_json::to_string(&block_2d_dsl::parse_dsl(block_2d_dsl::BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default())
    .workflow("block2d", "Block 2D", "model")
}
//#endregion 🔖️Manifest

pub fn register_block2d_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Block2dPlayApp>(BLOCK_2D_SCHEMA);
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp};

    #[test]
    fn renders_document_tree_and_inspector() {
        let mut app = testkit::new_app::<Block2dPlayApp>();
        let node = app.render(BLOCK2D_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Handle Kinds"));
        let inspector = app.render(BLOCK2D_BODY_INSPECTOR, None, &ViewState::default()).expect("render");
        let inspector_json = serde_json::to_string(&inspector).unwrap();
        assert!(inspector_json.contains("Name"));
    }

    #[test]
    fn add_handle_kind_then_add_handle_then_remove_round_trips() {
        let mut app = testkit::new_app::<Block2dPlayApp>();
        app.handle_action("addHandleKind", None, &ViewState::default(), &testkit::meta("local")).expect("add handle kind");
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 1);
        app.handle_action("addHandle", None, &ViewState::default(), &testkit::meta("local")).expect("add handle");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.handles.len(), 1);
        let handle_id = projection.handles[0].id.clone();
        app.handle_action("removeHandle", Some(&json!({ "id": handle_id })), &ViewState::default(), &testkit::meta("local")).expect("remove handle");
        assert_eq!(app.projection().expect("projection").handles.len(), 0);
    }

    #[test]
    fn patch_node_kind_updates_name() {
        let mut app = testkit::new_app::<Block2dPlayApp>();
        app.handle_action("patchNodeKind", Some(&json!({ "field": "name", "value": "Renamed" })), &ViewState::default(), &testkit::meta("local")).expect("patch");
        assert_eq!(app.projection().expect("projection").node_kind.name, "Renamed");
    }

    #[test]
    fn set_active_example_loads_left_fixture() {
        let mut app = testkit::new_app::<Block2dPlayApp>();
        app.handle_action("setActiveExample", Some(&json!({ "id": BLOCK2D_EXAMPLE_LEFT })), &ViewState::default(), &testkit::meta("local")).expect("load example");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.node_kind.id, "Hexagonal Cut Concrete Forest Left");
        assert_eq!(projection.handles.len(), 11);
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = testkit::new_app::<Block2dPlayApp>();
        app.handle_action("addHandleKind", None, &ViewState::default(), &testkit::meta("local")).expect("add handle kind");
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 1);
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 0);
        app.handle_action("redo", None, &ViewState::default(), &testkit::meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 1);
    }
}
//#endregion 🧪️Tests
