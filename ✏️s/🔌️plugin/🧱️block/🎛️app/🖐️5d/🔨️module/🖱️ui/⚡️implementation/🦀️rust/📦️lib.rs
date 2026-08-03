//! 👯️ Block 5D app — DocumentApp impl, render, manifest (constitutional: ui). B1: pure-trait
//! conversion (mirrors `shooting_ui`'s pilot) — `Block5dPlayApp` is a unit struct; the former
//! `selected_ids` `RefCell` field now lives in `block_5d_engine::Block5dConfig`, written via
//! `block_5d_op::Block5dConfigOperation`s (real `backwards`, no ad hoc `InverseAction`); every action
//! dispatches through the single typed `block_5d_protocol::Block5dCommand` channel via
//! `DocumentApp::handle`.

use block_5d::{Block5dDefinition, Block5dGripKind, Block5dGripTemplate, BLOCK_5D_SCHEMA};
use block_5d_engine::Block5dConfig;
use block_5d_op::{Block5dConfigOperation, Block5dOperation};
use block_5d_protocol::Block5dCommand;
use semio_framework_plugin::{
    create_default_layout, tree_item_with_action, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionDescriptor,
    App, AppLabels, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, Label, Locale, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder,
    SurfaceKind, Terminology, UiFieldNode, UiInspectorFieldGroup, UiInputNode, UiNode, UiPresence, UiTreeItemNode,
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
/// 🗂️ The `s/plugin/puzzle` 5d catalog artifact kind block5d's `"catalog:out"` port produces — see
/// `block_5d_engine::block5d_io` and `Block5dPlayApp::export_media`.
const KIT_CATALOG_ARTIFACT_ID: &str = "kit.catalog";
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterpart to the deleted `ViewState`-driven
/// `semio_framework_plugin::resolve_labels` — `Block5dConfig` carries no terminology axis, so this
/// app is always `Terminology::Native`. `cfg.locale` is a BCP-47 tag, lenient-parsed the same way
/// `detectShellLocale` does on the TS side — see `home_ui`'s identical pair.
fn block5d_locale(cfg: &Block5dConfig) -> Locale {
    if cfg.locale.starts_with("de") { Locale::De } else { Locale::En }
}

fn resolve_labels<L: AppLabels>(cfg: &Block5dConfig) -> &'static L {
    L::labels(block5d_locale(cfg), Terminology::Native)
}
//#endregion 🔖️Locale

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the block-5d app; one field per label makes every locale×terminology combination compile-checked.
    struct Block5dLabels {
        window_board: native_en "Board", native_de "Board", reuse_en "Board", reuse_de "Board";
        window_world: native_en "World", native_de "Welt", reuse_en "World", reuse_de "Welt";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        grip_kinds: native_en "Grip Kinds", native_de "Griffarten", reuse_en "Grip Kinds", reuse_de "Griffarten";
        grips: native_en "Grips", native_de "Griffe", reuse_en "Grips", reuse_de "Griffe";
        no_grip_kinds: native_en "(no grip kinds)", native_de "(keine Griffarten)", reuse_en "(no grip kinds)", reuse_de "(keine Griffarten)";
        no_grips: native_en "(no grips)", native_de "(keine Griffe)", reuse_en "(no grips)", reuse_de "(keine Griffe)";
        summary: native_en "Part kind", native_de "Teilart", reuse_en "Part kind", reuse_de "Teilart";
    }
}
//#endregion 🔖️Terminology

fn block5d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: BLOCK5D_PLAY_APP_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

//#region 🔖️Panels
fn build_document_tree(definition: &Block5dDefinition, selected: &[String], labels: &Block5dLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("block5d-play-document");
    let grip_kind_items: Vec<UiTreeItemNode> = definition
        .grip_kinds
        .iter()
        .map(|kind| UiTreeItemNode { icon_id: Some("circle".into()), menu: None,
            ..tree_item_with_action(builder.item_id("grip-kind", &kind.id), Label::data(kind.label.clone()), Some(kind.color.clone()), block5d_action("setSelection", None))
        })
        .collect();
    let grip_items: Vec<UiTreeItemNode> = definition
        .grips
        .iter()
        .map(|grip| UiTreeItemNode { icon_id: Some("circle-dot".into()), menu: None,
            ..tree_item_with_action(builder.item_id("grip", &grip.id), Label::data(grip.grip_kind.clone()), Some(format!("{:.2}", grip.angle)), block5d_action("setSelection", None))
        })
        .collect();
    builder
        .section_or_placeholder("block5d-play-document.grip-kinds", Some(labels.grip_kinds.into()), true, grip_kind_items, labels.no_grip_kinds)
        .section_or_placeholder("block5d-play-document.grips", Some(labels.grips.into()), true, grip_items, labels.no_grips)
        .selected(selected.to_vec())
        .selection_change(block5d_action("setSelection", None))
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
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "block5d-play-inspector".into(),
        label: labels.summary.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            text_field("block5d-play-inspector.name", labels.name, &definition.part_kind.name, "name"),
            text_field("block5d-play-inspector.label", labels.label, &definition.part_kind.label, "label"),
            ui_inspector_readonly_field("block5d-play-inspector.grip-count", labels.grips, definition.grips.len().to_string()),
        ],
    }])
}

