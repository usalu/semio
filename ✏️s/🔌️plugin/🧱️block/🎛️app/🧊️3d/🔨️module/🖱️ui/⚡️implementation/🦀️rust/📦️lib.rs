//! 🏙️ Block 3D app — DocumentApp impl, render, manifest (constitutional: ui).

use block_3d::{Block3dDefinition, Block3dVortexKind, Block3dVortexTemplate, BLOCK_3D_SCHEMA};
use block_3d_op::Block3dOperation;
use block_shared::BlockRepresentation;
use semio_framework_plugin::{
    is_de_locale, localized_label_map, resolve_labels, selection_ids, tree_item_with_action, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionDescriptor, ActionEmit, App,
    AppLabelsOverlay, AppLabelsOverlayExt, ArtifactKindSpec, DocumentApp, DocumentView, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder, SurfaceKind, UiFieldNode,
    UiInputNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiTreeItemNode, ViewState,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const BLOCK3D_PLAY_APP_ID: &str = "block3d-play";
const BLOCK3D_BODY_WORLD: &str = "block3d.play.world";
const BLOCK3D_BODY_DOCUMENT: &str = "block3d.play.document";
const BLOCK3D_BODY_INSPECTOR: &str = "block3d.play.inspector";
const BLOCK3D_WINDOW_WORLD: &str = "block3d-world";
const BLOCK3D_EXAMPLE_CAPSULE: &str = "nakagin-capsule";
const BLOCK3D_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";
//#endregion 🔖️Constants

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    struct Block3dLabels {
        window_world: &'static str = en: "Object Kind", de: "Objektart";
        name: &'static str = en: "Name", de: "Name";
        label: &'static str = en: "Label", de: "Bezeichnung";
        representation: &'static str = en: "Representation", de: "Darstellung";
        representations: &'static str = en: "Representations", de: "Darstellungen";
        vortex_kinds: &'static str = en: "Vortex Kinds", de: "Wirbelarten";
        vortices: &'static str = en: "Vortices", de: "Wirbel";
        no_representations: &'static str = en: "(no representations)", de: "(keine Darstellungen)";
        no_vortices: &'static str = en: "(no vortices)", de: "(keine Wirbel)";
        summary: &'static str = en: "Object kind", de: "Objektart";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
fn block3d_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("patchObjectKind", "Patch Object Kind", "Objektart bearbeiten"),
        ("addRepresentation", "Add Representation", "Darstellung hinzufügen"),
        ("removeRepresentation", "Remove Representation", "Darstellung entfernen"),
        ("addVortexKind", "Add Vortex Kind", "Wirbelart hinzufügen"),
        ("removeVortexKind", "Remove Vortex Kind", "Wirbelart entfernen"),
        ("addVortex", "Add Vortex", "Wirbel hinzufügen"),
        ("removeVortex", "Remove Vortex", "Wirbel entfernen"),
        ("setActiveRepresentation", "Set Active Representation", "Aktive Darstellung festlegen"),
        ("edit", "Edit", "Bearbeiten"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
    ];
    localized_label_map(is_de, ENTRIES)
}
//#endregion 🔖️CommandLabels

fn block3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: BLOCK3D_PLAY_APP_ID.into(), action: action.into(), args }
}

