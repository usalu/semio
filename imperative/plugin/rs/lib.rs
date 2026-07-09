//! ⚡ Imperative plugin — declarative imperative play app bundled as a hot-swappable WASM component.

use imperative_core::{default_document, ImperativeDocument, ImperativeHost};
use imperative_engine::Step;
use semio_framework_plugin::{SurfaceKind, PanelGroup, 
    build_table_scene, build_text_editor_scene, create_stack_layout, ui_declarative_sections_to_tree,
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, CommandDescriptor, PluginApp, PluginBundle,
    TableScene, TextEditorScene, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;

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
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImperativePlayRuntime {
    #[serde(default)]
    selected_step_ids: Vec<String>,
    #[serde(default)]
    run_output_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImperativePlayEnvelope {
    document: ImperativeDocument,
    #[serde(default)]
    runtime: ImperativePlayRuntime,
    #[serde(default)]
    undo_stack: Vec<ImperativeDocument>,
    #[serde(default)]
    redo_stack: Vec<ImperativeDocument>,
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
        document: default_document(),
        runtime: ImperativePlayRuntime::default(),
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
    }
}

fn parse_envelope(document_json: &str) -> ImperativePlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &ImperativePlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn push_undo_imperative(play: &mut ImperativePlayEnvelope) {
    play.undo_stack.push(play.document.clone());
    if play.undo_stack.len() > 32 {
        play.undo_stack.remove(0);
    }
    play.redo_stack.clear();
}

fn imperative_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: IMPERATIVE_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn host_from_envelope(envelope: &ImperativePlayEnvelope) -> ImperativeHost {
    ImperativeHost::from_document(envelope.document.clone())
}

/// 📍 Reads `owner`/`slot` off command args into a [`imperative_core::PathRef`] so nested
/// control-step bodies (e.g. `control.if` then/else) resolve correctly; falls back to the root
/// path unless both are present and `owner` names a real top-level step, avoiding an unresolvable
/// or unknown reference that would otherwise panic the host.
fn path_ref_from_args(args: Option<&Value>, document: &ImperativeDocument) -> imperative_core::PathRef {
    let owner = args.and_then(|value| value.get("owner")).and_then(|value| value.as_str()).map(str::to_string);
    let slot = args.and_then(|value| value.get("slot")).and_then(|value| value.as_str()).map(str::to_string);
    match (owner, slot) {
        (Some(owner), Some(slot)) if document.path.steps.iter().any(|step| step.id == owner) => {
            imperative_core::PathRef { owner: Some(owner), slot: Some(slot) }
        }
        _ => imperative_core::PathRef::default(),
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
        command: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn tree_item_with_command(id: impl Into<String>, label: impl Into<String>, description: Option<String>, command: CommandDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
        selected: None,
        default_open: None,
        command: Some(command),
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}
//#endregion 🔖DocumentHelpers

//#region 🔖Panels
fn build_document_tree(document: &ImperativeDocument, selected: &[String]) -> UiNode {
    let step_items: Vec<UiTreeItemNode> = document
        .path
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| {
            tree_item_with_command(
                format!("imperative-play-document.step.{}", step.id),
                format!("{}. {}", index + 1, step.kind),
                Some(step.id.clone()),
                imperative_cmd("setSelection", Some(json!({ "ids": [step.id.clone()] }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "imperative-play-document.steps".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            default_open: Some(true),
            items: if step_items.is_empty() {
                vec![tree_item("imperative-play-document.steps.empty", "(none)")]
            } else {
                step_items
            },
        }],
        selected_ids: Some(selected.iter().map(|id| format!("imperative-play-document.step.{id}")).collect()),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    let actions = [
        ("state.set", "Set state"),
        ("log.print", "Print log"),
        ("control.if", "If"),
        ("control.while", "While"),
        ("math.add", "Add"),
    ];
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "imperative-play-catalogue.actions".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            items: actions
                .iter()
                .map(|(kind, label)| {
                    tree_item_with_command(
                        format!("imperative-play-catalogue.action.{kind}"),
                        *label,
                        Some((*kind).into()),
                        imperative_cmd("addStep", Some(json!({ "kind": kind }))),
                    )
                })
                .collect(),
        }],
        selected_ids: Some(vec![]),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(document: &ImperativeDocument, selected: &[String]) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "imperative-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Select a step in the document.")],
        }]);
    }
    let steps: Vec<&Step> = selected
        .iter()
        .filter_map(|id| document.path.steps.iter().find(|step| &step.id == id))
        .collect();
    if steps.is_empty() {
        return ui_stack_vertical(vec![ui_text("Step not found")]);
    }
    ui_stack_vertical(vec![
        ui_inspector_readonly_field("imperative-play-inspector.id", "Id", steps[0].id.clone()),
        ui_inspector_readonly_field("imperative-play-inspector.kind", "Kind", steps[0].kind.clone()),
        ui_inspector_readonly_field(
            "imperative-play-inspector.params",
            "Params",
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

fn render_main_table(envelope: &ImperativePlayEnvelope) -> UiNode {
    let mut rows_json = table_rows(&envelope.document.path.steps);
    if !envelope.runtime.run_output_json.is_empty() {
        if let Ok(mut rows) = serde_json::from_str::<Vec<TableRow>>(&rows_json) {
            rows.extend(run_output_rows(&envelope.runtime.run_output_json, rows.len()));
            rows_json = serde_json::to_string(&rows).unwrap_or(rows_json);
        }
    }
    build_table_scene(
        IMPERATIVE_PLAY_SURFACE_MAIN,
        IMPERATIVE_PLAY_APP_ID,
        TableScene {
            columns_json: json!([{"id":"index","label":"#"},{"id":"id","label":"Id"},{"id":"kind","label":"Kind"}]).to_string(),
            rows_json,
        },
    )
}

fn render_script(envelope: &ImperativePlayEnvelope) -> UiNode {
    let host = host_from_envelope(envelope);
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

    fn handle_command_patch_ops(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        let mut host = host_from_envelope(&envelope);
        match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                envelope.runtime.selected_step_ids = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                return vec![set_document_op(&envelope)];
            }
            "addStep" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("log.print");
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).map(|value| value as usize);
                push_undo_imperative(&mut envelope);
                let id = host.add_step(kind, index);
                envelope.document = host.document;
                envelope.runtime.selected_step_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "addStepAt" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("log.print");
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).map(|value| value as usize);
                let path_ref = path_ref_from_args(args, &envelope.document);
                push_undo_imperative(&mut envelope);
                let id = host.add_step_at(&path_ref, kind, index);
                envelope.document = host.document;
                envelope.runtime.selected_step_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "removeStepAt" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                if let Some(id) = id {
                    let path_ref = path_ref_from_args(args, &envelope.document);
                    push_undo_imperative(&mut envelope);
                    if host.remove_step_at(&path_ref, id) {
                        envelope.document = host.document;
                        envelope.runtime.selected_step_ids.retain(|step_id| step_id != id);
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "moveStepAt" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let new_index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).map(|value| value as usize);
                if let (Some(id), Some(new_index)) = (id, new_index) {
                    let path_ref = path_ref_from_args(args, &envelope.document);
                    push_undo_imperative(&mut envelope);
                    if host.move_step_at(&path_ref, id, new_index) {
                        envelope.document = host.document;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setStepParamsAt" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let params = args.and_then(|value| value.get("params"));
                if let (Some(id), Some(params)) = (id, params) {
                    let path_ref = path_ref_from_args(args, &envelope.document);
                    push_undo_imperative(&mut envelope);
                    if host.set_step_params_at(&path_ref, id, &params.to_string()).is_ok() {
                        envelope.document = host.document;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "removeStep" => {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    push_undo_imperative(&mut envelope);
                    if host.remove_step(id) {
                        envelope.document = host.document;
                        envelope.runtime.selected_step_ids.retain(|step_id| step_id != id);
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "moveStep" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let new_index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).map(|value| value as usize);
                if let (Some(id), Some(new_index)) = (id, new_index) {
                    push_undo_imperative(&mut envelope);
                    if host.move_step(id, new_index) {
                        envelope.document = host.document;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setStepParams" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let params = args.and_then(|value| value.get("params"));
                if let (Some(id), Some(params)) = (id, params) {
                    push_undo_imperative(&mut envelope);
                    if host.set_step_params_json(id, &params.to_string()).is_ok() {
                        envelope.document = host.document;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "run" => {
                let result = host.run();
                envelope.runtime.run_output_json =
                    serde_json::to_string(&result.scope).unwrap_or_else(|_| format!("{:?}", result.scope));
                return vec![set_document_op(&envelope)];
            }
            "undo" => {
                if let Some(previous) = envelope.undo_stack.pop() {
                    envelope.redo_stack.push(envelope.document.clone());
                    envelope.document = previous;
                    return vec![set_document_op(&envelope)];
                }
            }
            "redo" => {
                if let Some(next) = envelope.redo_stack.pop() {
                    envelope.undo_stack.push(envelope.document.clone());
                    envelope.document = next;
                    return vec![set_document_op(&envelope)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            IMPERATIVE_PLAY_BODY_MAIN => render_main_table(&envelope),
            IMPERATIVE_PLAY_BODY_SCRIPT => render_script(&envelope),
            IMPERATIVE_PLAY_BODY_DOCUMENT => build_document_tree(&envelope.document, &envelope.runtime.selected_step_ids),
            IMPERATIVE_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            IMPERATIVE_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope.document, &envelope.runtime.selected_step_ids),
            _ => ui_text(format!("Unknown body: {body_key}")),
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
    fn renders_table_scene() {
        let app = ImperativePlayApp;
        let document = app.initial_document_json();
        let node = app.render(IMPERATIVE_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("table"));
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
        let envelope = default_envelope();
        assert_eq!(envelope.document.path.steps.len(), 2);
    }

    #[test]
    fn add_step_command_appends_step() {
        let mut app = ImperativePlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("addStep", Some(&json!({ "kind": "log.print" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated_op: Value = serde_json::from_str(&ops[0]).unwrap();
        let updated: ImperativePlayEnvelope = serde_json::from_value(updated_op["document"].clone()).unwrap();
        assert!(updated.document.path.steps.len() > 2);
    }

    fn apply_ops(document: &str, ops: &[String]) -> ImperativePlayEnvelope {
        let mut envelope = parse_envelope(document);
        for op in ops {
            let parsed: Value = serde_json::from_str(op).unwrap();
            envelope = serde_json::from_value(parsed["document"].clone()).unwrap();
        }
        envelope
    }

    #[test]
    fn add_step_at_owner_slot_nests_into_control_body() {
        let mut app = ImperativePlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("addStepAt", Some(&json!({ "kind": "control.if" })), &document, &ViewState::default());
        let with_owner = apply_ops(&document, &ops);
        let owner_id = with_owner.runtime.selected_step_ids[0].clone();
        let document = serde_json::to_string(&with_owner).unwrap();
        let ops = app.handle_command_patch_ops(
            "addStepAt",
            Some(&json!({ "kind": "log.print", "owner": owner_id, "slot": "then" })),
            &document,
            &ViewState::default(),
        );
        let nested = apply_ops(&document, &ops);
        let owner_step = nested.document.path.steps.iter().find(|step| step.id == owner_id).expect("owner step");
        assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
        assert_eq!(nested.document.path.steps.len(), with_owner.document.path.steps.len(), "nested step lives in the slot, not the root path");
    }

    #[test]
    fn add_step_at_falls_back_to_root_for_unknown_owner() {
        let mut app = ImperativePlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
            "addStepAt",
            Some(&json!({ "kind": "log.print", "owner": "missing-step", "slot": "then" })),
            &document,
            &ViewState::default(),
        );
        let updated = apply_ops(&document, &ops);
        assert!(updated.document.path.steps.iter().any(|step| step.id == updated.runtime.selected_step_ids[0]));
    }

    #[test]
    fn run_command_expands_scope_into_readable_rows_without_truncation() {
        let mut app = ImperativePlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("run", None, &document, &ViewState::default());
        let ran = apply_ops(&document, &ops);
        assert!(!ran.runtime.run_output_json.is_empty());
        let node = app.render(IMPERATIVE_PLAY_BODY_MAIN, &serde_json::to_string(&ran).unwrap(), &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("counter"), "run output row shows the full scope key, not an 80-char blob");
    }
}
