//! 👯️ Block 5D app — DocumentApp impl, render, manifest (constitutional: ui).

use block_5d::{Block5dDefinition, Block5dGripKind, Block5dGripTemplate, BLOCK_5D_SCHEMA};
use block_5d_op::Block5dOperation;
use semio_framework_plugin::{
    create_default_layout, is_de_locale, localized_label_map, resolve_labels, selection_ids, tree_item_with_action, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionDescriptor,
    ActionEmit, App, AppLabelsOverlay, AppLabelsOverlayExt, ArtifactKindSpec, DocumentApp, DocumentView, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder,
    SurfaceKind, UiFieldNode, UiInputNode, UiNode, UiPresence, UiTreeItemNode, ViewState,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const BLOCK5D_PLAY_APP_ID: &str = "block5d-play";
const BLOCK5D_BODY_BOARD: &str = "block5d.play.board";
const BLOCK5D_BODY_WORLD: &str = "block5d.play.world";
const BLOCK5D_BODY_DOCUMENT: &str = "block5d.play.document";
const BLOCK5D_BODY_INSPECTOR: &str = "block5d.play.inspector";
const BLOCK5D_WINDOW_BOARD: &str = "block5d-board";
const BLOCK5D_WINDOW_WORLD: &str = "block5d-world";
const BLOCK5D_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";
const BLOCK5D_EXAMPLE_CAPSULE: &str = "nakagin-capsule";
//#endregion 🔖️Constants

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    struct Block5dLabels {
        window_board: &'static str = en: "Board", de: "Board";
        window_world: &'static str = en: "World", de: "Welt";
        name: &'static str = en: "Name", de: "Name";
        label: &'static str = en: "Label", de: "Bezeichnung";
        grip_kinds: &'static str = en: "Grip Kinds", de: "Griffarten";
        grips: &'static str = en: "Grips", de: "Griffe";
        no_grip_kinds: &'static str = en: "(no grip kinds)", de: "(keine Griffarten)";
        no_grips: &'static str = en: "(no grips)", de: "(keine Griffe)";
        summary: &'static str = en: "Part kind", de: "Teilart";
    }
}
//#endregion 🔖️Terminology

fn block5d_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("patchPartKind", "Patch Part Kind", "Teilart bearbeiten"),
        ("addGripKind", "Add Grip Kind", "Griffart hinzufügen"),
        ("removeGripKind", "Remove Grip Kind", "Griffart entfernen"),
        ("addGrip", "Add Grip", "Griff hinzufügen"),
        ("removeGrip", "Remove Grip", "Griff entfernen"),
        ("edit", "Edit", "Bearbeiten"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
    ];
    localized_label_map(is_de, ENTRIES)
}

fn block5d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: BLOCK5D_PLAY_APP_ID.into(), action: action.into(), args }
}

