//! 🎛️ S Studio app — `DocumentApp` impl, render, manifest (constitutional: ui).
//!
//! 🕳️ Deviation from the usual "ui" content: this app's `DocumentApp::Projection`/`Operation` are
//! `semio_framework_os::{OsProjection, OsOperation}` — see `space_op`'s doc comment. This crate also
//! regular-depends on `home_ui` (`semio-s-app-space-home-ui`): the Studio app resolves/loads studio
//! documents through the Home launcher's own catalog port (`openSpace`, `exportStudioPack`,
//! `exportStudioDsl`, `importSpacePackPayload`) — a real, non-test dependency, not just a test fixture.

use std::cell::RefCell;
use space::{
    SpacePanelState, SpaceProgramEntry, StudioRuntimeState, S_PLAY_APP_ID, S_PLAY_BODY_COMPILED_DAG,
    S_PLAY_BODY_MEDIA_VFS, S_PLAY_BODY_WORKFLOW, S_PLAY_CATALOGUE_BODY_KEY, S_PLAY_CATALOGUE_DRAG_MIME,
    S_PLAY_CATALOGUE_TAB_ID, S_PLAY_CONTROLLER_ID, S_PLAY_INSPECTOR_BODY_KEY, S_PLAY_INSPECTOR_TAB_ID,
    S_PLAY_PARAMETERS_BODY_KEY, S_PLAY_PARAMETERS_TAB_ID, S_PLAY_SURFACE_COMPILED_DAG, S_PLAY_SURFACE_MEDIA_VFS,
    S_PLAY_SURFACE_WORKFLOW, S_PLAY_WINDOW_COMPILED_DAG, S_PLAY_WINDOW_MEDIA_VFS, S_PLAY_WINDOW_WORKFLOW,
    S_STUDIO_EXAMPLES,
};
use space_engine::{
    add_parameter_operation, compiled_dag_wire_literal, flatten_media_vfs_rows,
    negotiate_media_connect, parameter_entity_id, parse_panel_state, panel_json,
    patch_parameter_operation, primary_selected_instance_id, selected_instance_ids, spawn_app_instance_operation,
    OsParameterId,
};
use space_shared::{demo_space_projection, ensure_space_fixtures_registered, parse_demo_space_document};
use semio_framework_os::{
    apply_flow_fixture_to_os_workflow, build_os_workflow_operator_infos, create_empty_os_document,
    create_os_id, default_os_projection, list_os_workflows,
    materialize_os_app_instance_document_json, materialize_os_projection,
    os_app_registration, os_document_to_json, os_workflow_to_flow_fixture, os_workflow_to_node_graph_payload,
    os_workflow_vfs_schema, os_parameter_types_compatible, os_parameter_value,
    OsAppInstance, OsOperation, OsParameter, OsParameterFieldBinding, OsParameterType, OsProjection,
    OsWorkflowCamera, OS_WORKFLOW_VFS_ROOT_ID, OS_SPACE_SCHEMA,
};
// 🕳️ `export_os_space_pack`/`export_os_space_dsl`/`import_os_space_from_pack` (wave 1, `host`
// region) aren't in `framework/product/os/core/rs/lib.rs`'s crate-root `pub use host::{...}` list
// (that file's non-test code is out of this family's edit scope — see the pack-rollout ticket's
// wave 2 family note) — reached via `host::` directly instead, since `pub mod host` makes every
// `pub fn` inside it reachable that way regardless of the root re-export list.
use semio_framework_os::host::{export_os_space_dsl, export_os_space_pack, import_os_space_from_pack};
use semio_framework_plugin::{
    app_labels, build_node_graph_scene, build_text_editor_scene, build_virtual_file_system_scene,
    create_default_layout, host_now_ms, is_de_locale, localized_label_map, resolve_labels, tree_item_desc,
    ui_declarative_sections_to_tree, ui_inspector_all_equal, ui_text, IconName, MeasureSelectItem, WindowEngagementStatus,
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionEmit, ActionKind, App,
    AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DocumentView, HostEffect, NodeGraphScene, PanelGroup,
    PanelTreeBuilder, SurfaceKind, TextEditorScene, UiButtonNode, UiFieldNode, UiInputNode, UiNode, UiPresence,
    UiNumberStepperNode, UiSectionNode, UiSelectItem, UiSelectNode, UiToggleNode, UiTreeItemNode, ViewState,
    VirtualFileSystemScene, WindowEngagement, WindowEngagementInput, WindowEngagementSlot, WindowLayout,
    WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
};
use semio_framework_plugin::optional_json_to_dsl;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::sync::{LazyLock, Mutex};

//#region 🔖️DocumentHelpers
fn s_play_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: S_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args: optional_json_to_dsl(args),
    }
}

fn workflow_context_menu_json(labels: &SStudioLabels) -> String {
    json!([
        { "id": "open-instance", "label": labels.context_open_instance, "icon": "external-link", "action": "openInstance" },
        { "id": "duplicate-instance", "label": labels.context_duplicate, "icon": "copy", "action": "duplicateAppInstance" },
        { "id": "copy-instance", "label": labels.context_copy, "icon": "clipboard-copy", "action": "copyAppInstance" },
        { "id": "paste-instance", "label": labels.context_paste, "icon": "clipboard", "action": "pasteAppInstance" },
        { "id": "rename-instance", "label": labels.context_rename_label, "icon": "edit-3", "action": "renameAppInstance" },
        { "id": "sep-remove", "separator": true },
        { "id": "remove-instance", "label": labels.context_remove, "icon": "trash", "action": "removeAppInstance", "destructive": true },
        { "id": "sep-selection", "separator": true },
        { "id": "select-all", "label": labels.context_select_all, "icon": "maximize-2", "action": "setMediaNodeSelection", "args": { "selectAll": true } },
        { "id": "clear-selection", "label": labels.context_clear_selection, "icon": "square-dashed", "action": "setMediaNodeSelection", "args": { "nodeIds": [] } },
        { "id": "reorganize", "label": labels.context_reorganize, "icon": "layout-grid", "action": "reorganizeWorkflow" }
    ])
    .to_string()
}

// 🫀️ The shared `presence:` backbone-URI hack (`read_os_presence_peers`/`write_os_presence`/
// `OsPresencePeer`) was deleted from os-core — presence now flows through the semio_hub's duplex
// `PresencePeer`/`Presence` frames via `framework/sync`'s `DocumentEvent::Presence` for migrated
// apps. `s` isn't wired onto `DocumentHost` yet (WS-F's last wave), so it keeps this tiny
// self-contained in-memory heartbeat map until then — same upsert/prune/exclude-self semantics as
// before, just owned locally instead of delegated to a shared cross-process mechanism.
#[derive(Clone)]
struct SPresencePeerLocal {
    client_id: String,
    name: String,
    selection: Vec<String>,
    updated_at_ms: f64,
}

const S_PRESENCE_STALE_MS: f64 = 15_000.0;

static PRESENCE_PEERS: LazyLock<Mutex<HashMap<String, HashMap<String, SPresencePeerLocal>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn runtime_space_id(runtime: &StudioRuntimeState) -> String {
    runtime.space_id.clone().unwrap_or_else(|| "default".into())
}

fn presence_peers_json(runtime: &StudioRuntimeState) -> String {
    let space_id = runtime_space_id(runtime);
    let self_client_id = runtime.client_id.clone().unwrap_or_default();
    let now_ms = host_now_ms();
    let peers: Vec<Value> = PRESENCE_PEERS
        .lock()
        .ok()
        .and_then(|registry| registry.get(&space_id).cloned())
        .unwrap_or_default()
        .into_values()
        .filter(|peer| peer.client_id != self_client_id && now_ms - peer.updated_at_ms <= S_PRESENCE_STALE_MS)
        .map(|peer| {
            json!({
                "clientId": peer.client_id,
                "name": peer.name,
                "selectionCount": peer.selection.len(),
            })
        })
        .collect();
    serde_json::to_string(&peers).unwrap_or_else(|_| "[]".into())
}

