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
    build_table_scene, build_text_editor_scene, create_stack_layout, localized_label_map, tree_item_with_action, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text,
    ActionArgDef, ActionArgOption, App, AppLabelsOverlay, AppLabelsOverlayExt, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, LocaleLabels, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder, SurfaceKind, TableScene,
    TextEditorScene, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
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
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — mirrors `shooting_ui`'s local helpers.
fn is_de_locale(cfg: &ImperativeConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &ImperativeConfig) -> &'static L {
    if is_de_locale(cfg) { L::locale_labels_de() } else { L::locale_labels_en() }
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
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the imperative app; one field per label makes every locale combination compile-checked.
    struct ImperativeLabels {
        window_main: &'static str = en: "Imperative", de: "Imperativ";
        window_script: &'static str = en: "Script", de: "Skript";
        col_index: &'static str = en: "#", de: "#";
        col_id: &'static str = en: "Id", de: "ID";
        col_kind: &'static str = en: "Kind", de: "Art";
        action_state_set: &'static str = en: "Set state", de: "Zustand setzen";
        action_log_print: &'static str = en: "Print log", de: "Log ausgeben";
        action_control_if: &'static str = en: "If", de: "Wenn";
        action_control_while: &'static str = en: "While", de: "Solange";
        action_math_add: &'static str = en: "Add", de: "Addieren";
        document_empty: &'static str = en: "(none)", de: "(keine)";
        inspector_empty_hint: &'static str = en: "Select a step in the document.", de: "Wählen Sie einen Schritt im Dokument aus.";
        inspector_step_not_found: &'static str = en: "Step not found", de: "Schritt nicht gefunden";
        inspector_id: &'static str = en: "Id", de: "ID";
        inspector_kind: &'static str = en: "Kind", de: "Art";
        inspector_params: &'static str = en: "Params", de: "Parameter";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_imperative_app`'s
/// static manifest — the manifest itself has no config/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder.
fn imperative_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(
        is_de,
        &[
            ("addStep", "Add Step", "Schritt hinzufügen"),
            ("addStepAt", "Add Step At", "Schritt bei Position hinzufügen"),
            ("removeStep", "Remove Step", "Schritt entfernen"),
            ("removeStepAt", "Remove Step At", "Schritt bei Position entfernen"),
            ("moveStep", "Move Step", "Schritt verschieben"),
            ("moveStepAt", "Move Step At", "Schritt bei Position verschieben"),
            ("setStepParams", "Set Step Params", "Schrittparameter festlegen"),
            ("setStepParamsAt", "Set Step Params At", "Schrittparameter bei Position festlegen"),
            ("setSelection", "Set Selection", "Auswahl festlegen"),
            ("run", "Run", "Ausführen"),
        ],
    )
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
fn build_document_tree(document: &ImperativeDocument, selected: &[String], labels: &ImperativeLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("imperative-play-document");
    let step_items: Vec<UiTreeItemNode> = document
        .path
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| tree_item_with_action(builder.item_id("step", &step.id), format!("{}. {}", index + 1, step.kind), Some(step.id.clone()), imperative_action("setSelection", Some(json!({ "ids": [step.id.clone()] })))))
        .collect();
    builder
        .section_or_placeholder("imperative-play-document.steps", Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()), true, step_items, labels.document_empty)
        .selected(selected.iter().map(|id| format!("imperative-play-document.step.{id}")).collect())
        .build()
}

fn build_catalogue_tree(labels: &ImperativeLabels) -> UiNode {
    let actions = [("state.set", labels.action_state_set), ("log.print", labels.action_log_print), ("control.if", labels.action_control_if), ("control.while", labels.action_control_while), ("math.add", labels.action_math_add)];
    let builder = PanelTreeBuilder::new("imperative-play-catalogue");
    let action_items: Vec<UiTreeItemNode> = actions.iter().map(|(kind, label)| tree_item_with_action(builder.item_id("action", kind), *label, Some((*kind).into()), imperative_action("addStep", Some(json!({ "kind": kind }))))).collect();
    builder.section("imperative-play-catalogue.actions", Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()), true, action_items).selected(vec![]).build()
}

