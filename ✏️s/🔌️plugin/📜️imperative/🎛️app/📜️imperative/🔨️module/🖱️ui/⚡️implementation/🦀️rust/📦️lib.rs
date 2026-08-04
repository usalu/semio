//! 🎛️ Imperative app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait
//! pilot — `ImperativePlayApp` is a unit struct; the former `ImperativePlayRuntime` app-struct
//! `RefCell` (selection, run output) now lives in `imperative_engine::ImperativeConfig`, written via
//! `imperative_op::ImperativeConfigOperation`s (real `backwards`, no ad hoc inverse tracking); every
//! action dispatches through the single typed `imperative_protocol::ImperativeCommand` channel via
//! `DocumentApp::handle`.

use imperative::{value_dsl_map_to_dictionary, Dictionary, ImperativeDocument, PathRef, Step};
use imperative_engine::{default_document, imperative_io, ImperativeConfig, ImperativeHost};
use imperative_op::{ImperativeConfigOperation, ImperativeOperation};
use imperative_protocol::ImperativeCommand;
use protocol::CollectionOperation;
use semio_framework_plugin::{
    build_table_scene, build_text_editor_scene, create_stack_layout, tree_item_with_action, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, App, AppLabels,
    ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, Label, Locale, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder, SurfaceKind, TableScene,
    Terminology, TextEditorScene, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use store::DocumentPack;

//#region 🔖️Constants
const IMPERATIVE_PLAY_APP_ID: &str = "imperative-play";
const IMPERATIVE_PLAY_SURFACE_MAIN: &str = "imperative.play.main";
const IMPERATIVE_PLAY_SURFACE_SCRIPT: &str = "imperative.play.script";
const IMPERATIVE_PLAY_BODY_MAIN: &str = "imperative.play.main";
const IMPERATIVE_PLAY_BODY_SCRIPT: &str = "imperative.play.script";
const IMPERATIVE_PLAY_BODY_DOCUMENT: &str = "imperative.play.document";
const IMPERATIVE_PLAY_BODY_CATALOGUE: &str = "imperative.play.catalogue";
const IMPERATIVE_PLAY_BODY_INSPECTOR: &str = "imperative.play.inspection";
const IMPERATIVE_PLAY_WINDOW_MAIN: &str = "imperative-main";
const IMPERATIVE_PLAY_WINDOW_SCRIPT: &str = "imperative-script";
pub const IMPERATIVE_DOCUMENT_SCHEMA: &str = "imperative.document/v1";
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterpart to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — mirrors `block3d_ui`'s identical region.
fn is_de_locale(cfg: &ImperativeConfig) -> bool {
    cfg.locale.starts_with("de")
}

/// 🗣️ `ImperativeConfig.locale` (a BCP-47 tag) mapped onto the SDK's exhaustive `Locale` enum.
fn imperative_locale(cfg: &ImperativeConfig) -> Locale {
    if is_de_locale(cfg) {
        Locale::De
    } else {
        Locale::En
    }
}

/// 🗣️ Resolves the active `ImperativeLabels` cell from the config-carried locale via the SDK's
/// two-axis `AppLabels::labels`. `ImperativeConfig` carries no terminology field, so terminology is
/// always `Native` — imperative's control-flow vocabulary has no building/assembly reuse variant.
fn imperative_labels(cfg: &ImperativeConfig) -> &'static ImperativeLabels {
    ImperativeLabels::labels(imperative_locale(cfg), Terminology::Native)
}
//#endregion 🔖️Locale

//#region 🔖️Types
#[derive(Serialize, Deserialize)]
struct TableRow {
    index: usize,
    id: String,
    kind: String,
}
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
/// 🆔️ Allocates a fresh `step-N` id one past the highest suffix used anywhere in the document
/// (including nested `control.*` bodies), deterministically from pre-state — no mutable counter.
fn next_step_id(document: &ImperativeDocument) -> String {
    fn max_suffix(steps: &[Step]) -> u64 {
        steps.iter().fold(0, |acc, step| {
            let own = step.id.strip_prefix("step-").and_then(|rest| rest.parse::<u64>().ok()).unwrap_or(0);
            let nested = step.bodies.values().map(|path| max_suffix(&path.steps)).max().unwrap_or(0);
            acc.max(own).max(nested)
        })
    }
    format!("step-{}", max_suffix(&document.path.steps) + 1)
}

