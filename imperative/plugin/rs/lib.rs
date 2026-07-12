//! ⚡ Imperative plugin — declarative imperative play app bundled as a hot-swappable WASM component.

use imperative_core::{default_document, Dictionary, ImperativeDocument, ImperativeHost, ImperativeOp, PathRef};
use imperative_engine::Step;
use semio_framework_plugin::{SurfaceKind, PanelGroup,
    build_table_scene, build_text_editor_scene, create_stack_layout, ui_declarative_sections_to_tree,
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, ActionDescriptor, PluginApp, PluginBundle,
    TableScene, TextEditorScene, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use vcs::{
    create_document_vcs_envelope, materialize_document_projection, CollectionOp, DocumentVcsCommand,
    DocumentVcsEnvelope, DocumentVcsStore,
};

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
type ImperativeStore = DocumentVcsStore<ImperativeDocument, ImperativeOp>;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImperativePlayRuntime {
    #[serde(default)]
    selected_step_ids: Vec<String>,
    #[serde(default)]
    run_output_json: String,
}

/// 🗄️ The document is a VCS envelope materialized by operation replay — no plain-JSON document,
/// no separate undo/redo stacks; history lives entirely in `envelope.vcs.edits`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImperativePlayEnvelope {
    envelope: DocumentVcsEnvelope<ImperativeDocument, ImperativeOp>,
    #[serde(default)]
    applied_edit_ids: Vec<String>,
    #[serde(default)]
    redo_edit_ids: Vec<String>,
    #[serde(default)]
    runtime: ImperativePlayRuntime,
}