fn publish_presence(runtime: &StudioRuntimeState) {
    let (Some(client_id), Some(client_name)) = (&runtime.client_id, &runtime.client_name) else {
        return;
    };
    let space_id = runtime_space_id(runtime);
    let now_ms = host_now_ms();
    if let Ok(mut registry) = PRESENCE_PEERS.lock() {
        let peers = registry.entry(space_id).or_default();
        peers.retain(|_, entry| now_ms - entry.updated_at_ms <= S_PRESENCE_STALE_MS);
        peers.insert(
            client_id.clone(),
            SPresencePeerLocal {
                client_id: client_id.clone(),
                name: client_name.clone(),
                selection: runtime.selected_media_node_ids.clone(),
                updated_at_ms: now_ms,
            },
        );
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the Studio app; one field per label makes every locale combination compile-checked.
    struct SStudioLabels {
        apps_section: &'static str = en: "Apps", de: "Apps";
        media_vfs_empty_message: &'static str = en: "No app instances in the workflow.", de: "Keine App-Instanzen im Workflows.";
        add_parameter: &'static str = en: "Add Parameter", de: "Parameter hinzufügen";
        name: &'static str = en: "Name", de: "Name";
        value: &'static str = en: "Value", de: "Wert";
        min: &'static str = en: "Min", de: "Min";
        max: &'static str = en: "Max", de: "Max";
        step: &'static str = en: "Step", de: "Schritt";
        add_option: &'static str = en: "Add option", de: "Option hinzufügen";
        new_option_placeholder: &'static str = en: "New option", de: "Neue Option";
        remove: &'static str = en: "Remove", de: "Entfernen";
        node_id: &'static str = en: "Node id", de: "Knoten-ID";
        label: &'static str = en: "Label", de: "Beschriftung";
        direct_value: &'static str = en: "Direct value", de: "Direkter Wert";
        workflow_node: &'static str = en: "Workflow node", de: "Workflow-Knoten";
        workflow_nodes: &'static str = en: "Workflow nodes", de: "Workflow-Knoten";
        app_instance: &'static str = en: "App instance", de: "App-Instanz";
        app_instances: &'static str = en: "App instances", de: "App-Instanzen";
        select_hint: &'static str = en: "Select workflow nodes or app instances in the canvas.", de: "Wähle Workflow-Knoten oder App-Instanzen im Arbeitsbereich aus.";
        program_prefix: &'static str = en: "Program", de: "Programm";
        app_prefix: &'static str = en: "App", de: "App";
        instance_id_prefix: &'static str = en: "Instance id", de: "Instanz-ID";
        bound_value_prefix: &'static str = en: "Bound value", de: "Gebundener Wert";
        active_app: &'static str = en: "Active app", de: "Aktive App";
        window_workflow: &'static str = en: "Workflow", de: "Workflow";
        window_media_vfs: &'static str = en: "Media VFS", de: "Media-VFS";
        window_compiled_dag: &'static str = en: "Compiled DAG", de: "Kompilierter DAG";
        toggle_on: &'static str = en: "On", de: "An";
        toggle_off: &'static str = en: "Off", de: "Aus";
        mixed_placeholder: &'static str = en: "Mixed", de: "Gemischt";
        parameter_count_suffix: &'static str = en: "parameter(s)", de: "Parameter";
        media_node_count_label: &'static str = en: "media node(s)", de: "Medienknoten";
        app_instance_count_label: &'static str = en: "app instance(s)", de: "App-Instanz(en)";
        context_open_instance: &'static str = en: "Open instance", de: "Instanz öffnen";
        context_duplicate: &'static str = en: "Duplicate", de: "Duplizieren";
        context_copy: &'static str = en: "Copy", de: "Kopieren";
        context_paste: &'static str = en: "Paste", de: "Einfügen";
        context_rename_label: &'static str = en: "Rename label…", de: "Bezeichnung umbenennen…";
        context_remove: &'static str = en: "Remove", de: "Entfernen";
        context_select_all: &'static str = en: "Select all", de: "Alle auswählen";
        context_clear_selection: &'static str = en: "Clear selection", de: "Auswahl aufheben";
        context_reorganize: &'static str = en: "Reorganize", de: "Neu anordnen";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action/shell-action declared in
/// `create_space_app`'s static manifest — same rationale as `app_home`'s `s_home_action_labels`.
fn s_studio_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        // 🔧️ Document-mutating operations
        ("setParameter", "Set Parameter", "Parameter festlegen"),
        ("patchParameter", "Patch Parameter", "Parameter aktualisieren"),
        ("addParameter", "Add Parameter", "Parameter hinzufügen"),
        ("removeParameter", "Remove Parameter", "Parameter entfernen"),
        ("spawnApp", "Spawn App", "App erzeugen"),
        ("moveMediaNode", "Move Media Node", "Medienknoten verschieben"),
        ("connectMediaPorts", "Connect Media Ports", "Medien-Ports verbinden"),
        ("disconnectMediaEdge", "Disconnect Media Edge", "Medienverbindung trennen"),
        ("removeAppInstance", "Remove App Instance", "App-Instanz entfernen"),
        ("deleteSelection", "Delete Selection", "Auswahl löschen"),
        ("copyAppInstance", "Copy App Instance", "App-Instanz kopieren"),
        ("duplicateAppInstance", "Duplicate App Instance", "App-Instanz duplizieren"),
        ("pasteAppInstance", "Paste App Instance", "App-Instanz einfügen"),
        ("renameAppInstance", "Rename App Instance", "App-Instanz umbenennen"),
        ("patchMediaNodes", "Patch Media Nodes", "Medienknoten aktualisieren"),
        ("patchAppInstances", "Patch App Instances", "App-Instanzen aktualisieren"),
        ("bindParameterField", "Bind Parameter Field", "Parameterfeld verknüpfen"),
        ("unbindParameterField", "Unbind Parameter Field", "Parameterfeld lösen"),
        ("reorganizeWorkflow", "Reorganize Workflow", "Workflow neu anordnen"),
        ("workflowEngagementSubmit", "Workflow Engagement Submit", "Workflow-Eingabe bestätigen"),
        ("compiledDagEngagementSubmit", "Compiled DAG Engagement Submit", "Kompilierter-DAG-Eingabe bestätigen"),
        ("nodeGraphEdit", "Edit Workflow", "Workflow bearbeiten"),
        // 👁️ Ephemeral view state
        ("setActivePanelTab", "Set Active Panel Tab", "Aktiven Panel-Tab festlegen"),
        ("selectInstance", "Select Instance", "Instanz auswählen"),
        ("nodeGraphSelect", "Select Graph Node", "Graphknoten auswählen"),
        ("setMediaNodeSelection", "Set Media Node Selection", "Medienknoten-Auswahl festlegen"),
        ("nodeGraphHover", "Hover Graph Node", "Graphknoten hovern"),
        ("textHover", "Text Hover", "Text-Hover"),
        ("nodeGraphViewport", "Set Graph Viewport", "Graph-Ansichtsfenster festlegen"),
        ("presenceHeartbeat", "Presence Heartbeat", "Anwesenheits-Heartbeat"),
        ("setAppInstanceSelection", "Set App Instance Selection", "App-Instanz-Auswahl festlegen"),
        ("workflowEngagementInput", "Workflow Engagement Input", "Workflow-Eingabe"),
        ("compiledDagEngagementInput", "Compiled DAG Engagement Input", "Kompilierter-DAG-Eingabe"),
        // 🗨️ Shell-only effects
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("exportMedia", "Export Media", "Medien exportieren"),
        ("importMedia", "Import Media", "Medien importieren"),
        ("importMediaPayload", "Import Media Payload", "Medien-Payload importieren"),
        ("exportStudioPack", "Export Studio Pack", "Studio-Paket exportieren"),
        ("exportStudioDsl", "Export Studio DSL", "Studio-DSL exportieren"),
        ("importSpacePack", "Import Studio Pack", "Studio-Paket importieren"),
        ("importSpacePackPayload", "Import Studio Pack Payload", "Studio-Paket-Payload importieren"),
        ("openSpace", "Open Studio", "Studio öffnen"),
        ("openInstance", "Open Instance", "Instanz öffnen"),
        ("closeFocusedInstance", "Close Focused Instance", "Fokussierte Instanz schließen"),
        ("goHome", "Go Home", "Zur Startseite"),
        ("navigateVirtualFileSystemNode", "Navigate File System Node", "Dateisystemknoten navigieren"),
    ])
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
#[derive(Default)]
struct AppCatalogueNode {
    children: BTreeMap<String, AppCatalogueNode>,
    app: Option<SpaceProgramEntry>,
}

/// 🌳️ Builds a catalogue tree item on top of the SDK's `tree_item_desc` skeleton — only the
/// per-app drag-data/icon/children extensions are this app's own concern.
fn app_catalogue_item(path: &[String], label: &str, node: AppCatalogueNode) -> UiTreeItemNode {
    let id_path = path.join(".");
    let children = node
        .children
        .into_iter()
        .map(|(segment, child)| {
            let mut child_path = path.to_vec();
            child_path.push(segment.clone());
            app_catalogue_item(&child_path, &segment, child)
        })
        .collect::<Vec<_>>();
    let app = node.app;
    let description = app.as_ref().and_then(|entry| (!entry.yields.is_empty()).then(|| entry.yields.clone()));
    let mut item = tree_item_desc(format!("s-play-catalogue.document.{id_path}"), label, description);
    item.icon_id = app.as_ref().and_then(|entry| IconName::from_str(&entry.app_id));
    item.default_open = (!children.is_empty()).then_some(true);
    if let Some(app) = &app {
        let mut drag_data = HashMap::new();
        drag_data.insert(
            S_PLAY_CATALOGUE_DRAG_MIME.into(),
            json!({ "pluginId": app.plugin_id, "appId": app.app_id, "label": app.label }).to_string(),
        );
        item.draggable = Some(true);
        item.drag_data = Some(drag_data);
    }
    item.items = (!children.is_empty()).then_some(children);
    item
}

fn build_catalogue_tree(panel: &SpacePanelState, labels: &SStudioLabels) -> UiNode {
    let workflows: Vec<SpaceProgramEntry> = if panel.workflows.is_empty() {
        let mut entries = Vec::new();
        for program in list_os_workflows() {
            if program.id == "s.system" {
                continue;
            }
            for app in program.apps {
                entries.push(SpaceProgramEntry {
                    plugin_id: program.id.clone(),
                    workflow_step_id: app.id.clone(),
                    app_id: app.id,
                    label: app.label,
                    document: app.document,
                    yields: app
                        .outputs
                        .first()
                        .map(|port| port.artifact_kind.clone())
                        .unwrap_or_default(),
                });
            }
        }
        entries
    } else {
        panel.workflows.clone()
    };
    let mut document = AppCatalogueNode::default();
    for workflow in workflows {
        let mut node = &mut document;
        for segment in &workflow.document {
            node = node.children.entry(segment.clone()).or_default();
        }
        node.app = Some(workflow);
    }
    let items = document
        .children
        .into_iter()
        .map(|(segment, node)| app_catalogue_item(&[segment.clone()], &segment, node))
        .collect();
    PanelTreeBuilder::new(S_PLAY_CATALOGUE_TAB_ID)
        .section(S_PLAY_CATALOGUE_TAB_ID, Some(labels.apps_section.into()), true, items)
        .build()
}

fn parameter_value_control(parameter: &OsParameter, labels: &SStudioLabels) -> UiNode {
    match parameter {
        OsParameter::Numeric { id, value, step, .. } => UiNode::NumberStepper(UiNumberStepperNode {presence: UiPresence::default(),
            id: format!("s-play-parameters.{id}.value"),
            value: *value,
            step: step.unwrap_or(1.0),
            uniform: true,
            on_absolute: s_play_action(
                "patchParameter",
                Some(json!({ "parameterId": id, "field": "value" })),
            ),
            on_delta: s_play_action(
                "patchParameter",
                Some(json!({ "parameterId": id, "field": "value" })),
            ),
            menu: None,
        }),
        OsParameter::Categorical { id, value, options, .. } => UiNode::Select(UiSelectNode {presence: UiPresence::default(),
            id: format!("s-play-parameters.{id}.value"),
            value: value.clone(),
            items: options
                .iter()
                .map(|option| UiSelectItem {
                    value: option.clone(),
                    label: option.clone(),
                })
                .collect(),
            placeholder: None,
            on_change: s_play_action(
                "patchParameter",
                Some(json!({ "parameterId": id, "field": "value" })),
            ),
            menu: None,
        }),
        OsParameter::Toggle { id, value, .. } => UiNode::Toggle(UiToggleNode {
            id: format!("s-play-parameters.{id}.value"),
            icon_id: "toggle-left".into(),
            presence: UiPresence::selected(*value),
            text: Some(if *value { labels.toggle_on.into() } else { labels.toggle_off.into() }),
            on_change: s_play_action(
                "patchParameter",
                Some(json!({ "parameterId": id, "field": "value" })),
            ),
            menu: None,
        }),
        OsParameter::Text { id, value, .. } => UiNode::Input(UiInputNode {presence: UiPresence::default(),
            id: format!("s-play-parameters.{id}.value"),
            input_kind: "text".into(),
            value: value.clone(),
            placeholder: None,
            commit: None,
            on_change: s_play_action(
                "patchParameter",
                Some(json!({ "parameterId": id, "field": "value" })),
            ),
            min: None,
            max: None,
            step: None,
            accept: None,
            menu: None,
        }),
    }
}

fn parameter_constraint_fields(parameter: &OsParameter, labels: &SStudioLabels) -> Vec<UiNode> {
    match parameter {
        OsParameter::Numeric {
            id,
            min,
            max,
            step,
            ..
        } => vec![
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: format!("s-play-parameters.{id}.min"),
                label: labels.min.into(),
                child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {presence: UiPresence::default(),
                    id: format!("s-play-parameters.{id}.min.stepper"),
                    value: min.unwrap_or(0.0),
                    step: 1.0,
                    uniform: true,
                    on_absolute: s_play_action(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "min" })),
                    ),
                    on_delta: s_play_action(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "min" })),
                    ),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: format!("s-play-parameters.{id}.max"),
                label: labels.max.into(),
                child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {presence: UiPresence::default(),
                    id: format!("s-play-parameters.{id}.max.stepper"),
                    value: max.unwrap_or(0.0),
                    step: 1.0,
                    uniform: true,
                    on_absolute: s_play_action(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "max" })),
                    ),
                    on_delta: s_play_action(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "max" })),
                    ),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: format!("s-play-parameters.{id}.step"),
                label: labels.step.into(),
                child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {presence: UiPresence::default(),
                    id: format!("s-play-parameters.{id}.step.stepper"),
                    value: step.unwrap_or(0.0),
                    step: 0.1,
                    uniform: true,
                    on_absolute: s_play_action(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "step" })),
                    ),
                    on_delta: s_play_action(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "step" })),
                    ),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        ],
        OsParameter::Categorical { id, options, .. } => {
            let mut fields: Vec<UiNode> = options
                .iter()
                .map(|option| {
                    UiNode::Field(UiFieldNode {
                        id: format!("s-play-parameters.{id}.option.{option}"),
                        label: option.clone(),
                        presence: UiPresence::default(),
                        child: Box::new(UiNode::Button(UiButtonNode {
                            id: Some(format!("s-play-parameters.{id}.option.{option}.remove")),
                            icon_id: "trash-2".into(),
                            label: labels.remove.into(),
                            action: s_play_action(
                                "patchParameter",
                                Some(json!({ "parameterId": id, "field": "removeOption", "value": option })),
                            ),
                            style: None,
                            presence: UiPresence::default(),
                            menu: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                        menu: None,
                    })
                })
                .collect();
            fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: format!("s-play-parameters.{id}.add-option"),
                label: labels.add_option.into(),
                child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
                    id: format!("s-play-parameters.{id}.add-option.input"),
                    input_kind: "text".into(),
                    value: String::new(),
                    placeholder: Some(labels.new_option_placeholder.into()),
                    commit: None,
                    on_change: s_play_action(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "addOption" })),
                    ),
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
            }));
            fields
        }
        _ => Vec::new(),
    }
}

fn build_parameters_tree(projection: &OsProjection, labels: &SStudioLabels) -> UiNode {
    let mut children = vec![UiSectionNode {
        id: "s-play-parameters.header".into(),
        label: Some(FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL.into()),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![
            UiNode::Button(UiButtonNode {
                id: Some("s-play-parameters.add".into()),
                icon_id: "plus".into(),
                label: labels.add_parameter.into(),
                action: s_play_action("addParameter", Some(json!({ "type": "numeric" }))),
                style: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            ui_text(format!("{} {}", projection.parameters.len(), labels.parameter_count_suffix)),
        ],
        menu: None,
    }];
    for parameter in &projection.parameters {
        let parameter_id = parameter_entity_id(parameter).to_string();
        let mut parameter_children = vec![
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: format!("s-play-parameters.{parameter_id}.name"),
                label: labels.name.into(),
                child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
                    id: format!("s-play-parameters.{parameter_id}.name.input"),
                    input_kind: "text".into(),
                    value: match parameter {
                        OsParameter::Numeric { name, .. }
                        | OsParameter::Categorical { name, .. }
                        | OsParameter::Toggle { name, .. }
                        | OsParameter::Text { name, .. } => name.clone(),
                    },
                    placeholder: None,
                    commit: None,
                    on_change: s_play_action(
                        "patchParameter",
                        Some(json!({ "parameterId": parameter_id, "field": "name" })),
                    ),
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
            }),
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: format!("s-play-parameters.{parameter_id}.value-field"),
                label: labels.value.into(),
                child: Box::new(parameter_value_control(parameter, labels)),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        ];
        parameter_children.extend(parameter_constraint_fields(parameter, labels));
        parameter_children.push(UiNode::Button(UiButtonNode {
            id: Some(format!("s-play-parameters.{parameter_id}.remove")),
            icon_id: "trash-2".into(),
            label: labels.remove.into(),
            action: s_play_action(
                "removeParameter",
                Some(json!({ "parameterId": parameter_id })),
            ),
            style: None,
            presence: UiPresence::default(),
            menu: None,
        }));
        children.push(UiSectionNode {
            id: format!("s-play-parameters.{parameter_id}"),
            label: Some(match parameter {
                OsParameter::Numeric { name, .. }
                | OsParameter::Categorical { name, .. }
                | OsParameter::Toggle { name, .. }
                | OsParameter::Text { name, .. } => name.clone(),
            }),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: parameter_children,
            menu: None,
        });
    }
    ui_declarative_sections_to_tree(&children)
}

