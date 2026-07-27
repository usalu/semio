//! ⚡ Imperative plugin — declarative imperative play app bundled as a hot-swappable WASM component.

use imperative_core::{default_document, Dictionary, ImperativeDocument, ImperativeHost, ImperativeOperation, PathRef};
use imperative_engine::Step;
use semio_framework_plugin::{
    build_table_scene, build_text_editor_scene, create_stack_layout, is_de_locale, localized_label_map, resolve_labels, selection_ids, tree_item_with_action, ui_declarative_sections_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text,
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionEmit, App, AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DocumentView, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder, ResourceKindSpec, SurfaceKind, TableScene, TextEditorScene,
    UiNode, UiPresence, UiTreeItemNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use vcs::CollectionOperation;

//#region 🔖Constants
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
const IMPERATIVE_DOCUMENT_SCHEMA: &str = "imperative.document/v1";
//#endregion 🔖Constants

//#region 🔖Types
/// 🎛️ Ephemeral view state (step selection + last run output) — lives in the app struct, not the
/// document, so it never pollutes undo history.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ImperativePlayRuntime {
    selected_step_ids: Vec<String>,
    run_output_json: String,
}

#[derive(Serialize, Deserialize)]
struct TableRow {
    index: usize,
    id: String,
    kind: String,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
/// 🆔 Allocates a fresh `step-N` id one past the highest suffix used anywhere in the document
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

/// 📍 Reads `owner`/`slot` off action args into a [`imperative_core::PathRef`] so nested
/// control-step bodies (e.g. `control.if` then/else) resolve correctly; falls back to the root
/// path unless both are present and `owner` names a real top-level step, avoiding an unresolvable
/// or unknown reference that would otherwise address nothing.
fn path_ref_from_args(args: Option<&Value>, document: &ImperativeDocument) -> PathRef {
    let owner = args.and_then(|value| value.get("owner")).and_then(|value| value.as_str()).map(str::to_string);
    let slot = args.and_then(|value| value.get("slot")).and_then(|value| value.as_str()).map(str::to_string);
    match (owner, slot) {
        (Some(owner), Some(slot)) if document.path.steps.iter().any(|step| step.id == owner) => PathRef { owner: Some(owner), slot: Some(slot) },
        _ => PathRef::default(),
    }
}

fn table_rows(steps: &[Step]) -> String {
    let rows: Vec<TableRow> = steps.iter().enumerate().map(|(index, step)| TableRow { index: index + 1, id: step.id.clone(), kind: step.kind.clone() }).collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

fn imperative_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: IMPERATIVE_PLAY_APP_ID.into(), action: action.into(), args }
}
//#endregion 🔖DocumentHelpers

//#region 🔖Terminology
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
//#endregion 🔖Terminology

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_imperative_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the
/// command palette and Actions rail get a translated label without threading locale through the builder.
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
//#endregion 🔖CommandLabels

//#region 🔖Panels
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
}]);
    }
    let steps: Vec<&Step> = selected.iter().filter_map(|id| document.path.steps.iter().find(|step| &step.id == id)).collect();
    if steps.is_empty() {
        return ui_stack_vertical(vec![ui_text(labels.inspector_step_not_found)]);
    }
    ui_stack_vertical(vec![
        ui_inspector_readonly_field("imperative-play-inspector.id", labels.inspector_id, steps[0].id.clone()),
        ui_inspector_readonly_field("imperative-play-inspector.kind", labels.inspector_kind, steps[0].kind.clone()),
        ui_inspector_readonly_field("imperative-play-inspector.params", labels.inspector_params, serde_json::to_string(&steps[0].params).unwrap_or_else(|_| "{}".into())),
    ])
}
//#endregion 🔖Panels

//#region 🔖Render
/// 📤 One table row per scope key so the full run output is legible instead of an 80-char
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
//#endregion 🔖Render

//#region 🔖ImperativePlayApp
#[derive(Default)]
struct ImperativePlayApp {
    runtime: ImperativePlayRuntime,
}

impl DocumentApp for ImperativePlayApp {
    type Projection = ImperativeDocument;
    type Operation = ImperativeOperation;