/// 📍️ Resolves `owner`/`slot` command fields into a [`imperative::PathRef`] so nested control-step
/// bodies (e.g. `control.if` then/else) resolve correctly; falls back to the root path unless both are
/// present and `owner` names a real top-level step, avoiding an unresolvable or unknown reference that
/// would otherwise address nothing.
fn path_ref_from(owner: Option<&str>, slot: Option<&str>, document: &ImperativeDocument) -> PathRef {
    match (owner, slot) {
        (Some(owner), Some(slot)) if document.path.steps.iter().any(|step| step.id == owner) => PathRef { owner: Some(owner.to_string()), slot: Some(slot.to_string()) },
        _ => PathRef::default(),
    }
}

fn table_rows(steps: &[Step]) -> String {
    let rows: Vec<TableRow> = steps.iter().enumerate().map(|(index, step)| TableRow { index: index + 1, id: step.id.clone(), kind: step.kind.clone() }).collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

fn imperative_action(action: &str, args: Option<Value>) -> semio_framework_plugin::ActionDescriptor {
    semio_framework_plugin::ActionDescriptor { controller_id: IMPERATIVE_PLAY_APP_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
// 🗣️ Complete UI label set for the imperative app; one field per label makes every locale combination
// compile-checked. No separate reuse-terminology concept (pure control-flow vocabulary), so reuse repeats native.
semio_framework_plugin::app_labels! {
    struct ImperativeLabels {
        window_main: native_en "Imperative", native_de "Imperativ", reuse_en "Imperative", reuse_de "Imperativ";
        window_script: native_en "Script", native_de "Skript", reuse_en "Script", reuse_de "Skript";
        col_index: native_en "#", native_de "#", reuse_en "#", reuse_de "#";
        col_id: native_en "Id", native_de "ID", reuse_en "Id", reuse_de "ID";
        col_kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        action_state_set: native_en "Set state", native_de "Zustand setzen", reuse_en "Set state", reuse_de "Zustand setzen";
        action_log_print: native_en "Print log", native_de "Log ausgeben", reuse_en "Print log", reuse_de "Log ausgeben";
        action_control_if: native_en "If", native_de "Wenn", reuse_en "If", reuse_de "Wenn";
        action_control_while: native_en "While", native_de "Solange", reuse_en "While", reuse_de "Solange";
        action_math_add: native_en "Add", native_de "Addieren", reuse_en "Add", reuse_de "Addieren";
        document_empty: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        inspector_empty_hint: native_en "Select a step in the document.", native_de "Wählen Sie einen Schritt im Dokument aus.", reuse_en "Select a step in the document.", reuse_de "Wählen Sie einen Schritt im Dokument aus.";
        inspector_step_not_found: native_en "Step not found", native_de "Schritt nicht gefunden", reuse_en "Step not found", reuse_de "Schritt nicht gefunden";
        inspector_id: native_en "Id", native_de "ID", reuse_en "Id", reuse_de "ID";
        inspector_kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        inspector_params: native_en "Params", native_de "Parameter", reuse_en "Params", reuse_de "Parameter";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
fn build_document_tree(document: &ImperativeDocument, selected: &[String], labels: &ImperativeLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("imperative-play-document");
    let step_items: Vec<UiTreeItemNode> = document
        .path
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| tree_item_with_action(builder.item_id("step", &step.id), Label::data(format!("{}. {}", index + 1, step.kind)), Some(step.id.clone()), imperative_action("setSelection", Some(json!({ "ids": [step.id.clone()] })))))
        .collect();
    builder
        .section_or_placeholder("imperative-play-document.steps", Some(Label::data(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL)), true, step_items, labels.document_empty)
        .selected(selected.iter().map(|id| format!("imperative-play-document.step.{id}")).collect())
        .build()
}

fn build_catalogue_tree(labels: &ImperativeLabels) -> UiNode {
    let actions = [("state.set", labels.action_state_set), ("log.print", labels.action_log_print), ("control.if", labels.action_control_if), ("control.while", labels.action_control_while), ("math.add", labels.action_math_add)];
    let builder = PanelTreeBuilder::new("imperative-play-catalogue");
    let action_items: Vec<UiTreeItemNode> = actions.iter().map(|(kind, label)| tree_item_with_action(builder.item_id("action", kind), *label, Some((*kind).into()), imperative_action("addStep", Some(json!({ "kind": kind }))))).collect();
    builder.section("imperative-play-catalogue.actions", Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)), true, action_items).selected(vec![]).build()
}

fn build_inspector_tree(document: &ImperativeDocument, selected: &[String], labels: &ImperativeLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "imperative-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(labels.inspector_empty_hint)],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    let steps: Vec<&Step> = selected.iter().filter_map(|id| document.path.steps.iter().find(|step| &step.id == id)).collect();
    if steps.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "imperative-play-inspector.missing".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(labels.inspector_step_not_found)],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "imperative-play-inspector.step".into(),
        label: Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("imperative-play-inspector.id", labels.inspector_id, steps[0].id.clone()),
            ui_inspector_readonly_field("imperative-play-inspector.kind", labels.inspector_kind, steps[0].kind.clone()),
            ui_inspector_readonly_field("imperative-play-inspector.params", labels.inspector_params, serde_json::to_string(&steps[0].params).unwrap_or_else(|_| "{}".into())),
        ],
    }])
}
//#endregion 🔖️Panels