#[derive(Serialize, Deserialize)]
struct TableRow {
    index: usize,
    id: String,
    kind: String,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
fn default_envelope() -> ImperativePlayEnvelope {
    ImperativePlayEnvelope {
        envelope: create_document_vcs_envelope(IMPERATIVE_DOCUMENT_SCHEMA, IMPERATIVE_PLAY_APP_ID, default_document(), None),
        applied_edit_ids: Vec::new(),
        redo_edit_ids: Vec::new(),
        runtime: ImperativePlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> ImperativePlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(play: &ImperativePlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": play }).to_string()
}

/// @emoji 🗄️ Reconstructs the VCS store from the persisted envelope + applied/redo edit ids.
fn store_from_play(play: &ImperativePlayEnvelope) -> ImperativeStore {
    let mut store = ImperativeStore::new(play.envelope.clone());
    store.set_state(play.envelope.clone(), play.applied_edit_ids.clone(), play.redo_edit_ids.clone());
    store
}

/// @emoji 💾 Persists the store's envelope + applied/redo edit ids back onto the play state.
fn sync_play_from_store(play: &mut ImperativePlayEnvelope, store: &ImperativeStore) {
    play.envelope = store.envelope().clone();
    play.applied_edit_ids = store.applied_edit_ids().to_vec();
    play.redo_edit_ids = store.redo_edit_ids().to_vec();
}

/// @emoji 🔂 Materializes the current document by replaying applied edits over the initial projection.
fn document(play: &ImperativePlayEnvelope) -> ImperativeDocument {
    materialize_document_projection(&play.envelope, &play.applied_edit_ids)
        .unwrap_or_else(|_| play.envelope.vcs.initial_projection.clone())
}

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
/// or unknown reference that would otherwise panic the host.
fn path_ref_from_args(args: Option<&Value>, document: &ImperativeDocument) -> PathRef {
    let owner = args.and_then(|value| value.get("owner")).and_then(|value| value.as_str()).map(str::to_string);
    let slot = args.and_then(|value| value.get("slot")).and_then(|value| value.as_str()).map(str::to_string);
    match (owner, slot) {
        (Some(owner), Some(slot)) if document.path.steps.iter().any(|step| step.id == owner) => {
            PathRef { owner: Some(owner), slot: Some(slot) }
        }
        _ => PathRef::default(),
    }
}

fn table_rows(steps: &[Step]) -> String {
    let rows: Vec<TableRow> = steps
        .iter()
        .enumerate()
        .map(|(index, step)| TableRow {
            index: index + 1,
            id: step.id.clone(),
            kind: step.kind.clone(),
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
        selected: None,
        default_open: None,
        action: Some(action),
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn imperative_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: IMPERATIVE_PLAY_APP_ID.into(),
        action: action.into(),
        args,
    }
}
//#endregion 🔖DocumentHelpers

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the imperative app; one field per label makes every locale combination compile-checked.
struct ImperativeLabels {
    window_main: &'static str,
    window_script: &'static str,
    col_index: &'static str,
    col_id: &'static str,
    col_kind: &'static str,
    action_state_set: &'static str,
    action_log_print: &'static str,
    action_control_if: &'static str,
    action_control_while: &'static str,
    action_math_add: &'static str,
    document_empty: &'static str,
    inspector_empty_hint: &'static str,
    inspector_step_not_found: &'static str,
    inspector_id: &'static str,
    inspector_kind: &'static str,
    inspector_params: &'static str,
}

const IMPERATIVE_LABELS_NATIVE_EN: ImperativeLabels = ImperativeLabels {
    window_main: "Imperative",
    window_script: "Script",
    col_index: "#",
    col_id: "Id",
    col_kind: "Kind",
    action_state_set: "Set state",
    action_log_print: "Print log",
    action_control_if: "If",
    action_control_while: "While",
    action_math_add: "Add",
    document_empty: "(none)",
    inspector_empty_hint: "Select a step in the document.",
    inspector_step_not_found: "Step not found",
    inspector_id: "Id",
    inspector_kind: "Kind",
    inspector_params: "Params",
};

const IMPERATIVE_LABELS_NATIVE_DE: ImperativeLabels = ImperativeLabels {
    window_main: "Imperativ",
    window_script: "Skript",
    col_index: "#",
    col_id: "ID",
    col_kind: "Art",
    action_state_set: "Zustand setzen",
    action_log_print: "Log ausgeben",
    action_control_if: "Wenn",
    action_control_while: "Solange",
    action_math_add: "Addieren",
    document_empty: "(keine)",
    inspector_empty_hint: "Wählen Sie einen Schritt im Dokument aus.",
    inspector_step_not_found: "Schritt nicht gefunden",
    inspector_id: "ID",
    inspector_kind: "Art",
    inspector_params: "Parameter",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; unknown locales fall back to native English.
fn imperative_labels(view_state: &ViewState) -> &'static ImperativeLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de {
        &IMPERATIVE_LABELS_NATIVE_DE
    } else {
        &IMPERATIVE_LABELS_NATIVE_EN
    }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn build_document_tree(document: &ImperativeDocument, selected: &[String], labels: &ImperativeLabels) -> UiNode {
    let step_items: Vec<UiTreeItemNode> = document
        .path
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| {
            tree_item_with_action(
                format!("imperative-play-document.step.{}", step.id),
                format!("{}. {}", index + 1, step.kind),
                Some(step.id.clone()),
                imperative_action("setSelection", Some(json!({ "ids": [step.id.clone()] }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "imperative-play-document.steps".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            default_open: Some(true),
            items: if step_items.is_empty() {
                vec![tree_item("imperative-play-document.steps.empty", labels.document_empty)]
            } else {
                step_items
            },
        }],
        selected_ids: Some(selected.iter().map(|id| format!("imperative-play-document.step.{id}")).collect()),
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn build_catalogue_tree(labels: &ImperativeLabels) -> UiNode {
    let actions = [
        ("state.set", labels.action_state_set),
        ("log.print", labels.action_log_print),
        ("control.if", labels.action_control_if),
        ("control.while", labels.action_control_while),
        ("math.add", labels.action_math_add),
    ];
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "imperative-play-catalogue.actions".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items: actions
                .iter()
                .map(|(kind, label)| {
                    tree_item_with_action(
                        format!("imperative-play-catalogue.action.{kind}"),
                        *label,
                        Some((*kind).into()),
                        imperative_action("addStep", Some(json!({ "kind": kind }))),
                    )
                })
                .collect(),
        }],
        selected_ids: Some(vec![]),
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn build_inspector_tree(document: &ImperativeDocument, selected: &[String], labels: &ImperativeLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "imperative-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text(labels.inspector_empty_hint)],
        }]);
    }
    let steps: Vec<&Step> = selected
        .iter()
        .filter_map(|id| document.path.steps.iter().find(|step| &step.id == id))
        .collect();
    if steps.is_empty() {
        return ui_stack_vertical(vec![ui_text(labels.inspector_step_not_found)]);
    }
    ui_stack_vertical(vec![
        ui_inspector_readonly_field("imperative-play-inspector.id", labels.inspector_id, steps[0].id.clone()),
        ui_inspector_readonly_field("imperative-play-inspector.kind", labels.inspector_kind, steps[0].kind.clone()),
        ui_inspector_readonly_field(
            "imperative-play-inspector.params",
            labels.inspector_params,
            serde_json::to_string(&steps[0].params).unwrap_or_else(|_| "{}".into()),
        ),
    ])
}
//#endregion 🔖Panels

//#region 🔖Render
/// 📤 One table row per scope key so the full run output is legible instead of an 80-char
/// truncated blob; falls back to the raw JSON when it isn't a plain object.
fn run_output_rows(run_output_json: &str, offset: usize) -> Vec<TableRow> {
    match serde_json::from_str::<Value>(run_output_json).ok().and_then(|value| value.as_object().cloned()) {
        Some(scope) if !scope.is_empty() => scope
            .into_iter()
            .enumerate()
            .map(|(index, (key, value))| TableRow {
                index: offset + index + 1,
                id: format!("run-output.{key}"),
                kind: format!("{key} = {}", serde_json::to_string(&value).unwrap_or_else(|_| "null".into())),
            })
            .collect(),
        _ => vec![TableRow {
            index: offset + 1,
            id: "run-output".into(),
            kind: run_output_json.to_string(),
        }],
    }
}

fn render_main_table(play: &ImperativePlayEnvelope, labels: &ImperativeLabels) -> UiNode {
    let document = document(play);
    let mut rows_json = table_rows(&document.path.steps);
    if !play.runtime.run_output_json.is_empty() {
        if let Ok(mut rows) = serde_json::from_str::<Vec<TableRow>>(&rows_json) {
            rows.extend(run_output_rows(&play.runtime.run_output_json, rows.len()));
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

fn render_script(play: &ImperativePlayEnvelope) -> UiNode {
    let host = ImperativeHost::from_document(document(play));
    build_text_editor_scene(
        IMPERATIVE_PLAY_SURFACE_SCRIPT,
        IMPERATIVE_PLAY_APP_ID,
        TextEditorScene::base(host.compile_text(), Some("imperative".into()), None),
    )
}
//#endregion 🔖Render

//#region 🔖ImperativePlayApp
struct ImperativePlayApp;

impl PluginApp for ImperativePlayApp {
    fn app_id(&self) -> &str {
        IMPERATIVE_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("imperative envelope json")
    }

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        match action {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                play.runtime.selected_step_ids = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                return vec![set_document_op(&play)];
            }
            "addStep" | "addStepAt" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("log.print");
                let index = args
                    .and_then(|value| value.get("index"))
                    .and_then(|value| value.as_u64())
                    .map(|value| value as usize)
                    .unwrap_or(usize::MAX);
                let pre = document(&play);
                let path_ref = path_ref_from_args(args, &pre);
                let id = next_step_id(&pre);
                let step = Step {
                    id: id.clone(),
                    kind: kind.into(),
                    params: Dictionary::new(),
                    bodies: BTreeMap::new(),
                };
                let op = ImperativeOp { path_ref, collection: CollectionOp::Add { index, item: step } };
                let mut store = store_from_play(&play);
                let _ = store.dispatch(DocumentVcsCommand::Apply { operations: vec![op], description: None });
                sync_play_from_store(&mut play, &store);
                play.runtime.selected_step_ids = vec![id];
                return vec![set_document_op(&play)];
            }
            "removeStep" | "removeStepAt" => {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    let pre = document(&play);
                    let path_ref = path_ref_from_args(args, &pre);
                    let op = ImperativeOp { path_ref, collection: CollectionOp::Remove { id: id.into() } };
                    let mut store = store_from_play(&play);
                    let _ = store.dispatch(DocumentVcsCommand::Apply { operations: vec![op], description: None });
                    sync_play_from_store(&mut play, &store);
                    play.runtime.selected_step_ids.retain(|step_id| step_id != id);
                    return vec![set_document_op(&play)];
                }
            }
            "moveStep" | "moveStepAt" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let new_index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).map(|value| value as usize);
                if let (Some(id), Some(new_index)) = (id, new_index) {
                    let pre = document(&play);
                    let path_ref = path_ref_from_args(args, &pre);
                    let op = ImperativeOp { path_ref, collection: CollectionOp::Move { id: id.into(), to_index: new_index } };
                    let mut store = store_from_play(&play);
                    let _ = store.dispatch(DocumentVcsCommand::Apply { operations: vec![op], description: None });
                    sync_play_from_store(&mut play, &store);
                    return vec![set_document_op(&play)];
                }
            }
            "setStepParams" | "setStepParamsAt" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let params = args.and_then(|value| value.get("params"));
                if let (Some(id), Some(params)) = (id, params) {
                    if let Ok(patch) = serde_json::from_value::<Dictionary>(params.clone()) {
                        let pre = document(&play);
                        let path_ref = path_ref_from_args(args, &pre);
                        let op = ImperativeOp { path_ref, collection: CollectionOp::Patch { id: id.into(), patch } };
                        let mut store = store_from_play(&play);
                        let _ = store.dispatch(DocumentVcsCommand::Apply { operations: vec![op], description: None });
                        sync_play_from_store(&mut play, &store);
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "run" => {
                let host = ImperativeHost::from_document(document(&play));
                let result = host.run();
                play.runtime.run_output_json =
                    serde_json::to_string(&result.scope).unwrap_or_else(|_| format!("{:?}", result.scope));
                return vec![set_document_op(&play)];
            }
            "undo" => {
                let mut store = store_from_play(&play);
                let _ = store.dispatch(DocumentVcsCommand::Undo);
                sync_play_from_store(&mut play, &store);
                return vec![set_document_op(&play)];
            }
            "redo" => {
                let mut store = store_from_play(&play);
                let _ = store.dispatch(DocumentVcsCommand::Redo);
                sync_play_from_store(&mut play, &store);
                return vec![set_document_op(&play)];
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        let labels = imperative_labels(view_state);
        match body_key {
            IMPERATIVE_PLAY_BODY_MAIN => render_main_table(&play, labels),
            IMPERATIVE_PLAY_BODY_SCRIPT => render_script(&play),
            IMPERATIVE_PLAY_BODY_DOCUMENT => build_document_tree(&document(&play), &play.runtime.selected_step_ids, labels),
            IMPERATIVE_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            IMPERATIVE_PLAY_BODY_INSPECTOR => build_inspector_tree(&document(&play), &play.runtime.selected_step_ids, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = imperative_labels(view_state);
        semio_framework_plugin::AppLabelsOverlay {
            app_label: None,
            window_kind_labels: std::collections::HashMap::from([
                (IMPERATIVE_PLAY_WINDOW_MAIN.to_string(), labels.window_main.to_string()),
                (IMPERATIVE_PLAY_WINDOW_SCRIPT.to_string(), labels.window_script.to_string()),
            ]),
            panel_tab_labels: std::collections::HashMap::new(),
            mode_labels: std::collections::HashMap::new(),
        }
    }
}
//#endregion 🔖ImperativePlayApp

//#region 🔖Manifest
fn create_imperative_app() -> App {
    App::from_builder(
        App::builder(IMPERATIVE_PLAY_APP_ID, "Imperative").document(["semio", "imperative"])
            .icon_id("imperative")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(IMPERATIVE_PLAY_WINDOW_MAIN, "Imperative", IMPERATIVE_PLAY_BODY_MAIN, SurfaceKind::NodeGraph)
            .window_kind(IMPERATIVE_PLAY_WINDOW_SCRIPT, "Script", IMPERATIVE_PLAY_BODY_SCRIPT, SurfaceKind::TextEditor)
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
            .operation("addStep", "Add Step")
            .operation("addStepAt", "Add Step At")
            .operation("removeStep", "Remove Step")
            .operation("removeStepAt", "Remove Step At")
            .operation("moveStep", "Move Step")
            .operation("moveStepAt", "Move Step At")
            .operation("setStepParams", "Set Step Params")
            .operation("setStepParamsAt", "Set Step Params At")
            .view_action("setSelection", "Set Selection")
            .view_action("run", "Run")
            .shell_action("setDocument", "Set Document")
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("demo", "Demo", serde_json::to_string(&default_envelope()).unwrap())
    .program("imperative", "Imperative", "graph")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("imperative", "Imperative", "0.1.0").register_app(create_imperative_app(), || Box::new(ImperativePlayApp))
}

semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖Manifest

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_definition_builds_without_panicking() {
        let app = create_imperative_app();
        assert!(app.definition.actions.iter().any(|action| action.id == "addStep"));
        assert!(app.definition.actions.iter().any(|action| action.id == "undo"));
    }

    #[test]
    fn renders_table_scene() {
        let app = ImperativePlayApp;
        let document = app.initial_document_json();
        let node = app.render(IMPERATIVE_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("table"));
    }

    #[test]
    fn imperative_labels_resolve_native_by_default() {
        let app = ImperativePlayApp;
        let document = app.initial_document_json();
        let node = app.render(IMPERATIVE_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Set state"));
        assert!(json.contains("Print log"));
        assert!(json.contains("While"));
    }

    #[test]
    fn imperative_labels_resolve_native_in_german() {
        let app = ImperativePlayApp;
        let document = app.initial_document_json();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let node = app.render(IMPERATIVE_PLAY_BODY_CATALOGUE, &document, &view_state);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Zustand setzen"));
        assert!(json.contains("Log ausgeben"));
        assert!(json.contains("Solange"));
    }

    #[test]
    fn renders_script_editor() {
        let app = ImperativePlayApp;
        let document = app.initial_document_json();
        let node = app.render(IMPERATIVE_PLAY_BODY_SCRIPT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn default_document_has_steps() {
        let play = default_envelope();
        assert_eq!(document(&play).path.steps.len(), 2);
    }

    #[test]
    fn add_step_action_appends_step() {
        let mut app = ImperativePlayApp;
        let document_json = app.initial_document_json();
        let ops = app.handle_action_patch_ops("addStep", Some(&json!({ "kind": "log.print" })), &document_json, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated_op: Value = serde_json::from_str(&ops[0]).unwrap();
        let updated: ImperativePlayEnvelope = serde_json::from_value(updated_op["document"].clone()).unwrap();
        assert!(document(&updated).path.steps.len() > 2);
    }

    fn apply_ops(document_json: &str, ops: &[String]) -> ImperativePlayEnvelope {
        let mut play = parse_envelope(document_json);
        for op in ops {
            let parsed: Value = serde_json::from_str(op).unwrap();
            play = serde_json::from_value(parsed["document"].clone()).unwrap();
        }
        play
    }

    #[test]
    fn add_step_at_owner_slot_nests_into_control_body() {
        let mut app = ImperativePlayApp;
        let document_json = app.initial_document_json();
        let ops = app.handle_action_patch_ops("addStepAt", Some(&json!({ "kind": "control.if" })), &document_json, &ViewState::default());
        let with_owner = apply_ops(&document_json, &ops);
        let owner_id = with_owner.runtime.selected_step_ids[0].clone();
        let document_json = serde_json::to_string(&with_owner).unwrap();
        let ops = app.handle_action_patch_ops(
            "addStepAt",
            Some(&json!({ "kind": "log.print", "owner": owner_id, "slot": "then" })),
            &document_json,
            &ViewState::default(),
        );
        let nested = apply_ops(&document_json, &ops);
        let nested_document = document(&nested);
        let owner_step = nested_document.path.steps.iter().find(|step| step.id == owner_id).expect("owner step");
        assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
        assert_eq!(
            nested_document.path.steps.len(),
            document(&with_owner).path.steps.len(),
            "nested step lives in the slot, not the root path"
        );
    }

    #[test]
    fn add_step_at_falls_back_to_root_for_unknown_owner() {
        let mut app = ImperativePlayApp;
        let document_json = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "addStepAt",
            Some(&json!({ "kind": "log.print", "owner": "missing-step", "slot": "then" })),
            &document_json,
            &ViewState::default(),
        );
        let updated = apply_ops(&document_json, &ops);
        let updated_document = document(&updated);
        assert!(updated_document.path.steps.iter().any(|step| step.id == updated.runtime.selected_step_ids[0]));
    }

    #[test]
    fn run_action_expands_scope_into_readable_rows_without_truncation() {
        let mut app = ImperativePlayApp;
        let document_json = app.initial_document_json();
        let ops = app.handle_action_patch_ops("run", None, &document_json, &ViewState::default());
        let ran = apply_ops(&document_json, &ops);
        assert!(!ran.runtime.run_output_json.is_empty());
        let node = app.render(IMPERATIVE_PLAY_BODY_MAIN, &serde_json::to_string(&ran).unwrap(), &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("counter"), "run output row shows the full scope key, not an 80-char blob");
    }

    #[test]
    fn undo_after_add_step_restores_original_document_exactly() {
        let mut app = ImperativePlayApp;
        let document_json = app.initial_document_json();
        let original = document(&parse_envelope(&document_json));
        let ops = app.handle_action_patch_ops("addStep", Some(&json!({ "kind": "log.print" })), &document_json, &ViewState::default());
        let after_add = apply_ops(&document_json, &ops);
        let document_json = serde_json::to_string(&after_add).unwrap();
        let ops = app.handle_action_patch_ops("undo", None, &document_json, &ViewState::default());
        let after_undo = apply_ops(&document_json, &ops);
        assert_eq!(document(&after_undo), original, "undo is a true inverse of addStep");
        let ops = app.handle_action_patch_ops("redo", None, &serde_json::to_string(&after_undo).unwrap(), &ViewState::default());
        let after_redo = apply_ops(&serde_json::to_string(&after_undo).unwrap(), &ops);
        assert_eq!(document(&after_redo), document(&after_add), "redo restores the post-apply state");
    }

    #[test]
    fn remove_step_action_is_exact_inverse_of_add() {
        let mut app = ImperativePlayApp;
        let document_json = app.initial_document_json();
        let original = document(&parse_envelope(&document_json));
        let ops = app.handle_action_patch_ops("addStep", Some(&json!({ "kind": "math.add" })), &document_json, &ViewState::default());
        let after_add = apply_ops(&document_json, &ops);
        let added_id = after_add.runtime.selected_step_ids[0].clone();
        let document_json = serde_json::to_string(&after_add).unwrap();
        let ops = app.handle_action_patch_ops("removeStep", Some(&json!({ "id": added_id })), &document_json, &ViewState::default());
        let after_remove = apply_ops(&document_json, &ops);
        assert_eq!(document(&after_remove), original);
    }
}
