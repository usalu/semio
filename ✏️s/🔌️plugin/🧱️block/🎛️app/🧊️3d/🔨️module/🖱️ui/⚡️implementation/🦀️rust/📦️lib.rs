//! 🏙️ Block 3D app — DocumentApp impl, render, manifest (constitutional: ui). B1: pure-trait
//! conversion (mirrors `shooting_ui`'s pilot) — `Block3dPlayApp` is a unit struct; every former
//! `RefCell` runtime field (`selected_ids`/`active_representation_id`) now lives in
//! `block_3d_engine::Block3dConfig`, written via `block_3d_op::Block3dConfigOperation`s (real
//! `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `block_3d_protocol::Block3dCommand` channel via `DocumentApp::handle`.

use block_3d::{Block3dDefinition, Block3dVortexKind, Block3dVortexTemplate, BLOCK_3D_SCHEMA};
use block_3d_engine::Block3dConfig;
use block_3d_op::{Block3dConfigOperation, Block3dOperation};
use block_3d_protocol::Block3dCommand;
use block_shared::BlockRepresentation;
use semio_framework_plugin::{
    localized_label_map, tree_item_with_action, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionDescriptor, App,
    AppLabelsOverlay, AppLabelsOverlayExt, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, LocaleLabels, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder, SurfaceKind, UiFieldNode, UiInspectorFieldGroup,
    UiInputNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiTreeItemNode,
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
/// 🗂️ The `s/plugin/puzzle` 3d catalog artifact kind block3d's `"catalog:out"` port produces — see
/// `block_3d_engine::block3d_io` and `Block3dPlayApp::export_media`.
const KIT_CATALOG_ARTIFACT_ID: &str = "kit.catalog";
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — mirrors `shooting_ui`'s identical region.
fn is_de_locale(cfg: &Block3dConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &Block3dConfig) -> &'static L {
    if is_de_locale(cfg) { L::locale_labels_de() } else { L::locale_labels_en() }
}
//#endregion 🔖️Locale

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
    ActionDescriptor { controller_id: BLOCK3D_PLAY_APP_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
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
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        menu: None,
    })
}

fn build_inspection_tree(definition: &Block3dDefinition, active_representation_id: Option<&str>, labels: &Block3dLabels) -> UiNode {
    let representation_select = UiNode::Select(UiSelectNode {
        id: "block3d-play-inspector.representation".into(),
        value: active_representation_id.unwrap_or_default().into(),
        items: definition.representations.iter().map(|representation| UiSelectItem { value: representation.id.clone(), label: representation.name.clone(),
        }).collect(),
        placeholder: None,
        on_change: block3d_action("setActiveRepresentation", None),
        presence: UiPresence::default(),
        menu: None,
    });
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "block3d-play-inspector".into(),
        label: labels.summary.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            text_field("block3d-play-inspector.name", labels.name, &definition.object_kind.name, "name"),
            text_field("block3d-play-inspector.label", labels.label, &definition.object_kind.label, "label"),
            UiNode::Field(UiFieldNode { presence: UiPresence::default(), id: "block3d-play-inspector.representation-field".into(), label: labels.representation.into(), child: Box::new(representation_select), description: None, required: None, error: None,
                menu: None,
            }),
            ui_inspector_readonly_field("block3d-play-inspector.vortex-count", labels.vortices, definition.vortices.len().to_string()),
        ],
    }])
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
/// 🧪️ B1: unit struct — every former `RefCell` field now lives in `block_3d_engine::Block3dConfig`
/// (see `DocumentApp::Config`), written through `block_3d_op::Block3dConfigOperation`s.
#[derive(Default)]
pub struct Block3dPlayApp;

impl DocumentApp for Block3dPlayApp {
    type Projection = Block3dDefinition;
    type Operation = Block3dOperation;
    type Config = Block3dConfig;
    type ConfigOperation = Block3dConfigOperation;
    type Command = Block3dCommand;

    fn app_id(&self) -> &str {
        BLOCK3D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        BLOCK_3D_SCHEMA
    }

    fn initial_projection(&self) -> Block3dDefinition {
        block_3d_engine::empty_block3d_definition()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(block_3d_engine::block3d_io())
    }