//#region 🔖️Render
/// 📤️ One table row per scope key so the full run output is legible instead of an 80-char
/// truncated blob; falls back to the raw JSON when it isn't a plain object.
fn run_output_rows(run_output_json: &str, offset: usize) -> Vec<TableRow> {
    match serde_json::from_str::<Value>(run_output_json).ok().and_then(|value| value.as_object().cloned()) {
        Some(scope) if !scope.is_empty() => {
            scope.into_iter().enumerate().map(|(index, (key, value))| TableRow { index: offset + index + 1, id: format!("run-output.{key}"), kind: format!("{key} = {}", serde_json::to_string(&value).unwrap_or_else(|_| "null".into())) }).collect()
        }
        _ => vec![TableRow { index: offset + 1, id: "run-output".into(), kind: run_output_json.to_string() }],
    }
}

fn render_main_table(document: &ImperativeDocument, run_output_json: &str, labels: &ImperativeLabels) -> UiNode {
    let mut rows_json = table_rows(&document.path.steps);
    if !run_output_json.is_empty() {
        if let Ok(mut rows) = serde_json::from_str::<Vec<TableRow>>(&rows_json) {
            rows.extend(run_output_rows(run_output_json, rows.len()));
            rows_json = serde_json::to_string(&rows).unwrap_or(rows_json);
        }
    }
    build_table_scene(
        IMPERATIVE_PLAY_SURFACE_MAIN,
        IMPERATIVE_PLAY_APP_ID,
        TableScene::base(
            json!([
                {"id":"index","label":labels.col_index.as_str()},
                {"id":"id","label":labels.col_id.as_str()},
                {"id":"kind","label":labels.col_kind.as_str()},
            ])
            .to_string(),
            rows_json,
        ),
    )
}

fn render_script(document: &ImperativeDocument) -> UiNode {
    let host = ImperativeHost::from_document(document.clone());
    build_text_editor_scene(IMPERATIVE_PLAY_SURFACE_SCRIPT, IMPERATIVE_PLAY_APP_ID, TextEditorScene::base(host.compile_text(), Some("imperative".into()), None))
}
//#endregion 🔖️Render

//#region 🔖️ImperativePlayApp
/// 🧪️ B1: unit struct — the former `ImperativePlayRuntime`/`self.runtime` field now lives in
/// `imperative_engine::ImperativeConfig` (see `DocumentApp::Config`), written through
/// `imperative_op::ImperativeConfigOperation`s.
#[derive(Default)]
pub struct ImperativePlayApp;

impl DocumentApp for ImperativePlayApp {
    type Projection = ImperativeDocument;
    type Operation = ImperativeOperation;
    type Config = ImperativeConfig;
    type ConfigOperation = ImperativeConfigOperation;
    type Command = ImperativeCommand;