//#region 🔖️Panels
fn build_document_tree(definition: &Block5dDefinition, selected: &[String], labels: &Block5dLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("block5d-play-document");
    let grip_kind_items: Vec<UiTreeItemNode> = definition
        .grip_kinds
        .iter()
        .map(|kind| UiTreeItemNode { icon_id: Some("circle".into()), menu: None,
            ..tree_item_with_action(builder.item_id("grip-kind", &kind.id), kind.label.clone(), Some(kind.color.clone()), block5d_action("setSelection", None))
        })
        .collect();
    let grip_items: Vec<UiTreeItemNode> = definition
        .grips
        .iter()
        .map(|grip| UiTreeItemNode { icon_id: Some("circle-dot".into()), menu: None,
            ..tree_item_with_action(builder.item_id("grip", &grip.id), grip.grip_kind.clone(), Some(format!("{:.2}", grip.angle)), block5d_action("setSelection", None))
        })
        .collect();
    builder
        .section_or_placeholder("block5d-play-document.grip-kinds", Some(labels.grip_kinds.into()), true, grip_kind_items, labels.no_grip_kinds)
        .section_or_placeholder("block5d-play-document.grips", Some(labels.grips.into()), true, grip_items, labels.no_grips)
        .selected(selected.to_vec())
        .selection_change(block5d_action("setSelection", None))
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
            on_change: block5d_action("patchPartKind", Some(json!({ "field": field }))),
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

fn build_inspection_tree(definition: &Block5dDefinition, labels: &Block5dLabels) -> UiNode {
    ui_stack_vertical(vec![
        text_field("block5d-play-inspector.name", labels.name, &definition.part_kind.name, "name"),
        text_field("block5d-play-inspector.label", labels.label, &definition.part_kind.label, "label"),
        ui_inspector_readonly_field("block5d-play-inspector.grip-count", labels.grips, definition.grips.len().to_string()),
    ])
}

fn render_board(definition: &Block5dDefinition, labels: &Block5dLabels) -> UiNode {
    ui_stack_vertical(vec![
        ui_text(format!("{}: {}", labels.summary, if definition.part_kind.label.is_empty() { "—" } else { &definition.part_kind.label })),
        ui_text(format!("2d grips: {}", definition.grips.len())),
    ])
}

fn render_world(definition: &Block5dDefinition, labels: &Block5dLabels) -> UiNode {
    let mesh_url = definition.representations.first().and_then(|representation| representation.mesh_url.as_deref()).unwrap_or("—");
    ui_stack_vertical(vec![ui_text(format!("{}: {}", labels.summary, if definition.part_kind.label.is_empty() { "—" } else { &definition.part_kind.label })), ui_text(format!("mesh: {mesh_url}"))])
}
//#endregion 🔖️Panels

//#region 🔖️Block5dPlayApp
#[derive(Default)]
pub struct Block5dPlayApp {
    selected_ids: Vec<String>,
}

impl DocumentApp for Block5dPlayApp {
    type Projection = Block5dDefinition;
    type Operation = Block5dOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        BLOCK5D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        BLOCK_5D_SCHEMA
    }

    fn initial_projection(&self) -> Block5dDefinition {
        block_5d_engine::empty_block5d_definition()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, Block5dDefinition>, _view_state: &ViewState) -> ActionEmit<Block5dOperation> {
        match action {
            "setSelection" => {
                self.selected_ids = selection_ids(args);
                ActionEmit::default()
            }
            "patchPartKind" => {
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                let mut part_kind = doc.projection.part_kind.clone();
                match field {
                    "name" => part_kind.name = value.to_string(),
                    "label" => part_kind.label = value.to_string(),
                    "variant" => part_kind.variant = if value.is_empty() { None } else { Some(value.to_string()) },
                    "description" => part_kind.description = value.to_string(),
                    _ => return ActionEmit::default(),
                }
                ActionEmit::operations(vec![Block5dOperation::SetPartKind { part_kind }])
            }
            "addGripKind" => {
                let id = block_5d_engine::next_id(doc.projection.grip_kinds.iter().map(|kind| kind.id.as_str()), "grip-kind-");
                let grip_kind = Block5dGripKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_rope_kind: "rope.link".into() };
                ActionEmit::operations(vec![Block5dOperation::SetGripKind { index: doc.projection.grip_kinds.len(), grip_kind }])
            }
            "removeGripKind" => {
                let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) else { return ActionEmit::default() };
                ActionEmit::operations(vec![Block5dOperation::RemoveGripKind { id: id.to_string() }])
            }
            "addGrip" => {
                let Some(grip_kind_id) = doc.projection.grip_kinds.first().map(|kind| kind.id.clone()) else { return ActionEmit::default() };
                let id = block_5d_engine::next_id(doc.projection.grips.iter().map(|grip| grip.id.as_str()), "grip-");
                let grip = Block5dGripTemplate { id, grip_kind: grip_kind_id, angle: 0.0, radius_2d: 0.36, position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 };
                ActionEmit::operations(vec![Block5dOperation::SetGrip { index: doc.projection.grips.len(), grip }])
            }
            "removeGrip" => {
                let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) else { return ActionEmit::default() };
                ActionEmit::operations(vec![Block5dOperation::RemoveGrip { id: id.to_string() }])
            }
            "setActiveExample" => {
                let example = match args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    Some(BLOCK5D_EXAMPLE_FOREST_LEFT) => block_5d_dsl::parse_dsl(block_5d_dsl::BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
                    Some(BLOCK5D_EXAMPLE_CAPSULE) => block_5d_dsl::parse_dsl(block_5d_dsl::BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).ok(),
                    _ => None,
                };
                match example {
                    Some(document) => ActionEmit::operations(vec![Block5dOperation::SetDocument { document }]),
                    None => ActionEmit::default(),
                }
            }
            "edit" | "textEdit" => {
                let Some(text) = args.and_then(|value| value.get("text")).and_then(|value| value.as_str()) else { return ActionEmit::default() };
                match serde_json::from_str::<Block5dDefinition>(text) {
                    Ok(document) if &document != doc.projection => ActionEmit::operations(vec![Block5dOperation::SetDocument { document }]),
                    _ => ActionEmit::default(),
                }
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Block5dDefinition>, view_state: &ViewState) -> UiNode {
        let labels = resolve_labels::<Block5dLabels>(view_state);
        match body_key {
            BLOCK5D_BODY_BOARD => render_board(doc.projection, labels),
            BLOCK5D_BODY_WORLD => render_world(doc.projection, labels),
            BLOCK5D_BODY_DOCUMENT => build_document_tree(doc.projection, &self.selected_ids, labels),
            BLOCK5D_BODY_INSPECTOR => build_inspection_tree(doc.projection, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<Block5dLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(BLOCK5D_WINDOW_BOARD, labels.window_board)
            .window_kind_label(BLOCK5D_WINDOW_WORLD, labels.window_world)
            .action_labels(block5d_action_labels(is_de))
    }
}
//#endregion 🔖️Block5dPlayApp

//#region 🔖️Manifest
pub fn create_block5d_app() -> App {
    App::from_builder(
        App::builder(BLOCK5D_PLAY_APP_ID, "Block 5D")
            .document(["semio", "block", "5d"])
            .artifact_kind(ArtifactKindSpec {
                id: "5d.block".into(),
                name: "Part Kind".into(),
                source_format: BLOCK_5D_SCHEMA.into(),
                component_kind: "block5d".into(),
                dimension: "5d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: BLOCK_5D_SCHEMA.into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("layers")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(BLOCK5D_WINDOW_BOARD, "Board", BLOCK5D_BODY_BOARD, SurfaceKind::Board2d, "layout-grid")
            .window_kind(BLOCK5D_WINDOW_WORLD, "World", BLOCK5D_BODY_WORLD, SurfaceKind::World3d, "box")
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, BLOCK5D_BODY_DOCUMENT)
            .panel_tab("framework.panel.inspection", "Inspection", PanelGroup::Details, BLOCK5D_BODY_INSPECTOR)
            .operation("patchPartKind", "Patch Part Kind")
            .operation("addGripKind", "Add Grip Kind")
            .operation("removeGripKind", "Remove Grip Kind")
            .operation("addGrip", "Add Grip")
            .operation("removeGrip", "Remove Grip")
            .operation("setActiveExample", "Set Active Example")
            .operation("edit", "Edit")
            .view_action("setSelection", "Set Selection")
            .default_layout(create_default_layout(&[BLOCK5D_WINDOW_BOARD.into(), BLOCK5D_WINDOW_WORLD.into()], "row", Some(&[50.0, 50.0]), Some(&["Board".into(), "World".into()]))),
    )
    .example(BLOCK5D_EXAMPLE_FOREST_LEFT, "Hexagonal Cut Concrete Forest Left", serde_json::to_string(&block_5d_dsl::parse_dsl(block_5d_dsl::BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default())
    .example(BLOCK5D_EXAMPLE_CAPSULE, "Nakagin Capsule", serde_json::to_string(&block_5d_dsl::parse_dsl(block_5d_dsl::BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default())
    .workflow("block5d", "Block 5D", "model")
}
//#endregion 🔖️Manifest

pub fn register_block5d_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Block5dPlayApp>(BLOCK_5D_SCHEMA);
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp};

    #[test]
    fn renders_document_tree_board_and_world() {
        let mut app = testkit::new_app::<Block5dPlayApp>();
        let node = app.render(BLOCK5D_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("Grip Kinds"));
        let board = app.render(BLOCK5D_BODY_BOARD, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&board).unwrap().contains("2d grips"));
        let world = app.render(BLOCK5D_BODY_WORLD, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&world).unwrap().contains("mesh:"));
    }

    #[test]
    fn add_grip_kind_then_add_grip_then_remove_round_trips() {
        let mut app = testkit::new_app::<Block5dPlayApp>();
        app.handle_action("addGripKind", None, &ViewState::default(), &testkit::meta("local")).expect("add grip kind");
        app.handle_action("addGrip", None, &ViewState::default(), &testkit::meta("local")).expect("add grip");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.grips.len(), 1);
        let grip_id = projection.grips[0].id.clone();
        app.handle_action("removeGrip", Some(&json!({ "id": grip_id })), &ViewState::default(), &testkit::meta("local")).expect("remove grip");
        assert_eq!(app.projection().expect("projection").grips.len(), 0);
    }

    #[test]
    fn set_active_example_loads_forest_left_fixture() {
        let mut app = testkit::new_app::<Block5dPlayApp>();
        app.handle_action("setActiveExample", Some(&json!({ "id": BLOCK5D_EXAMPLE_FOREST_LEFT })), &ViewState::default(), &testkit::meta("local")).expect("load example");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.part_kind.id, "Hexagonal Cut Concrete Forest Left");
        assert_eq!(projection.grips.len(), 1);
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = testkit::new_app::<Block5dPlayApp>();
        app.handle_action("addGripKind", None, &ViewState::default(), &testkit::meta("local")).expect("add grip kind");
        assert_eq!(app.projection().expect("projection").grip_kinds.len(), 1);
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").grip_kinds.len(), 0);
        app.handle_action("redo", None, &ViewState::default(), &testkit::meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").grip_kinds.len(), 1);
    }
}
//#endregion 🧪️Tests