fn build_inspector_tree(projection: &OsProjection, runtime: &StudioRuntimeState, term_labels: &SStudioLabels) -> UiNode {
    let media_node_ids = &runtime.selected_media_node_ids;
    let instance_ids = &runtime.selected_app_instance_ids;
    let mut children = vec![UiSectionNode {
        id: "s-play-inspector.header".into(),
        label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![ui_text(format!(
            "{} {} · {} {}",
            media_node_ids.len(),
            term_labels.media_node_count_label,
            instance_ids.len(),
            term_labels.app_instance_count_label
        ))],
        menu: None,
    }];
    if !media_node_ids.is_empty() {
        let nodes: Vec<_> = media_node_ids
            .iter()
            .filter_map(|node_id| projection.workflow.nodes.iter().find(|node| &node.id == node_id))
            .collect();
        let xs: Vec<_> = nodes.iter().map(|node| node.x).collect();
        let ys: Vec<_> = nodes.iter().map(|node| node.y).collect();
        let x_uniform = ui_inspector_all_equal(&xs.iter().map(|v| v.to_string()).collect::<Vec<_>>());
        let y_uniform = ui_inspector_all_equal(&ys.iter().map(|v| v.to_string()).collect::<Vec<_>>());
        let mut node_fields = Vec::new();
        if media_node_ids.len() == 1 {
            node_fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "s-play-inspector.media-node.id".into(),
                label: term_labels.node_id.into(),
                child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
                    id: "s-play-inspector.media-node.id.input".into(),
                    input_kind: "text".into(),
                    value: media_node_ids[0].clone(),
                    placeholder: None,
                    commit: None,
                    on_change: s_play_action("noOperation", None),
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
            }));
        }
        node_fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
            id: "s-play-inspector.media-node.x".into(),
            label: "X".into(),
            child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
                id: "s-play-inspector.media-node.x.input".into(),
                input_kind: "number".into(),
                value: if x_uniform {
                    xs.first().map(|v| v.to_string()).unwrap_or_default()
                } else {
                    String::new()
                },
                placeholder: if x_uniform { None } else { Some(term_labels.mixed_placeholder.into()) },
                commit: None,
                on_change: s_play_action(
                    "patchMediaNodes",
                    Some(json!({ "nodeIds": media_node_ids, "field": "position", "axis": "x" })),
                ),
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
        }));
        node_fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
            id: "s-play-inspector.media-node.y".into(),
            label: "Y".into(),
            child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
                id: "s-play-inspector.media-node.y.input".into(),
                input_kind: "number".into(),
                value: if y_uniform {
                    ys.first().map(|v| v.to_string()).unwrap_or_default()
                } else {
                    String::new()
                },
                placeholder: if y_uniform { None } else { Some(term_labels.mixed_placeholder.into()) },
                commit: None,
                on_change: s_play_action(
                    "patchMediaNodes",
                    Some(json!({ "nodeIds": media_node_ids, "field": "position", "axis": "y" })),
                ),
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
        }));
        children.push(UiSectionNode {
            id: "s-play-inspector.media-nodes".into(),
            label: Some(if media_node_ids.len() == 1 {
                term_labels.workflow_node.into()
            } else {
                format!("{} ({})", term_labels.workflow_nodes, media_node_ids.len())
            }),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: node_fields,
            menu: None,
        });
    }
    if !instance_ids.is_empty() {
        let instances: Vec<_> = instance_ids
            .iter()
            .filter_map(|id| projection.app_instances.iter().find(|instance| &instance.id == id))
            .collect();
        let labels: Vec<_> = instances.iter().map(|instance| instance.label.clone()).collect();
        let programs: Vec<_> = instances.iter().map(|instance| instance.plugin_id.clone()).collect();
        let apps: Vec<_> = instances.iter().map(|instance| instance.app_id.clone()).collect();
        let label_uniform = ui_inspector_all_equal(&labels);
        let program_uniform = ui_inspector_all_equal(&programs);
        let app_uniform = ui_inspector_all_equal(&apps);
        let mut instance_fields = vec![
            ui_text(format!(
                "{}: {}",
                term_labels.program_prefix,
                if program_uniform {
                    programs.first().cloned().unwrap_or_default()
                } else {
                    term_labels.mixed_placeholder.into()
                }
            )),
            ui_text(format!(
                "{}: {}",
                term_labels.app_prefix,
                if app_uniform {
                    apps.first().cloned().unwrap_or_default()
                } else {
                    term_labels.mixed_placeholder.into()
                }
            )),
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "s-play-inspector.app-instance.label".into(),
                label: term_labels.label.into(),
                child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
                    id: "s-play-inspector.app-instance.label.input".into(),
                    input_kind: "text".into(),
                    value: if label_uniform {
                        labels.first().cloned().unwrap_or_default()
                    } else {
                        String::new()
                    },
                    placeholder: if label_uniform { None } else { Some(term_labels.mixed_placeholder.into()) },
                    commit: None,
                    on_change: s_play_action(
                        "patchAppInstances",
                        Some(json!({ "instanceIds": instance_ids, "field": "label" })),
                    ),
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
            }),
        ];
        if instance_ids.len() == 1 {
            instance_fields.insert(2, ui_text(format!("{}: {}", term_labels.instance_id_prefix, instance_ids[0])));
        }
        if instance_ids.len() == 1 {
            if let Some(instance) = instances.first() {
                if let Some(registration) = os_app_registration(&instance.plugin_id, &instance.app_id) {
                    for field_spec in &registration.parameter_fields {
                        let binding = projection.parameter_bindings.iter().find(|entry| {
                            entry.instance_id == instance.id && entry.field_path == field_spec.field_path
                        });
                        let compatible: Vec<_> = projection
                            .parameters
                            .iter()
                            .filter(|parameter| {
                                os_parameter_types_compatible(
                                    match parameter {
                                        OsParameter::Numeric { .. } => &OsParameterType::Numeric,
                                        OsParameter::Categorical { .. } => &OsParameterType::Categorical,
                                        OsParameter::Toggle { .. } => &OsParameterType::Toggle,
                                        OsParameter::Text { .. } => &OsParameterType::Text,
                                    },
                                    &field_spec.parameter_type,
                                )
                            })
                            .collect();
                        let mut items = vec![UiSelectItem {
                            value: "__direct__".into(),
                            label: term_labels.direct_value.into(),
                        }];
                        for parameter in compatible {
                            items.push(UiSelectItem {
                                value: parameter_entity_id(parameter).into(),
                                label: match parameter {
                                    OsParameter::Numeric { name, .. }
                                    | OsParameter::Categorical { name, .. }
                                    | OsParameter::Toggle { name, .. }
                                    | OsParameter::Text { name, .. } => name.clone(),
                                },
                            });
                        }
                        instance_fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                            id: format!("s-play-inspector.app-parameter.{}", field_spec.field_path),
                            label: field_spec.label.clone(),
                            child: Box::new(UiNode::Select(UiSelectNode {presence: UiPresence::default(),
                                id: format!(
                                    "s-play-inspector.app-parameter.{}.select",
                                    field_spec.field_path
                                ),
                                value: binding
                                    .map(|entry| entry.parameter_id.clone())
                                    .unwrap_or_else(|| "__direct__".into()),
                                items,
                                placeholder: None,
                                on_change: s_play_action(
                                    "bindParameterField",
                                    Some(json!({
                                        "instanceId": instance.id,
                                        "fieldPath": field_spec.field_path,
                                    })),
                                ),
                                menu: None,
                            })),
                            description: None,
                            required: None,
                            error: None,
                            menu: None,
                        }));
                        if let Some(binding) = binding {
                            if let Some(parameter) = projection
                                .parameters
                                .iter()
                                .find(|entry| entry.id() == binding.parameter_id)
                            {
                                instance_fields.push(ui_text(format!(
                                    "{}: {}",
                                    term_labels.bound_value_prefix,
                                    os_parameter_value(parameter)
                                )));
                            }
                        }
                    }
                }
            }
        }
        children.push(UiSectionNode {
            id: "s-play-inspector.app-instances".into(),
            label: Some(if instance_ids.len() == 1 {
                term_labels.app_instance.into()
            } else {
                format!("{} ({})", term_labels.app_instances, instance_ids.len())
            }),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: instance_fields,
            menu: None,
        });
    }
    if media_node_ids.is_empty() && instance_ids.is_empty() {
        children[0].children.push(ui_text(term_labels.select_hint));
    }
    ui_declarative_sections_to_tree(&children)
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_workflow(projection: &OsProjection, runtime: &StudioRuntimeState, labels: &SStudioLabels) -> UiNode {
    let graph_payload = os_workflow_to_node_graph_payload(&projection.workflow, &projection.app_instances);
    let camera = runtime.workflow_camera.clone().unwrap_or_default();
    let fixture = os_workflow_to_flow_fixture(&projection.workflow, &projection.app_instances, &camera);
    let operators = build_os_workflow_operator_infos(
        &projection.workflow,
        &projection.app_instances,
        &projection.parameters,
    );
    let selection_json = if runtime.selected_media_node_ids.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&runtime.selected_media_node_ids).unwrap_or_else(|_| "[]".into()))
    };
    let hover_json = runtime.hovered_media_node_id.as_ref().map(|id| {
        json!({ "nodeId": id }).to_string()
    });
    build_node_graph_scene(
        S_PLAY_SURFACE_WORKFLOW,
        S_PLAY_CONTROLLER_ID,
        NodeGraphScene {
            editable: Some(true),
            operators_json: Some(serde_json::to_string(&operators).unwrap_or_else(|_| "[]".into())),
            context_menu_json: Some(workflow_context_menu_json(labels)),
            find_items_json: Some(graph_payload.find_items_json),
            selection_json,
            hover_json,
            capabilities_json: Some(r#"{"engine":"flow","spotlight":false,"noteEdit":false,"clusters":false}"#.into()),
            fixture_json: Some(fixture.to_string()),
            presence_peers_json: Some(presence_peers_json(runtime)),
            ..NodeGraphScene::base(
                graph_payload.nodes_json,
                graph_payload.edges_json,
                serde_json::to_string(&camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into()),
            )
        },
    )
}

fn render_compiled_dag(projection: &OsProjection) -> UiNode {
    let wire = compiled_dag_wire_literal(projection);
    build_text_editor_scene(
        S_PLAY_SURFACE_COMPILED_DAG,
        S_PLAY_CONTROLLER_ID,
        TextEditorScene::base(wire, Some("wire".into()), None),
    )
}

fn render_media_vfs(projection: &OsProjection, labels: &SStudioLabels) -> UiNode {
    let mut rows = vec![json!({
        "id": OS_WORKFLOW_VFS_ROOT_ID,
        "fileNodeKindId": "root",
        "name": "Workflow",
        "path": "/",
        "parentId": null,
        "hasChildren": true,
        "descriptorValues": {}
    })];
    flatten_media_vfs_rows(
        OS_WORKFLOW_VFS_ROOT_ID,
        &projection.app_instances,
        &projection.workflow,
        &projection.parameter_bindings,
        &projection.parameters,
        &mut rows,
    );
    let schema = os_workflow_vfs_schema();
    build_virtual_file_system_scene(
        S_PLAY_SURFACE_MEDIA_VFS,
        S_PLAY_CONTROLLER_ID,
        VirtualFileSystemScene {
            schema_json: serde_json::to_string(&schema).unwrap_or_else(|_| "{}".into()),
            rows_json: serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()),
            selected_row_ids_json: None,
            hovered_row_id: None,
            empty_message: Some(labels.media_vfs_empty_message.into()),
            drag_drop_enabled: Some(true),
        },
        Some(S_PLAY_WINDOW_MEDIA_VFS.into()),
        None,
    )
}
//#endregion 🔖️Render

//#region 🔖️SpaceApp
pub struct SpaceApp {
    runtime: RefCell<StudioRuntimeState>,
}

impl SpaceApp {
    /// @emoji 🎬️ Empty studio runtime — the host loads the target catalog/example document via
    /// `openSpace` → `HostEffect::LoadDocument`; demo content is no longer the silent default.
    pub fn new() -> Self {
        Self {
            runtime: RefCell::new(StudioRuntimeState::default()),
        }
    }
}

impl Default for SpaceApp {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentApp for SpaceApp {
    type Projection = OsProjection;
    type Operation = OsOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        S_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        OS_SPACE_SCHEMA
    }

    fn initial_projection(&self) -> OsProjection {
        default_os_projection()
    }