    fn app_id(&self) -> &str {
        IMPERATIVE_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        IMPERATIVE_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> ImperativeDocument {
        default_document()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, ImperativeDocument>, _view_state: &ViewState) -> ActionEmit<ImperativeOperation> {
        let document = doc.projection;
        match action {
            "setSelection" => {
                self.runtime.selected_step_ids = selection_ids(args);
                ActionEmit::default()
            }
            "addStep" | "addStepAt" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("log.print");
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).map(|value| value as usize).unwrap_or(usize::MAX);
                let path_ref = path_ref_from_args(args, document);
                let id = next_step_id(document);
                let step = Step { id: id.clone(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() };
                self.runtime.selected_step_ids = vec![id];
                ActionEmit::operations(vec![ImperativeOperation { path_ref, collection: CollectionOperation::Add { index, item: step } }])
            }
            "removeStep" | "removeStepAt" => {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    if resolve_contains(document, args, id) {
                        let path_ref = path_ref_from_args(args, document);
                        self.runtime.selected_step_ids.retain(|step_id| step_id != id);
                        return ActionEmit::operations(vec![ImperativeOperation { path_ref, collection: CollectionOperation::Remove { id: id.into() } }]);
                    }
                }
                ActionEmit::default()
            }
            "moveStep" | "moveStepAt" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let new_index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).map(|value| value as usize);
                if let (Some(id), Some(new_index)) = (id, new_index) {
                    if resolve_contains(document, args, id) {
                        let path_ref = path_ref_from_args(args, document);
                        return ActionEmit::operations(vec![ImperativeOperation { path_ref, collection: CollectionOperation::Move { id: id.into(), to_index: new_index } }]);
                    }
                }
                ActionEmit::default()
            }
            "setStepParams" | "setStepParamsAt" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let params = args.and_then(|value| value.get("params"));
                if let (Some(id), Some(params)) = (id, params) {
                    if let Ok(patch) = serde_json::from_value::<Dictionary>(params.clone()) {
                        if resolve_contains(document, args, id) {
                            let path_ref = path_ref_from_args(args, document);
                            return ActionEmit::operations(vec![ImperativeOperation { path_ref, collection: CollectionOperation::Patch { id: id.into(), patch } }]);
                        }
                    }
                }
                ActionEmit::default()
            }
            "run" => {
                let host = ImperativeHost::from_document(document.clone());
                let result = host.run();
                self.runtime.run_output_json = serde_json::to_string(&result.scope).unwrap_or_else(|_| format!("{:?}", result.scope));
                ActionEmit::default()
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, ImperativeDocument>, view_state: &ViewState) -> UiNode {
        let document = doc.projection;
        let labels = resolve_labels::<ImperativeLabels>(view_state);
        match body_key {
            IMPERATIVE_PLAY_BODY_MAIN => render_main_table(document, &self.runtime.run_output_json, labels),
            IMPERATIVE_PLAY_BODY_SCRIPT => render_script(document),
            IMPERATIVE_PLAY_BODY_DOCUMENT => build_document_tree(document, &self.runtime.selected_step_ids, labels),
            IMPERATIVE_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            IMPERATIVE_PLAY_BODY_INSPECTOR => build_inspector_tree(document, &self.runtime.selected_step_ids, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<ImperativeLabels>(view_state);
        AppLabelsOverlay::default().window_kind_label(IMPERATIVE_PLAY_WINDOW_MAIN, labels.window_main).window_kind_label(IMPERATIVE_PLAY_WINDOW_SCRIPT, labels.window_script).action_labels(imperative_action_labels(is_de_locale(view_state)))
    }
}

/// 🔎 Resolves the step list a `PathRef` addresses — the root path, or a nested `control.*` step's
/// slot (an unmaterialized slot reads as empty).
fn steps_at<'a>(document: &'a ImperativeDocument, path_ref: &PathRef) -> &'a [Step] {
    match (&path_ref.owner, &path_ref.slot) {
        (Some(owner), Some(slot)) => document.path.steps.iter().find(|step| &step.id == owner).and_then(|step| step.bodies.get(slot)).map(|path| path.steps.as_slice()).unwrap_or(&[]),
        _ => document.path.steps.as_slice(),
    }
}