//#region 🔖️Panels
fn build_document_tree(definition: &Block3dDefinition, selected: &[String], labels: &Block3dLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("block3d-play-document");
    let representation_items: Vec<UiTreeItemNode> = definition
        .representations
        .iter()
        .map(|representation| {
            UiTreeItemNode {
                icon_id: Some("box".into()),
                ..tree_item_with_action(builder.item_id("representation", &representation.id), representation.name.clone(), representation.mesh_url.clone(), block3d_action("setSelection", None))
            }
        })
        .collect();
    let vortex_items: Vec<UiTreeItemNode> = definition
        .vortices
        .iter()
        .map(|vortex| {
            UiTreeItemNode {
                icon_id: Some("circle-dot".into()),
                ..tree_item_with_action(builder.item_id("vortex", &vortex.id), vortex.vortex_kind.clone(), None, block3d_action("setSelection", None))
            }
        })
        .collect();
    builder
        .section_or_placeholder("block3d-play-document.representations", Some(labels.representations.into()), true, representation_items, labels.no_representations)
        .section_or_placeholder("block3d-play-document.vortices", Some(labels.vortices.into()), true, vortex_items, labels.no_vortices)
        .selected(selected.to_vec())
        .selection_change(block3d_action("setSelection", None))
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
            on_change: block3d_action("patchObjectKind", Some(json!({ "field": field }))),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn build_inspection_tree(definition: &Block3dDefinition, active_representation_id: Option<&str>, labels: &Block3dLabels) -> UiNode {
    let representation_select = UiNode::Select(UiSelectNode {
        id: "block3d-play-inspector.representation".into(),
        value: active_representation_id.unwrap_or_default().into(),
        items: definition.representations.iter().map(|representation| UiSelectItem { value: representation.id.clone(), label: representation.name.clone() }).collect(),
        placeholder: None,
        on_change: block3d_action("setActiveRepresentation", None),
        presence: UiPresence::default(),
    });
    ui_stack_vertical(vec![
        text_field("block3d-play-inspector.name", labels.name, &definition.object_kind.name, "name"),
        text_field("block3d-play-inspector.label", labels.label, &definition.object_kind.label, "label"),
        UiNode::Field(UiFieldNode { presence: UiPresence::default(), id: "block3d-play-inspector.representation-field".into(), label: labels.representation.into(), child: Box::new(representation_select), description: None, required: None, error: None }),
        ui_inspector_readonly_field("block3d-play-inspector.vortex-count", labels.vortices, definition.vortices.len().to_string()),
    ])
}

fn render_world(definition: &Block3dDefinition, active_representation_id: Option<&str>, labels: &Block3dLabels) -> UiNode {
    let mesh_url = active_representation_id
        .and_then(|id| definition.representations.iter().find(|representation| representation.id == id))
        .or_else(|| definition.representations.first())
        .and_then(|representation| representation.mesh_url.as_deref())
        .unwrap_or("—");
    ui_stack_vertical(vec![
        ui_text(format!("{}: {}", labels.summary, if definition.object_kind.label.is_empty() { "—" } else { &definition.object_kind.label })),
        ui_text(format!("mesh: {mesh_url}")),
        ui_text(format!("{} {}", definition.vortices.len(), labels.vortices)),
    ])
}
//#endregion 🔖️Panels

//#region 🔖️Block3dPlayApp
#[derive(Default)]
pub struct Block3dPlayApp {
    selected_ids: Vec<String>,
    active_representation_id: Option<String>,
}

impl DocumentApp for Block3dPlayApp {
    type Projection = Block3dDefinition;
    type Operation = Block3dOperation;

    fn app_id(&self) -> &str {
        BLOCK3D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        BLOCK_3D_SCHEMA
    }

    fn initial_projection(&self) -> Block3dDefinition {
        block_3d_engine::empty_block3d_definition()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, Block3dDefinition>, _view_state: &ViewState) -> ActionEmit<Block3dOperation> {
        match action {
            "setSelection" => {
                self.selected_ids = selection_ids(args);
                ActionEmit::default()
            }
            "setActiveRepresentation" => {
                self.active_representation_id = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map(str::to_string);
                ActionEmit::default()
            }
            "patchObjectKind" => {
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                let mut object_kind = doc.projection.object_kind.clone();
                match field {
                    "name" => object_kind.name = value.to_string(),
                    "label" => object_kind.label = value.to_string(),
                    "variant" => object_kind.variant = if value.is_empty() { None } else { Some(value.to_string()) },
                    "description" => object_kind.description = value.to_string(),
                    _ => return ActionEmit::default(),
                }
                ActionEmit::operations(vec![Block3dOperation::SetObjectKind { object_kind }])
            }
            "addRepresentation" => {
                let id = block_3d_engine::next_id(doc.projection.representations.iter().map(|representation| representation.id.as_str()), "representation-");
                let representation = BlockRepresentation { id: id.clone(), name: id, mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() };
                ActionEmit::operations(vec![Block3dOperation::SetRepresentation { index: doc.projection.representations.len(), representation }])
            }
            "removeRepresentation" => {
                let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) else { return ActionEmit::default() };
                ActionEmit::operations(vec![Block3dOperation::RemoveRepresentation { id: id.to_string() }])
            }
            "addVortexKind" => {
                let id = block_3d_engine::next_id(doc.projection.vortex_kinds.iter().map(|kind| kind.id.as_str()), "vortex-kind-");
                let vortex_kind = Block3dVortexKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_cable_kind: "cable.link".into() };
                ActionEmit::operations(vec![Block3dOperation::SetVortexKind { index: doc.projection.vortex_kinds.len(), vortex_kind }])
            }
            "removeVortexKind" => {
                let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) else { return ActionEmit::default() };
                ActionEmit::operations(vec![Block3dOperation::RemoveVortexKind { id: id.to_string() }])
            }
            "addVortex" => {
                let Some(vortex_kind_id) = doc.projection.vortex_kinds.first().map(|kind| kind.id.clone()) else { return ActionEmit::default() };
                let id = block_3d_engine::next_id(doc.projection.vortices.iter().map(|vortex| vortex.id.as_str()), "vortex-");
                let vortex = Block3dVortexTemplate { id, vortex_kind: vortex_kind_id, position: [0.0, 0.0, 0.0], direction: [0.0, 0.0, 1.0], radius: 0.3, label: None };
                ActionEmit::operations(vec![Block3dOperation::SetVortex { index: doc.projection.vortices.len(), vortex }])
            }
            "removeVortex" => {
                let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) else { return ActionEmit::default() };
                ActionEmit::operations(vec![Block3dOperation::RemoveVortex { id: id.to_string() }])
            }
            "setActiveExample" => {
                let example = match args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    Some(BLOCK3D_EXAMPLE_CAPSULE) => block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).ok(),
                    Some(BLOCK3D_EXAMPLE_FOREST_LEFT) => block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
                    _ => None,
                };
                match example {
                    Some(document) => ActionEmit::operations(vec![Block3dOperation::SetDocument { document }]),
                    None => ActionEmit::default(),
                }
            }
            "edit" | "textEdit" => {
                let Some(text) = args.and_then(|value| value.get("text")).and_then(|value| value.as_str()) else { return ActionEmit::default() };
                match serde_json::from_str::<Block3dDefinition>(text) {
                    Ok(document) if &document != doc.projection => ActionEmit::operations(vec![Block3dOperation::SetDocument { document }]),
                    _ => ActionEmit::default(),
                }
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Block3dDefinition>, view_state: &ViewState) -> UiNode {
        let labels = resolve_labels::<Block3dLabels>(view_state);
        match body_key {
            BLOCK3D_BODY_WORLD => render_world(doc.projection, self.active_representation_id.as_deref(), labels),
            BLOCK3D_BODY_DOCUMENT => build_document_tree(doc.projection, &self.selected_ids, labels),
            BLOCK3D_BODY_INSPECTOR => build_inspection_tree(doc.projection, self.active_representation_id.as_deref(), labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<Block3dLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default().window_kind_label(BLOCK3D_WINDOW_WORLD, labels.window_world).action_labels(block3d_action_labels(is_de))
    }
}
//#endregion 🔖️Block3dPlayApp

//#region 🔖️Manifest
pub fn create_block3d_app() -> App {
    App::from_builder(
        App::builder(BLOCK3D_PLAY_APP_ID, "Block 3D")
            .document(["semio", "block", "3d"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.block".into(),
                name: "Object Kind".into(),
                source_format: BLOCK_3D_SCHEMA.into(),
                component_kind: "block3d".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: BLOCK_3D_SCHEMA.into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("box")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(BLOCK3D_WINDOW_WORLD, "Object Kind", BLOCK3D_BODY_WORLD, SurfaceKind::World3d, "box")
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, BLOCK3D_BODY_DOCUMENT)
            .panel_tab("framework.panel.inspection", "Inspection", PanelGroup::Details, BLOCK3D_BODY_INSPECTOR)
            .operation("patchObjectKind", "Patch Object Kind")
            .operation("addRepresentation", "Add Representation")
            .operation("removeRepresentation", "Remove Representation")
            .operation("addVortexKind", "Add Vortex Kind")
            .operation("removeVortexKind", "Remove Vortex Kind")
            .operation("addVortex", "Add Vortex")
            .operation("removeVortex", "Remove Vortex")
            .operation("setActiveExample", "Set Active Example")
            .operation("edit", "Edit")
            .view_action("setSelection", "Set Selection")
            .view_action("setActiveRepresentation", "Set Active Representation"),
    )
    .example(BLOCK3D_EXAMPLE_CAPSULE, "Nakagin Capsule", serde_json::to_string(&block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default())
    .example(BLOCK3D_EXAMPLE_FOREST_LEFT, "Hexagonal Cut Concrete Forest Left", serde_json::to_string(&block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default())
    .workflow("block3d", "Block 3D", "model")
}
//#endregion 🔖️Manifest

pub fn register_block3d_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Block3dPlayApp>(BLOCK_3D_SCHEMA);
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp};

    #[test]
    fn renders_document_tree_and_inspector() {
        let mut app = testkit::new_app::<Block3dPlayApp>();
        let node = app.render(BLOCK3D_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Representations"));
    }

    #[test]
    fn add_representation_then_set_active_then_render_world_shows_mesh() {
        let mut app = testkit::new_app::<Block3dPlayApp>();
        app.handle_action("addRepresentation", None, &ViewState::default(), &testkit::meta("local")).expect("add representation");
        let representation_id = app.projection().expect("projection").representations[0].id.clone();
        app.handle_action("setActiveRepresentation", Some(&json!({ "value": representation_id })), &ViewState::default(), &testkit::meta("local")).expect("set active");
        let node = app.render(BLOCK3D_BODY_WORLD, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("mesh:"));
    }

    #[test]
    fn add_vortex_kind_then_add_vortex_then_remove_round_trips() {
        let mut app = testkit::new_app::<Block3dPlayApp>();
        app.handle_action("addVortexKind", None, &ViewState::default(), &testkit::meta("local")).expect("add vortex kind");
        app.handle_action("addVortex", None, &ViewState::default(), &testkit::meta("local")).expect("add vortex");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.vortices.len(), 1);
        let vortex_id = projection.vortices[0].id.clone();
        app.handle_action("removeVortex", Some(&json!({ "id": vortex_id })), &ViewState::default(), &testkit::meta("local")).expect("remove vortex");
        assert_eq!(app.projection().expect("projection").vortices.len(), 0);
    }

    #[test]
    fn set_active_example_loads_capsule_fixture() {
        let mut app = testkit::new_app::<Block3dPlayApp>();
        app.handle_action("setActiveExample", Some(&json!({ "id": BLOCK3D_EXAMPLE_CAPSULE })), &ViewState::default(), &testkit::meta("local")).expect("load example");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.object_kind.id, "Capsule J");
        assert_eq!(projection.representations.len(), 2);
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = testkit::new_app::<Block3dPlayApp>();
        app.handle_action("addVortexKind", None, &ViewState::default(), &testkit::meta("local")).expect("add vortex kind");
        assert_eq!(app.projection().expect("projection").vortex_kinds.len(), 1);
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").vortex_kinds.len(), 0);
        app.handle_action("redo", None, &ViewState::default(), &testkit::meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").vortex_kinds.len(), 1);
    }
}
//#endregion 🧪️Tests