fn render_board(definition: &Block5dDefinition, labels: &Block5dLabels) -> UiNode {
    ui_stack_vertical(vec![
        ui_text(Label::data(format!("{}: {}", labels.summary.as_str(), if definition.part_kind.label.is_empty() { "—" } else { &definition.part_kind.label }))),
        ui_text(Label::data(format!("2d grips: {}", definition.grips.len()))),
    ])
}

fn render_world(definition: &Block5dDefinition, labels: &Block5dLabels) -> UiNode {
    let mesh_url = definition.representations.first().and_then(|representation| representation.mesh_url.as_deref()).unwrap_or("—");
    ui_stack_vertical(vec![ui_text(Label::data(format!("{}: {}", labels.summary.as_str(), if definition.part_kind.label.is_empty() { "—" } else { &definition.part_kind.label }))), ui_text(Label::data(format!("mesh: {mesh_url}")))])
}
//#endregion 🔖️Panels

//#region 🔖️Block5dPlayApp
/// 🧪️ B1: unit struct — the former `selected_ids` `RefCell` field now lives in
/// `block_5d_engine::Block5dConfig` (see `DocumentApp::Config`), written through
/// `block_5d_op::Block5dConfigOperation`s.
#[derive(Default)]
pub struct Block5dPlayApp;

impl DocumentApp for Block5dPlayApp {
    type Projection = Block5dDefinition;
    type Operation = Block5dOperation;
    type Config = Block5dConfig;
    type ConfigOperation = Block5dConfigOperation;
    type Command = Block5dCommand;

    fn app_id(&self) -> &str {
        BLOCK5D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        BLOCK_5D_SCHEMA
    }

    fn initial_projection(&self) -> Block5dDefinition {
        block_5d_engine::empty_block5d_definition()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(block_5d_engine::block5d_io())
    }

    /// 🏷️ Maps each `Block5dCommand` variant back to the action id it was declared under in
    /// `create_block5d_app` — used for command-log labeling and the registry's View-kind discipline
    /// check.
    fn command_id(&self, command: &Block5dCommand) -> &str {
        match command {
            Block5dCommand::PatchPartKind { .. } => "patchPartKind",
            Block5dCommand::AddGripKind => "addGripKind",
            Block5dCommand::RemoveGripKind { .. } => "removeGripKind",
            Block5dCommand::AddGrip => "addGrip",
            Block5dCommand::RemoveGrip { .. } => "removeGrip",
            Block5dCommand::SetActiveExample { .. } => "setActiveExample",
            Block5dCommand::Edit { .. } => "edit",
            Block5dCommand::SetSelection { .. } => "setSelection",
        }
    }