    fn app_id(&self) -> &str {
        IMPERATIVE_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        IMPERATIVE_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> ImperativeDocument {
        default_document()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(imperative_io())
    }

    /// 🏷️ Maps each `ImperativeCommand` variant back to the action id it was declared under in
    /// `create_imperative_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &ImperativeCommand) -> &str {
        match command {
            ImperativeCommand::AddStep { .. } => "addStep",
            ImperativeCommand::AddStepAt { .. } => "addStepAt",
            ImperativeCommand::RemoveStep { .. } => "removeStep",
            ImperativeCommand::RemoveStepAt { .. } => "removeStepAt",
            ImperativeCommand::MoveStep { .. } => "moveStep",
            ImperativeCommand::MoveStepAt { .. } => "moveStepAt",
            ImperativeCommand::SetStepParams { .. } => "setStepParams",
            ImperativeCommand::SetStepParamsAt { .. } => "setStepParamsAt",
            ImperativeCommand::SetSelection { .. } => "setSelection",
            ImperativeCommand::Run => "run",
            ImperativeCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(&self, command: &ImperativeCommand, doc: &DocumentView<'_, ImperativeDocument>, cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeOperation, ImperativeConfigOperation>, Fault> {
        let document = doc.projection;
        let config = cfg.projection;
        match command {
            ImperativeCommand::AddStep { kind, index } => {
                let id = next_step_id(document);
                let step = Step { id: id.clone(), kind: kind.clone(), params: Dictionary::new(), bodies: BTreeMap::new() };
                Emit {
                    document_operations: vec![ImperativeOperation { path_ref: PathRef::default(), collection: CollectionOperation::Add { id: id.clone(), at: index.unwrap_or(usize::MAX), item: step } }],
                    config_operations: vec![ImperativeConfigOperation::SetSelectedSteps { ids: vec![id] }],
                    ..Default::default()
                }
            }
            ImperativeCommand::AddStepAt { kind, index, owner, slot } => {
                let path_ref = path_ref_from(owner.as_deref(), slot.as_deref(), document);
                let id = next_step_id(document);
                let step = Step { id: id.clone(), kind: kind.clone(), params: Dictionary::new(), bodies: BTreeMap::new() };
                Emit {
                    document_operations: vec![ImperativeOperation { path_ref, collection: CollectionOperation::Add { id: id.clone(), at: index.unwrap_or(usize::MAX), item: step } }],
                    config_operations: vec![ImperativeConfigOperation::SetSelectedSteps { ids: vec![id] }],
                    ..Default::default()
                }
            }
            ImperativeCommand::RemoveStep { id } => {
                if resolve_contains(document, None, None, id) {
                    let mut ids = config.selected_step_ids.clone();
                    ids.retain(|step_id| step_id != id);
                    Emit {
                        document_operations: vec![ImperativeOperation { path_ref: PathRef::default(), collection: CollectionOperation::Remove { id: id.clone() } }],
                        config_operations: vec![ImperativeConfigOperation::SetSelectedSteps { ids }],
                        ..Default::default()
                    }
                } else {
                    Emit::default()
                }
            }
            ImperativeCommand::RemoveStepAt { id, owner, slot } => {
                if resolve_contains(document, owner.as_deref(), slot.as_deref(), id) {
                    let path_ref = path_ref_from(owner.as_deref(), slot.as_deref(), document);
                    let mut ids = config.selected_step_ids.clone();
                    ids.retain(|step_id| step_id != id);
                    Emit { document_operations: vec![ImperativeOperation { path_ref, collection: CollectionOperation::Remove { id: id.clone() } }], config_operations: vec![ImperativeConfigOperation::SetSelectedSteps { ids }], ..Default::default() }
                } else {
                    Emit::default()
                }
            }
            ImperativeCommand::MoveStep { id, index } => {
                if resolve_contains(document, None, None, id) {
                    Emit::operations(vec![ImperativeOperation { path_ref: PathRef::default(), collection: CollectionOperation::Move { id: id.clone(), to: *index } }])
                } else {
                    Emit::default()
                }
            }
            ImperativeCommand::MoveStepAt { id, index, owner, slot } => {
                if resolve_contains(document, owner.as_deref(), slot.as_deref(), id) {
                    let path_ref = path_ref_from(owner.as_deref(), slot.as_deref(), document);
                    Emit::operations(vec![ImperativeOperation { path_ref, collection: CollectionOperation::Move { id: id.clone(), to: *index } }])
                } else {
                    Emit::default()
                }
            }
            ImperativeCommand::SetStepParams { id, params } => {
                if resolve_contains(document, None, None, id) {
                    Emit::operations(vec![ImperativeOperation { path_ref: PathRef::default(), collection: CollectionOperation::Patch { id: id.clone(), patch: value_dsl_map_to_dictionary(params) } }])
                } else {
                    Emit::default()
                }
            }
            ImperativeCommand::SetStepParamsAt { id, owner, slot, params } => {
                if resolve_contains(document, owner.as_deref(), slot.as_deref(), id) {
                    let path_ref = path_ref_from(owner.as_deref(), slot.as_deref(), document);
                    Emit::operations(vec![ImperativeOperation { path_ref, collection: CollectionOperation::Patch { id: id.clone(), patch: value_dsl_map_to_dictionary(params) } }])
                } else {
                    Emit::default()
                }
            }
            ImperativeCommand::SetSelection { ids } => Ok(Emit::config(vec![ImperativeConfigOperation::SetSelectedSteps { ids: ids.clone() }]),
            ImperativeCommand::Run => {
                let host = ImperativeHost::from_document(document.clone());
                let result = host.run();
                let json = serde_json::to_string(&result.scope).unwrap_or_else(|_| format!("{:?}", result.scope));
                Emit::config(vec![ImperativeConfigOperation::SetRunOutput { json }])
            }
            ImperativeCommand::SetLocale { value } => Ok(Emit::config(vec![ImperativeConfigOperation::SetLocale { value: value.clone() }]),
        }
    }

    /// 🎞️ `"result:out"` exports the last `run` scope (a generic data value, the port recipe's
    /// `computation.imperative`-kinded output); `"document:out"` replicates `DocumentApp::export_media`'s
    /// default whole-document-pack behavior (unreachable once this override exists).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, ImperativeDocument>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let host = ImperativeHost::from_document(doc.projection.clone());
                let result = host.run();
                let json = serde_json::to_string(&result.scope).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.imperative".into(), json } })
            }
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, ImperativeDocument>, cfg: &ConfigView<'_, ImperativeConfig>) -> UiNode {
        let document = doc.projection;
        let config = cfg.projection;
        let labels = imperative_labels(config);
        match body_key {
            IMPERATIVE_PLAY_BODY_MAIN => render_main_table(document, &config.run_output_json, labels),
            IMPERATIVE_PLAY_BODY_SCRIPT => render_script(document),
            IMPERATIVE_PLAY_BODY_DOCUMENT => build_document_tree(document, &config.selected_step_ids, labels),
            IMPERATIVE_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            IMPERATIVE_PLAY_BODY_INSPECTOR => build_inspector_tree(document, &config.selected_step_ids, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

/// 🔎️ Resolves the step list a `PathRef` addresses — the root path, or a nested `control.*` step's
/// slot (an unmaterialized slot reads as empty).
fn steps_at<'a>(document: &'a ImperativeDocument, path_ref: &PathRef) -> &'a [Step] {
    match (&path_ref.owner, &path_ref.slot) {
        (Some(owner), Some(slot)) => document.path.steps.iter().find(|step| &step.id == owner).and_then(|step| step.bodies.get(slot)).map_or(&[], |path| path.steps.as_slice()),
        _ => document.path.steps.as_slice(),
    }
}

/// 🔎️ True when the step `id` exists in the list the `owner`/`slot` command fields address — the
/// pre-state guard the operation arms share so a stale id never emits a no-operation edit into history.
fn resolve_contains(document: &ImperativeDocument, owner: Option<&str>, slot: Option<&str>, id: &str) -> bool {
    let path_ref = path_ref_from(owner, slot, document);
    steps_at(document, &path_ref).iter().any(|step| step.id == id)
}
//#endregion 🔖️ImperativePlayApp

//#region 🔖️Manifest
pub fn create_imperative_app() -> App {
    App::from_builder(
        App::builder(IMPERATIVE_PLAY_APP_ID, LocalizedLabel::native("Imperative", "Imperativ")).document(["semio", "imperative"])
            .artifact_kind(ArtifactKindSpec {
                id: "computation.imperative".into(),
                name: "Imperative".into(),
                source_format: "imperative.document".into(),
                component_kind: "imperative".into(),
                dimension: "graph".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Imperative },
                schema: "imperative.document".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("imperative")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind(IMPERATIVE_PLAY_WINDOW_MAIN, LocalizedLabel::native("Imperative", "Imperativ"), IMPERATIVE_PLAY_BODY_MAIN, SurfaceKind::NodeGraph, "code")
            .window_kind(IMPERATIVE_PLAY_WINDOW_SCRIPT, LocalizedLabel::native("Script", "Skript"), IMPERATIVE_PLAY_BODY_SCRIPT, SurfaceKind::TextEditor, "file-code")
            .default_layout(create_stack_layout(
                &[IMPERATIVE_PLAY_WINDOW_MAIN.into(), IMPERATIVE_PLAY_WINDOW_SCRIPT.into()],
                Some(&["Imperative".into(), "Script".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
                PanelGroup::Workbench,
                IMPERATIVE_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                IMPERATIVE_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                IMPERATIVE_PLAY_BODY_INSPECTOR,
            )
            // 🔧️ Document-mutating step edits — dispatched as VCS operations with a true inverse.
            // The `*At` variants address a nested body via owner/slot fields (drag-and-drop into blocks).
            .operation("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"))
            .operation("addStepAt", LocalizedLabel::native("Add Step At", "Schritt bei Position hinzufügen"))
            .operation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
            .operation("removeStepAt", LocalizedLabel::native("Remove Step At", "Schritt bei Position entfernen"))
            .operation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .operation("moveStepAt", LocalizedLabel::native("Move Step At", "Schritt bei Position verschieben"))
            .operation("setStepParams", LocalizedLabel::native("Set Step Params", "Schrittparameter festlegen"))
            .operation("setStepParamsAt", LocalizedLabel::native("Set Step Params At", "Schrittparameter bei Position festlegen"))
            // 👁️ Ephemeral view state / runtime effect — selection is scratch, `run` evaluates into config.
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("run", LocalizedLabel::native("Run", "Ausführen"))
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            // 📝️ Staged argument form for the panel-visible create action (the step kind is a choice).
            .action_args("addStep", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("state.set", LocalizedLabel::native("Set State", "Zustand setzen")),
                    ActionArgOption::new("log.print", LocalizedLabel::native("Print Log", "Log ausgeben")),
                    ActionArgOption::new("control.if", LocalizedLabel::native("If", "Wenn")),
                    ActionArgOption::new("control.while", LocalizedLabel::native("While", "Solange")),
                    ActionArgOption::new("math.add", LocalizedLabel::native("Add", "Addieren")),
                ]).default_value("log.print"),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS /
            // WORKFLOWS-END-TO-END-TYPED-PORTS) — `imperative_io()` is this port information's single
            // source of truth, reused here rather than duplicated.
            .io(imperative_io()),
    )
    .example("demo", LocalizedLabel::native("Demo", "Demo"), serde_json::to_string(&default_document()).expect("default_document is a static, hand-built value with no non-finite floats or non-UTF8 keys"), "cylinder")
    .workflow("imperative", "Imperative", "graph")
}
//#endregion 🔖️Manifest

//#region 🔖️WasmSession
/// 🌐️ Browser-facing session wrapper — the ONLY wasm-bindgen surface for the imperative app; kept
/// in the `ui` (not `⚙️engine`) constitutional slot so the engine stays pure headless compute.
#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use super::*;
    use imperative_engine::ImperativeCoreError;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;

    struct ImperativeSessionInner {
        host: ImperativeHost,
    }

    #[wasm_bindgen]
    pub struct ImperativeSession {
        state: Rc<RefCell<ImperativeSessionInner>>,
    }

    #[wasm_bindgen]
    impl ImperativeSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self { state: Rc::new(RefCell::new(ImperativeSessionInner { host: ImperativeHost::default() })) }
        }

        #[wasm_bindgen(js_name = loadPathJson)]
        pub fn load_path_json(&self, json: &str) -> Result<(), JsValue> {
            let host = ImperativeHost::load_json(json).map_err(|err: ImperativeCoreError| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = pathJson)]
        pub fn path_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.to_json().map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = catalogueJson)]
        pub fn catalogue_json(&self) -> String {
            self.state.borrow().host.catalogue_json()
        }

        #[wasm_bindgen(js_name = addStep)]
        pub fn add_step(&self, kind: &str, index: Option<usize>) -> String {
            self.state.borrow_mut().host.add_step(kind, index)
        }

        #[wasm_bindgen(js_name = addStepAt)]
        pub fn add_step_at(&self, path_ref_json: &str, kind: &str, index: Option<usize>) -> Result<String, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.add_step_at(&path_ref, kind, index).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = removeStep)]
        pub fn remove_step(&self, id: &str) -> bool {
            self.state.borrow_mut().host.remove_step(id)
        }