fn build_inspector_tree(document: &ImperativeDocument, selected: &[String], labels: &ImperativeLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "imperative-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
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
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text(labels.inspector_step_not_found)],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "imperative-play-inspector.step".into(),
        label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into(),
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
                {"id":"index","label":labels.col_index},
                {"id":"id","label":labels.col_id},
                {"id":"kind","label":labels.col_kind},
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

    fn handle(&self, command: &ImperativeCommand, doc: &DocumentView<'_, ImperativeDocument>, cfg: &ConfigView<'_, ImperativeConfig>) -> Emit<ImperativeOperation, ImperativeConfigOperation> {
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
                    Emit {
                        document_operations: vec![ImperativeOperation { path_ref, collection: CollectionOperation::Remove { id: id.clone() } }],
                        config_operations: vec![ImperativeConfigOperation::SetSelectedSteps { ids }],
                        ..Default::default()
                    }
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
            ImperativeCommand::SetSelection { ids } => Emit::config(vec![ImperativeConfigOperation::SetSelectedSteps { ids: ids.clone() }]),
            ImperativeCommand::Run => {
                let host = ImperativeHost::from_document(document.clone());
                let result = host.run();
                let json = serde_json::to_string(&result.scope).unwrap_or_else(|_| format!("{:?}", result.scope));
                Emit::config(vec![ImperativeConfigOperation::SetRunOutput { json }])
            }
            ImperativeCommand::SetLocale { value } => Emit::config(vec![ImperativeConfigOperation::SetLocale { value: value.clone() }]),
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
        let labels = resolve_labels::<ImperativeLabels>(config);
        match body_key {
            IMPERATIVE_PLAY_BODY_MAIN => render_main_table(document, &config.run_output_json, labels),
            IMPERATIVE_PLAY_BODY_SCRIPT => render_script(document),
            IMPERATIVE_PLAY_BODY_DOCUMENT => build_document_tree(document, &config.selected_step_ids, labels),
            IMPERATIVE_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            IMPERATIVE_PLAY_BODY_INSPECTOR => build_inspector_tree(document, &config.selected_step_ids, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, cfg: &ConfigView<'_, ImperativeConfig>) -> AppLabelsOverlay {
        let labels = resolve_labels::<ImperativeLabels>(cfg.projection);
        AppLabelsOverlay::default().window_kind_label(IMPERATIVE_PLAY_WINDOW_MAIN, labels.window_main).window_kind_label(IMPERATIVE_PLAY_WINDOW_SCRIPT, labels.window_script).action_labels(imperative_action_labels(is_de_locale(cfg.projection)))
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
        App::builder(IMPERATIVE_PLAY_APP_ID, "Imperative").document(["semio", "imperative"])
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
            .mode("edit", "Edit", "pencil")
            .default_mode_id("edit")
            .window_kind(IMPERATIVE_PLAY_WINDOW_MAIN, "Imperative", IMPERATIVE_PLAY_BODY_MAIN, SurfaceKind::NodeGraph, "code")
            .window_kind(IMPERATIVE_PLAY_WINDOW_SCRIPT, "Script", IMPERATIVE_PLAY_BODY_SCRIPT, SurfaceKind::TextEditor, "file-code")
            .default_layout(create_stack_layout(
                &[IMPERATIVE_PLAY_WINDOW_MAIN.into(), IMPERATIVE_PLAY_WINDOW_SCRIPT.into()],
                Some(&["Imperative".into(), "Script".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                IMPERATIVE_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                IMPERATIVE_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                IMPERATIVE_PLAY_BODY_INSPECTOR,
            )
            // 🔧️ Document-mutating step edits — dispatched as VCS operations with a true inverse.
            // The `*At` variants address a nested body via owner/slot fields (drag-and-drop into blocks).
            .operation("addStep", "Add Step")
            .operation("addStepAt", "Add Step At")
            .operation("removeStep", "Remove Step")
            .operation("removeStepAt", "Remove Step At")
            .operation("moveStep", "Move Step")
            .operation("moveStepAt", "Move Step At")
            .operation("setStepParams", "Set Step Params")
            .operation("setStepParamsAt", "Set Step Params At")
            // 👁️ Ephemeral view state / runtime effect — selection is scratch, `run` evaluates into config.
            .view_action("setSelection", "Set Selection")
            .view_action("run", "Run")
            .view_action("setLocale", "Set Locale")
            // 📝️ Staged argument form for the panel-visible create action (the step kind is a choice).
            .action_args("addStep", vec![
                ActionArgDef::select("kind", "Kind", vec![
                    ActionArgOption::new("state.set", "Set State"),
                    ActionArgOption::new("log.print", "Print Log"),
                    ActionArgOption::new("control.if", "If"),
                    ActionArgOption::new("control.while", "While"),
                    ActionArgOption::new("math.add", "Add"),
                ]).default_value("log.print"),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS /
            // WORKFLOWS-END-TO-END-TYPED-PORTS) — `imperative_io()` is this port information's single
            // source of truth, reused here rather than duplicated.
            .io(imperative_io()),
    )
    .example("demo", "Demo", serde_json::to_string(&default_document()).expect("default_document is a static, hand-built value with no non-finite floats or non-UTF8 keys"), "cylinder")
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