    fn handle_action(
        &self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, OsProjection>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
    ) -> ActionEmit<OsOperation> {
        let projection = doc.projection;
        let mut operations: Vec<OsOperation> = Vec::new();
        let mut effects: Vec<HostEffect> = Vec::new();
        let mut coalesce_key: Option<String> = None;
        let mut ui_scope = semio_framework_core::kernel::UiDirtyScope::Full;
        match action {
            "setActivePanelTab" => {
                if let Some(tab) = args.and_then(|value| value.get("tabId")).and_then(|value| value.as_str()) {
                    let mut panel = parse_panel_state(view_state);
                    panel.active_panel_tab = tab.into();
                    return ActionEmit::effect(HostEffect::SetPanel { panel_json: panel_json(&panel) });
                }
                return ActionEmit::default();
            }
            "navigateVirtualFileSystemNode" => {
                if let Some(space_id) = args.and_then(|value| value.get("spaceId")).and_then(|value| value.as_str()) {
                    return ActionEmit::effect(HostEffect::Navigate { uri: format!("/spaces/{space_id}") });
                }
                return ActionEmit::default();
            }
            "setParameter" | "patchParameter" => {
                let parameter_id = args
                    .and_then(|value| value.get("id").or_else(|| value.get("parameterId")))
                    .and_then(|value| value.as_str());
                let field = args
                    .and_then(|value| value.get("field"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("value");
                let value = args.and_then(|value| value.get("value")).cloned();
                if let Some(parameter_id) = parameter_id {
                    let patch = if field == "addOption" {
                        value
                            .and_then(|value| value.as_str().map(str::to_string))
                            .and_then(|option| {
                                projection
                                    .parameters
                                    .iter()
                                    .find(|entry| parameter_entity_id(entry) == parameter_id)
                                    .and_then(|entry| match entry {
                                        OsParameter::Categorical { options, .. } => {
                                            let mut next_options = options.clone();
                                            if !next_options.iter().any(|row| row == &option) {
                                                next_options.push(option.clone());
                                            }
                                            Some(json!({ "options": next_options, "value": option }))
                                        }
                                        _ => None,
                                    })
                            })
                    } else if field == "removeOption" {
                        value
                            .and_then(|value| value.as_str().map(str::to_string))
                            .and_then(|option| {
                                projection
                                    .parameters
                                    .iter()
                                    .find(|entry| parameter_entity_id(entry) == parameter_id)
                                    .and_then(|entry| match entry {
                                        OsParameter::Categorical { options, value, .. } => {
                                            let next_options: Vec<_> = options
                                                .iter()
                                                .filter(|row| row.as_str() != option)
                                                .cloned()
                                                .collect();
                                            let next_value = if next_options.iter().any(|row| row == value) {
                                                value.clone()
                                            } else {
                                                next_options.first().cloned().unwrap_or_default()
                                            };
                                            Some(json!({ "options": next_options, "value": next_value }))
                                        }
                                        _ => None,
                                    })
                            })
                    } else {
                        value.map(|value| json!({ field: value }))
                    };
                    if let Some(patch) = patch {
                        if let Some(operation) = patch_parameter_operation(projection, parameter_id, &patch) {
                            operations.push(operation);
                        }
                    }
                }
            }
            "addParameter" => {
                let parameter_type = match args
                    .and_then(|value| value.get("type"))
                    .and_then(|value| value.as_str())
                {
                    Some("categorical") => OsParameterType::Categorical,
                    Some("toggle") => OsParameterType::Toggle,
                    Some("text") => OsParameterType::Text,
                    _ => OsParameterType::Numeric,
                };
                let name = args
                    .and_then(|value| value.get("name"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("Parameter");
                operations.push(add_parameter_operation(&parameter_type, name));
            }
            "removeParameter" => {
                if let Some(parameter_id) = args
                    .and_then(|value| value.get("parameterId"))
                    .and_then(|value| value.as_str())
                {
                    operations.push(OsOperation::RemoveParameter {
                        parameter_id: parameter_id.into(),
                    });
                }
            }
            "spawnApp" => {
                let plugin_id = args
                    .and_then(|value| value.get("pluginId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let app_id = args
                    .and_then(|value| value.get("appId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if !plugin_id.is_empty() && !app_id.is_empty() {
                    let position = args
                        .and_then(|value| value.get("position"))
                        .and_then(|value| value.as_object())
                        .map(|position| semio_framework_os::WorkflowPosition {
                            x: position
                                .get("x")
                                .and_then(|value| value.as_f64())
                                .unwrap_or(80.0),
                            y: position
                                .get("y")
                                .and_then(|value| value.as_f64())
                                .unwrap_or(80.0),
                        })
                        .unwrap_or(semio_framework_os::WorkflowPosition { x: 80.0, y: 80.0 });
                    if let Some((operation, instance_id)) = spawn_app_instance_operation(plugin_id, app_id, None, position) {
                        self.runtime.borrow_mut().active_instance_id = Some(instance_id);
                        operations.push(operation);
                    }
                }
            }
            "moveMediaNode" => {
                if let (Some(node_id), Some(x), Some(y)) = (
                    args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()),
                    args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()),
                    args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()),
                ) {
                    coalesce_key = Some(format!("moveMediaNode:{node_id}"));
                    operations.push(OsOperation::MoveWorkflowNode {
                        node_id: node_id.into(),
                        x,
                        y,
                    });
                }
            }
            "connectMediaPorts" => {
                if let (
                    Some(source_node_id),
                    Some(source_port_id),
                    Some(target_node_id),
                    Some(target_port_id),
                ) = (
                    args.and_then(|value| value.get("sourceNodeId"))
                        .and_then(|value| value.as_str()),
                    args.and_then(|value| value.get("sourcePortId"))
                        .and_then(|value| value.as_str()),
                    args.and_then(|value| value.get("targetNodeId"))
                        .and_then(|value| value.as_str()),
                    args.and_then(|value| value.get("targetPortId"))
                        .and_then(|value| value.as_str()),
                ) {
                    match negotiate_media_connect(projection, source_node_id, source_port_id, target_node_id, target_port_id) {
                        Ok(contract) => operations.push(OsOperation::ConnectWorkflowPorts {
                            edge: semio_framework_os::OsWorkflowEdge {
                                id: create_os_id("edge"),
                                source_node_id: source_node_id.into(),
                                source_port_id: source_port_id.into(),
                                target_node_id: target_node_id.into(),
                                target_port_id: target_port_id.into(),
                                contract,
                            },
                        }),
                        Err(reason) => effects.push(HostEffect::Notify { message: reason }),
                    }
                }
            }
            "disconnectMediaEdge" => {
                if let Some(edge_id) = args
                    .and_then(|value| value.get("edgeId"))
                    .and_then(|value| value.as_str())
                {
                    operations.push(OsOperation::DisconnectWorkflowEdge {
                        edge_id: edge_id.into(),
                    });
                }
            }
            "removeAppInstance" => {
                let instance_id = args
                    .and_then(|value| value.get("instanceId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| primary_selected_instance_id(&*self.runtime.borrow(), projection));
                if let Some(instance_id) = instance_id {
                    operations.push(OsOperation::RemoveAppInstance {
                        instance_id: instance_id.clone(),
                    });
                    if self.runtime.borrow_mut().active_instance_id.as_deref() == Some(instance_id.as_str()) {
                        self.runtime.borrow_mut().active_instance_id = None;
                    }
                    if self.runtime.borrow_mut().focused_instance_id.as_deref() == Some(instance_id.as_str()) {
                        self.runtime.borrow_mut().focused_instance_id = None;
                    }
                }
            }
            "deleteSelection" => {
                let instance_ids = selected_instance_ids(&*self.runtime.borrow(), projection);
                for instance_id in instance_ids {
                    operations.push(OsOperation::RemoveAppInstance {
                        instance_id: instance_id.clone(),
                    });
                }
                self.runtime.borrow_mut().selected_app_instance_ids.clear();
                self.runtime.borrow_mut().selected_media_node_ids.clear();
                self.runtime.borrow_mut().active_instance_id = None;
                self.runtime.borrow_mut().focused_instance_id = None;
            }
            "copyAppInstance" => {
                self.runtime.borrow_mut().clipboard_instance_ids = selected_instance_ids(&*self.runtime.borrow(), projection);
            }
            "duplicateAppInstance" | "pasteAppInstance" => {
                let source_ids = if action == "pasteAppInstance" {
                    self.runtime.borrow_mut().clipboard_instance_ids.clone()
                } else {
                    selected_instance_ids(&*self.runtime.borrow(), projection)
                };
                for instance_id in source_ids {
                    let Some(instance) = projection
                        .app_instances
                        .iter()
                        .find(|row| row.id == instance_id)
                    else {
                        continue;
                    };
                    let position = projection
                        .workflow
                        .nodes
                        .iter()
                        .find(|node| node.instance_id == instance_id)
                        .map(|node| semio_framework_os::WorkflowPosition {
                            x: node.x + 40.0,
                            y: node.y + 40.0,
                        })
                        .unwrap_or(semio_framework_os::WorkflowPosition { x: 80.0, y: 80.0 });
                    let label = format!("{} Copy", instance.label);
                    if let Some((operation, new_id)) = spawn_app_instance_operation(
                        &instance.plugin_id,
                        &instance.app_id,
                        Some(&label),
                        position,
                    ) {
                        self.runtime.borrow_mut().active_instance_id = Some(new_id);
                        operations.push(operation);
                    }
                }
            }
            "renameAppInstance" => {
                if let Some(instance_id) = primary_selected_instance_id(&*self.runtime.borrow(), projection) {
                    let next_label = args
                        .and_then(|value| value.get("label"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string)
                        .or_else(|| {
                            projection
                                .app_instances
                                .iter()
                                .find(|row| row.id == instance_id)
                                .map(|instance| format!("{} (renamed)", instance.label))
                        });
                    if let Some(label) = next_label {
                        operations.push(OsOperation::PatchAppInstance {
                            instance_id,
                            label: Some(label),
                        });
                    }
                }
            }
            "setActiveExample" => {
                if let Some(example_id) = args
                    .and_then(|value| value.get("exampleId"))
                    .and_then(|value| value.as_str())
                    .filter(|value| !value.is_empty())
                {
                    // 🧭️ Examples are catalog documents in the new topology — selecting one navigates
                    // the shell to that studio route; the host's `openDocument(ref)` loads it (no
                    // in-place whole-document swap on the plugin side anymore); an empty id is the
                    // shell's "no example" reset and keeps the current studio route.
                    return ActionEmit::effect(HostEffect::Navigate { uri: format!("/spaces/{example_id}") });
                }
                return ActionEmit::default();
            }
            "exportMedia" => {
                if let (Some(instance_id), Some(format)) = (
                    args.and_then(|value| value.get("instanceId")).and_then(|value| value.as_str()),
                    args.and_then(|value| value.get("format")).and_then(|value| value.as_str()),
                ) {
                    if let Some(instance) = projection
                        .app_instances
                        .iter()
                        .find(|row| row.id == instance_id)
                    {
                        ensure_space_fixtures_registered();
                        // 📤️ `s` is a shell: the instance's live content lives in its own
                        // `OsDocumentRef`-addressed document (owned by the host's backbone), so this
                        // seeds a bare schema doc for the export handler rather than reaching into
                        // another document's store from here.
                        let document_json = materialize_os_app_instance_document_json(
                            &json!({ "schema": instance.document.schema }).to_string(),
                            &instance.id,
                            &projection.parameter_bindings,
                            &projection.parameters,
                        );
                        let document_value: Value = serde_json::from_str(&document_json).unwrap_or_else(|_| json!({}));
                        let export_format = semio_framework_os::OsMediaFormat::parse(format)
                            .unwrap_or(semio_framework_os::OsMediaFormat::Svg);
                        if let Ok(result) = semio_framework_os::export_os_app_instance_media(
                            instance,
                            &document_value,
                            export_format,
                        ) {
                            effects.push(HostEffect::DownloadMediaExport {
                                filename: result.file_name,
                                mime_type: result.mime_type,
                                data: result.data,
                                encoding: result.encoding,
                            });
                        }
                    }
                }
            }
            "importMedia" => {
                if let (Some(instance_id), Some(format)) = (
                    args.and_then(|value| value.get("instanceId")).and_then(|value| value.as_str()),
                    args.and_then(|value| value.get("format")).and_then(|value| value.as_str()),
                ) {
                    self.runtime.borrow_mut().pending_import_instance_id = Some(instance_id.to_string());
                    self.runtime.borrow_mut().pending_import_format = Some(format.to_string());
                    return ActionEmit::effect(HostEffect::RequestFileOpen {
                        accept: format!(".{format}"),
                        read_as: Some("dataUrl".into()),
                        import_action: "importMediaPayload".into(),
                        multiple: false,
                    });
                }
                return ActionEmit::default();
            }
            "importMediaPayload" => {
                if let (Some(instance_id), Some(format_name)) = (self.runtime.borrow_mut().pending_import_instance_id.take(), self.runtime.borrow_mut().pending_import_format.take()) {
                    let payload = args.and_then(|value| value.get("payload")).and_then(|value| value.as_str());
                    let format = semio_framework_os::OsMediaFormat::parse(&format_name);
                    if let (Some(payload), Some(format)) = (payload, format) {
                        use base64::Engine;
                        let base64_part = payload.split_once(',').map(|(_, data)| data).unwrap_or(payload);
                        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(base64_part) {
                            if let Some(instance) = projection.app_instances.iter().find(|row| row.id == instance_id) {
                                // 📥️ Decoding/validation happens here; the decoded content is applied
                                // to the instance's own `OsDocumentRef` document by the host (a
                                // cross-document operation the shell can't author from its own store), so
                                // this arm emits no studio operation.
                                let _ = semio_framework_os::import_os_app_instance_media(instance, &bytes, format);
                            }
                        }
                    }
                }
                return ActionEmit::default();
            }
            "exportStudioPack" => {
                let space_id = runtime_space_id(&*self.runtime.borrow());
                if let Some(document) = home_ui::resolve_studio_document(&space_id) {
                    // 📦️ Whole-document pack<->dsl codec (`register_document_codec_for_app::<SpaceApp>`
                    // in `register_s_exports`, see `🔖️DocumentCodecs`), not the per-instance media
                    // export above — mirrors `exportMedia`'s effect shape (base64 binary +
                    // plain-text sidecar) one level up, at the studio document itself.
                    if let Ok(pack_files) = export_os_space_pack(&document) {
                        use base64::Engine;
                        effects.push(HostEffect::DownloadMediaExport {
                            filename: format!("{space_id}.pack"),
                            mime_type: "application/octet-stream".into(),
                            data: base64::engine::general_purpose::STANDARD.encode(&pack_files.pack),
                            encoding: Some("base64".into()),
                        });
                        effects.push(HostEffect::DownloadMediaExport {
                            filename: format!("{space_id}.ops"),
                            mime_type: "text/plain".into(),
                            data: pack_files.ops,
                            encoding: None,
                        });
                    }
                }
            }
            "exportStudioDsl" => {
                let space_id = runtime_space_id(&*self.runtime.borrow());
                if let Some(document) = home_ui::resolve_studio_document(&space_id) {
                    if let Ok(text_files) = export_os_space_dsl(&document) {
                        effects.push(HostEffect::DownloadMediaExport {
                            filename: format!("{space_id}.os"),
                            mime_type: "text/plain".into(),
                            data: text_files.dsl,
                            encoding: None,
                        });
                    }
                }
            }
            "importSpacePack" => {
                return ActionEmit::effect(HostEffect::RequestFileOpen {
                    accept: ".pack".into(),
                    read_as: Some("dataUrl".into()),
                    import_action: "importSpacePackPayload".into(),
                    multiple: false,
                });
            }
            "importSpacePackPayload" => {
                if let Some(payload) = args.and_then(|value| value.get("payload")).and_then(|value| value.as_str()) {
                    use base64::Engine;
                    let base64_part = payload.split_once(',').map(|(_, data)| data).unwrap_or(payload);
                    if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(base64_part) {
                        // 🌱️ A single `.pack` file carries no separate `.spr` sidecar (unlike
                        // `exportStudioPack`'s two-file output) — `store::empty_document_spr`
                        // builds a bare, edit-free op log so the pack+spr-first codec path still
                        // decodes to a document with no replayed edit history, i.e. its bare
                        // initial projection, exactly like `importSpace`'s JSON-envelope
                        // counterpart.
                        let empty_spr = store::empty_document_spr("", OS_SPACE_SCHEMA);
                        let _ = import_os_space_from_pack(&bytes, &empty_spr, home_ui::catalog_port());
                    }
                }
                return ActionEmit::default();
            }
            "selectInstance" => {
                self.runtime.borrow_mut().active_instance_id = args
                    .and_then(|value| value.get("instanceId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                if let Some(instance_id) = self.runtime.borrow_mut().active_instance_id.clone() {
                    let node_id = projection
                        .workflow
                        .nodes
                        .iter()
                        .find(|node| node.instance_id == instance_id)
                        .map(|node| node.id.clone());
                    self.runtime.borrow_mut().selected_app_instance_ids = vec![instance_id];
                    self.runtime.borrow_mut().selected_media_node_ids = node_id.into_iter().collect();
                }
            }
            "nodeGraphSelect" | "setMediaNodeSelection" => {
                let node_ids: Vec<String> = if args
                    .and_then(|value| value.get("selectAll"))
                    .and_then(|value| value.as_bool())
                    .unwrap_or(false)
                {
                    projection.workflow.nodes.iter().map(|node| node.id.clone()).collect()
                } else {
                    args
                        .and_then(|value| value.get("nodeIds"))
                        .and_then(|value| value.as_array())
                        .map(|rows| {
                            rows.iter()
                                .filter_map(|value| value.as_str().map(str::to_string))
                                .collect()
                        })
                        .unwrap_or_default()
                };
                self.runtime.borrow_mut().selected_media_node_ids = node_ids.clone();
                self.runtime.borrow_mut().selected_app_instance_ids = node_ids
                    .iter()
                    .filter_map(|node_id| {
                        projection
                            .workflow
                            .nodes
                            .iter()
                            .find(|node| node.id == *node_id)
                            .map(|node| node.instance_id.clone())
                    })
                    .collect();
                if self.runtime.borrow_mut().selected_app_instance_ids.len() == 1 {
                    self.runtime.borrow_mut().active_instance_id = self.runtime.borrow_mut().selected_app_instance_ids.first().cloned();
                }
            }
            "reorganizeWorkflow" => {
                let node_ids: Vec<String> = if self.runtime.borrow_mut().selected_media_node_ids.is_empty() {
                    projection.workflow.nodes.iter().map(|node| node.id.clone()).collect()
                } else {
                    self.runtime.borrow_mut().selected_media_node_ids.clone()
                };
                for (index, node_id) in node_ids.iter().enumerate() {
                    let col = (index % 4) as f64;
                    let row = (index / 4) as f64;
                    operations.push(OsOperation::MoveWorkflowNode {
                        node_id: node_id.clone(),
                        x: 80.0 + col * 220.0,
                        y: 80.0 + row * 160.0,
                    });
                }
            }
            "nodeGraphHover" | "textHover" => {
                self.runtime.borrow_mut().hovered_media_node_id = args
                    .and_then(|value| value.get("hoverJson"))
                    .and_then(|value| {
                        if value.is_null() {
                            None
                        } else if let Some(text) = value.as_str() {
                            serde_json::from_str::<Value>(text)
                                .ok()
                                .and_then(|parsed| parsed.get("nodeId").and_then(|id| id.as_str().map(str::to_string)))
                                .or_else(|| Some(text.to_string()))
                        } else {
                            value.get("nodeId").and_then(|id| id.as_str().map(str::to_string))
                        }
                    });
            }
            "nodeGraphViewport" => {
                if let Some(camera) = args
                    .and_then(|value| value.get("viewportJson"))
                    .and_then(|value| value.as_str())
                    .and_then(|viewport_json| serde_json::from_str::<OsWorkflowCamera>(viewport_json).ok())
                {
                    self.runtime.borrow_mut().workflow_camera = Some(camera);
                }
            }
            "nodeGraphEdit" => {
                let edit_operations = args
                    .and_then(|value| value.get("operations"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                for edit in &edit_operations {
                    match edit.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                        "setFixture" => {
                            if let Some(fixture_json) = edit.get("fixtureJson").and_then(|value| value.as_str()) {
                                if let Some(camera) = serde_json::from_str::<Value>(fixture_json)
                                    .ok()
                                    .and_then(|fixture| fixture.get("camera").cloned())
                                    .and_then(|camera| serde_json::from_value::<OsWorkflowCamera>(camera).ok())
                                {
                                    self.runtime.borrow_mut().workflow_camera = Some(camera);
                                }
                                operations.extend(apply_flow_fixture_to_os_workflow(&projection.workflow, fixture_json));
                            }
                        }
                        "move" => {
                            if let (Some(node_id), Some(x), Some(y)) = (
                                edit.get("nodeId").and_then(|value| value.as_str()),
                                edit.get("x").and_then(|value| value.as_f64()),
                                edit.get("y").and_then(|value| value.as_f64()),
                            ) {
                                operations.push(OsOperation::MoveWorkflowNode { node_id: node_id.into(), x, y });
                            }
                        }
                        "connect" => {
                            if let (Some(source_node_id), Some(source_port_id), Some(target_node_id), Some(target_port_id)) = (
                                edit.get("sourceNodeId").and_then(|value| value.as_str()),
                                edit.get("sourcePortId").and_then(|value| value.as_str()),
                                edit.get("targetNodeId").and_then(|value| value.as_str()),
                                edit.get("targetPortId").and_then(|value| value.as_str()),
                            ) {
                                match negotiate_media_connect(projection, source_node_id, source_port_id, target_node_id, target_port_id) {
                                    Ok(contract) => operations.push(OsOperation::ConnectWorkflowPorts {
                                        edge: semio_framework_os::OsWorkflowEdge {
                                            id: create_os_id("edge"),
                                            source_node_id: source_node_id.into(),
                                            source_port_id: source_port_id.into(),
                                            target_node_id: target_node_id.into(),
                                            target_port_id: target_port_id.into(),
                                            contract,
                                        },
                                    }),
                                    Err(reason) => effects.push(HostEffect::Notify { message: reason }),
                                }
                            }
                        }
                        "deleteSelection" => {
                            for node_id in &*self.runtime.borrow().selected_media_node_ids {
                                if let Some(node) = projection.workflow.nodes.iter().find(|node| node.id == *node_id) {
                                    operations.push(OsOperation::RemoveAppInstance { instance_id: node.instance_id.clone() });
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            "presenceHeartbeat" => {
                if let Some(client_id) = args.and_then(|value| value.get("clientId")).and_then(|value| value.as_str()) {
                    self.runtime.borrow_mut().client_id = Some(client_id.into());
                    self.runtime.borrow_mut().client_name = Some(
                        args.and_then(|value| value.get("name"))
                            .and_then(|value| value.as_str())
                            .unwrap_or("Guest")
                            .into(),
                    );
                }
                // 🐢️ A heartbeat only records this client's own identity for the presence broadcast below
                // — it never changes anything this instance's own UI should re-render.
                ui_scope = semio_framework_core::kernel::UiDirtyScope::None;
            }
            "setAppInstanceSelection" => {
                let instance_ids: Vec<String> = args
                    .and_then(|value| value.get("instanceIds"))
                    .and_then(|value| value.as_array())
                    .map(|rows| {
                        rows.iter()
                            .filter_map(|value| value.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default();
                self.runtime.borrow_mut().selected_app_instance_ids = instance_ids.clone();
                self.runtime.borrow_mut().selected_media_node_ids = instance_ids
                    .iter()
                    .filter_map(|instance_id| {
                        projection
                            .workflow
                            .nodes
                            .iter()
                            .find(|node| node.instance_id == *instance_id)
                            .map(|node| node.id.clone())
                    })
                    .collect();
                if instance_ids.len() == 1 {
                    self.runtime.borrow_mut().active_instance_id = Some(instance_ids[0].clone());
                }
            }
            "patchMediaNodes" => {
                let node_ids: Vec<String> = args
                    .and_then(|value| value.get("nodeIds"))
                    .and_then(|value| value.as_array())
                    .map(|rows| {
                        rows.iter()
                            .filter_map(|value| value.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str());
                let axis = args.and_then(|value| value.get("axis")).and_then(|value| value.as_str());
                let numeric = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_f64())
                    .or_else(|| {
                        args.and_then(|value| value.get("value"))
                            .and_then(|value| value.as_str())
                            .and_then(|value| value.parse().ok())
                    });
                if field == Some("position") && numeric.is_some() {
                    for node_id in node_ids {
                        if let Some(node) = projection.workflow.nodes.iter().find(|row| row.id == node_id) {
                            let x = if axis == Some("x") {
                                numeric.unwrap()
                            } else {
                                node.x
                            };
                            let y = if axis == Some("y") {
                                numeric.unwrap()
                            } else {
                                node.y
                            };
                            operations.push(OsOperation::MoveWorkflowNode {
                                node_id,
                                x,
                                y,
                            });
                        }
                    }
                }
            }
            "patchAppInstances" => {
                let instance_ids: Vec<String> = args
                    .and_then(|value| value.get("instanceIds"))
                    .and_then(|value| value.as_array())
                    .map(|rows| {
                        rows.iter()
                            .filter_map(|value| value.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str());
                let value = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str());
                if field == Some("label") {
                    for instance_id in instance_ids {
                        if let Some(label) = value {
                            operations.push(OsOperation::PatchAppInstance {
                                instance_id,
                                label: Some(label.into()),
                            });
                        }
                    }
                }
            }
            "bindParameterField" => {
                let instance_id = args
                    .and_then(|value| value.get("instanceId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let field_path = args
                    .and_then(|value| value.get("fieldPath"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let parameter_id = args
                    .and_then(|value| value.get("parameterId").or_else(|| value.get("value")))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if !instance_id.is_empty() && !field_path.is_empty() {
                    if parameter_id.is_empty() || parameter_id == "__direct__" {
                        operations.push(OsOperation::UnbindParameterField {
                            instance_id: instance_id.into(),
                            field_path: field_path.into(),
                        });
                    } else {
                        operations.push(OsOperation::BindParameterField {
                            binding: OsParameterFieldBinding {
                                parameter_id: parameter_id.into(),
                                instance_id: instance_id.into(),
                                field_path: field_path.into(),
                            },
                        });
                    }
                }
            }
            "unbindParameterField" => {
                let instance_id = args
                    .and_then(|value| value.get("instanceId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let field_path = args
                    .and_then(|value| value.get("fieldPath"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if !instance_id.is_empty() && !field_path.is_empty() {
                    operations.push(OsOperation::UnbindParameterField {
                        instance_id: instance_id.into(),
                        field_path: field_path.into(),
                    });
                }
            }
            "openSpace" => {
                if let Some(space_id) = args
                    .and_then(|value| value.get("spaceId"))
                    .and_then(|value| value.as_str())
                {
                    self.runtime.borrow_mut().space_id = Some(space_id.into());
                    self.runtime.borrow_mut().focused_instance_id = None;
                    self.runtime.borrow_mut().selected_media_node_ids.clear();
                    self.runtime.borrow_mut().selected_app_instance_ids.clear();
                    self.runtime.borrow_mut().clipboard_instance_ids.clear();
                    let document = home_ui::resolve_studio_document(space_id)
                        .or_else(|| {
                            if space_id == "demo" {
                                Some(parse_demo_space_document())
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| create_empty_os_document(space_id, "Untitled Studio"));
                    if let Ok(projection) = materialize_os_projection(&document, &[]) {
                        self.runtime.borrow_mut().active_instance_id =
                            projection.app_instances.first().map(|instance| instance.id.clone());
                    } else {
                        self.runtime.borrow_mut().active_instance_id = None;
                    }
                    if let Some(files) = home_ui::space_document_envelope_pack(&document) {
                        eprintln!(
                            "[DEBUG] openSpace id={} instances={} backbone={:?}",
                            space_id,
                            document.vcs.initial_projection.app_instances.len(),
                            document.backbone.as_ref().map(|row| row.uri.clone())
                        );
                        return ActionEmit::effect(HostEffect::LoadDocument { pack: files.pack, spr: files.spr });
                    }
                    eprintln!("[DEBUG] openSpace missing envelope id={space_id}");
                    self.runtime.borrow_mut().active_instance_id = None;
                }
                return ActionEmit::default();
            }
            "openInstance" => {
                let instance_id = args
                    .and_then(|value| value.get("instanceId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| primary_selected_instance_id(&*self.runtime.borrow(), projection));
                if let Some(instance_id) = instance_id {
                    self.runtime.borrow_mut().focused_instance_id = Some(instance_id.clone());
                    self.runtime.borrow_mut().active_instance_id = Some(instance_id.clone());
                    self.runtime.borrow_mut().selected_app_instance_ids = vec![instance_id.clone()];
                    if let Some(node) = projection
                        .workflow
                        .nodes
                        .iter()
                        .find(|row| row.instance_id == instance_id)
                    {
                        self.runtime.borrow_mut().selected_media_node_ids = vec![node.id.clone()];
                    }
                    if let Some(instance) = projection
                        .app_instances
                        .iter()
                        .find(|row| row.id == instance_id)
                    {
                        effects.push(HostEffect::OpenPluginInstance {
                            plugin_id: instance.plugin_id.clone(),
                            app_id: instance.app_id.clone(),
                            os_instance_id: Some(instance.id.clone()),
                        });
                    }
                }
            }
            "closeFocusedInstance" => {
                self.runtime.borrow_mut().focused_instance_id = None;
                let mut panel = parse_panel_state(view_state);
                panel.active_spawned_id = None;
                return ActionEmit::effect(HostEffect::SetPanel { panel_json: panel_json(&panel) });
            }
            "goHome" => return ActionEmit::effect(HostEffect::Navigate { uri: "/".into() }),
            "workflowEngagementInput" => {
                self.runtime.borrow_mut().workflow_engagement_input = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .into();
            }
            "workflowEngagementSubmit" => {
                let raw = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| self.runtime.borrow_mut().workflow_engagement_input.clone());
                let mut parts = raw.split_whitespace();
                if let (Some(plugin_id), Some(app_id)) = (parts.next(), parts.next()) {
                    if let Some((operation, instance_id)) = spawn_app_instance_operation(
                        plugin_id,
                        app_id,
                        None,
                        semio_framework_os::WorkflowPosition { x: 80.0, y: 80.0 },
                    ) {
                        self.runtime.borrow_mut().active_instance_id = Some(instance_id);
                        operations.push(operation);
                    }
                }
            }
            "compiledDagEngagementInput" => {
                self.runtime.borrow_mut().compiled_dag_engagement_input = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .into();
            }
            "compiledDagEngagementSubmit" => {}
            _ => {}
        }
        if matches!(
            action,
            "presenceHeartbeat" | "nodeGraphSelect" | "setMediaNodeSelection" | "selectInstance" | "setAppInstanceSelection" | "deleteSelection"
        ) {
            publish_presence(&*self.runtime.borrow());
        }
        ActionEmit {
            operations,
            coalesce_key,
            effects,
            ui_scope,
            ..Default::default()
        }
    }

    fn render(
        &self,
        body_key: &str,
        doc: &DocumentView<'_, OsProjection>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
    ) -> UiNode {
        let projection = doc.projection;
        let panel = parse_panel_state(view_state);
        let labels = resolve_labels::<SStudioLabels>(view_state);
        match body_key {
            S_PLAY_BODY_WORKFLOW => render_workflow(projection, &*self.runtime.borrow(), labels),
            S_PLAY_BODY_MEDIA_VFS => render_media_vfs(projection, labels),
            S_PLAY_BODY_COMPILED_DAG => render_compiled_dag(projection),
            S_PLAY_CATALOGUE_BODY_KEY => build_catalogue_tree(&panel, labels),
            S_PLAY_PARAMETERS_BODY_KEY => build_parameters_tree(projection, labels),
            S_PLAY_INSPECTOR_BODY_KEY => build_inspector_tree(projection, &*self.runtime.borrow(), labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_measures(
        &self,
        doc: &DocumentView<'_, OsProjection>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
    ) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = resolve_labels::<SStudioLabels>(view_state);
        HashMap::from([(
            S_PLAY_WINDOW_WORKFLOW.into(),
            workflow_measures(&*self.runtime.borrow(), &doc.projection.app_instances, labels),
        )])
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<SStudioLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(S_PLAY_WINDOW_WORKFLOW, labels.window_workflow)
            .window_kind_label(S_PLAY_WINDOW_MEDIA_VFS, labels.window_media_vfs)
            .window_kind_label(S_PLAY_WINDOW_COMPILED_DAG, labels.window_compiled_dag)
            .action_labels(s_studio_action_labels(is_de))
    }
}
//#endregion 🔖️SpaceApp

//#region 🔖️SpaceManifest
fn space_play_layout() -> WindowLayout {
    create_default_layout(
        &[
            S_PLAY_WINDOW_WORKFLOW.into(),
            S_PLAY_WINDOW_MEDIA_VFS.into(),
            S_PLAY_WINDOW_COMPILED_DAG.into(),
        ],
        "row",
        Some(&[40.0, 30.0, 30.0]),
        Some(&[
            "Workflow".into(),
            "Media VFS".into(),
            "Compiled DAG".into(),
        ]),
    )
}

fn workflow_engagement(runtime: &StudioRuntimeState, node_count: usize, app_count: usize) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("s-media-catalogue-hint".into()),
            value: Some(runtime.workflow_engagement_input.clone()),
            placeholder: Some("Drag apps from Catalogue workbench tab".into()),
            on_change: Some(s_play_action("workflowEngagementInput", None)),
            on_submit: Some(s_play_action("workflowEngagementSubmit", None)),
            disabled: None,
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: "s-media-count".into(),
            text: format!("{node_count} nodes · {app_count} apps"),
        }]),
        possible_engagements: None,
    }
}

fn workflow_measures(runtime: &StudioRuntimeState, instances: &[OsAppInstance], labels: &SStudioLabels) -> Vec<WindowMeasure> {
    vec![WindowMeasure::Select {
        id: "s-media-active-instance".into(),
        label: Some(labels.active_app.into()),
        value: runtime.active_instance_id.clone().unwrap_or_default(),
        items: instances
            .iter()
            .map(|instance| MeasureSelectItem {
                id: instance.id.clone(),
                value: instance.id.clone(),
                label: instance.label.clone(),
            })
            .collect(),
        on_change: s_play_action("selectInstance", None),
    }]
}

fn compiled_dag_engagement(projection: &OsProjection) -> WindowEngagement {
    let wire = compiled_dag_wire_literal(projection);
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: None,
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: "s-compiled-dag-status".into(),
            text: if wire.trim().is_empty() { "Empty".into() } else { "Compiled".into() },
        }]),
        possible_engagements: None,
    }
}

pub fn create_space_app() -> App {
    let projection = demo_space_projection();
    let runtime = StudioRuntimeState {
        active_instance_id: projection.app_instances.first().map(|instance| instance.id.clone()),
        ..StudioRuntimeState::default()
    };
    let engagement = workflow_engagement(
        &runtime,
        projection.workflow.nodes.len(),
        projection.app_instances.len(),
    );
    let measures = workflow_measures(&runtime, &projection.app_instances, resolve_labels::<SStudioLabels>(&ViewState::default()));
    let builder = App::builder(S_PLAY_APP_ID, "Space").document(["semio", "s", "studio"])
        .icon_id("s")
        .mode("main", "Space", "globe")
        .default_mode_id("main")
        .window_kind(S_PLAY_WINDOW_WORKFLOW, "Workflow", S_PLAY_BODY_WORKFLOW, SurfaceKind::NodeGraph, "graph-media")
        .window_kind(S_PLAY_WINDOW_MEDIA_VFS, "Media VFS", S_PLAY_BODY_MEDIA_VFS, SurfaceKind::VirtualFileSystem, "folder")
        .window_kind(
            S_PLAY_WINDOW_COMPILED_DAG,
            "Compiled DAG",
            S_PLAY_BODY_COMPILED_DAG,
            SurfaceKind::NodeGraph,
            "git-merge",
        )
        .panel_tab(
            S_PLAY_CATALOGUE_TAB_ID,
            FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
            PanelGroup::Workbench,
            S_PLAY_CATALOGUE_BODY_KEY,
        )
        .panel_tab(
            S_PLAY_PARAMETERS_TAB_ID,
            FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
            PanelGroup::Workbench,
            S_PLAY_PARAMETERS_BODY_KEY,
        )
        .panel_tab(
            S_PLAY_INSPECTOR_TAB_ID,
            FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
            PanelGroup::Details,
            S_PLAY_INSPECTOR_BODY_KEY,
        )
        .default_layout(space_play_layout())
        .operation("setParameter", "Set Parameter")
        .operation("patchParameter", "Patch Parameter")
        .operation("addParameter", "Add Parameter")
        .operation("removeParameter", "Remove Parameter")
        .operation("spawnApp", "Spawn App")
        .operation("moveMediaNode", "Move Media Node")
        .operation("connectMediaPorts", "Connect Media Ports")
        .operation("disconnectMediaEdge", "Disconnect Media Edge")
        .operation("removeAppInstance", "Remove App Instance")
        .operation("deleteSelection", "Delete Selection")
        .operation("copyAppInstance", "Copy App Instance")
        .operation("duplicateAppInstance", "Duplicate App Instance")
        .operation("pasteAppInstance", "Paste App Instance")
        .operation("renameAppInstance", "Rename App Instance")
        .operation("patchMediaNodes", "Patch Media Nodes")
        .operation("patchAppInstances", "Patch App Instances")
        .operation("bindParameterField", "Bind Parameter Field")
        .operation("unbindParameterField", "Unbind Parameter Field")
        .operation("reorganizeWorkflow", "Reorganize Workflow")
        .operation("workflowEngagementSubmit", "Workflow Engagement Submit")
        .operation("compiledDagEngagementSubmit", "Compiled DAG Engagement Submit")
        .operation("nodeGraphEdit", "Edit Workflow")
        .view_action("setActivePanelTab", "Set Active Panel Tab")
        .view_action("selectInstance", "Select Instance")
        .view_action("nodeGraphSelect", "Select Graph Node")
        .view_action("setMediaNodeSelection", "Set Media Node Selection")
        .view_action("nodeGraphHover", "Hover Graph Node")
        .view_action("textHover", "Text Hover")
        .view_action("nodeGraphViewport", "Set Graph Viewport")
        .view_action("presenceHeartbeat", "Presence Heartbeat")
        .view_action("setAppInstanceSelection", "Set App Instance Selection")
        .view_action("workflowEngagementInput", "Workflow Engagement Input")
        .view_action("compiledDagEngagementInput", "Compiled DAG Engagement Input")
        .shell_action("setActiveExample", "Set Active Example")
        .shell_action("exportMedia", "Export Media")
        .shell_action("importMedia", "Import Media")
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importMediaPayload", "Import Media Payload", ActionKind::Shell) })
        .shell_action("exportStudioPack", "Export Studio Pack")
        .shell_action("exportStudioDsl", "Export Studio DSL")
        .shell_action("importSpacePack", "Import Studio Pack")
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importSpacePackPayload", "Import Studio Pack Payload", ActionKind::Shell) })
        .shell_action("openSpace", "Open Studio")
        .shell_action("openInstance", "Open Instance")
        .shell_action("closeFocusedInstance", "Close Focused Instance")
        .shell_action("goHome", "Go Home")
        .shell_action("navigateVirtualFileSystemNode", "Navigate File System Node")
        // 📝️ Staged argument form for parameter creation (spawnApp/exportMedia stay context/registry-driven).
        .action_args("addParameter", vec![
            ActionArgDef::text("name", "Name").default_value("Parameter"),
            ActionArgDef::select("type", "Type", vec![
                ActionArgOption::new("numeric", "Numeric"),
                ActionArgOption::new("categorical", "Categorical"),
                ActionArgOption::new("toggle", "Toggle"),
                ActionArgOption::new("text", "Text"),
            ]).default_value("numeric"),
        ])
        // 📇️ Per-window action scoping — the Workflow (NodeGraph) window owns all graph/instance/
        // parameter editing plus the per-instance media import/export; the Media VFS
        // (VirtualFileSystem) window only navigates the media file tree; the read-only Compiled DAG
        // window only drives its own engagement. Navigation, panel-tab, presence, example and generic
        // node-graph view actions stay unscoped orphans and appear on every window.
        .window_kind_actions(S_PLAY_WINDOW_WORKFLOW, vec![
            "setParameter".into(), "patchParameter".into(), "addParameter".into(), "removeParameter".into(),
            "spawnApp".into(), "moveMediaNode".into(), "connectMediaPorts".into(), "disconnectMediaEdge".into(),
            "removeAppInstance".into(), "deleteSelection".into(), "copyAppInstance".into(),
            "duplicateAppInstance".into(), "pasteAppInstance".into(), "renameAppInstance".into(),
            "patchMediaNodes".into(), "patchAppInstances".into(), "bindParameterField".into(),
            "unbindParameterField".into(), "reorganizeWorkflow".into(), "workflowEngagementSubmit".into(),
            "workflowEngagementInput".into(), "nodeGraphEdit".into(), "selectInstance".into(),
            "setMediaNodeSelection".into(), "setAppInstanceSelection".into(), "exportMedia".into(),
            "importMedia".into(), "importMediaPayload".into(),
        ])
        .window_kind_actions(S_PLAY_WINDOW_MEDIA_VFS, vec![
            "navigateVirtualFileSystemNode".into(),
        ])
        .window_kind_actions(S_PLAY_WINDOW_COMPILED_DAG, vec![
            "compiledDagEngagementSubmit".into(), "compiledDagEngagementInput".into(),
        ])
        .keybinding("mod+z", "undo")
        .keybinding("mod+shift+z", "redo")
        .keybinding("mod+s", "commitCheckpoint");
    let mut definition = builder.build_definition();
    if let Some(window) = definition
        .window_kinds
        .iter_mut()
        .find(|window| window.id == S_PLAY_WINDOW_WORKFLOW)
    {
        window.options.measures = measures;
        window.options.engagement = WindowEngagementSlot::Some(engagement);
    }
    let compiled_engagement = compiled_dag_engagement(&demo_space_projection());
    if let Some(window) = definition
        .window_kinds
        .iter_mut()
        .find(|window| window.id == S_PLAY_WINDOW_COMPILED_DAG)
    {
        window.options.engagement = WindowEngagementSlot::Some(compiled_engagement);
    }
    let mut app = App {
        definition,
        examples: vec![],
        workflow: None,
    };
    app.definition.controller_id = S_PLAY_CONTROLLER_ID.into();
    let mut app = app.workflow("s", "S Studio", "studio");
    for (id, label) in S_STUDIO_EXAMPLES {
        let json = os_document_to_json(&parse_demo_space_document()).expect("serialize demo studio document");
        app = app.example(*id, *label, json, "file-text");
    }
    app
}
//#endregion 🔖️SpaceManifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_os::{
        apply_os_operation, merge_os_plugin_definition, os_baseline_resource, os_in_port, os_out_port, register_artifact_descriptor, validate_workflow, MediaClass, MediaForm, MediaType, MediaWireFormat, OsAppResourceSpec,
        OsMediaFormat, OsWorkflowNode, OsMediaPort, OsPlatformAppInput, OsPlatformInput, ArtifactKindSpec,
    };
    use semio_framework_plugin::{testkit, HistoryView, ModeDefinition, PluginApp, UiControlNode, UiNode, VcsDocumentApp};

    //#region 🔧️Harness
    fn empty_history() -> HistoryView {
        HistoryView::empty()
    }

    /// 🎛️ Drives the typed `SpaceApp::handle_action` against a projection snapshot, returning its emit.
    ///
    /// 🩹️ Renamed from the pre-split monolith's mismatched pair: the harness fn was defined as
    /// `space_emit` but every call site below already called it `studio_emit` — a latent bug (every
    /// call site would have failed to resolve). Renamed the definition to match the (majority, and
    /// pre-existing) call sites rather than the other way around.
    fn studio_emit(app: &mut SpaceApp, projection: &OsProjection, action: &str, args: Value) -> ActionEmit<OsOperation> {
        let history = empty_history();
        let doc = DocumentView { projection, history: &history };
        app.handle_action(action, Some(&args), &doc, &ViewState::default())
    }

    /// 📽️ Folds studio operations onto a projection the way the store would (minus history), for operation-application asserts.
    fn apply_operations(projection: &OsProjection, operations: &[OsOperation]) -> OsProjection {
        operations.iter().fold(projection.clone(), |current, operation| apply_os_operation(&current, operation))
    }
    //#endregion 🔧️Harness

    #[test]
    fn initial_projection_is_empty_not_demo() {
        let app = SpaceApp::new();
        assert!(app.initial_projection().app_instances.is_empty());
        assert!(app.runtime.active_instance_id.is_none());
    }

    #[test]
    fn open_studio_loads_created_empty_catalog_studio() {
        use semio_framework_os::{create_os_space, MemoryBackbonePort};
        use std::sync::Arc;
        let port: Arc<dyn semio_framework_os::OsBackbonePort> = Arc::new(MemoryBackbonePort::new());
        let entry = create_os_space("Opened Empty", port.clone()).expect("create");
        home_ui::register_studio_port_for_test(&entry.id, port);
        let mut app = SpaceApp::new();
        let empty = default_os_projection();
        let emit = studio_emit(&mut app, &empty, "openSpace", json!({ "spaceId": entry.id }));
        assert_eq!(app.runtime.space_id.as_deref(), Some(entry.id.as_str()));
        assert!(app.runtime.active_instance_id.is_none());
        assert!(
            emit.effects
                .iter()
                .any(|effect| matches!(effect, HostEffect::LoadDocument { .. }))
        );
        assert!(!emit.effects.iter().any(|effect| matches!(effect, HostEffect::Navigate { .. })));
    }

    fn load_document_projection(emit: &ActionEmit<OsOperation>) -> (OsProjection, String) {
        let (pack, spr) = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::LoadDocument { pack, spr } => Some((pack.as_slice(), spr.as_slice())),
                _ => None,
            })
            .expect("load document");
        let parsed: store::ParsedDocumentText<OsProjection, OsOperation> = store::parse_document_pack(pack, spr).expect("parse document pack");
        let id = parsed.envelope.id.clone();
        (parsed.projection, id)
    }

    #[test]
    fn open_studio_unknown_id_loads_empty_not_demo() {
        let mut app = SpaceApp::new();
        let empty = default_os_projection();
        let emit = studio_emit(&mut app, &empty, "openSpace", json!({ "spaceId": "unknown-studio-id" }));
        let (projection, id) = load_document_projection(&emit);
        assert_eq!(id, "unknown-studio-id");
        assert!(projection.app_instances.is_empty());
        assert_ne!(id, "demo");
        assert!(app.runtime.active_instance_id.is_none());
    }

    #[test]
    fn open_studio_demo_explicit_loads_demo_fixture() {
        let mut app = SpaceApp::new();
        let empty = default_os_projection();
        let emit = studio_emit(&mut app, &empty, "openSpace", json!({ "spaceId": "demo" }));
        let (projection, id) = load_document_projection(&emit);
        assert!(id.contains("demo-studio"));
        assert!(!projection.app_instances.is_empty());
    }

    #[test]
    fn open_studio_loads_ephemeral_created_studio() {
        let mut home = home_ui::HomeApp;
        let projection = home.initial_projection();
        let history = empty_history();
        let doc = DocumentView { projection: &projection, history: &history };
        let create = home.handle_action("createStudio", Some(&json!({ "name": "Ephemeral Open" })), &doc, &ViewState::default());
        let space_id = create
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::Navigate { uri } => Some(uri.trim_start_matches("/spaces/").to_string()),
                _ => None,
            })
            .expect("navigate");
        let mut app = SpaceApp::new();
        let empty = default_os_projection();
        let emit = studio_emit(&mut app, &empty, "openSpace", json!({ "spaceId": space_id.clone() }));
        let (projection, id) = load_document_projection(&emit);
        assert_eq!(id, space_id);
        assert!(projection.app_instances.is_empty());
    }

    fn seed_draw_plugin() {
        let mut resources = HashMap::new();
        resources.insert(
            "draw".into(),
            os_baseline_resource("2d.drawing", "draw.document", "draw"),
        );
        merge_os_plugin_definition(
            "draw",
            &OsPlatformInput {
                id: "draw".into(),
                name: "Draw".into(),
                api_version: "1".into(),
                apps: vec![OsPlatformAppInput {
                    id: "draw".into(),
                    label: "Draw".into(),
                    document: vec!["semio".into(), "draw".into()],
                    controller_id: "draw-play".into(),
                    modes: vec![ModeDefinition {
                        id: "edit".into(),
                        label: "Edit".into(),
                icon_id: "square-pen".into(),
                        tools: vec![],
                        layout_id: None,
                        commands: vec![],
                    }],
                    default_mode_id: None,
                }],
            },
            &resources,
        )
        .expect("merge draw");
    }

    #[test]
    fn demo_document_has_instances_and_edges() {
        let projection = demo_space_projection();
        assert!(projection.app_instances.len() >= 5);
        assert!(projection.workflow.nodes.len() >= 2);
        assert!(projection.workflow.edges.len() >= 1);
        assert!(validate_workflow(&projection.workflow).ok);
    }

    #[test]
    fn renders_workflow_scene() {
        let mut app = VcsDocumentApp::new(SpaceApp::new());
        let node = app.render(S_PLAY_BODY_WORKFLOW, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
    }

    #[test]
    fn space_window_kind_actions_scope_editing_to_workflow() {
        let definition = create_space_app().definition;
        let resolve = |window_id: &str| -> Vec<String> {
            let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
            semio_framework_plugin::resolve_window_actions(&definition, window)
                .into_iter()
                .map(|action| action.id.clone())
                .collect()
        };
        let graph = resolve(S_PLAY_WINDOW_WORKFLOW);
        let vfs = resolve(S_PLAY_WINDOW_MEDIA_VFS);
        let dag = resolve(S_PLAY_WINDOW_COMPILED_DAG);
        for graph_operation in ["spawnApp", "connectMediaPorts", "removeAppInstance", "exportMedia", "addParameter"] {
            assert!(graph.contains(&graph_operation.to_string()), "Workflow must expose {graph_operation}");
            assert!(!vfs.contains(&graph_operation.to_string()), "Media VFS must NOT expose {graph_operation}");
            assert!(!dag.contains(&graph_operation.to_string()), "Compiled DAG must NOT expose {graph_operation}");
        }
        assert!(vfs.contains(&"navigateVirtualFileSystemNode".to_string()));
        assert!(!graph.contains(&"navigateVirtualFileSystemNode".to_string()));
        assert!(dag.contains(&"compiledDagEngagementSubmit".to_string()));
        assert!(!graph.contains(&"compiledDagEngagementSubmit".to_string()));
        // 🌐️ Global navigation/utility actions stay orphans on every window.
        for shared in ["setActiveExample", "goHome"] {
            assert!(graph.contains(&shared.to_string()) && vfs.contains(&shared.to_string()) && dag.contains(&shared.to_string()), "{shared} stays global");
        }
    }

    #[test]
    fn renders_compiled_dag_editor() {
        let mut app = VcsDocumentApp::new(SpaceApp::new());
        let node = app.render(S_PLAY_BODY_COMPILED_DAG, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("text-editor"));
        let wire = compiled_dag_wire_literal(&demo_space_projection());
        assert!(wire.contains("appInstance") || wire.contains("draw"));
    }

    #[test]
    fn space_manifest_uses_studio_app_id() {
        let app = create_space_app();
        assert_eq!(app.definition.id, "studio");
        assert_eq!(app.definition.controller_id, "s-play");
        assert_eq!(app.workflow.as_ref().map(|p| p.workflow_step_id.as_str()), Some("s"));
    }

    #[test]
    fn move_media_node_emits_coalesced_move_operation() {
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let node_id = projection.workflow.nodes.first().expect("node").id.clone();
        let emit = studio_emit(&mut app, &projection, "moveMediaNode", json!({ "nodeId": node_id, "x": 120.0, "y": 160.0 }));
        assert_eq!(emit.coalesce_key.as_deref(), Some(format!("moveMediaNode:{node_id}").as_str()));
        let node = apply_operations(&projection, &emit.operations)
            .workflow
            .nodes
            .into_iter()
            .find(|row| row.id == node_id)
            .expect("node");
        assert!((node.x - 120.0).abs() < 0.01);
        assert!((node.y - 160.0).abs() < 0.01);
    }

    //#region 🔖️MediaContractConnect
    #[test]
    fn connect_media_ports_rejects_incompatible_types_via_notice() {
        register_artifact_descriptor(&ArtifactKindSpec {
            id: "test.contract.2d".into(),
            name: "Test 2D".into(),
            source_format: "test.2d".into(),
            component_kind: "test".into(),
            dimension: "2d".into(),
            media_capability: semio_framework_os::OsMediaCapability::MeshOnly,
            media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
            schema: "test.contract.2d.schema".into(),
            export_formats: vec![OsMediaFormat::Svg],
            import_formats: vec![OsMediaFormat::Svg],
        });
        register_artifact_descriptor(&ArtifactKindSpec {
            id: "test.contract.3d".into(),
            name: "Test 3D".into(),
            source_format: "test.3d".into(),
            component_kind: "test".into(),
            dimension: "3d".into(),
            media_capability: semio_framework_os::OsMediaCapability::MeshOnly,
            media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
            schema: "test.contract.3d.schema".into(),
            export_formats: vec![OsMediaFormat::Glb],
            import_formats: vec![OsMediaFormat::Glb],
        });
        let mut projection = demo_space_projection();
        projection.workflow.nodes.push(OsWorkflowNode {
            id: "contract-src".into(),
            instance_id: "contract-src".into(),
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            inputs: vec![],
            outputs: vec![OsMediaPort { id: "contract-src:out".into(), artifact_kind: "test.contract.2d".into(), direction: "out".into() }],
        });
        projection.workflow.nodes.push(OsWorkflowNode {
            id: "contract-dst".into(),
            instance_id: "contract-dst".into(),
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            inputs: vec![OsMediaPort { id: "contract-dst:in".into(), artifact_kind: "test.contract.3d".into(), direction: "in".into() }],
            outputs: vec![],
        });
        let mut app = SpaceApp::new();
        let emit = studio_emit(
            &mut app,
            &projection,
            "connectMediaPorts",
            json!({ "sourceNodeId": "contract-src", "sourcePortId": "contract-src:out", "targetNodeId": "contract-dst", "targetPortId": "contract-dst:in" }),
        );
        assert!(emit.operations.is_empty(), "an incompatible connect must not push OsOperation::ConnectWorkflowPorts");
        assert!(matches!(emit.effects.first(), Some(HostEffect::Notify { .. })), "an incompatible connect must surface a Notify effect instead");
    }

    #[test]
    fn connect_media_ports_negotiates_a_contract_for_compatible_types() {
        register_artifact_descriptor(&ArtifactKindSpec {
            id: "test.contract.doc-a".into(),
            name: "Test Doc A".into(),
            source_format: "test.doc".into(),
            component_kind: "test".into(),
            dimension: "data".into(),
            media_capability: semio_framework_os::OsMediaCapability::MeshOnly,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            schema: "test.contract.doc.schema".into(),
            export_formats: vec![],
            import_formats: vec![],
        });
        register_artifact_descriptor(&ArtifactKindSpec {
            id: "test.contract.doc-b".into(),
            name: "Test Doc B".into(),
            source_format: "test.doc".into(),
            component_kind: "test".into(),
            dimension: "data".into(),
            media_capability: semio_framework_os::OsMediaCapability::MeshOnly,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            schema: "test.contract.doc.schema".into(),
            export_formats: vec![],
            import_formats: vec![],
        });
        let mut projection = demo_space_projection();
        projection.workflow.nodes.push(OsWorkflowNode {
            id: "contract-src-2".into(),
            instance_id: "contract-src-2".into(),
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            inputs: vec![],
            outputs: vec![OsMediaPort { id: "contract-src-2:out".into(), artifact_kind: "test.contract.doc-a".into(), direction: "out".into() }],
        });
        projection.workflow.nodes.push(OsWorkflowNode {
            id: "contract-dst-2".into(),
            instance_id: "contract-dst-2".into(),
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            inputs: vec![OsMediaPort { id: "contract-dst-2:in".into(), artifact_kind: "test.contract.doc-b".into(), direction: "in".into() }],
            outputs: vec![],
        });
        let mut app = SpaceApp::new();
        let emit = studio_emit(
            &mut app,
            &projection,
            "connectMediaPorts",
            json!({ "sourceNodeId": "contract-src-2", "sourcePortId": "contract-src-2:out", "targetNodeId": "contract-dst-2", "targetPortId": "contract-dst-2:in" }),
        );
        let edge = emit
            .operations
            .iter()
            .find_map(|operation| match operation {
                OsOperation::ConnectWorkflowPorts { edge } if edge.source_node_id == "contract-src-2" => Some(edge.clone()),
                _ => None,
            })
            .expect("a compatible connect must push OsOperation::ConnectWorkflowPorts with a negotiated contract");
        assert_eq!(edge.contract.kind_id, "test.contract.doc-b");
        assert_eq!(edge.contract.wire, MediaWireFormat::Document { schema: "test.contract.doc.schema".into() });
        assert!(edge.contract.conversion.is_none());
        let next = apply_operations(&projection, &emit.operations);
        assert!(validate_workflow(&next.workflow).ok, "a freshly negotiated edge must pass validate_workflow's contract-consistency check");
    }
    //#endregion 🔖️MediaContractConnect

    #[test]
    fn spawns_draw_app_instance() {
        seed_draw_plugin();
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let emit = studio_emit(&mut app, &projection, "spawnApp", json!({ "pluginId": "draw", "appId": "draw" }));
        assert!(!emit.operations.is_empty());
        let next = apply_operations(&projection, &emit.operations);
        assert_eq!(next.app_instances.len(), projection.app_instances.len() + 1);
        assert_eq!(app.runtime.active_instance_id, next.app_instances.last().map(|i| i.id.clone()));
    }

    #[test]
    fn spawns_draw_app_instance_at_drop_position() {
        seed_draw_plugin();
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let existing: HashSet<String> = projection.app_instances.iter().map(|i| i.id.clone()).collect();
        let emit = studio_emit(
            &mut app,
            &projection,
            "spawnApp",
            json!({ "pluginId": "draw", "appId": "draw", "position": { "x": 321.0, "y": 654.0 } }),
        );
        let next = apply_operations(&projection, &emit.operations);
        let instance = next
            .app_instances
            .iter()
            .find(|i| i.plugin_id == "draw" && !existing.contains(&i.id))
            .expect("newly spawned draw instance");
        let node = next
            .workflow
            .nodes
            .iter()
            .find(|n| n.instance_id == instance.id)
            .expect("media node for spawned instance");
        assert!((node.x - 321.0).abs() < 0.01);
        assert!((node.y - 654.0).abs() < 0.01);
    }

    #[test]
    fn open_instance_emits_open_plugin_instance_effect_matching_instance() {
        seed_draw_plugin();
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let instance = projection.app_instances.iter().find(|i| i.plugin_id == "draw").expect("draw instance").clone();
        let emit = studio_emit(&mut app, &projection, "openInstance", json!({ "instanceId": instance.id }));
        assert!(emit.operations.is_empty(), "opening an instance is a host effect, not a document operation");
        let opened = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::OpenPluginInstance { plugin_id, app_id, os_instance_id } => {
                    Some((plugin_id.clone(), app_id.clone(), os_instance_id.clone()))
                }
                _ => None,
            })
            .expect("OpenPluginInstance effect");
        assert_eq!(opened.0, "draw");
        assert_eq!(opened.1, "draw");
        assert_eq!(opened.2.as_deref(), Some(instance.id.as_str()));
    }

    #[test]
    fn export_media_emits_download_effect_and_import_requests_file_open() {
        use base64::Engine;
        seed_draw_plugin();
        semio_framework_os::register_os_media_export_handler("2d.drawing", OsMediaFormat::Dwg, |_doc| {
            let drawing = semio_framework_os::DwgDrawing::default();
            let bytes = semio_framework_os::dwg_to_bytes(&drawing).map_err(|error| error)?;
            Ok(semio_framework_os::OsMediaExportResult {
                data: base64::engine::general_purpose::STANDARD.encode(bytes),
                mime_type: OsMediaFormat::Dwg.mime_type().into(),
                file_name: "draw.dwg".into(),
                encoding: Some("base64".into()),
            })
        });
        semio_framework_os::register_dwg_import_handler("2d.drawing", |_drawing| Ok(json!({ "schema": "draw.document", "imported": true })));

        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let instance = projection.app_instances.iter().find(|i| i.plugin_id == "draw").expect("draw instance").clone();

        let export = studio_emit(&mut app, &projection, "exportMedia", json!({ "instanceId": instance.id, "format": "dwg" }));
        let (data, encoding) = export
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::DownloadMediaExport { data, encoding, .. } => Some((data.clone(), encoding.clone())),
                _ => None,
            })
            .expect("DownloadMediaExport effect");
        assert!(!data.is_empty());
        assert_eq!(encoding.as_deref(), Some("base64"));

        let import = studio_emit(&mut app, &projection, "importMedia", json!({ "instanceId": instance.id, "format": "dwg" }));
        assert!(import.effects.iter().any(|effect| matches!(
            effect,
            HostEffect::RequestFileOpen { import_action, .. } if import_action == "importMediaPayload"
        )));

        // Decoding is exercised here; the decoded content is applied to the instance's own
        // `OsDocumentRef` document by the host, so this arm emits no studio operation.
        let payload = studio_emit(&mut app, &projection, "importMediaPayload", json!({ "payload": format!("data:image/vnd.dwg;base64,{data}") }));
        assert!(payload.operations.is_empty());
    }

    #[test]
    fn commit_checkpoint_round_trips_projection() {
        let mut app = VcsDocumentApp::new(SpaceApp::new());
        let before = app.projection().expect("projection").app_instances.len();
        app.handle_action("commitCheckpoint", Some(&json!({ "message": "snapshot" })), &ViewState::default(), &testkit::meta("local"))
            .expect("commit");
        assert_eq!(app.projection().expect("projection").app_instances.len(), before);
    }

    #[test]
    fn patch_parameter_action_updates_value() {
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let emit = studio_emit(
            &mut app,
            &projection,
            "patchParameter",
            json!({ "parameterId": "param-brush-size", "field": "value", "value": 48.0 }),
        );
        assert_eq!(emit.operations.len(), 1);
        let next = apply_operations(&projection, &emit.operations);
        match next.parameters.iter().find(|entry| entry.id() == "param-brush-size").expect("parameter") {
            OsParameter::Numeric { value, .. } => assert_eq!(*value, 48.0),
            _ => panic!("expected numeric"),
        }
    }

    /// 🧪️ Undo/redo round trip on a real operation, driven through the shared testkit harness
    /// instead of a hand-rolled `meta()`/repeated assert body.
    #[test]
    fn undo_redo_round_trip_on_spawn() {
        seed_draw_plugin();
        let mut app = VcsDocumentApp::new(SpaceApp::new());
        let before = app.projection().expect("projection").app_instances.len();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            "spawnApp",
            Some(&json!({ "pluginId": "draw", "appId": "draw" })),
            |app| app.projection().expect("projection").app_instances.len(),
            before,
            before + 1,
        );
    }

    #[test]
    fn catalogue_tree_nests_apps_by_canonical_document() {
        let panel = SpacePanelState {
            workflows: vec![
                SpaceProgramEntry {
                    plugin_id: "puzzle".into(),
                    workflow_step_id: "puzzle2d".into(),
                    app_id: "puzzle2d-play".into(),
                    label: "Puzzle 2D".into(),
                    document: vec!["semio".into(), "puzzle".into(), "2d".into()],
                    yields: "layout".into(),
                },
                SpaceProgramEntry {
                    plugin_id: "puzzle".into(),
                    workflow_step_id: "puzzle3d".into(),
                    app_id: "puzzle3d-play".into(),
                    label: "Puzzle 3D".into(),
                    document: vec!["semio".into(), "puzzle".into(), "3d".into()],
                    yields: "model".into(),
                },
            ],
            ..Default::default()
        };
        let tree = build_catalogue_tree(&panel, resolve_labels::<SStudioLabels>(&ViewState::default()));
        let json = serde_json::to_string(&tree).unwrap();
        assert!(json.contains("s-play-catalogue.document.semio.puzzle.2d"));
        assert!(json.contains("s-play-catalogue.document.semio.puzzle.3d"));
        assert_eq!(json.matches("\"label\":\"puzzle\"").count(), 1);
    }

    #[test]
    fn patch_app_instances_updates_labels() {
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let ids: Vec<String> = projection.app_instances.iter().take(2).map(|i| i.id.clone()).collect();
        let emit = studio_emit(
            &mut app,
            &projection,
            "patchAppInstances",
            json!({ "instanceIds": ids, "field": "label", "value": "Batch Label" }),
        );
        let next = apply_operations(&projection, &emit.operations);
        let labels: Vec<String> = next
            .app_instances
            .iter()
            .filter(|i| ids.contains(&i.id))
            .map(|i| i.label.clone())
            .collect();
        assert!(labels.iter().all(|label| label == "Batch Label"));
    }

    #[test]
    fn open_and_close_focused_instance() {
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let instance_id = projection.app_instances.first().expect("instance").id.clone();
        assert!(app.runtime.focused_instance_id.is_none());
        studio_emit(&mut app, &projection, "openInstance", json!({ "instanceId": instance_id }));
        assert_eq!(app.runtime.focused_instance_id.as_deref(), Some(instance_id.as_str()));
        let emit = studio_emit(&mut app, &projection, "closeFocusedInstance", json!({}));
        assert!(app.runtime.focused_instance_id.is_none());
        assert!(emit.effects.iter().any(|effect| matches!(effect, HostEffect::SetPanel { .. })));
    }

    #[test]
    fn inspector_tree_exposes_label_field() {
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let ids: Vec<String> = projection.app_instances.iter().take(2).map(|i| i.id.clone()).collect();
        app.runtime.selected_app_instance_ids = ids;
        let history = empty_history();
        let doc = DocumentView { projection: &projection, history: &history };
        let tree = app.render(S_PLAY_INSPECTOR_BODY_KEY, &doc, &ViewState::default());
        let UiNode::Tree(tree_node) = tree else {
            panic!("expected tree");
        };
        let section = tree_node
            .sections
            .iter()
            .find(|section| section.id == "s-play-inspector.app-instances")
            .expect("instances section");
        let label_field = section
            .items
            .iter()
            .find(|item| item.id == "s-play-inspector.app-instance.label")
            .expect("label field");
        let control = label_field.control.as_ref().expect("label control");
        let UiControlNode::Input(input) = control else {
            panic!("expected input control");
        };
        assert_eq!(input.on_change.action, "patchAppInstances");
    }

    fn seed_multi_port_plugins() {
        let mut puzzle_resources = HashMap::new();
        puzzle_resources.insert(
            "puzzle5d".into(),
            OsAppResourceSpec {
                inputs: vec![os_in_port("topology", "in-a", "In A", false)],
                outputs: vec![
                    os_out_port("topology", "out-a", "Out A"),
                    os_out_port("topology", "out-b", "Out B"),
                ],
                source_format: "puzzle5d.document".into(),
                component_kind: "world-3d".into(),
                modes: vec![ModeDefinition {
                    id: "edit".into(),
                    label: "Edit".into(),
                icon_id: "square-pen".into(),
                    tools: vec![],
                    layout_id: None,
                    commands: vec![],
                }],
                default_mode_id: None,
                parameter_fields: Vec::new(),
                config: semio_framework_core::ConfigSpec::empty(),
            },
        );
        merge_os_plugin_definition(
            "puzzle.5d",
            &OsPlatformInput {
                id: "puzzle.5d".into(),
                name: "Puzzle 5D".into(),
                api_version: "1".into(),
                apps: vec![OsPlatformAppInput {
                    id: "puzzle5d".into(),
                    label: "Puzzle 5D".into(),
                    document: vec!["semio".into(), "puzzle".into(), "5d".into()],
                    controller_id: "puzzle5d-play".into(),
                    modes: vec![ModeDefinition {
                        id: "edit".into(),
                        label: "Edit".into(),
                icon_id: "square-pen".into(),
                        tools: vec![],
                        layout_id: None,
                        commands: vec![],
                    }],
                    default_mode_id: None,
                }],
            },
            &puzzle_resources,
        )
        .expect("merge puzzle5d");

        let mut shooting_resources = HashMap::new();
        shooting_resources.insert(
            "shooting".into(),
            OsAppResourceSpec {
                inputs: vec![os_in_port("2d.shooting", "scene-in", "Scene", true)],
                outputs: vec![os_out_port("2d.shooting", "scene-out", "Scene")],
                source_format: "shooting.document".into(),
                component_kind: "world-3d".into(),
                modes: vec![ModeDefinition {
                    id: "edit".into(),
                    label: "Edit".into(),
                icon_id: "square-pen".into(),
                    tools: vec![],
                    layout_id: None,
                    commands: vec![],
                }],
                default_mode_id: None,
                parameter_fields: Vec::new(),
                config: semio_framework_core::ConfigSpec::empty(),
            },
        );
        merge_os_plugin_definition(
            "shooting",
            &OsPlatformInput {
                id: "shooting".into(),
                name: "Shooting".into(),
                api_version: "1".into(),
                apps: vec![OsPlatformAppInput {
                    id: "shooting".into(),
                    label: "Shooting".into(),
                    document: vec!["semio".into(), "shooting".into()],
                    controller_id: "shooting-play".into(),
                    modes: vec![ModeDefinition {
                        id: "edit".into(),
                        label: "Edit".into(),
                icon_id: "square-pen".into(),
                        tools: vec![],
                        layout_id: None,
                        commands: vec![],
                    }],
                    default_mode_id: None,
                }],
            },
            &shooting_resources,
        )
        .expect("merge shooting");
    }

    #[test]
    fn spawns_puzzle5d_and_shooting_with_multi_port_registrations() {
        seed_multi_port_plugins();
        let mut app = SpaceApp::new();
        let mut projection = demo_space_projection();
        let emit = studio_emit(
            &mut app,
            &projection,
            "spawnApp",
            json!({ "pluginId": "puzzle.5d", "appId": "puzzle5d", "position": { "x": 200, "y": 100 } }),
        );
        projection = apply_operations(&projection, &emit.operations);
        let emit = studio_emit(
            &mut app,
            &projection,
            "spawnApp",
            json!({ "pluginId": "shooting", "appId": "shooting", "position": { "x": 300, "y": 100 } }),
        );
        projection = apply_operations(&projection, &emit.operations);
        let puzzle_instance = projection.app_instances.iter().rev().nth(1).expect("puzzle");
        let shooting_instance = projection.app_instances.last().expect("shooting");
        let puzzle_node = projection
            .workflow
            .nodes
            .iter()
            .find(|node| node.instance_id == puzzle_instance.id)
            .expect("puzzle node");
        let shooting_node = projection
            .workflow
            .nodes
            .iter()
            .find(|node| node.instance_id == shooting_instance.id)
            .expect("shooting node");
        assert_eq!(puzzle_node.outputs.len(), 2);
        assert_eq!(shooting_node.inputs.len(), 1);
    }

    #[test]
    fn unbind_parameter_field_removes_binding() {
        let mut app = SpaceApp::new();
        let mut projection = demo_space_projection();
        let instance = projection.app_instances.first().expect("instance").clone();
        let parameter_id = parameter_entity_id(projection.parameters.first().expect("parameter")).to_string();
        let emit = studio_emit(
            &mut app,
            &projection,
            "bindParameterField",
            json!({ "instanceId": instance.id, "fieldPath": "label", "parameterId": parameter_id }),
        );
        projection = apply_operations(&projection, &emit.operations);
        assert!(projection
            .parameter_bindings
            .iter()
            .any(|row| row.instance_id == instance.id && row.field_path == "label"));
        let emit = studio_emit(
            &mut app,
            &projection,
            "unbindParameterField",
            json!({ "instanceId": instance.id, "fieldPath": "label" }),
        );
        projection = apply_operations(&projection, &emit.operations);
        assert!(!projection
            .parameter_bindings
            .iter()
            .any(|row| row.instance_id == instance.id && row.field_path == "label"));
    }

    #[test]
    fn checkout_checkpoint_restores_projection() {
        seed_draw_plugin();
        let mut app = VcsDocumentApp::new(SpaceApp::new());
        let before = app.projection().expect("projection").app_instances.len();
        app.handle_action("spawnApp", Some(&json!({ "pluginId": "draw", "appId": "draw" })), &ViewState::default(), &testkit::meta("local"))
            .expect("spawn");
        app.handle_action("commitCheckpoint", Some(&json!({ "message": "after-first-spawn" })), &ViewState::default(), &testkit::meta("local"))
            .expect("commit");
        let after_first = app.projection().expect("projection").app_instances.len();
        assert!(after_first > before);
        let files = app.document_pack().expect("document pack");
        let parsed: store::ParsedDocumentText<OsProjection, OsOperation> = store::parse_document_pack(&files.pack, &files.spr).expect("parse document pack");
        let checkpoint_id = parsed.envelope.vcs.checkpoints[0].id.clone();
        app.handle_action("spawnApp", Some(&json!({ "pluginId": "draw", "appId": "draw" })), &ViewState::default(), &testkit::meta("local"))
            .expect("spawn2");
        assert!(app.projection().expect("projection").app_instances.len() > after_first);
        app.handle_action("checkoutCheckpoint", Some(&json!({ "checkpointId": checkpoint_id })), &ViewState::default(), &testkit::meta("local"))
            .expect("checkout");
        assert_eq!(app.projection().expect("projection").app_instances.len(), after_first);
    }

    /// 🧪️ The definitional proof: two independent instances start from the same deterministic demo
    /// projection, apply DISJOINT edits (A spawns a new draw instance, B renames an existing
    /// instance), and exchanging operations over a backbone converges both sides onto the same
    /// projection — impossible under whole-document `setDocument` snapshots, where one side's write
    /// would clobber the other's. Driven through the shared testkit convergence harness instead of
    /// a hand-rolled `MemoryBackbone::pair` + manual drain/ingest.
    #[test]
    fn two_instances_converge_on_disjoint_edits_via_backbone() {
        seed_draw_plugin();
        let instance_id = demo_space_projection().app_instances.first().expect("instance").id.clone();
        let rename_args = json!({ "instanceIds": [instance_id.clone()], "field": "label", "value": "Renamed" });
        testkit::assert_two_instances_converge::<SpaceApp, (usize, bool)>(
            "mem://s-studio-convergence",
            ("spawnApp", Some(&json!({ "pluginId": "draw", "appId": "draw" }))),
            ("patchAppInstances", Some(&rename_args)),
            move |app| {
                let projection = app.projection().expect("projection");
                let draw_count = projection.app_instances.iter().filter(|i| i.plugin_id == "draw").count();
                let renamed = projection
                    .app_instances
                    .iter()
                    .find(|i| i.id == instance_id)
                    .map(|i| i.label == "Renamed")
                    .unwrap_or(false);
                (draw_count, renamed)
            },
        );
    }

    #[test]
    fn space_declares_expected_actions_and_examples() {
        let studio = create_space_app();
        assert!(studio.definition.actions.iter().any(|action| action.id == "spawnApp"));
        assert!(studio.definition.actions.iter().any(|action| action.id == "reorganizeWorkflow"));
        assert_eq!(studio.examples.len(), S_STUDIO_EXAMPLES.len());
    }

    #[test]
    fn workflow_scene_uses_flow_engine_with_fixture() {
        let mut app = VcsDocumentApp::new(SpaceApp::new());
        let node = app.render(S_PLAY_BODY_WORKFLOW, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#"\"engine\":\"flow\""#));
        assert!(json.contains("fixtureJson"));
        assert!(json.contains(r#"\"schema\":\"flow.fixture\""#));
    }

    #[test]
    fn node_graph_edit_set_fixture_moves_node_and_persists_camera() {
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let node = projection.workflow.nodes.first().expect("node").clone();
        let camera = OsWorkflowCamera { x: 40.0, y: -20.0, zoom: 2.0 };
        let mut fixture = os_workflow_to_flow_fixture(&projection.workflow, &projection.app_instances, &camera);
        fixture["layout"][&node.id] = json!({ "x": 500.0 + node.width / 2.0, "y": 300.0 + node.height / 2.0 });
        let emit = studio_emit(
            &mut app,
            &projection,
            "nodeGraphEdit",
            json!({ "operations": [{ "operation": "setFixture", "fixtureJson": fixture.to_string() }] }),
        );
        let moved = apply_operations(&projection, &emit.operations)
            .workflow
            .nodes
            .into_iter()
            .find(|row| row.id == node.id)
            .expect("node");
        assert!((moved.x - 500.0).abs() < 0.01);
        assert!((moved.y - 300.0).abs() < 0.01);
        assert_eq!(app.runtime.workflow_camera, Some(camera));
    }

    #[test]
    fn node_graph_viewport_persists_camera() {
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        studio_emit(&mut app, &projection, "nodeGraphViewport", json!({ "viewportJson": r#"{"x":7.0,"y":9.0,"zoom":0.5}"# }));
        assert_eq!(app.runtime.workflow_camera, Some(OsWorkflowCamera { x: 7.0, y: 9.0, zoom: 0.5 }));
    }

    #[test]
    fn presence_heartbeat_publishes_peer_for_other_clients() {
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let first_node_id = projection.workflow.nodes[0].id.clone();
        studio_emit(&mut app, &projection, "nodeGraphSelect", json!({ "nodeIds": [first_node_id] }));
        studio_emit(&mut app, &projection, "presenceHeartbeat", json!({ "clientId": "client-test-a", "name": "Ada" }));
        let other_runtime = StudioRuntimeState {
            client_id: Some("client-test-b".into()),
            space_id: app.runtime.space_id.clone(),
            ..StudioRuntimeState::default()
        };
        let peers = presence_peers_json(&other_runtime);
        assert!(peers.contains("client-test-a"));
        assert!(peers.contains("Ada"));
        assert!(peers.contains(r#""selectionCount":1"#));
        let self_view = presence_peers_json(&app.runtime);
        assert!(!self_view.contains("client-test-a"));
    }

    /// 🐢️ Perf round 3: a heartbeat only records this client's own identity for the presence broadcast
    /// — it must declare `None` so it never triggers a full-shell `refresh-ui` for the sending client.
    #[test]
    fn presence_heartbeat_declares_none_ui_scope() {
        use semio_framework_core::kernel::UiDirtyScope;
        let mut app = SpaceApp::new();
        let projection = demo_space_projection();
        let emit = studio_emit(&mut app, &projection, "presenceHeartbeat", json!({ "clientId": "client-test-c", "name": "Cass" }));
        assert!(matches!(emit.ui_scope, UiDirtyScope::None), "presenceHeartbeat must declare None, got {:?}", emit.ui_scope);
    }

    #[test]
    fn space_labels_resolve_native_english_by_default() {
        let history = empty_history();
        let app = SpaceApp::new();
        let projection = demo_space_projection();
        let doc = DocumentView { projection: &projection, history: &history };
        let catalogue_json = serde_json::to_string(&app.render(S_PLAY_CATALOGUE_BODY_KEY, &doc, &ViewState::default())).unwrap();
        assert!(catalogue_json.contains("\"Apps\""));

        let parameters_json = serde_json::to_string(&app.render(S_PLAY_PARAMETERS_BODY_KEY, &doc, &ViewState::default())).unwrap();
        assert!(parameters_json.contains("Add Parameter"));
        assert!(parameters_json.contains("\"Name\""));
        assert!(parameters_json.contains("\"Remove\""));
        assert!(!parameters_json.contains("Parameter hinzufügen"));
    }

    #[test]
    fn space_labels_resolve_native_german_locale() {
        let history = empty_history();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let app = SpaceApp::new();
        let projection = demo_space_projection();
        let doc = DocumentView { projection: &projection, history: &history };
        let parameters_json = serde_json::to_string(&app.render(S_PLAY_PARAMETERS_BODY_KEY, &doc, &view_state)).unwrap();
        assert!(parameters_json.contains("Parameter hinzufügen"));
        assert!(parameters_json.contains("\"Entfernen\""));
        assert!(!parameters_json.contains("Add Parameter"));

        let inspector_json = serde_json::to_string(&app.render(S_PLAY_INSPECTOR_BODY_KEY, &doc, &view_state)).unwrap();
        assert!(inspector_json.contains("Wähle Workflow-Knoten oder App-Instanzen im Arbeitsbereich aus."));
    }

    /// 🌉️ Moved from `home_ui`'s own test module: exercises BOTH apps together (Home's `createStudio`
    /// followed by Space's `openSpace`) — this crate already regular-depends on `home_ui`, so the
    /// integration test lives beside the app that owns the dependency edge instead of requiring a new
    /// dev-dependency cycle back from `home_ui` onto this crate.
    #[test]
    fn create_space_navigates_without_download_and_opens_empty() {
        let mut home = home_ui::HomeApp;
        let home_projection = home.initial_projection();
        let history = empty_history();
        let doc = DocumentView { projection: &home_projection, history: &history };
        let emit = home.handle_action(
            "createStudio",
            Some(&json!({ "name": "Fresh Studio" })),
            &doc,
            &ViewState::default(),
        );
        assert!(
            !emit.effects.iter().any(|effect| matches!(effect, HostEffect::DownloadMediaExport { .. })),
            "create must not download a file"
        );
        let uri = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::Navigate { uri } => Some(uri.as_str()),
                _ => None,
            })
            .expect("navigate");
        assert!(uri.starts_with("/spaces/"), "uri={uri}");
        assert!(!uri.ends_with("/demo") && !uri.ends_with("/default"), "uri={uri}");
        let space_id = uri.trim_start_matches("/spaces/");
        let document = home_ui::resolve_studio_document(space_id).expect("created studio");
        assert_eq!(document.name, "Fresh Studio");
        assert!(document.backbone.is_none(), "ephemeral studio must not attach backbone");
        assert!(document.vcs.initial_projection.app_instances.is_empty());

        let mut studio = SpaceApp::new();
        let empty = default_os_projection();
        let studio_doc = DocumentView { projection: &empty, history: &history };
        let open = studio.handle_action(
            "openSpace",
            Some(&json!({ "spaceId": space_id })),
            &studio_doc,
            &ViewState::default(),
        );
        assert!(
            open.effects
                .iter()
                .any(|effect| matches!(effect, HostEffect::LoadDocument { .. })),
            "openSpace must load the created studio"
        );
        assert!(!open.effects.iter().any(|effect| matches!(effect, HostEffect::Navigate { .. })));
        assert!(!open.effects.iter().any(|effect| matches!(effect, HostEffect::DownloadMediaExport { .. })));
    }
}
//#endregion 🧪️Tests
