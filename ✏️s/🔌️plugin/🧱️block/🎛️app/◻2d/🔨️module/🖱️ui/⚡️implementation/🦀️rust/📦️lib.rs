//! 🩻️ Block 2D app — DocumentApp impl, render, manifest (constitutional: ui). B1: pure-trait
//! conversion (mirrors `shooting_ui`'s pilot) — `Block2dPlayApp` is a unit struct; the former
//! `selected_ids` `RefCell` field now lives in `block_2d_engine::Block2dConfig`, written via
//! `block_2d_op::Block2dConfigOperation`s (real `backwards`, no ad hoc `InverseAction`); every action
//! dispatches through the single typed `block_2d_protocol::Block2dCommand` channel via
//! `DocumentApp::handle`.

use block_2d::{Block2dDefinition, Block2dHandleKind, Block2dHandleTemplate, BLOCK_2D_SCHEMA};
use block_2d_engine::Block2dConfig;
use block_2d_op::{Block2dConfigOperation, Block2dOperation};
use block_2d_protocol::Block2dCommand;
use block_shared::BlockCompatibilityRule;
use semio_framework_plugin::{
        tree_item_with_action, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionDescriptor, App, AppLabels, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, Label, Locale, LocalizedLabel, Media,
    MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder, SurfaceKind, Terminology, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode,
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
/// 🗂️ The `s/plugin/puzzle` 2d catalog artifact kind block2d's `"catalog:out"` port produces — see
/// `block_2d_engine::block2d_io` and `Block2dPlayApp::export_media`.
const KIT_CATALOG_ARTIFACT_ID: &str = "kit.catalog";
//#endregion 🔖️Constants

//#region 🔖️Locale

//#endregion 🔖️Locale

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the block-2d app; one field per label makes every locale×terminology combination compile-checked.
    struct Block2dLabels {
        window_board: native_en "Node Kind", native_de "Knotenart", reuse_en "Node Kind", reuse_de "Knotenart";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        variant: native_en "Variant", native_de "Variante", reuse_en "Variant", reuse_de "Variante";
        description: native_en "Description", native_de "Beschreibung", reuse_en "Description", reuse_de "Beschreibung";
        handle_kinds: native_en "Handle Kinds", native_de "Griffarten", reuse_en "Handle Kinds", reuse_de "Griffarten";
        handles: native_en "Handles", native_de "Griffe", reuse_en "Handles", reuse_de "Griffe";
        no_handle_kinds: native_en "(no handle kinds)", native_de "(keine Griffarten)", reuse_en "(no handle kinds)", reuse_de "(keine Griffarten)";
        no_handles: native_en "(no handles)", native_de "(keine Griffe)", reuse_en "(no handles)", reuse_de "(keine Griffe)";
        summary: native_en "Node kind", native_de "Knotenart", reuse_en "Node kind", reuse_de "Knotenart";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️DocumentHelpers
fn block2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(BLOCK2D_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Panels
fn build_document_tree(definition: &Block2dDefinition, selected: &[String], labels: &Block2dLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("block2d-play-document");
    let handle_kind_items: Vec<UiTreeItemNode> = definition
        .handle_kinds
        .iter()
        .map(|kind| UiTreeItemNode { icon_id: Some("circle".into()), ..tree_item_with_action(builder.item_id("handle-kind", &kind.id), Label::data(kind.label.clone()), Some(kind.color.clone()), block2d_action("setSelection", None)) })
        .collect();
    let handle_items: Vec<UiTreeItemNode> = definition
        .handles
        .iter()
        .map(|handle| UiTreeItemNode {
            icon_id: Some("circle-dot".into()),
            ..tree_item_with_action(builder.item_id("handle", &handle.id), Label::data(handle.handle_kind.clone()), Some(format!("{:.2}", handle.angle)), block2d_action("setSelection", None))
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

fn text_field(id: &str, label: impl Into<Label>, value: &str, field: &str) -> UiNode {
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
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "block2d-play-inspector".into(),
        label: labels.summary.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            text_field("block2d-play-inspector.name", labels.name, &definition.node_kind.name, "name"),
            text_field("block2d-play-inspector.label", labels.label, &definition.node_kind.label, "label"),
            text_field("block2d-play-inspector.variant", labels.variant, definition.node_kind.variant.as_deref().unwrap_or(""), "variant"),
            text_field("block2d-play-inspector.description", labels.description, &definition.node_kind.description, "description"),
            ui_inspector_readonly_field("block2d-play-inspector.handle-count", labels.handles, definition.handles.len().to_string()),
        ],
    }])
}

fn render_board(definition: &Block2dDefinition, labels: &Block2dLabels) -> UiNode {
    ui_stack_vertical(vec![
        ui_text(Label::data(format!("{}: {}", labels.summary.as_str(), if definition.node_kind.label.is_empty() { "—" } else { &definition.node_kind.label }))),
        ui_text(Label::data(format!("{} {}, {} {}", definition.handle_kinds.len(), labels.handle_kinds.as_str(), definition.handles.len(), labels.handles.as_str()))),
    ])
}
//#endregion 🔖️Panels

//#region 🔖️Block2dPlayApp
/// 🧪️ B1: unit struct — the former `selected_ids` `RefCell` field now lives in
/// `block_2d_engine::Block2dConfig` (see `DocumentApp::Config`), written through
/// `block_2d_op::Block2dConfigOperation`s.
#[derive(Default)]
pub struct Block2dPlayApp;

impl DocumentApp for Block2dPlayApp {
    type Projection = Block2dDefinition;
    type Operation = Block2dOperation;
    type Config = Block2dConfig;
    type ConfigOperation = Block2dConfigOperation;
    type Command = Block2dCommand;

    fn app_id(&self) -> &str {
        BLOCK2D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        BLOCK_2D_SCHEMA
    }

    fn initial_projection(&self) -> Block2dDefinition {
        block_2d_engine::empty_block2d_definition()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(block_2d_engine::block2d_io())
    }

    /// 🏷️ Maps each `Block2dCommand` variant back to the action id it was declared under in
    /// `create_block2d_app` — used for command-log labeling and the registry's View-kind discipline
    /// check.
    fn command_id(&self, command: &Block2dCommand) -> &str {
        match command {
            Block2dCommand::PatchNodeKind { .. } => "patchNodeKind",
            Block2dCommand::AddHandleKind => "addHandleKind",
            Block2dCommand::RemoveHandleKind { .. } => "removeHandleKind",
            Block2dCommand::AddHandle => "addHandle",
            Block2dCommand::RemoveHandle { .. } => "removeHandle",
            Block2dCommand::AddCompatibilityRule { .. } => "addCompatibilityRule",
            Block2dCommand::RemoveCompatibilityRule { .. } => "removeCompatibilityRule",
            Block2dCommand::SetActiveExample { .. } => "setActiveExample",
            Block2dCommand::Edit { .. } => "edit",
            Block2dCommand::SetSelection { .. } => "setSelection",
        }
    }


    fn command_from_action(&self, action: &str, args: Option<&Value>) -> Result<Self::Command, String> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        let str_vec_field = |key: &str| -> Vec<String> {
            args.and_then(|value| value.get(key))
                .and_then(|value| value.as_array())
                .map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect())
                .unwrap_or_default()
        };
        match action {
            "patchNodeKind" => Ok(Block2dCommand::PatchNodeKind { field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() }),
            "addHandleKind" => Ok(Block2dCommand::AddHandleKind),
            "removeHandleKind" => Ok(Block2dCommand::RemoveHandleKind { id: str_field("id").unwrap_or_default() }),
            "addHandle" => Ok(Block2dCommand::AddHandle),
            "removeHandle" => Ok(Block2dCommand::RemoveHandle { id: str_field("id").unwrap_or_default() }),
            "addCompatibilityRule" => Ok(Block2dCommand::AddCompatibilityRule { source: str_field("source").unwrap_or_default(), target: str_field("target").unwrap_or_default() }),
            "removeCompatibilityRule" => Ok(Block2dCommand::RemoveCompatibilityRule { id: str_field("id").unwrap_or_default() }),
            "setActiveExample" => Ok(Block2dCommand::SetActiveExample { id: str_field("exampleId").or_else(|| str_field("id")).unwrap_or_default() }),
            "edit" => Ok(Block2dCommand::Edit { text: str_field("text").unwrap_or_default() }),
            "setSelection" => Ok(Block2dCommand::SetSelection { ids: str_vec_field("ids") }),
            other => Err(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) —                  app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            )),
        }
    }

    fn handle(&self, command: &Block2dCommand, doc: &DocumentView<'_, Block2dDefinition>, _cfg: &ConfigView<'_, Block2dConfig>) -> Emit<Block2dOperation, Block2dConfigOperation> {
        match command {
            Block2dCommand::PatchNodeKind { field, value } => {
                let mut node_kind = doc.projection.node_kind.clone();
                match field.as_str() {
                    "name" => node_kind.name = value.clone(),
                    "label" => node_kind.label = value.clone(),
                    "variant" => node_kind.variant = if value.is_empty() { None } else { Some(value.clone()) },
                    "description" => node_kind.description = value.clone(),
                    _ => return Emit::default(),
                }
                Emit::operations(vec![Block2dOperation::SetNodeKind { node_kind }])
            }
            Block2dCommand::AddHandleKind => {
                let id = block_2d_engine::next_id(doc.projection.handle_kinds.iter().map(|kind| kind.id.as_str()), "handle-kind-");
                let handle_kind = Block2dHandleKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_wire_kind: "cable.link".into() };
                Emit::operations(vec![Block2dOperation::SetHandleKind { index: doc.projection.handle_kinds.len(), handle_kind }])
            }
            Block2dCommand::RemoveHandleKind { id } => Emit::operations(vec![Block2dOperation::RemoveHandleKind { id: id.clone() }]),
            Block2dCommand::AddHandle => {
                let Some(handle_kind_id) = doc.projection.handle_kinds.first().map(|kind| kind.id.clone()) else { return Emit::default() };
                let id = block_2d_engine::next_id(doc.projection.handles.iter().map(|handle| handle.id.as_str()), "handle-");
                let handle = Block2dHandleTemplate { id, handle_kind: handle_kind_id, angle: 0.0, radius: 0.36 };
                Emit::operations(vec![Block2dOperation::SetHandle { index: doc.projection.handles.len(), handle }])
            }
            Block2dCommand::RemoveHandle { id } => Emit::operations(vec![Block2dOperation::RemoveHandle { id: id.clone() }]),
            Block2dCommand::AddCompatibilityRule { source, target } => {
                if source.is_empty() || target.is_empty() {
                    return Emit::default();
                }
                let id = block_2d_engine::next_id(doc.projection.compatibility.iter().map(|rule| rule.id.as_str()), "compat-");
                let rule = BlockCompatibilityRule { id, source: source.clone(), target: target.clone(), bidirectional: true };
                Emit::operations(vec![Block2dOperation::SetCompatibilityRule { index: doc.projection.compatibility.len(), rule }])
            }
            Block2dCommand::RemoveCompatibilityRule { id } => Emit::operations(vec![Block2dOperation::RemoveCompatibilityRule { id: id.clone() }]),
            Block2dCommand::SetActiveExample { id } => {
                let example = match id.as_str() {
                    BLOCK2D_EXAMPLE_LEFT => block_2d_dsl::parse_dsl(block_2d_dsl::BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
                    BLOCK2D_EXAMPLE_RIGHT => block_2d_dsl::parse_dsl(block_2d_dsl::BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT).ok(),
                    _ => None,
                };
                match example {
                    Some(document) => Emit::operations(vec![Block2dOperation::SetDocument { document }]),
                    None => Emit::default(),
                }
            }
            Block2dCommand::Edit { text } => match serde_json::from_str::<Block2dDefinition>(text) {
                Ok(document) if &document != doc.projection => Emit::operations(vec![Block2dOperation::SetDocument { document }]),
                _ => Emit::default(),
            },
            Block2dCommand::SetSelection { ids } => Emit::config(vec![Block2dConfigOperation::SetSelection { ids: ids.clone() }]),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Block2dDefinition>, cfg: &ConfigView<'_, Block2dConfig>) -> UiNode {
        let labels = semio_framework_plugin::resolve_labels_for_locale::<Block2dLabels>(&cfg.projection.locale);
        match body_key {
            BLOCK2D_BODY_BOARD => render_board(doc.projection, labels),
            BLOCK2D_BODY_DOCUMENT | BLOCK2D_BODY_KINDS => build_document_tree(doc.projection, &cfg.projection.selected_ids, labels),
            BLOCK2D_BODY_INSPECTOR => build_inspection_tree(doc.projection, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🌉️ `puzzle2d_manifest_fragment`'s first real caller — wraps the block-2d document's
    /// puzzle2d-shaped catalog fragment (`portKinds`/`wireKinds`/`edgeKinds`/`nodeKinds`/
    /// `kindCompatibility`) as a `kit.catalog`-schema `Media` value for the `"catalog:out"` port
    /// declared in `block_2d_engine::block2d_io`. Falls through to the default whole-document pack
    /// export for every other port (`"document:out"`).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Block2dDefinition>) -> Result<Media, MediaError> {
        if port != "catalog:out" {
            // 🌉️ Reimplements `DocumentApp::export_media`'s default `"document:out"` behavior
            // verbatim — overriding the trait method forfeits the ability to delegate back to its
            // own default body, so the whole-document pack export is duplicated here rather than
            // left unreachable for this app.
            if port != "document:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Kit, form: MediaForm::Type });
            let bytes = store::DocumentPack::encode_pack(doc.projection);
            return Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } });
        }
        let fragment = block_2d_engine::puzzle2d_manifest_fragment(doc.projection);
        Ok(Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: MediaPayload::Structured { schema: KIT_CATALOG_ARTIFACT_ID.into(), json: fragment.to_string() } })
    }
}
//#endregion 🔖️Block2dPlayApp