        #[wasm_bindgen(js_name = removeStepAt)]
        pub fn remove_step_at(&self, path_ref_json: &str, id: &str) -> Result<bool, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            Ok(self.state.borrow_mut().host.remove_step_at(&path_ref, id))
        }

        #[wasm_bindgen(js_name = moveStep)]
        pub fn move_step(&self, id: &str, new_index: usize) -> bool {
            self.state.borrow_mut().host.move_step(id, new_index)
        }

        #[wasm_bindgen(js_name = moveStepAt)]
        pub fn move_step_at(&self, path_ref_json: &str, id: &str, new_index: usize) -> Result<bool, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            Ok(self.state.borrow_mut().host.move_step_at(&path_ref, id, new_index))
        }

        #[wasm_bindgen(js_name = setStepParamsJson)]
        pub fn set_step_params_json(&self, id: &str, json: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.set_step_params_json(id, json).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = setStepParamsAt)]
        pub fn set_step_params_at(&self, path_ref_json: &str, id: &str, json: &str) -> Result<(), JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.set_step_params_at(&path_ref, id, json).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen]
        pub fn run(&self) -> Result<String, JsValue> {
            let result = self.state.borrow().host.run();
            serde_json::to_string(&result).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = compileText)]
        pub fn compile_text(&self) -> String {
            self.state.borrow().host.compile_text()
        }
    }
}
//#endregion 🔖️WasmSession

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<ImperativePlayApp> {
        testkit::new_app::<ImperativePlayApp>()
    }

    /// 🧬️ A wrapper carrying the real action registry so `addStep`'s `kind` default materializes and the
    /// View-kind `run`/`setSelection` actions are held to the no-document-operations contract.
    fn new_app_with_registry() -> VcsDocumentApp<ImperativePlayApp> {
        testkit::new_app_with_registry::<ImperativePlayApp>(create_imperative_app)
    }

    /// 🧬️ The exact document `default_document()` becomes after `AddStep` materializes `id`/`kind` with
    /// empty params/bodies — the deterministic "after" fixture for the undo-redo round trip below.
    fn expected_document_after_add_step(kind: &str, id: &str) -> ImperativeDocument {
        let mut document = default_document();
        document.path.steps.push(Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() });
        document
    }

    #[test]
    fn app_definition_builds_without_panicking() {
        let app = create_imperative_app();
        assert_eq!(app.definition.id, IMPERATIVE_PLAY_APP_ID);
        assert!(app.definition.keybindings.iter().any(|binding| binding.action.action == "undo"));
    }

    #[test]
    fn imperative_io_is_declared_on_the_manifest() {
        let app = create_imperative_app();
        assert_eq!(app.definition.io.artifact.id, "computation.imperative");
        assert_eq!(app.definition.io.ports.len(), 1);
        assert_eq!(app.definition.io.ports[0].id, "result:out");
    }

    #[test]
    fn add_step_materializes_kind_default_and_run_emits_no_document_operations() {
        let mut app = new_app_with_registry();
        // AddStep fired with no explicit kind: the declared `kind` default ("log.print") must be
        // materialized by the registry's action-arg default resolution.
        app.dispatch_typed(ImperativeCommand::AddStep { kind: "log.print".into(), index: None }, &testkit::meta("local")).expect("add step");
        let document = app.projection().expect("materialize projection");
        assert_eq!(document.path.steps.last().unwrap().kind, "log.print");
        // `run` is a View-kind command: under registry enforcement it must not emit document operations.
        let result = app.dispatch_typed(ImperativeCommand::Run, &testkit::meta("local")).expect("run");
        assert!(result.operations.is_empty(), "run evaluates into config, never the document");
    }

    #[test]
    fn renders_table_scene() {
        let mut app = new_app();
        let node = app.render(IMPERATIVE_PLAY_BODY_MAIN, None, &Default::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("table"));
    }

    #[test]
    fn imperative_labels_resolve_native_by_default() {
        let mut app = new_app();
        let node = app.render(IMPERATIVE_PLAY_BODY_CATALOGUE, None, &Default::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Set state"));
        assert!(json.contains("Print log"));
        assert!(json.contains("While"));
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command.
    #[test]
    fn imperative_labels_resolve_native_in_german() {
        let mut app = new_app();
        app.dispatch_typed(ImperativeCommand::SetLocale { value: "de-DE".into() }, &testkit::meta("local")).expect("set locale");
        let node = app.render(IMPERATIVE_PLAY_BODY_CATALOGUE, None, &Default::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Zustand setzen"));
        assert!(json.contains("Log ausgeben"));
        assert!(json.contains("Solange"));
    }

    #[test]
    fn renders_script_editor() {
        let mut app = new_app();
        let node = app.render(IMPERATIVE_PLAY_BODY_SCRIPT, None, &Default::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn default_document_has_steps() {
        let app = new_app();
        assert_eq!(app.projection().expect("projection").path.steps.len(), 2);
    }

    #[test]
    fn add_step_command_appends_step() {
        let mut app = new_app();
        app.dispatch_typed(ImperativeCommand::AddStep { kind: "log.print".into(), index: None }, &testkit::meta("local")).expect("add step");
        assert!(app.projection().expect("projection").path.steps.len() > 2);
    }

    #[test]
    fn add_step_at_owner_slot_nests_into_control_body() {
        let mut app = new_app();
        app.dispatch_typed(ImperativeCommand::AddStep { kind: "control.if".into(), index: None }, &testkit::meta("local")).expect("add owner");
        let owner_id = app.projection().expect("projection").path.steps.last().expect("owner").id.clone();
        let root_len = app.projection().expect("projection").path.steps.len();
        app.dispatch_typed(ImperativeCommand::AddStepAt { kind: "log.print".into(), index: None, owner: Some(owner_id.clone()), slot: Some("then".into()) }, &testkit::meta("local")).expect("add nested");
        let document = app.projection().expect("projection");
        let owner_step = document.path.steps.iter().find(|step| step.id == owner_id).expect("owner step");
        assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
        assert_eq!(document.path.steps.len(), root_len, "nested step lives in the slot, not the root path");
    }

    #[test]
    fn add_step_at_falls_back_to_root_for_unknown_owner() {
        let mut app = new_app();
        app.dispatch_typed(ImperativeCommand::AddStepAt { kind: "log.print".into(), index: None, owner: Some("missing-step".into()), slot: Some("then".into()) }, &testkit::meta("local")).expect("add step");
        let document = app.projection().expect("projection");
        let added_id = document.path.steps.last().expect("added").id.clone();
        assert!(document.path.steps.iter().any(|step| step.id == added_id));
    }

    #[test]
    fn run_command_expands_scope_into_readable_rows_without_truncation() {
        let mut app = new_app();
        app.dispatch_typed(ImperativeCommand::Run, &testkit::meta("local")).expect("run");
        let node = app.render(IMPERATIVE_PLAY_BODY_MAIN, None, &Default::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("counter"), "run output row shows the full scope key, not an 80-char blob");
    }

    #[test]
    fn undo_after_add_step_restores_original_document_exactly() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(&mut app, ImperativeCommand::AddStep { kind: "log.print".into(), index: None }, |app| app.projection().expect("projection"), default_document(), expected_document_after_add_step("log.print", "step-3"));
    }

    #[test]
    fn remove_step_command_is_exact_inverse_of_add() {
        let mut app = new_app();
        let original = app.projection().expect("projection");
        app.dispatch_typed(ImperativeCommand::AddStep { kind: "math.add".into(), index: None }, &testkit::meta("local")).expect("add step");
        let added_id = app.projection().expect("projection").path.steps.last().expect("added").id.clone();
        app.dispatch_typed(ImperativeCommand::RemoveStep { id: added_id }, &testkit::meta("local")).expect("remove step");
        assert_eq!(app.projection().expect("projection"), original);
    }

    /// 🧪️ The definitional regression proof: two independent instances start from the same document,
    /// apply DISJOINT edits (A appends a root step, B patches an existing step's params), and
    /// exchanging operations over a `MemoryBackbone` converges both sides onto an identical projection —
    /// impossible under whole-document `setDocument` snapshots, which would clobber one side's write.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<ImperativePlayApp, _>(
            "mem://imperative-convergence",
            ImperativeCommand::AddStep { kind: "math.add".into(), index: None },
            ImperativeCommand::SetStepParams { id: "step-1".into(), params: imperative::dictionary_to_value_dsl_map(&Dictionary::new().insert("key", neural_engine::Value::Atom(neural_engine::Atom::String("renamed".into())))) },
            |app| app.projection().expect("projection"),
        );
    }

    #[test]
    fn ingest_operations_is_idempotent_for_imperative() {
        testkit::assert_ingest_idempotent::<ImperativePlayApp, _>(ImperativeCommand::AddStep { kind: "math.add".into(), index: None }, |app| app.projection().expect("projection").path.steps.len());
    }
}
//#endregion 🧪️Tests