/// 🔎 True when the step `id` exists in the list the `owner`/`slot` args address — the pre-state
/// guard the operation arms share so a stale id never emits a no-operation edit into history.
fn resolve_contains(document: &ImperativeDocument, args: Option<&Value>, id: &str) -> bool {
    let path_ref = path_ref_from_args(args, document);
    steps_at(document, &path_ref).iter().any(|step| step.id == id)
}
//#endregion 🔖ImperativePlayApp

//#region 🔖Manifest
fn create_imperative_app() -> App {
    App::from_builder(
        App::builder(IMPERATIVE_PLAY_APP_ID, "Imperative").document(["semio", "imperative"])
            .resource_kind(ResourceKindSpec {
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
            .mode("edit", "Edit")
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
            // 🔧 Document-mutating step edits — dispatched as VCS operations with a true inverse.
            // The `*At` variants address a nested body via owner/slot args (drag-and-drop into blocks).
            .operation("addStep", "Add Step")
            .operation("addStepAt", "Add Step At")
            .operation("removeStep", "Remove Step")
            .operation("removeStepAt", "Remove Step At")
            .operation("moveStep", "Move Step")
            .operation("moveStepAt", "Move Step At")
            .operation("setStepParams", "Set Step Params")
            .operation("setStepParamsAt", "Set Step Params At")
            // 👁️ Ephemeral view state / runtime effect — selection is scratch, `run` evaluates into runtime.
            .view_action("setSelection", "Set Selection")
            .view_action("run", "Run")
            // 📝 Staged argument form for the panel-visible create action (the step kind is a choice).
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
            .keybinding("mod+shift+z", "redo"),
    )
    .example("demo", "Demo", serde_json::to_string(&default_document()).expect("default_document is a static, hand-built value with no non-finite floats or non-UTF8 keys"))
    .program("imperative", "Imperative", "graph")
}

fn register_imperative_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<ImperativePlayApp>(IMPERATIVE_DOCUMENT_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "imperative",
    label: "Imperative",
    version: "0.1.0",
    setup: register_imperative_exports,
    apps: [ create_imperative_app => ImperativePlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<ImperativePlayApp> {
        testkit::new_app::<ImperativePlayApp>()
    }

    /// 🧬 A wrapper carrying the real action registry so `addStep`'s `kind` default materializes and the
    /// View-kind `run` action is held to the no-operations contract.
    fn new_app_with_registry() -> VcsDocumentApp<ImperativePlayApp> {
        testkit::new_app_with_registry::<ImperativePlayApp>(create_imperative_app)
    }

    /// 🧬 The exact document `default_document()` becomes after `addStep` materializes `id`/`kind` with
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
    fn add_step_materializes_kind_default_and_run_emits_no_operations() {
        let mut app = new_app_with_registry();
        // addStep fired with no args: the declared `kind` default ("log.print") must be materialized.
        app.handle_action("addStep", None, &ViewState::default(), &testkit::meta("local")).expect("add step");
        let document = app.projection().expect("materialize projection");
        assert_eq!(document.path.steps.last().unwrap().kind, "log.print", "kind default materialized from the registry");
        // `run` is a View-kind action: under registry enforcement it must not emit document operations.
        let result = app.handle_action("run", None, &ViewState::default(), &testkit::meta("local")).expect("run");
        assert!(result.operations.is_empty(), "run evaluates into runtime, never the document");
    }

    #[test]
    fn renders_table_scene() {
        let mut app = new_app();
        let node = app.render(IMPERATIVE_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("table"));
    }

    #[test]
    fn imperative_labels_resolve_native_by_default() {
        let mut app = new_app();
        let node = app.render(IMPERATIVE_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Set state"));
        assert!(json.contains("Print log"));
        assert!(json.contains("While"));
    }

    #[test]
    fn imperative_labels_resolve_native_in_german() {
        let mut app = new_app();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let node = app.render(IMPERATIVE_PLAY_BODY_CATALOGUE, None, &view_state).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Zustand setzen"));
        assert!(json.contains("Log ausgeben"));
        assert!(json.contains("Solange"));
    }

    #[test]
    fn renders_script_editor() {
        let mut app = new_app();
        let node = app.render(IMPERATIVE_PLAY_BODY_SCRIPT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn default_document_has_steps() {
        let app = new_app();
        assert_eq!(app.projection().expect("projection").path.steps.len(), 2);
    }

    #[test]
    fn add_step_action_appends_step() {
        let mut app = new_app();
        app.handle_action("addStep", Some(&json!({ "kind": "log.print" })), &ViewState::default(), &testkit::meta("local")).expect("add step");
        assert!(app.projection().expect("projection").path.steps.len() > 2);
    }

    #[test]
    fn add_step_at_owner_slot_nests_into_control_body() {
        let mut app = new_app();
        app.handle_action("addStepAt", Some(&json!({ "kind": "control.if" })), &ViewState::default(), &testkit::meta("local")).expect("add owner");
        let owner_id = app.projection().expect("projection").path.steps.last().expect("owner").id.clone();
        let root_len = app.projection().expect("projection").path.steps.len();
        app.handle_action("addStepAt", Some(&json!({ "kind": "log.print", "owner": owner_id, "slot": "then" })), &ViewState::default(), &testkit::meta("local")).expect("add nested");
        let document = app.projection().expect("projection");
        let owner_step = document.path.steps.iter().find(|step| step.id == owner_id).expect("owner step");
        assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
        assert_eq!(document.path.steps.len(), root_len, "nested step lives in the slot, not the root path");
    }

    #[test]
    fn add_step_at_falls_back_to_root_for_unknown_owner() {
        let mut app = new_app();
        app.handle_action("addStepAt", Some(&json!({ "kind": "log.print", "owner": "missing-step", "slot": "then" })), &ViewState::default(), &testkit::meta("local")).expect("add step");
        let document = app.projection().expect("projection");
        let added_id = document.path.steps.last().expect("added").id.clone();
        assert!(document.path.steps.iter().any(|step| step.id == added_id));
    }

    #[test]
    fn run_action_expands_scope_into_readable_rows_without_truncation() {
        let mut app = new_app();
        app.handle_action("run", None, &ViewState::default(), &testkit::meta("local")).expect("run");
        let node = app.render(IMPERATIVE_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("counter"), "run output row shows the full scope key, not an 80-char blob");
    }

    #[test]
    fn undo_after_add_step_restores_original_document_exactly() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(&mut app, "addStep", Some(&json!({ "kind": "log.print" })), |app| app.projection().expect("projection"), default_document(), expected_document_after_add_step("log.print", "step-3"));
    }

    #[test]
    fn remove_step_action_is_exact_inverse_of_add() {
        let mut app = new_app();
        let original = app.projection().expect("projection");
        app.handle_action("addStep", Some(&json!({ "kind": "math.add" })), &ViewState::default(), &testkit::meta("local")).expect("add step");
        let added_id = app.projection().expect("projection").path.steps.last().expect("added").id.clone();
        app.handle_action("removeStep", Some(&json!({ "id": added_id })), &ViewState::default(), &testkit::meta("local")).expect("remove step");
        assert_eq!(app.projection().expect("projection"), original);
    }

    /// 🧪 The definitional regression proof: two independent instances start from the same document,
    /// apply DISJOINT edits (A appends a root step, B patches an existing step's params), and
    /// exchanging operations over a `MemoryBackbone` converges both sides onto an identical projection —
    /// impossible under whole-document `setDocument` snapshots, which would clobber one side's write.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<ImperativePlayApp, _>("mem://imperative-convergence", ("addStep", Some(&json!({ "kind": "math.add" }))), ("setStepParams", Some(&json!({ "id": "step-1", "params": { "key": "renamed" } }))), |app| {
            app.projection().expect("projection")
        });
    }

    #[test]
    fn ingest_operations_is_idempotent_for_imperative() {
        testkit::assert_ingest_idempotent::<ImperativePlayApp, _>("addStep", Some(&json!({ "kind": "math.add" })), |app| app.projection().expect("projection").path.steps.len());
    }
}
//#endregion 🧪Tests