//#region 🔖️Manifest
pub fn create_block2d_app() -> App {
    App::from_builder(
        App::builder(BLOCK2D_PLAY_APP_ID, LocalizedLabel::native("Block 2D", "Block 2D"))
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
            // 🗂️ The puzzle2d catalog artifact this app's new `"catalog:out"` port produces — see
            // `block_2d_engine::block2d_io`/`Block2dPlayApp::export_media`.
            .artifact_kind(ArtifactKindSpec {
                id: KIT_CATALOG_ARTIFACT_ID.into(),
                name: "Kit Catalog".into(),
                source_format: KIT_CATALOG_ARTIFACT_ID.into(),
                component_kind: "kit-catalog".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: KIT_CATALOG_ARTIFACT_ID.into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("layout-grid")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind(BLOCK2D_WINDOW_BOARD, LocalizedLabel::native("Node Kind", "Knotenart"), BLOCK2D_BODY_BOARD, SurfaceKind::Board2d, "layout-grid")
            .panel_tab("framework.panel.document", LocalizedLabel::native("Document", "Dokument"), PanelGroup::Workbench, BLOCK2D_BODY_DOCUMENT)
            .panel_tab("framework.panel.inspection", LocalizedLabel::native("Inspection", "Inspektion"), PanelGroup::Details, BLOCK2D_BODY_INSPECTOR)
            .operation("patchNodeKind", LocalizedLabel::native("Patch Node Kind", "Knotenart bearbeiten"))
            .operation("addHandleKind", LocalizedLabel::native("Add Handle Kind", "Griffart hinzufügen"))
            .operation("removeHandleKind", LocalizedLabel::native("Remove Handle Kind", "Griffart entfernen"))
            .operation("addHandle", LocalizedLabel::native("Add Handle", "Griff hinzufügen"))
            .operation("removeHandle", LocalizedLabel::native("Remove Handle", "Griff entfernen"))
            .operation("addCompatibilityRule", LocalizedLabel::native("Add Compatibility Rule", "Kompatibilitätsregel hinzufügen"))
            .operation("removeCompatibilityRule", LocalizedLabel::native("Remove Compatibility Rule", "Kompatibilitätsregel entfernen"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .operation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .io(block_2d_engine::block2d_io()),
    )
    .example(
        BLOCK2D_EXAMPLE_LEFT,
        LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Left"),
        serde_json::to_string(&block_2d_dsl::parse_dsl(block_2d_dsl::BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(),
        "list-tree",
    )
    .example(
        BLOCK2D_EXAMPLE_RIGHT,
        LocalizedLabel::native("Hexagonal Cut Concrete Forest Right", "Hexagonal Cut Concrete Forest Right"),
        serde_json::to_string(&block_2d_dsl::parse_dsl(block_2d_dsl::BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(),
        "list-tree",
    )
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
    use semio_framework_plugin::{testkit, PluginApp, ViewState};

    fn new_app() -> semio_framework_plugin::VcsDocumentApp<Block2dPlayApp> {
        testkit::new_app::<Block2dPlayApp>()
    }

    #[test]
    fn renders_document_tree_and_inspector() {
        let mut app = new_app();
        let node = app.render(BLOCK2D_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Handle Kinds"));
        let inspector = app.render(BLOCK2D_BODY_INSPECTOR, None, &ViewState::default()).expect("render");
        let inspector_json = serde_json::to_string(&inspector).unwrap();
        assert!(inspector_json.contains("\"type\":\"tree\""), "inspection body must be a tree like document");
        assert!(inspector_json.contains("Name"));
        assert!(!inspector_json.contains("\"type\":\"stack\""), "inspection body must not be a free-form stack");
    }

    #[test]
    fn add_handle_kind_then_add_handle_then_remove_round_trips() {
        let mut app = new_app();
        app.dispatch_typed(Block2dCommand::AddHandleKind, &testkit::meta("local")).expect("add handle kind");
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 1);
        app.dispatch_typed(Block2dCommand::AddHandle, &testkit::meta("local")).expect("add handle");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.handles.len(), 1);
        let handle_id = projection.handles[0].id.clone();
        app.dispatch_typed(Block2dCommand::RemoveHandle { id: handle_id }, &testkit::meta("local")).expect("remove handle");
        assert_eq!(app.projection().expect("projection").handles.len(), 0);
    }

    #[test]
    fn patch_node_kind_updates_name() {
        let mut app = new_app();
        app.dispatch_typed(Block2dCommand::PatchNodeKind { field: "name".into(), value: "Renamed".into() }, &testkit::meta("local")).expect("patch");
        assert_eq!(app.projection().expect("projection").node_kind.name, "Renamed");
    }

    #[test]
    fn set_active_example_loads_left_fixture() {
        let mut app = new_app();
        app.dispatch_typed(Block2dCommand::SetActiveExample { id: BLOCK2D_EXAMPLE_LEFT.into() }, &testkit::meta("local")).expect("load example");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.node_kind.id, "Hexagonal Cut Concrete Forest Left");
        assert_eq!(projection.handles.len(), 11);
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = new_app();
        app.dispatch_typed(Block2dCommand::AddHandleKind, &testkit::meta("local")).expect("add handle kind");
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 1);
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 0);
        app.handle_action("redo", None, &testkit::meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 1);
    }

    #[test]
    fn set_selection_writes_config_not_document() {
        let mut app = new_app();
        let result = app.dispatch_typed(Block2dCommand::SetSelection { ids: vec!["handle-kind:b-l".into()] }, &testkit::meta("local")).expect("select");
        assert!(result.operations.is_empty(), "setSelection is config-only and must emit no document operations");
    }

    /// 🌉️ `puzzle2d_manifest_fragment`'s new caller round-trips through the `"catalog:out"` media port.
    #[test]
    fn export_media_catalog_out_wraps_the_puzzle2d_fragment() {
        let mut app = new_app();
        app.dispatch_typed(Block2dCommand::SetActiveExample { id: BLOCK2D_EXAMPLE_LEFT.into() }, &testkit::meta("local")).expect("load example");
        let media = app.export_media("catalog:out").expect("export catalog");
        assert_eq!(media.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "kit.catalog");
                let value: Value = serde_json::from_str(&json).expect("valid json");
                assert_eq!(value["nodeKinds"][0]["id"], "Hexagonal Cut Concrete Forest Left");
            }
            other => panic!("expected Structured payload, got {other:?}"),
        }
    }

    #[test]
    fn block2d_io_is_wired_into_the_manifest() {
        let definition = create_block2d_app().definition;
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
    }
}

    #[test]
    fn command_from_action_bridges_set_active_example() {
        let app = Block2dPlayApp;
        assert!(matches!(app.command_from_action("setActiveExample", Some(&serde_json::json!({ "exampleId": "left" }))), Ok(Block2dCommand::SetActiveExample { id }) if id == "left"));
    }
//#endregion 🧪️Tests