    /// 🏷️ Maps each `Block3dCommand` variant back to the action id it was declared under in
    /// `create_block3d_app` — used for command-log labeling and the registry's View-kind discipline
    /// check.
    fn command_id(&self, command: &Block3dCommand) -> &str {
        match command {
            Block3dCommand::PatchObjectKind { .. } => "patchObjectKind",
            Block3dCommand::AddRepresentation => "addRepresentation",
            Block3dCommand::RemoveRepresentation { .. } => "removeRepresentation",
            Block3dCommand::AddVortexKind => "addVortexKind",
            Block3dCommand::RemoveVortexKind { .. } => "removeVortexKind",
            Block3dCommand::AddVortex => "addVortex",
            Block3dCommand::RemoveVortex { .. } => "removeVortex",
            Block3dCommand::SetActiveExample { .. } => "setActiveExample",
            Block3dCommand::Edit { .. } => "edit",
            Block3dCommand::SetSelection { .. } => "setSelection",
            Block3dCommand::SetActiveRepresentation { .. } => "setActiveRepresentation",
        }
    }

    fn handle(
        &self,
        command: &Block3dCommand,
        doc: &DocumentView<'_, Block3dDefinition>,
        _cfg: &ConfigView<'_, Block3dConfig>,
    ) -> Emit<Block3dOperation, Block3dConfigOperation> {
        match command {
            Block3dCommand::PatchObjectKind { field, value } => {
                let mut object_kind = doc.projection.object_kind.clone();
                match field.as_str() {
                    "name" => object_kind.name = value.clone(),
                    "label" => object_kind.label = value.clone(),
                    "variant" => object_kind.variant = if value.is_empty() { None } else { Some(value.clone()) },
                    "description" => object_kind.description = value.clone(),
                    _ => return Emit::default(),
                }
                Emit::operations(vec![Block3dOperation::SetObjectKind { object_kind }])
            }
            Block3dCommand::AddRepresentation => {
                let id = block_3d_engine::next_id(doc.projection.representations.iter().map(|representation| representation.id.as_str()), "representation-");
                let representation = BlockRepresentation { id: id.clone(), name: id, mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() };
                Emit::operations(vec![Block3dOperation::SetRepresentation { index: doc.projection.representations.len(), representation }])
            }
            Block3dCommand::RemoveRepresentation { id } => Emit::operations(vec![Block3dOperation::RemoveRepresentation { id: id.clone() }]),
            Block3dCommand::AddVortexKind => {
                let id = block_3d_engine::next_id(doc.projection.vortex_kinds.iter().map(|kind| kind.id.as_str()), "vortex-kind-");
                let vortex_kind = Block3dVortexKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_cable_kind: "cable.link".into() };
                Emit::operations(vec![Block3dOperation::SetVortexKind { index: doc.projection.vortex_kinds.len(), vortex_kind }])
            }
            Block3dCommand::RemoveVortexKind { id } => Emit::operations(vec![Block3dOperation::RemoveVortexKind { id: id.clone() }]),
            Block3dCommand::AddVortex => {
                let Some(vortex_kind_id) = doc.projection.vortex_kinds.first().map(|kind| kind.id.clone()) else { return Emit::default() };
                let id = block_3d_engine::next_id(doc.projection.vortices.iter().map(|vortex| vortex.id.as_str()), "vortex-");
                let vortex = Block3dVortexTemplate { id, vortex_kind: vortex_kind_id, position: [0.0, 0.0, 0.0], direction: [0.0, 0.0, 1.0], radius: 0.3, label: None };
                Emit::operations(vec![Block3dOperation::SetVortex { index: doc.projection.vortices.len(), vortex }])
            }
            Block3dCommand::RemoveVortex { id } => Emit::operations(vec![Block3dOperation::RemoveVortex { id: id.clone() }]),
            Block3dCommand::SetActiveExample { id } => {
                let example = match id.as_str() {
                    BLOCK3D_EXAMPLE_CAPSULE => block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).ok(),
                    BLOCK3D_EXAMPLE_FOREST_LEFT => block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
                    _ => None,
                };
                match example {
                    Some(document) => Emit::operations(vec![Block3dOperation::SetDocument { document }]),
                    None => Emit::default(),
                }
            }
            Block3dCommand::Edit { text } => match serde_json::from_str::<Block3dDefinition>(text) {
                Ok(document) if &document != doc.projection => Emit::operations(vec![Block3dOperation::SetDocument { document }]),
                _ => Emit::default(),
            },
            Block3dCommand::SetSelection { ids } => Emit::config(vec![Block3dConfigOperation::SetSelection { ids: ids.clone() }]),
            Block3dCommand::SetActiveRepresentation { representation_id } => Emit::config(vec![Block3dConfigOperation::SetActiveRepresentation { representation_id: representation_id.clone() }]),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Block3dDefinition>, cfg: &ConfigView<'_, Block3dConfig>) -> UiNode {
        let labels = resolve_labels::<Block3dLabels>(cfg.projection);
        let active_representation_id = cfg.projection.active_representation_id.as_deref();
        match body_key {
            BLOCK3D_BODY_WORLD => render_world(doc.projection, active_representation_id, labels),
            BLOCK3D_BODY_DOCUMENT => build_document_tree(doc.projection, &cfg.projection.selected_ids, labels),
            BLOCK3D_BODY_INSPECTOR => build_inspection_tree(doc.projection, active_representation_id, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, cfg: &ConfigView<'_, Block3dConfig>) -> AppLabelsOverlay {
        let labels = resolve_labels::<Block3dLabels>(cfg.projection);
        let is_de = is_de_locale(cfg.projection);
        AppLabelsOverlay::default().window_kind_label(BLOCK3D_WINDOW_WORLD, labels.window_world).action_labels(block3d_action_labels(is_de))
    }

    /// 🌉️ The flagship seam: `puzzle3d_catalog_fragment`'s first real caller. Wraps the block-3d
    /// document's puzzle3d-shaped catalog fragment (`objectKinds`/`vortexKinds`/`cableKinds`/
    /// `attractionKinds`/`kindCompatibility`) as a `kit.catalog`-schema `Media` value for the
    /// `"catalog:out"` port declared in `block_3d_engine::block3d_io`. `wanted_tags` should come from
    /// `cfg.wanted_tags` (`Block3dConfig`) but `DocumentApp::export_media`'s landed signature doesn't
    /// thread `ConfigView` through yet — see `Block3dConfig::wanted_tags`'s doc — so this always
    /// resolves the active representation with an empty (all-tags) filter until that lands. Falls
    /// through to the default whole-document pack export for every other port (`"document:out"`).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Block3dDefinition>) -> Result<Media, MediaError> {
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
        let fragment = block_3d_engine::puzzle3d_catalog_fragment(doc.projection, &[]);
        Ok(Media {
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            payload: MediaPayload::Structured { schema: KIT_CATALOG_ARTIFACT_ID.into(), json: fragment.to_string() },
        })
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
            // 🗂️ The puzzle3d catalog artifact this app's new `"catalog:out"` port produces — see
            // `block_3d_engine::block3d_io`/`Block3dPlayApp::export_media`. `source_format`/`schema`
            // both pin the `kit.catalog` JSON fragment shape `puzzle3d_catalog_fragment` builds.
            .artifact_kind(ArtifactKindSpec {
                id: KIT_CATALOG_ARTIFACT_ID.into(),
                name: "Kit Catalog".into(),
                source_format: KIT_CATALOG_ARTIFACT_ID.into(),
                component_kind: "kit-catalog".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: KIT_CATALOG_ARTIFACT_ID.into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("box")
            .mode("edit", "Edit", "pencil")
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
            .view_action("setActiveRepresentation", "Set Active Representation")
            .io(block_3d_engine::block3d_io()),
    )
    .example(BLOCK3D_EXAMPLE_CAPSULE, "Nakagin Capsule", serde_json::to_string(&block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(), "building")
    .example(BLOCK3D_EXAMPLE_FOREST_LEFT, "Hexagonal Cut Concrete Forest Left", serde_json::to_string(&block_3d_dsl::parse_dsl(block_3d_dsl::BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(), "list-tree")
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
    use semio_framework_plugin::{testkit, PluginApp, ViewState};

    fn new_app() -> semio_framework_plugin::VcsDocumentApp<Block3dPlayApp> {
        testkit::new_app::<Block3dPlayApp>()
    }

    #[test]
    fn renders_document_tree_and_inspector() {
        let mut app = new_app();
        let node = app.render(BLOCK3D_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Representations"));
        let inspector = app.render(BLOCK3D_BODY_INSPECTOR, None, &ViewState::default()).expect("render");
        let inspector_json = serde_json::to_string(&inspector).unwrap();
        assert!(inspector_json.contains("\"type\":\"tree\""), "inspection body must be a tree like document");
        assert!(inspector_json.contains("Name"));
        assert!(inspector_json.contains("Vortices"));
        assert!(!inspector_json.contains("\"type\":\"stack\""), "inspection body must not be a free-form stack");
    }

    #[test]
    fn add_representation_then_set_active_then_render_world_shows_mesh() {
        let mut app = new_app();
        app.dispatch_typed(Block3dCommand::AddRepresentation, &testkit::meta("local")).expect("add representation");
        let representation_id = app.projection().expect("projection").representations[0].id.clone();
        app.dispatch_typed(Block3dCommand::SetActiveRepresentation { representation_id: Some(representation_id) }, &testkit::meta("local")).expect("set active");
        let node = app.render(BLOCK3D_BODY_WORLD, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("mesh:"));
    }

    #[test]
    fn add_vortex_kind_then_add_vortex_then_remove_round_trips() {
        let mut app = new_app();
        app.dispatch_typed(Block3dCommand::AddVortexKind, &testkit::meta("local")).expect("add vortex kind");
        app.dispatch_typed(Block3dCommand::AddVortex, &testkit::meta("local")).expect("add vortex");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.vortices.len(), 1);
        let vortex_id = projection.vortices[0].id.clone();
        app.dispatch_typed(Block3dCommand::RemoveVortex { id: vortex_id }, &testkit::meta("local")).expect("remove vortex");
        assert_eq!(app.projection().expect("projection").vortices.len(), 0);
    }

    #[test]
    fn set_active_example_loads_capsule_fixture() {
        let mut app = new_app();
        app.dispatch_typed(Block3dCommand::SetActiveExample { id: BLOCK3D_EXAMPLE_CAPSULE.into() }, &testkit::meta("local")).expect("load example");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.object_kind.id, "Capsule J");
        assert_eq!(projection.representations.len(), 2);
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = new_app();
        app.dispatch_typed(Block3dCommand::AddVortexKind, &testkit::meta("local")).expect("add vortex kind");
        assert_eq!(app.projection().expect("projection").vortex_kinds.len(), 1);
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").vortex_kinds.len(), 0);
        app.handle_action("redo", None, &testkit::meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").vortex_kinds.len(), 1);
    }

    #[test]
    fn set_selection_writes_config_not_document() {
        let mut app = new_app();
        let result = app.dispatch_typed(Block3dCommand::SetSelection { ids: vec!["representation:r0".into()] }, &testkit::meta("local")).expect("select");
        assert!(result.operations.is_empty(), "setSelection is config-only and must emit no document operations");
    }

    /// 🌉️ `puzzle3d_catalog_fragment`'s new caller round-trips through the `"catalog:out"` media port.
    #[test]
    fn export_media_catalog_out_wraps_the_puzzle3d_fragment() {
        let mut app = new_app();
        app.dispatch_typed(Block3dCommand::SetActiveExample { id: BLOCK3D_EXAMPLE_CAPSULE.into() }, &testkit::meta("local")).expect("load example");
        let media = app.export_media("catalog:out").expect("export catalog");
        assert_eq!(media.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "kit.catalog");
                let value: Value = serde_json::from_str(&json).expect("valid json");
                assert_eq!(value["objectKinds"][0]["id"], "Capsule J");
            }
            other => panic!("expected Structured payload, got {other:?}"),
        }
    }

    #[test]
    fn block3d_io_is_wired_into_the_manifest() {
        let definition = create_block3d_app().definition;
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
    }
}
//#endregion 🧪️Tests