    fn handle(
        &self,
        command: &Block5dCommand,
        doc: &DocumentView<'_, Block5dDefinition>,
        _cfg: &ConfigView<'_, Block5dConfig>,
    ) -> Emit<Block5dOperation, Block5dConfigOperation> {
        match command {
            Block5dCommand::PatchPartKind { field, value } => {
                let mut part_kind = doc.projection.part_kind.clone();
                match field.as_str() {
                    "name" => part_kind.name = value.clone(),
                    "label" => part_kind.label = value.clone(),
                    "variant" => part_kind.variant = if value.is_empty() { None } else { Some(value.clone()) },
                    "description" => part_kind.description = value.clone(),
                    _ => return Emit::default(),
                }
                Emit::operations(vec![Block5dOperation::SetPartKind { part_kind }])
            }
            Block5dCommand::AddGripKind => {
                let id = block_5d_engine::next_id(doc.projection.grip_kinds.iter().map(|kind| kind.id.as_str()), "grip-kind-");
                let grip_kind = Block5dGripKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_rope_kind: "rope.link".into() };
                Emit::operations(vec![Block5dOperation::SetGripKind { index: doc.projection.grip_kinds.len(), grip_kind }])
            }
            Block5dCommand::RemoveGripKind { id } => Emit::operations(vec![Block5dOperation::RemoveGripKind { id: id.clone() }]),
            Block5dCommand::AddGrip => {
                let Some(grip_kind_id) = doc.projection.grip_kinds.first().map(|kind| kind.id.clone()) else { return Emit::default() };
                let id = block_5d_engine::next_id(doc.projection.grips.iter().map(|grip| grip.id.as_str()), "grip-");
                let grip = Block5dGripTemplate { id, grip_kind: grip_kind_id, angle: 0.0, radius_2d: 0.36, position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 };
                Emit::operations(vec![Block5dOperation::SetGrip { index: doc.projection.grips.len(), grip }])
            }
            Block5dCommand::RemoveGrip { id } => Emit::operations(vec![Block5dOperation::RemoveGrip { id: id.clone() }]),
            Block5dCommand::SetActiveExample { id } => {
                let example = match id.as_str() {
                    BLOCK5D_EXAMPLE_FOREST_LEFT => block_5d_dsl::parse_dsl(block_5d_dsl::BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
                    BLOCK5D_EXAMPLE_CAPSULE => block_5d_dsl::parse_dsl(block_5d_dsl::BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).ok(),
                    _ => None,
                };
                match example {
                    Some(document) => Emit::operations(vec![Block5dOperation::SetDocument { document }]),
                    None => Emit::default(),
                }
            }
            Block5dCommand::Edit { text } => match serde_json::from_str::<Block5dDefinition>(text) {
                Ok(document) if &document != doc.projection => Emit::operations(vec![Block5dOperation::SetDocument { document }]),
                _ => Emit::default(),
            },
            Block5dCommand::SetSelection { ids } => Emit::config(vec![Block5dConfigOperation::SetSelection { ids: ids.clone() }]),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Block5dDefinition>, cfg: &ConfigView<'_, Block5dConfig>) -> UiNode {
        let labels = resolve_labels::<Block5dLabels>(cfg.projection);
        match body_key {
            BLOCK5D_BODY_BOARD => render_board(doc.projection, labels),
            BLOCK5D_BODY_WORLD => render_world(doc.projection, labels),
            BLOCK5D_BODY_DOCUMENT => build_document_tree(doc.projection, &cfg.projection.selected_ids, labels),
            BLOCK5D_BODY_INSPECTOR => build_inspection_tree(doc.projection, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🌉️ `puzzle5d_catalog_fragment`'s first real caller — wraps the block-5d document's
    /// puzzle5d-shaped catalog fragment (`parts`/`grips`/`fasteners`/`ropes`/`kindCompatibility`) as
    /// a `kit.catalog`-schema `Media` value for the `"catalog:out"` port declared in
    /// `block_5d_engine::block5d_io`. Falls through to the default whole-document pack export for
    /// every other port (`"document:out"`).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Block5dDefinition>) -> Result<Media, MediaError> {
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
        let fragment = block_5d_engine::puzzle5d_catalog_fragment(doc.projection);
        Ok(Media {
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            payload: MediaPayload::Structured { schema: KIT_CATALOG_ARTIFACT_ID.into(), json: fragment.to_string() },
        })
    }
}
//#endregion 🔖️Block5dPlayApp

//#region 🔖️Manifest
pub fn create_block5d_app() -> App {
    App::from_builder(
        App::builder(BLOCK5D_PLAY_APP_ID, LocalizedLabel::native("Block 5D", "Block 5D"))
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
            // 🗂️ The puzzle5d catalog artifact this app's new `"catalog:out"` port produces — see
            // `block_5d_engine::block5d_io`/`Block5dPlayApp::export_media`.
            .artifact_kind(ArtifactKindSpec {
                id: KIT_CATALOG_ARTIFACT_ID.into(),
                name: "Kit Catalog".into(),
                source_format: KIT_CATALOG_ARTIFACT_ID.into(),
                component_kind: "kit-catalog".into(),
                dimension: "5d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: KIT_CATALOG_ARTIFACT_ID.into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("layers")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind(BLOCK5D_WINDOW_BOARD, LocalizedLabel::native("Board", "Board"), BLOCK5D_BODY_BOARD, SurfaceKind::Board2d, "layout-grid")
            .window_kind(BLOCK5D_WINDOW_WORLD, LocalizedLabel::native("World", "Welt"), BLOCK5D_BODY_WORLD, SurfaceKind::World3d, "box")
            .panel_tab("framework.panel.document", LocalizedLabel::native("Document", "Dokument"), PanelGroup::Workbench, BLOCK5D_BODY_DOCUMENT)
            .panel_tab("framework.panel.inspection", LocalizedLabel::native("Inspection", "Inspektion"), PanelGroup::Details, BLOCK5D_BODY_INSPECTOR)
            .operation("patchPartKind", LocalizedLabel::native("Patch Part Kind", "Teilart bearbeiten"))
            .operation("addGripKind", LocalizedLabel::native("Add Grip Kind", "Griffart hinzufügen"))
            .operation("removeGripKind", LocalizedLabel::native("Remove Grip Kind", "Griffart entfernen"))
            .operation("addGrip", LocalizedLabel::native("Add Grip", "Griff hinzufügen"))
            .operation("removeGrip", LocalizedLabel::native("Remove Grip", "Griff entfernen"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .operation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .default_layout(create_default_layout(&[BLOCK5D_WINDOW_BOARD.into(), BLOCK5D_WINDOW_WORLD.into()], "row", Some(&[50.0, 50.0]), Some(&["Board".into(), "World".into()])))
            .io(block_5d_engine::block5d_io()),
    )
    .example(BLOCK5D_EXAMPLE_FOREST_LEFT, LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Left"), serde_json::to_string(&block_5d_dsl::parse_dsl(block_5d_dsl::BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(), "list-tree")
    .example(BLOCK5D_EXAMPLE_CAPSULE, LocalizedLabel::native("Nakagin Capsule", "Nakagin Capsule"), serde_json::to_string(&block_5d_dsl::parse_dsl(block_5d_dsl::BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(), "building")
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
    use semio_framework_plugin::{testkit, PluginApp, ViewState};

    fn new_app() -> semio_framework_plugin::VcsDocumentApp<Block5dPlayApp> {
        testkit::new_app::<Block5dPlayApp>()
    }

    #[test]
    fn renders_document_tree_board_and_world() {
        let mut app = new_app();
        let node = app.render(BLOCK5D_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("Grip Kinds"));
        let board = app.render(BLOCK5D_BODY_BOARD, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&board).unwrap().contains("2d grips"));
        let world = app.render(BLOCK5D_BODY_WORLD, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&world).unwrap().contains("mesh:"));
    }

    #[test]
    fn add_grip_kind_then_add_grip_then_remove_round_trips() {
        let mut app = new_app();
        app.dispatch_typed(Block5dCommand::AddGripKind, &testkit::meta("local")).expect("add grip kind");
        app.dispatch_typed(Block5dCommand::AddGrip, &testkit::meta("local")).expect("add grip");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.grips.len(), 1);
        let grip_id = projection.grips[0].id.clone();
        app.dispatch_typed(Block5dCommand::RemoveGrip { id: grip_id }, &testkit::meta("local")).expect("remove grip");
        assert_eq!(app.projection().expect("projection").grips.len(), 0);
    }

    #[test]
    fn set_active_example_loads_forest_left_fixture() {
        let mut app = new_app();
        app.dispatch_typed(Block5dCommand::SetActiveExample { id: BLOCK5D_EXAMPLE_FOREST_LEFT.into() }, &testkit::meta("local")).expect("load example");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.part_kind.id, "Hexagonal Cut Concrete Forest Left");
        assert_eq!(projection.grips.len(), 1);
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = new_app();
        app.dispatch_typed(Block5dCommand::AddGripKind, &testkit::meta("local")).expect("add grip kind");
        assert_eq!(app.projection().expect("projection").grip_kinds.len(), 1);
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").grip_kinds.len(), 0);
        app.handle_action("redo", None, &testkit::meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").grip_kinds.len(), 1);
    }

    #[test]
    fn set_selection_writes_config_not_document() {
        let mut app = new_app();
        let result = app.dispatch_typed(Block5dCommand::SetSelection { ids: vec!["grip-kind:b-l".into()] }, &testkit::meta("local")).expect("select");
        assert!(result.operations.is_empty(), "setSelection is config-only and must emit no document operations");
    }

    /// 🌉️ `puzzle5d_catalog_fragment`'s new caller round-trips through the `"catalog:out"` media port.
    #[test]
    fn export_media_catalog_out_wraps_the_puzzle5d_fragment() {
        let mut app = new_app();
        app.dispatch_typed(Block5dCommand::SetActiveExample { id: BLOCK5D_EXAMPLE_FOREST_LEFT.into() }, &testkit::meta("local")).expect("load example");
        let media = app.export_media("catalog:out").expect("export catalog");
        assert_eq!(media.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "kit.catalog");
                let value: Value = serde_json::from_str(&json).expect("valid json");
                assert_eq!(value["parts"][0]["id"], "Hexagonal Cut Concrete Forest Left");
            }
            other => panic!("expected Structured payload, got {other:?}"),
        }
    }

    #[test]
    fn block5d_io_is_wired_into_the_manifest() {
        let definition = create_block5d_app().definition;
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
    }
}
//#endregion 🧪️Tests
