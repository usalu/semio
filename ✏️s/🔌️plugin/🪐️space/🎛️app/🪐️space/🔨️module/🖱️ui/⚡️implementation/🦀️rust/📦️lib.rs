//! 🎛️ S Studio app — `DocumentApp` impl, render, manifest (constitutional: ui). B1: the pure-trait
//! cutover — `SpaceApp` is a unit struct; every former `StudioRuntimeState` field (selection, hover,
//! camera, clipboard, presence identity, …) plus the deleted `ViewState.panel_json`-backed
//! `SpacePanelState.active_panel_tab` now live in `space_engine::SpaceConfig`, written via
//! `space_op::SpaceConfigOperation`s (real `backwards`, no ad hoc `InverseAction`); every action
//! dispatches through the single typed `space_protocol::SpaceCommand` channel via
//! `DocumentApp::handle`. A `WorkflowNode` IS the app instance now (see the kernel `🔁️workflow` crate's
//! `🔖️InstanceIdentity` doc) — every former `instance_id`/`OsAppInstance` join collapses onto
//! `projection.workflow.nodes` directly.
//!
//! 🕳️ Deviation from the usual "ui" content: this app's `DocumentApp::Projection`/`Operation` are
//! `semio_framework_os::{OsProjection, OsOperation}` — see `space_op`'s doc comment. This crate also
//! regular-depends on `home_ui` (`semio-s-app-space-home-ui`): the Studio app resolves/loads studio
//! documents through the Home launcher's own catalog port (`openSpace`, `exportStudioPack`,
//! `exportStudioDsl`, `importSpacePackPayload`) — a real, non-test dependency, not just a test fixture.

use space::{
    S_PLAY_APP_ID, S_PLAY_BODY_COMPILED_DAG, S_PLAY_BODY_MEDIA_VFS, S_PLAY_BODY_WORKFLOW,
    S_PLAY_CATALOGUE_BODY_KEY, S_PLAY_CATALOGUE_DRAG_MIME, S_PLAY_CATALOGUE_TAB_ID, S_PLAY_CONTROLLER_ID,
    S_PLAY_INSPECTOR_BODY_KEY, S_PLAY_INSPECTOR_TAB_ID, S_PLAY_PARAMETERS_BODY_KEY, S_PLAY_PARAMETERS_TAB_ID,
    S_PLAY_SURFACE_COMPILED_DAG, S_PLAY_SURFACE_MEDIA_VFS, S_PLAY_SURFACE_WORKFLOW, S_PLAY_WINDOW_COMPILED_DAG,
    S_PLAY_WINDOW_MEDIA_VFS, S_PLAY_WINDOW_WORKFLOW, S_STUDIO_EXAMPLES,
};
use space_engine::{
    add_parameter_operation, add_workflow_node_operation, compiled_dag_wire_literal, flatten_media_vfs_rows,
    negotiate_media_connect, parameter_entity_id, patch_parameter_operation,
    OsParameterId, SpaceConfig,
};
use space_op::SpaceConfigOperation;
use space_protocol::SpaceCommand;
use space_shared::{demo_space_projection, ensure_space_fixtures_registered, parse_demo_space_document};
use semio_framework_os::{
    apply_flow_fixture_to_os_workflow, build_os_workflow_operator_infos, create_empty_os_document,
    create_os_id, default_os_projection, materialize_os_app_instance_document_json, materialize_os_projection,
    os_app_primary_output_kind, os_app_registration, os_document_to_json, os_workflow_to_flow_fixture,
    os_workflow_to_node_graph_payload, os_workflow_vfs_schema, os_parameter_types_compatible, os_parameter_value,
    workflow_palette, MediaContract, OsOperation, OsParameter, OsParameterFieldBinding, OsParameterType,
    OsProjection, OsWorkflowCamera, WorkflowEdge, WorkflowNode, OS_WORKFLOW_VFS_ROOT_ID, OS_SPACE_SCHEMA,
};
use semio_framework_os::host::{export_os_space_dsl, export_os_space_pack, import_os_space_from_pack};
use semio_framework_plugin::{
    app_labels, build_node_graph_scene, build_text_editor_scene, build_virtual_file_system_scene,
    create_default_layout, host_now_ms, localized_label_map, tree_item_desc,
    ui_declarative_sections_to_tree, ui_inspector_all_equal, ui_text, IconName, MeasureSelectItem, WindowEngagementStatus,
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App,
    AppLabelsOverlay, AppLabelsOverlayExt, ConfigView, DocumentApp, DocumentView, Emit, HostEffect, LocaleLabels,
    NodeGraphScene, NodeGraphNodeRecord, NodeGraphEdgeRecord, NodeGraphFindItem, NodeGraphHover, NodeGraphOperatorRecord, NodeGraphViewport, PanelGroup,
    PanelTreeBuilder, SurfaceKind, TextEditorScene, UiButtonNode, UiFieldNode, UiInputNode, UiNode, UiPresence,
    UiNumberStepperNode, UiSectionNode, UiSelectItem, UiSelectNode, UiToggleNode, UiTreeItemNode,
    VirtualFileSystemScene, WindowEngagement, WindowEngagementInput, WindowEngagementSlot, WindowLayout,
    WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
};
use semio_framework_plugin::optional_json_to_dsl;
use protocol::Operation;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::sync::{LazyLock, Mutex};

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — see `shooting_ui`'s identical pair.
fn is_de_locale(cfg: &SpaceConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &SpaceConfig) -> &'static L {
    if is_de_locale(cfg) { L::locale_labels_de() } else { L::locale_labels_en() }
}
//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn s_play_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: S_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args: optional_json_to_dsl(args),
    }
}

/// 🖱️ On-demand space workflow context menu from hit-test and selection snapshot.
fn space_workflow_context_menu_items(
    registry: &semio_framework_plugin::AppActionRegistry,
    labels: &SStudioLabels,
    is_de: bool,
    surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>,
    selected_node_ids: &[String],
) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, selection_domains_from_surface, ContextMenuItemSpec, Menu};

    let hits = surface.map(|target| target.hits.as_slice()).unwrap_or(&[]);
    let (nodes, _) = selection_domains_from_surface(surface, selected_node_ids, &[]);
    let hit_node = hits.iter().find(|hit| hit.domain == "node").map(|hit| hit.id.as_str());
    let mut menu = Menu::of(registry);
    if hits.is_empty() {
        menu = menu
            .item(ContextMenuItemSpec {
                id: "paste-instance".into(),
                label: Some(labels.context_paste.into()),
                icon: Some("clipboard".into()),
                action: Some("pasteAppInstance".into()),
                ..Default::default()
            })
            .item(ContextMenuItemSpec {
                id: "select-all".into(),
                label: Some(labels.context_select_all.into()),
                icon: Some("maximize-2".into()),
                action: Some("setMediaNodeSelection".into()),
                args: optional_json_to_dsl(Some(json!({ "selectAll": true }))),
                ..Default::default()
            })
            .item(ContextMenuItemSpec {
                id: "reorganize".into(),
                label: Some(labels.context_reorganize.into()),
                icon: Some("layout-grid".into()),
                action: Some("reorganizeWorkflow".into()),
                ..Default::default()
            });
    }
    if hit_node.is_some() || !nodes.is_empty() {
        menu = menu
            .item(ContextMenuItemSpec {
                id: "open-instance".into(),
                label: Some(labels.context_open_instance.into()),
                icon: Some("external-link".into()),
                action: Some("openInstance".into()),
                ..Default::default()
            })
            .item(ContextMenuItemSpec {
                id: "duplicate-instance".into(),
                label: Some(labels.context_duplicate.into()),
                icon: Some("copy".into()),
                action: Some("duplicateAppInstance".into()),
                ..Default::default()
            })
            .item(ContextMenuItemSpec {
                id: "copy-instance".into(),
                label: Some(labels.context_copy.into()),
                icon: Some("clipboard-copy".into()),
                action: Some("copyAppInstance".into()),
                ..Default::default()
            })
            .item(ContextMenuItemSpec {
                id: "rename-instance".into(),
                label: Some(labels.context_rename_label.into()),
                icon: Some("edit-3".into()),
                action: Some("renameAppInstance".into()),
                ..Default::default()
            });
        let phrase = selection_count_phrase(
            is_de,
            &[(nodes.len().max(if hit_node.is_some() && nodes.is_empty() { 1 } else { 0 }), if is_de { "Knoten" } else { "node" }, if is_de { "Knoten" } else { "nodes" })],
        );
        let remove_label = if phrase.is_empty() {
            labels.context_remove.to_string()
        } else {
            format!("{} ({phrase})", labels.context_remove)
        };
        menu = menu.separator().item(ContextMenuItemSpec {
            id: "remove-instance".into(),
            label: Some(remove_label),
            icon: Some("trash".into()),
            action: Some("removeAppInstance".into()),
            destructive: Some(true),
            ..Default::default()
        });
        if !nodes.is_empty() {
            menu = menu.separator().item(ContextMenuItemSpec {
                id: "clear-selection".into(),
                label: Some(labels.context_clear_selection.into()),
                icon: Some("square-dashed".into()),
                action: Some("setMediaNodeSelection".into()),
                args: optional_json_to_dsl(Some(json!({ "nodeIds": [] }))),
                ..Default::default()
            });
        }
    }
    menu.build()
}

// 🫀️ The shared `presence:` backbone-URI hack (`read_os_presence_peers`/`write_os_presence`/
// `OsPresencePeer`) was deleted from os-core — presence now flows through the semio_hub's duplex
// `PresencePeer`/`Presence` frames via `framework/sync`'s `DocumentEvent::Presence` for migrated
// apps. `s` isn't wired onto `DocumentHost` yet (WS-F's last wave), so it keeps this tiny
// self-contained in-memory heartbeat map until then — same upsert/prune/exclude-self semantics as
// before, just owned locally instead of delegated to a shared cross-process mechanism. B1: keyed
// off `SpaceConfig` fields now instead of the deleted `StudioRuntimeState`.
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

fn config_space_id(config: &SpaceConfig) -> String {
    config.space_id.clone().unwrap_or_else(|| "default".into())
}

fn presence_peers_json(config: &SpaceConfig) -> String {
    let space_id = config_space_id(config);
    let self_client_id = config.client_id.clone().unwrap_or_default();
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

fn publish_presence(config: &SpaceConfig) {
    let (Some(client_id), Some(client_name)) = (&config.client_id, &config.client_name) else {
        return;
    };
    let space_id = config_space_id(config);
    let now_ms = host_now_ms();
    if let Ok(mut registry) = PRESENCE_PEERS.lock() {
        let peers = registry.entry(space_id).or_default();
        peers.retain(|_, entry| now_ms - entry.updated_at_ms <= S_PRESENCE_STALE_MS);
        peers.insert(
            client_id.clone(),
            SPresencePeerLocal {
                client_id: client_id.clone(),
                name: client_name.clone(),
                selection: config.selected_node_ids.clone(),
                updated_at_ms: now_ms,
            },
        );
    }
}

/// @emoji 🔎️ First selected node — the fallback target for actions that implicitly operate on "the"
/// current selection (rename/remove/open) when no explicit node id is supplied. Was
/// `primary_selected_instance_id(&StudioRuntimeState, &OsProjection)`; no join needed anymore (node IS
/// the instance).
fn primary_selected_node_id(config: &SpaceConfig) -> Option<String> {
    config.selected_node_ids.first().cloned().or_else(|| config.active_node_id.clone())
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
        select_hint: &'static str = en: "Select workflow nodes in the canvas.", de: "Wähle Workflow-Knoten im Arbeitsbereich aus.";
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
        media_node_count_label: &'static str = en: "node(s)", de: "Knoten";
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
/// `create_space_app`'s static manifest — same rationale as `app_home`'s `s_home_action_labels`. The
/// dead `"setParameter"` action (never dispatched by any real UI call site pre-B1) is dropped along
/// with its `SpaceCommand` variant.
fn s_studio_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        // 🔧️ Document-mutating operations
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
        // 👁️ Ephemeral view state (config-only now)
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
    app: Option<CatalogueAppEntry>,
}

/// 🎨️ One catalogue leaf's presentation — a thin projection of `registry::AppPaletteEntry` (the
/// `workflow_palette()` entry) plus its resolved `document` breadcrumb/`yields`, both sourced from
/// `os_app_registration` (`AppPaletteEntry` itself doesn't carry them). Replaces the pre-merge
/// `SpaceProgramEntry`/`SpacePanelState.workflows` cache — this is built fresh from the registry every
/// render, never cached in config.
struct CatalogueAppEntry {
    plugin_id: String,
    app_id: String,
    label: String,
    yields: String,
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

/// 🎨️ Builds the app catalogue tree straight from the production registry — `workflow_palette()`
/// (every registered `(plugin_id, app_id)`) joined with `os_app_registration` for the document
/// breadcrumb/primary output kind. Replaces the pre-merge `list_os_workflows()`
/// (`BUILTIN_WORKFLOWS`/`EXTENSION_WORKFLOWS`, deleted) + `SpacePanelState.workflows` cache fallback —
/// always live, never stale.
fn build_catalogue_tree(labels: &SStudioLabels) -> UiNode {
    let mut document = AppCatalogueNode::default();
    for entry in workflow_palette() {
        if entry.app_id == S_PLAY_APP_ID {
            continue;
        }
        let registration = os_app_registration(&entry.plugin_id, &entry.app_id);
        let doc_path = registration.as_ref().map(|row| row.document.clone()).unwrap_or_default();
        let yields = registration.as_ref().map(os_app_primary_output_kind).unwrap_or_default();
        let mut node = &mut document;
        for segment in &doc_path {
            node = node.children.entry(segment.clone()).or_default();
        }
        node.app = Some(CatalogueAppEntry { plugin_id: entry.plugin_id, app_id: entry.app_id, label: entry.label, yields });
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

/// 🔎️ Inspector body — a node's position (Transform-ish section) and identity/parameter-binding
/// facets (Properties-ish section), both driven off the SAME `SpaceConfig.selected_node_ids` now (a
/// node's position and its instance identity are the same record — see the module doc comment).
fn build_inspector_tree(projection: &OsProjection, config: &SpaceConfig, term_labels: &SStudioLabels) -> UiNode {
    let selected_node_ids = &config.selected_node_ids;
    let mut children = vec![UiSectionNode {
        id: "s-play-inspector.header".into(),
        label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![ui_text(format!("{} {}", selected_node_ids.len(), term_labels.media_node_count_label))],
        menu: None,
    }];
    let nodes: Vec<&WorkflowNode> = selected_node_ids
        .iter()
        .filter_map(|node_id| projection.workflow.nodes.iter().find(|node| &node.id == node_id))
        .collect();
    if !nodes.is_empty() {
        let xs: Vec<_> = nodes.iter().map(|node| node.x).collect();
        let ys: Vec<_> = nodes.iter().map(|node| node.y).collect();
        let x_uniform = ui_inspector_all_equal(&xs.iter().map(|v| v.to_string()).collect::<Vec<_>>());
        let y_uniform = ui_inspector_all_equal(&ys.iter().map(|v| v.to_string()).collect::<Vec<_>>());
        let mut node_fields = Vec::new();
        if selected_node_ids.len() == 1 {
            node_fields.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "s-play-inspector.media-node.id".into(),
                label: term_labels.node_id.into(),
                child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
                    id: "s-play-inspector.media-node.id.input".into(),
                    input_kind: "text".into(),
                    value: selected_node_ids[0].clone(),
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
                    Some(json!({ "nodeIds": selected_node_ids, "field": "position", "axis": "x" })),
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
                    Some(json!({ "nodeIds": selected_node_ids, "field": "position", "axis": "y" })),
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
            label: Some(if selected_node_ids.len() == 1 {
                term_labels.workflow_node.into()
            } else {
                format!("{} ({})", term_labels.workflow_nodes, selected_node_ids.len())
            }),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: node_fields,
            menu: None,
        });

        let labels: Vec<_> = nodes.iter().map(|node| node.label.clone()).collect();
        let programs: Vec<_> = nodes.iter().map(|node| node.plugin_id.clone()).collect();
        let apps: Vec<_> = nodes.iter().map(|node| node.app_id.clone()).collect();
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
                        Some(json!({ "nodeIds": selected_node_ids, "field": "label" })),
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
        if selected_node_ids.len() == 1 {
            instance_fields.insert(2, ui_text(format!("{}: {}", term_labels.instance_id_prefix, selected_node_ids[0])));
        }
        if selected_node_ids.len() == 1 {
            if let Some(node) = nodes.first() {
                if let Some(registration) = os_app_registration(&node.plugin_id, &node.app_id) {
                    for field_spec in &registration.parameter_fields {
                        let binding = projection.parameter_bindings.iter().find(|entry| {
                            entry.node_id == node.id && entry.field_path == field_spec.field_path
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
                                        "nodeId": node.id,
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
            label: Some(if selected_node_ids.len() == 1 {
                term_labels.app_instance.into()
            } else {
                format!("{} ({})", term_labels.app_instances, selected_node_ids.len())
            }),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: instance_fields,
            menu: None,
        });
    } else {
        children[0].children.push(ui_text(term_labels.select_hint));
    }
    ui_declarative_sections_to_tree(&children)
}
//#endregion 🔖️Panels

// TEMP(Wave 3): space producer flip pending — `os_workflow_to_node_graph_payload`/
// `build_os_workflow_operator_infos` still emit JSON-string payloads (W4 not landed yet), while
// `NodeGraphScene` is now typed (W5). These shims JSON-decode the existing wire shape into the new
// typed records so this one call site compiles without doing the real space/W4 cutover. Delete once
// the space producer is flipped to build the typed records directly.
fn json_array_to_node_graph_nodes(json: &str) -> Vec<NodeGraphNodeRecord> {
    serde_json::from_str(json).unwrap_or_default()
}

fn json_array_to_node_graph_edges(json: &str) -> Vec<NodeGraphEdgeRecord> {
    serde_json::from_str(json).unwrap_or_default()
}

fn json_array_to_node_graph_find_items(json: &str) -> Vec<NodeGraphFindItem> {
    serde_json::from_str(json).unwrap_or_default()
}

fn json_array_to_node_graph_operators<T: Serialize>(operators: &[T]) -> Vec<NodeGraphOperatorRecord> {
    serde_json::to_string(operators).ok().and_then(|json| serde_json::from_str(&json).ok()).unwrap_or_default()
}
// TEMP(Wave 3) end

//#region 🔖️Render
fn workflow_camera(config: &SpaceConfig) -> OsWorkflowCamera {
    config.camera.get(S_PLAY_WINDOW_WORKFLOW).copied().map(Into::into).unwrap_or_default()
}

fn render_workflow(projection: &OsProjection, config: &SpaceConfig, labels: &SStudioLabels) -> UiNode {
    let _ = labels;
    let graph_payload = os_workflow_to_node_graph_payload(&projection.workflow);
    let camera = workflow_camera(config);
    let fixture = os_workflow_to_flow_fixture(&projection.workflow, &camera);
    let operators = build_os_workflow_operator_infos(&projection.workflow, &projection.parameters);
    let selection = config.selected_node_ids.clone();
    let hover = config.hovered_node_id.as_ref().map(|id| NodeGraphHover { node_id: Some(id.clone()) });
    build_node_graph_scene(
        S_PLAY_SURFACE_WORKFLOW,
        S_PLAY_CONTROLLER_ID,
        NodeGraphScene {
            editable: Some(true),
            operators: json_array_to_node_graph_operators(&operators),
            find_items: json_array_to_node_graph_find_items(&graph_payload.find_items_json),
            selection,
            hover,
            capabilities_json: Some(r#"{"engine":"flow","spotlight":false,"noteEdit":false,"clusters":false}"#.into()),
            fixture_json: Some(fixture.to_string()),
            presence_peers_json: Some(presence_peers_json(config)),
            ..NodeGraphScene::base(
                json_array_to_node_graph_nodes(&graph_payload.nodes_json),
                json_array_to_node_graph_edges(&graph_payload.edges_json),
                NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom },
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
/// 🧪️ B1: unit struct — every former `StudioRuntimeState`/`self.config` field now lives in
/// `space_engine::SpaceConfig` (see `DocumentApp::Config`), written through
/// `space_op::SpaceConfigOperation`s.
#[derive(Default)]
pub struct SpaceApp;

/// @emoji 🤝️ Resolves the source/target ports for a proposed connect and negotiates their wire
/// contract via `space_engine::negotiate_media_connect`, converting a rejection into a `Notify`
/// effect — shared by `"connectMediaPorts"` and the `nodeGraphEdit`/`"connect"` fixture edit.
fn negotiate_connect_or_notify(projection: &OsProjection, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<MediaContract, HostEffect> {
    negotiate_media_connect(projection, source_node_id, source_port_id, target_node_id, target_port_id).map_err(|reason| HostEffect::Notify { message: reason })
}

fn connect_edge_operation(source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str, contract: MediaContract) -> OsOperation {
    OsOperation::ConnectWorkflowPorts {
        edge: WorkflowEdge {
            id: create_os_id("edge"),
            source_node_id: source_node_id.into(),
            source_port_id: source_port_id.into(),
            target_node_id: target_node_id.into(),
            target_port_id: target_port_id.into(),
            contract,
        },
    }
}

impl DocumentApp for SpaceApp {
    type Projection = OsProjection;
    type Operation = OsOperation;
    type Config = SpaceConfig;
    type ConfigOperation = SpaceConfigOperation;
    type Command = SpaceCommand;

    fn app_id(&self) -> &str {
        S_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        OS_SPACE_SCHEMA
    }

    fn initial_projection(&self) -> OsProjection {
        default_os_projection()
    }

    /// 🏷️ Maps each `SpaceCommand` variant back to the action id it was declared under in
    /// `create_space_app`.
    fn command_id(&self, command: &SpaceCommand) -> &str {
        match command {
            SpaceCommand::PatchParameter { .. } => "patchParameter",
            SpaceCommand::AddParameter { .. } => "addParameter",
            SpaceCommand::RemoveParameter { .. } => "removeParameter",
            SpaceCommand::SpawnApp { .. } => "spawnApp",
            SpaceCommand::MoveMediaNode { .. } => "moveMediaNode",
            SpaceCommand::ConnectMediaPorts { .. } => "connectMediaPorts",
            SpaceCommand::DisconnectMediaEdge { .. } => "disconnectMediaEdge",
            SpaceCommand::RemoveAppInstance { .. } => "removeAppInstance",
            SpaceCommand::DeleteSelection => "deleteSelection",
            SpaceCommand::CopyAppInstance => "copyAppInstance",
            SpaceCommand::DuplicateAppInstance => "duplicateAppInstance",
            SpaceCommand::PasteAppInstance => "pasteAppInstance",
            SpaceCommand::RenameAppInstance { .. } => "renameAppInstance",
            SpaceCommand::PatchMediaNodes { .. } => "patchMediaNodes",
            SpaceCommand::PatchAppInstances { .. } => "patchAppInstances",
            SpaceCommand::BindParameterField { .. } => "bindParameterField",
            SpaceCommand::UnbindParameterField { .. } => "unbindParameterField",
            SpaceCommand::ReorganizeWorkflow => "reorganizeWorkflow",
            SpaceCommand::WorkflowEngagementSubmit { .. } => "workflowEngagementSubmit",
            SpaceCommand::CompiledDagEngagementSubmit => "compiledDagEngagementSubmit",
            SpaceCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            SpaceCommand::SetActivePanelTab { .. } => "setActivePanelTab",
            SpaceCommand::SelectInstance { .. } => "selectInstance",
            SpaceCommand::NodeGraphSelect { .. } => "nodeGraphSelect",
            SpaceCommand::SetMediaNodeSelection { .. } => "setMediaNodeSelection",
            SpaceCommand::SetAppInstanceSelection { .. } => "setAppInstanceSelection",
            SpaceCommand::NodeGraphHover { .. } => "nodeGraphHover",
            SpaceCommand::TextHover { .. } => "textHover",
            SpaceCommand::NodeGraphViewport { .. } => "nodeGraphViewport",
            SpaceCommand::PresenceHeartbeat { .. } => "presenceHeartbeat",
            SpaceCommand::WorkflowEngagementInput { .. } => "workflowEngagementInput",
            SpaceCommand::CompiledDagEngagementInput { .. } => "compiledDagEngagementInput",
            SpaceCommand::SetActiveExample { .. } => "setActiveExample",
            SpaceCommand::ExportMedia { .. } => "exportMedia",
            SpaceCommand::ImportMedia { .. } => "importMedia",
            SpaceCommand::ImportMediaPayload { .. } => "importMediaPayload",
            SpaceCommand::ExportStudioPack => "exportStudioPack",
            SpaceCommand::ExportStudioDsl => "exportStudioDsl",
            SpaceCommand::ImportSpacePack => "importSpacePack",
            SpaceCommand::ImportSpacePackPayload { .. } => "importSpacePackPayload",
            SpaceCommand::OpenSpace { .. } => "openSpace",
            SpaceCommand::OpenInstance { .. } => "openInstance",
            SpaceCommand::CloseFocusedInstance => "closeFocusedInstance",
            SpaceCommand::GoHome => "goHome",
            SpaceCommand::NavigateVirtualFileSystemNode { .. } => "navigateVirtualFileSystemNode",
        }
    }

    fn handle(
        &self,
        command: &SpaceCommand,
        doc: &DocumentView<'_, OsProjection>,
        cfg: &ConfigView<'_, SpaceConfig>,
    ) -> Emit<OsOperation, SpaceConfigOperation> {
        let projection = doc.projection;
        let config = cfg.projection;
        match command {
            SpaceCommand::PatchParameter { parameter_id, field, value } => {
                let value_json: Value = serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.clone()));
                let patch = if field == "addOption" {
                    value_json.as_str().map(str::to_string).and_then(|option| {
                        projection.parameters.iter().find(|entry| parameter_entity_id(entry) == parameter_id).and_then(|entry| match entry {
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
                    value_json.as_str().map(str::to_string).and_then(|option| {
                        projection.parameters.iter().find(|entry| parameter_entity_id(entry) == parameter_id).and_then(|entry| match entry {
                            OsParameter::Categorical { options, value, .. } => {
                                let next_options: Vec<_> = options.iter().filter(|row| row.as_str() != option).cloned().collect();
                                let next_value = if next_options.iter().any(|row| row == value) { value.clone() } else { next_options.first().cloned().unwrap_or_default() };
                                Some(json!({ "options": next_options, "value": next_value }))
                            }
                            _ => None,
                        })
                    })
                } else {
                    Some(json!({ field: value_json }))
                };
                match patch.and_then(|patch| patch_parameter_operation(projection, parameter_id, &patch)) {
                    Some(operation) => Emit::operations(vec![operation]),
                    None => Emit::default(),
                }
            }
            SpaceCommand::AddParameter { name, kind } => {
                let parameter_type = match kind.as_str() {
                    "categorical" => OsParameterType::Categorical,
                    "toggle" => OsParameterType::Toggle,
                    "text" => OsParameterType::Text,
                    _ => OsParameterType::Numeric,
                };
                Emit::operations(vec![add_parameter_operation(&parameter_type, name)])
            }
            SpaceCommand::RemoveParameter { parameter_id } => Emit::operations(vec![OsOperation::RemoveParameter { parameter_id: parameter_id.clone() }]),
            SpaceCommand::SpawnApp { plugin_id, app_id, x, y } => match add_workflow_node_operation(plugin_id, app_id, None, *x, *y) {
                Some((operation, node_id)) => Emit {
                    document_operations: vec![operation],
                    config_operations: vec![SpaceConfigOperation::SetActiveNode { node_id: Some(node_id) }],
                    ..Default::default()
                },
                None => Emit::default(),
            },
            SpaceCommand::MoveMediaNode { node_id, x, y } => Emit::amend(vec![OsOperation::MoveWorkflowNode { node_id: node_id.clone(), x: *x, y: *y }], format!("moveMediaNode:{node_id}")),
            SpaceCommand::ConnectMediaPorts { source_node_id, source_port_id, target_node_id, target_port_id } => {
                match negotiate_connect_or_notify(projection, source_node_id, source_port_id, target_node_id, target_port_id) {
                    Ok(contract) => Emit::operations(vec![connect_edge_operation(source_node_id, source_port_id, target_node_id, target_port_id, contract)]),
                    Err(effect) => Emit::effect(effect),
                }
            }
            SpaceCommand::DisconnectMediaEdge { edge_id } => Emit::operations(vec![OsOperation::DisconnectWorkflowEdge { edge_id: edge_id.clone() }]),
            SpaceCommand::RemoveAppInstance { node_id } => match node_id.clone().or_else(|| primary_selected_node_id(config)) {
                Some(node_id) => {
                    let mut config_operations = Vec::new();
                    if config.active_node_id.as_deref() == Some(node_id.as_str()) {
                        config_operations.push(SpaceConfigOperation::SetActiveNode { node_id: None });
                    }
                    if config.focused_node_id.as_deref() == Some(node_id.as_str()) {
                        config_operations.push(SpaceConfigOperation::SetFocusedNode { node_id: None });
                    }
                    Emit { document_operations: vec![OsOperation::RemoveWorkflowNode { node_id }], config_operations, ..Default::default() }
                }
                None => Emit::default(),
            },
            SpaceCommand::DeleteSelection => {
                let document_operations = config.selected_node_ids.iter().cloned().map(|node_id| OsOperation::RemoveWorkflowNode { node_id }).collect();
                Emit {
                    document_operations,
                    config_operations: vec![
                        SpaceConfigOperation::SetSelection { node_ids: Vec::new() },
                        SpaceConfigOperation::SetActiveNode { node_id: None },
                        SpaceConfigOperation::SetFocusedNode { node_id: None },
                    ],
                    ..Default::default()
                }
            }
            SpaceCommand::CopyAppInstance => Emit::config(vec![SpaceConfigOperation::SetClipboard { node_ids: config.selected_node_ids.clone() }]),
            SpaceCommand::DuplicateAppInstance | SpaceCommand::PasteAppInstance => {
                let source_ids = if matches!(command, SpaceCommand::PasteAppInstance) { config.clipboard_node_ids.clone() } else { config.selected_node_ids.clone() };
                let mut document_operations = Vec::new();
                let mut new_active_node_id = None;
                for node_id in source_ids {
                    let Some(node) = projection.workflow.nodes.iter().find(|row| row.id == node_id) else { continue };
                    let label = format!("{} Copy", node.label);
                    if let Some((operation, new_id)) = add_workflow_node_operation(&node.plugin_id, &node.app_id, Some(&label), node.x + 40.0, node.y + 40.0) {
                        new_active_node_id = Some(new_id);
                        document_operations.push(operation);
                    }
                }
                let config_operations = new_active_node_id.into_iter().map(|node_id| SpaceConfigOperation::SetActiveNode { node_id: Some(node_id) }).collect();
                Emit { document_operations, config_operations, ..Default::default() }
            }
            SpaceCommand::RenameAppInstance { label } => match primary_selected_node_id(config) {
                Some(node_id) => {
                    let next_label = label.clone().or_else(|| projection.workflow.nodes.iter().find(|row| row.id == node_id).map(|node| format!("{} (renamed)", node.label)));
                    match next_label {
                        Some(next_label) => Emit::operations(vec![OsOperation::PatchWorkflowNode { node_id, label: next_label }]),
                        None => Emit::default(),
                    }
                }
                None => Emit::default(),
            },
            SpaceCommand::PatchMediaNodes { node_ids, field, axis, value } => {
                let numeric = value.parse::<f64>().ok();
                if field == "position" && numeric.is_some() {
                    let numeric = numeric.unwrap();
                    let document_operations = node_ids
                        .iter()
                        .filter_map(|node_id| {
                            let node = projection.workflow.nodes.iter().find(|row| &row.id == node_id)?;
                            let x = if axis.as_deref() == Some("x") { numeric } else { node.x };
                            let y = if axis.as_deref() == Some("y") { numeric } else { node.y };
                            Some(OsOperation::MoveWorkflowNode { node_id: node_id.clone(), x, y })
                        })
                        .collect();
                    Emit::operations(document_operations)
                } else {
                    Emit::default()
                }
            }
            SpaceCommand::PatchAppInstances { node_ids, field, value } => {
                if field == "label" {
                    Emit::operations(node_ids.iter().cloned().map(|node_id| OsOperation::PatchWorkflowNode { node_id, label: value.clone() }).collect())
                } else {
                    Emit::default()
                }
            }
            SpaceCommand::BindParameterField { node_id, field_path, parameter_id } => {
                if parameter_id.is_empty() || parameter_id == "__direct__" {
                    Emit::operations(vec![OsOperation::UnbindParameterField { node_id: node_id.clone(), field_path: field_path.clone() }])
                } else {
                    Emit::operations(vec![OsOperation::BindParameterField { binding: OsParameterFieldBinding { parameter_id: parameter_id.clone(), node_id: node_id.clone(), field_path: field_path.clone() } }])
                }
            }
            SpaceCommand::UnbindParameterField { node_id, field_path } => Emit::operations(vec![OsOperation::UnbindParameterField { node_id: node_id.clone(), field_path: field_path.clone() }]),
            SpaceCommand::ReorganizeWorkflow => {
                let node_ids: Vec<String> = if config.selected_node_ids.is_empty() { projection.workflow.nodes.iter().map(|node| node.id.clone()).collect() } else { config.selected_node_ids.clone() };
                let document_operations = node_ids
                    .iter()
                    .enumerate()
                    .map(|(index, node_id)| {
                        let col = (index % 4) as f64;
                        let row = (index / 4) as f64;
                        OsOperation::MoveWorkflowNode { node_id: node_id.clone(), x: 80.0 + col * 220.0, y: 80.0 + row * 160.0 }
                    })
                    .collect();
                Emit::operations(document_operations)
            }
            SpaceCommand::WorkflowEngagementSubmit { value } => {
                let raw = value.clone().unwrap_or_else(|| config.workflow_engagement_input.clone());
                let mut parts = raw.split_whitespace();
                match (parts.next(), parts.next()) {
                    (Some(plugin_id), Some(app_id)) => match add_workflow_node_operation(plugin_id, app_id, None, 80.0, 80.0) {
                        Some((operation, node_id)) => Emit {
                            document_operations: vec![operation],
                            config_operations: vec![SpaceConfigOperation::SetActiveNode { node_id: Some(node_id) }],
                            ..Default::default()
                        },
                        None => Emit::default(),
                    },
                    _ => Emit::default(),
                }
            }
            SpaceCommand::CompiledDagEngagementSubmit => Emit::default(),
            SpaceCommand::NodeGraphEdit { operations_json } => {
                let edit_operations = serde_json::from_str::<Value>(operations_json).ok().and_then(|value| value.get("operations").and_then(Value::as_array).cloned()).unwrap_or_default();
                let mut document_operations = Vec::new();
                let mut config_operations = Vec::new();
                let mut effects = Vec::new();
                for edit in &edit_operations {
                    match edit.get("operation").and_then(Value::as_str).unwrap_or("") {
                        "setFixture" => {
                            if let Some(fixture_json) = edit.get("fixtureJson").and_then(Value::as_str) {
                                if let Some(camera) = serde_json::from_str::<Value>(fixture_json).ok().and_then(|fixture| fixture.get("camera").cloned()).and_then(|camera| serde_json::from_value::<OsWorkflowCamera>(camera).ok()) {
                                    config_operations.push(SpaceConfigOperation::SetCamera { window_id: S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() });
                                }
                                document_operations.extend(apply_flow_fixture_to_os_workflow(&projection.workflow, fixture_json));
                            }
                        }
                        "move" => {
                            if let (Some(node_id), Some(x), Some(y)) = (edit.get("nodeId").and_then(Value::as_str), edit.get("x").and_then(Value::as_f64), edit.get("y").and_then(Value::as_f64)) {
                                document_operations.push(OsOperation::MoveWorkflowNode { node_id: node_id.into(), x, y });
                            }
                        }
                        "connect" => {
                            if let (Some(source_node_id), Some(source_port_id), Some(target_node_id), Some(target_port_id)) =
                                (edit.get("sourceNodeId").and_then(Value::as_str), edit.get("sourcePortId").and_then(Value::as_str), edit.get("targetNodeId").and_then(Value::as_str), edit.get("targetPortId").and_then(Value::as_str))
                            {
                                match negotiate_connect_or_notify(projection, source_node_id, source_port_id, target_node_id, target_port_id) {
                                    Ok(contract) => document_operations.push(connect_edge_operation(source_node_id, source_port_id, target_node_id, target_port_id, contract)),
                                    Err(effect) => effects.push(effect),
                                }
                            }
                        }
                        "deleteSelection" => {
                            for node_id in &config.selected_node_ids {
                                document_operations.push(OsOperation::RemoveWorkflowNode { node_id: node_id.clone() });
                            }
                        }
                        _ => {}
                    }
                }
                Emit { document_operations, config_operations, effects, ..Default::default() }
            }
            SpaceCommand::SetActivePanelTab { tab_id } => Emit::config(vec![SpaceConfigOperation::SetActivePanelTab { tab_id: tab_id.clone() }]),
            SpaceCommand::SelectInstance { node_id } => {
                let node_ids = node_id.iter().cloned().collect();
                Emit::config(vec![SpaceConfigOperation::SetActiveNode { node_id: node_id.clone() }, SpaceConfigOperation::SetSelection { node_ids }])
            }
            SpaceCommand::NodeGraphSelect { node_ids, select_all } | SpaceCommand::SetMediaNodeSelection { node_ids, select_all } => {
                let node_ids = if *select_all { projection.workflow.nodes.iter().map(|node| node.id.clone()).collect() } else { node_ids.clone() };
                let mut config_operations = vec![SpaceConfigOperation::SetSelection { node_ids: node_ids.clone() }];
                if node_ids.len() == 1 {
                    config_operations.push(SpaceConfigOperation::SetActiveNode { node_id: node_ids.first().cloned() });
                }
                let next_config = apply_config_operations(config, &config_operations);
                publish_presence(&next_config);
                Emit::config(config_operations)
            }
            SpaceCommand::SetAppInstanceSelection { node_ids } => {
                let mut config_operations = vec![SpaceConfigOperation::SetSelection { node_ids: node_ids.clone() }];
                if node_ids.len() == 1 {
                    config_operations.push(SpaceConfigOperation::SetActiveNode { node_id: node_ids.first().cloned() });
                }
                let next_config = apply_config_operations(config, &config_operations);
                publish_presence(&next_config);
                Emit::config(config_operations)
            }
            SpaceCommand::NodeGraphHover { hover_json } | SpaceCommand::TextHover { hover_json } => {
                let node_id = hover_json.as_deref().and_then(|text| {
                    serde_json::from_str::<Value>(text).ok().and_then(|parsed| parsed.get("nodeId").and_then(|id| id.as_str().map(str::to_string))).or_else(|| Some(text.to_string()))
                });
                Emit::config(vec![SpaceConfigOperation::SetHover { node_id }])
            }
            SpaceCommand::NodeGraphViewport { viewport_json } => match serde_json::from_str::<OsWorkflowCamera>(viewport_json) {
                Ok(camera) => Emit::config(vec![SpaceConfigOperation::SetCamera { window_id: S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() }]),
                Err(_) => Emit::default(),
            },
            SpaceCommand::PresenceHeartbeat { client_id, name } => {
                let config_operations = vec![SpaceConfigOperation::SetClient { client_id: Some(client_id.clone()), client_name: Some(name.clone()) }];
                let next_config = apply_config_operations(config, &config_operations);
                publish_presence(&next_config);
                Emit { config_operations, ui_scope: semio_framework_core::kernel::UiDirtyScope::None, ..Default::default() }
            }
            SpaceCommand::WorkflowEngagementInput { value } => Emit::config(vec![SpaceConfigOperation::SetWorkflowEngagementInput { value: value.clone() }]),
            SpaceCommand::CompiledDagEngagementInput { value } => Emit::config(vec![SpaceConfigOperation::SetCompiledDagEngagementInput { value: value.clone() }]),
            SpaceCommand::SetActiveExample { example_id } => {
                if example_id.is_empty() {
                    Emit::default()
                } else {
                    Emit::effect(HostEffect::Navigate { uri: format!("/spaces/{example_id}") })
                }
            }
            SpaceCommand::ExportMedia { node_id, format } => {
                match projection.workflow.nodes.iter().find(|row| &row.id == node_id) {
                    Some(node) => {
                        ensure_space_fixtures_registered();
                        let schema = os_app_registration(&node.plugin_id, &node.app_id).map(|row| row.source_format).unwrap_or_default();
                        let document_json = materialize_os_app_instance_document_json(&json!({ "schema": schema }).to_string(), &node.id, &projection.parameter_bindings, &projection.parameters);
                        let document_value: Value = serde_json::from_str(&document_json).unwrap_or_else(|_| json!({}));
                        let export_format = semio_framework_os::OsMediaFormat::parse(format).unwrap_or(semio_framework_os::OsMediaFormat::Svg);
                        match semio_framework_os::export_os_app_instance_media(node, &document_value, export_format) {
                            Ok(result) => Emit::effect(HostEffect::DownloadMediaExport { filename: result.file_name, mime_type: result.mime_type, data: result.data, encoding: result.encoding }),
                            Err(_) => Emit::default(),
                        }
                    }
                    None => Emit::default(),
                }
            }
            SpaceCommand::ImportMedia { node_id, format } => Emit {
                config_operations: vec![SpaceConfigOperation::SetPendingImport { node_id: Some(node_id.clone()), format: Some(format.clone()) }],
                effects: vec![HostEffect::RequestFileOpen { accept: format!(".{format}"), read_as: Some("dataUrl".into()), import_action: "importMediaPayload".into(), multiple: false }],
                ..Default::default()
            },
            SpaceCommand::ImportMediaPayload { payload } => {
                let mut config_operations = Vec::new();
                if let (Some(node_id), Some(format_name)) = (config.pending_import_node_id.clone(), config.pending_import_format.clone()) {
                    config_operations.push(SpaceConfigOperation::SetPendingImport { node_id: None, format: None });
                    if let Some(format) = semio_framework_os::OsMediaFormat::parse(&format_name) {
                        use base64::Engine;
                        let base64_part = payload.split_once(',').map(|(_, data)| data).unwrap_or(payload);
                        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(base64_part) {
                            if let Some(node) = projection.workflow.nodes.iter().find(|row| row.id == node_id) {
                                // 📥️ Decoding/validation happens here; the decoded content is applied to
                                // the node's own document-ref document by the host (a cross-document
                                // operation the shell can't author from its own store), so this arm emits
                                // no studio document operation.
                                let _ = semio_framework_os::import_os_app_instance_media(node, &bytes, format);
                            }
                        }
                    }
                }
                Emit::config(config_operations)
            }
            SpaceCommand::ExportStudioPack => {
                let space_id = config_space_id(config);
                match home_ui::resolve_studio_document(&space_id) {
                    Some(document) => match export_os_space_pack(&document) {
                        Ok(pack_files) => {
                            use base64::Engine;
                            Emit {
                                effects: vec![
                                    HostEffect::DownloadMediaExport { filename: format!("{space_id}.pack"), mime_type: "application/octet-stream".into(), data: base64::engine::general_purpose::STANDARD.encode(&pack_files.pack), encoding: Some("base64".into()) },
                                    HostEffect::DownloadMediaExport { filename: format!("{space_id}.ops"), mime_type: "text/plain".into(), data: pack_files.ops, encoding: None },
                                ],
                                ..Default::default()
                            }
                        }
                        Err(_) => Emit::default(),
                    },
                    None => Emit::default(),
                }
            }
            SpaceCommand::ExportStudioDsl => {
                let space_id = config_space_id(config);
                match home_ui::resolve_studio_document(&space_id) {
                    Some(document) => match export_os_space_dsl(&document) {
                        Ok(text_files) => Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{space_id}.os"), mime_type: "text/plain".into(), data: text_files.dsl, encoding: None }),
                        Err(_) => Emit::default(),
                    },
                    None => Emit::default(),
                }
            }
            SpaceCommand::ImportSpacePack => Emit::effect(HostEffect::RequestFileOpen { accept: ".pack".into(), read_as: Some("dataUrl".into()), import_action: "importSpacePackPayload".into(), multiple: false }),
            SpaceCommand::ImportSpacePackPayload { payload } => {
                use base64::Engine;
                let base64_part = payload.split_once(',').map(|(_, data)| data).unwrap_or(payload);
                if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(base64_part) {
                    // 🌱️ A single `.pack` file carries no separate `.spr` sidecar (unlike
                    // `exportStudioPack`'s two-file output) — `store::empty_document_spr` builds a bare,
                    // edit-free op log so the pack+spr-first codec path still decodes to a document with
                    // no replayed edit history, i.e. its bare initial projection.
                    let empty_spr = store::empty_document_spr("", OS_SPACE_SCHEMA);
                    let _ = import_os_space_from_pack(&bytes, &empty_spr, home_ui::catalog_port());
                }
                Emit::default()
            }
            SpaceCommand::OpenSpace { space_id } => {
                let document = home_ui::resolve_studio_document(space_id)
                    .or_else(|| if space_id == "demo" { Some(parse_demo_space_document()) } else { None })
                    .unwrap_or_else(|| create_empty_os_document(space_id, "Untitled Studio"));
                let mut config_operations = vec![
                    SpaceConfigOperation::SetSpaceId { space_id: Some(space_id.clone()) },
                    SpaceConfigOperation::SetFocusedNode { node_id: None },
                    SpaceConfigOperation::SetSelection { node_ids: Vec::new() },
                    SpaceConfigOperation::SetClipboard { node_ids: Vec::new() },
                ];
                match home_ui::space_document_envelope_pack(&document) {
                    Some(files) => {
                        let active_node_id = materialize_os_projection(&document, &[]).ok().and_then(|projection| projection.workflow.nodes.first().map(|node| node.id.clone()));
                        config_operations.push(SpaceConfigOperation::SetActiveNode { node_id: active_node_id });
                        eprintln!(
                            "[DEBUG] openSpace id={} nodes={} backbone={:?}",
                            space_id,
                            document.vcs.initial_projection.workflow.nodes.len(),
                            document.backbone.as_ref().map(|row| row.uri.clone())
                        );
                        Emit { config_operations, effects: vec![HostEffect::LoadDocument { pack: files.pack, spr: files.spr }], ..Default::default() }
                    }
                    None => {
                        eprintln!("[DEBUG] openSpace missing envelope id={space_id}");
                        config_operations.push(SpaceConfigOperation::SetActiveNode { node_id: None });
                        Emit::config(config_operations)
                    }
                }
            }
            SpaceCommand::OpenInstance { node_id } => match node_id.clone().or_else(|| primary_selected_node_id(config)) {
                Some(node_id) => match projection.workflow.nodes.iter().find(|row| row.id == node_id) {
                    Some(node) => Emit {
                        config_operations: vec![
                            SpaceConfigOperation::SetFocusedNode { node_id: Some(node_id.clone()) },
                            SpaceConfigOperation::SetActiveNode { node_id: Some(node_id.clone()) },
                            SpaceConfigOperation::SetSelection { node_ids: vec![node_id.clone()] },
                        ],
                        effects: vec![HostEffect::OpenPluginInstance { plugin_id: node.plugin_id.clone(), app_id: node.app_id.clone(), os_instance_id: Some(node.id.clone()) }],
                        ..Default::default()
                    },
                    None => Emit::default(),
                },
                None => Emit::default(),
            },
            SpaceCommand::CloseFocusedInstance => Emit::config(vec![SpaceConfigOperation::SetFocusedNode { node_id: None }]),
            SpaceCommand::GoHome => Emit::effect(HostEffect::Navigate { uri: "/".into() }),
            SpaceCommand::NavigateVirtualFileSystemNode { space_id } => Emit::effect(HostEffect::Navigate { uri: format!("/spaces/{space_id}") }),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, OsProjection>, cfg: &ConfigView<'_, SpaceConfig>) -> UiNode {
        let projection = doc.projection;
        let config = cfg.projection;
        let labels = resolve_labels::<SStudioLabels>(config);
        match body_key {
            S_PLAY_BODY_WORKFLOW => render_workflow(projection, config, labels),
            S_PLAY_BODY_MEDIA_VFS => render_media_vfs(projection, labels),
            S_PLAY_BODY_COMPILED_DAG => render_compiled_dag(projection),
            S_PLAY_CATALOGUE_BODY_KEY => build_catalogue_tree(labels),
            S_PLAY_PARAMETERS_BODY_KEY => build_parameters_tree(projection, labels),
            S_PLAY_INSPECTOR_BODY_KEY => build_inspector_tree(projection, config, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_measures(&self, doc: &DocumentView<'_, OsProjection>, cfg: &ConfigView<'_, SpaceConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = resolve_labels::<SStudioLabels>(cfg.projection);
        HashMap::from([(S_PLAY_WINDOW_WORKFLOW.into(), workflow_measures(cfg.projection, &doc.projection.workflow.nodes, labels))])
    }

    fn app_labels(&self, cfg: &ConfigView<'_, SpaceConfig>) -> AppLabelsOverlay {
        let labels = resolve_labels::<SStudioLabels>(cfg.projection);
        let is_de = is_de_locale(cfg.projection);
        AppLabelsOverlay::default()
            .window_kind_label(S_PLAY_WINDOW_WORKFLOW, labels.window_workflow)
            .window_kind_label(S_PLAY_WINDOW_MEDIA_VFS, labels.window_media_vfs)
            .window_kind_label(S_PLAY_WINDOW_COMPILED_DAG, labels.window_compiled_dag)
            .action_labels(s_studio_action_labels(is_de))
    }

    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &DocumentView<'_, OsProjection>,
        cfg: &ConfigView<'_, SpaceConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let labels = resolve_labels::<SStudioLabels>(cfg.projection);
        let is_de = is_de_locale(cfg.projection);
        space_workflow_context_menu_items(registry, labels, is_de, request.surface.as_ref(), &cfg.projection.selected_node_ids)
    }
}

/// 🔧️ Small pure fold applying a batch of `SpaceConfigOperation`s onto a snapshot — used where
/// `handle()` needs the POST-command config (not the pre-command `cfg.projection`) to build a
/// derived side value (the presence broadcast) in the very same call, without reaching back into a
/// store this pure function doesn't own.
fn apply_config_operations(config: &SpaceConfig, operations: &[SpaceConfigOperation]) -> SpaceConfig {
    operations.iter().fold(config.clone(), |acc, operation| operation.diff(&acc))
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

fn workflow_engagement(config: &SpaceConfig, node_count: usize) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("s-media-catalogue-hint".into()),
            value: Some(config.workflow_engagement_input.clone()),
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
            text: format!("{node_count} nodes"),
        }]),
        possible_engagements: None,
    }
}

fn workflow_measures(config: &SpaceConfig, nodes: &[WorkflowNode], labels: &SStudioLabels) -> Vec<WindowMeasure> {
    vec![WindowMeasure::Select {
        id: "s-media-active-instance".into(),
        label: Some(labels.active_app.into()),
        value: config.active_node_id.clone().unwrap_or_default(),
        items: nodes
            .iter()
            .map(|node| MeasureSelectItem {
                id: node.id.clone(),
                value: node.id.clone(),
                label: node.label.clone(),
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
    let config = SpaceConfig::default();
    let engagement = workflow_engagement(&config, projection.workflow.nodes.len());
    let measures = workflow_measures(&config, &projection.workflow.nodes, resolve_labels::<SStudioLabels>(&config));
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
            "patchParameter".into(), "addParameter".into(), "removeParameter".into(),
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
    let mut app = App { definition, examples: Vec::new() };
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
        apply_os_operation, register_app_io, register_artifact_descriptor, validate_workflow, ArtifactKindSpec, ArtifactPresentation, AppDefinition, MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, MediaWireFormat,
        OsMediaFormat, PortMultiplicity,
    };
    use semio_framework_plugin::{testkit, AppIo, HistoryView, PluginApp, UiControlNode, UiNode, VcsDocumentApp, ViewState};
    use std::collections::HashSet;

    //#region 🔧️Harness
    fn empty_history() -> HistoryView {
        HistoryView::empty()
    }

    /// 🎛️ Drives the typed `SpaceApp::handle` against a projection/config snapshot, returning its emit.
    fn studio_emit(projection: &OsProjection, config: &SpaceConfig, command: SpaceCommand) -> Emit<OsOperation, SpaceConfigOperation> {
        let history = empty_history();
        let doc = DocumentView { projection, history: &history };
        let cfg = ConfigView { projection: config };
        SpaceApp.handle(&command, &doc, &cfg)
    }

    /// 📽️ Folds studio document operations onto a projection the way the store would (minus history).
    fn apply_operations(projection: &OsProjection, operations: &[OsOperation]) -> OsProjection {
        operations.iter().fold(projection.clone(), |current, operation| apply_os_operation(&current, operation))
    }

    /// 📽️ Folds studio config operations onto a config snapshot the way the store would.
    fn apply_config(config: &SpaceConfig, operations: &[SpaceConfigOperation]) -> SpaceConfig {
        apply_config_operations(config, operations)
    }

    /// 🌱️ Registers a minimal `AppDefinition` directly into the production registry (`register_app_io`)
    /// — the B1 replacement for the pre-merge `merge_os_plugin_definition`/`OsAppResourceSpec` test
    /// seeding (both deleted). Every declared port additionally carries the implicit `document:in`/
    /// `document:out` pair (see `AppIo::all_ports`).
    fn seed_app(plugin_id: &str, app_id: &str, label: &str, document: &[&str], document_schema: &str, ports: Vec<MediaPortSpec>) -> AppDefinition {
        let definition = App::builder(app_id, label)
            .document(document.iter().map(|segment| segment.to_string()))
            .mode("edit", "Edit", "square-pen")
            .window_kind("main", "Main", format!("{app_id}.main"), SurfaceKind::Canvas2d, "square-pen")
            .io(AppIo::from_document(document_schema, MediaType { class: MediaClass::Data, form: MediaForm::Value }, ArtifactPresentation { id: app_id.into(), name: label.into(), dimension: String::new(), component_kind: app_id.into() }).with_ports(ports))
            .build_definition();
        register_app_io(plugin_id, &definition);
        definition
    }

    fn seed_draw_plugin() {
        seed_app("draw", "draw", "Draw", &["semio", "draw"], "draw.document", Vec::new());
    }

    fn seed_catalogue_apps() {
        seed_app("puzzle", "puzzle2d-play", "Puzzle 2D", &["semio", "puzzle", "2d"], "puzzle2d.document", Vec::new());
        seed_app("puzzle", "puzzle3d-play", "Puzzle 3D", &["semio", "puzzle", "3d"], "puzzle3d.document", Vec::new());
    }

    fn seed_multi_port_plugins() {
        let puzzle_ports = vec![
            MediaPortSpec { id: "in-a".into(), label: "In A".into(), direction: MediaPortDirection::In, media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, kind_id: Some("topology".into()), required: false, multiplicity: PortMultiplicity::One },
            MediaPortSpec { id: "out-a".into(), label: "Out A".into(), direction: MediaPortDirection::Out, media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, kind_id: Some("topology".into()), required: false, multiplicity: PortMultiplicity::One },
            MediaPortSpec { id: "out-b".into(), label: "Out B".into(), direction: MediaPortDirection::Out, media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, kind_id: Some("topology".into()), required: false, multiplicity: PortMultiplicity::One },
        ];
        seed_app("puzzle.5d", "puzzle5d", "Puzzle 5D", &["semio", "puzzle", "5d"], "puzzle5d.document", puzzle_ports);

        let shooting_ports = vec![MediaPortSpec { id: "scene-in".into(), label: "Scene".into(), direction: MediaPortDirection::In, media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, kind_id: Some("2d.shooting".into()), required: true, multiplicity: PortMultiplicity::One }];
        seed_app("shooting", "shooting", "Shooting", &["semio", "shooting"], "shooting.document", shooting_ports);
    }

    fn test_node(id: &str, inputs: Vec<semio_framework_os::WorkflowMediaPort>, outputs: Vec<semio_framework_os::WorkflowMediaPort>) -> WorkflowNode {
        WorkflowNode {
            id: id.into(),
            plugin_id: "test".into(),
            app_id: "test".into(),
            label: id.into(),
            yields: String::new(),
            document_ref: format!("documents/{id}"),
            config_ref: format!("config/{id}"),
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            inputs,
            outputs,
        }
    }

    fn test_port(node_id: &str, spec_id: &str, direction: MediaPortDirection, media_type: MediaType, kind_id: &str) -> semio_framework_os::WorkflowMediaPort {
        let dir_word = match direction {
            MediaPortDirection::In => "in",
            MediaPortDirection::Out => "out",
        };
        semio_framework_os::WorkflowMediaPort {
            id: format!("{node_id}:{spec_id}:{dir_word}"),
            spec: MediaPortSpec { id: spec_id.into(), label: spec_id.into(), direction, media_type, kind_id: Some(kind_id.into()), required: false, multiplicity: PortMultiplicity::One },
        }
    }
    //#endregion 🔧️Harness

    #[test]
    fn initial_projection_is_empty_not_demo() {
        let app = SpaceApp;
        assert!(app.initial_projection().workflow.nodes.is_empty());
    }

    #[test]
    fn open_studio_loads_created_empty_catalog_studio() {
        use semio_framework_os::{create_os_space, MemoryBackbonePort};
        use std::sync::Arc;
        let port: Arc<dyn semio_framework_os::OsBackbonePort> = Arc::new(MemoryBackbonePort::new());
        let entry = create_os_space("Opened Empty", port.clone()).expect("create");
        home_ui::register_studio_port_for_test(&entry.id, port);
        let empty = default_os_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&empty, &config, SpaceCommand::OpenSpace { space_id: entry.id.clone() });
        assert!(emit.config_operations.contains(&SpaceConfigOperation::SetSpaceId { space_id: Some(entry.id.clone()) }));
        assert!(emit.config_operations.contains(&SpaceConfigOperation::SetActiveNode { node_id: None }));
        assert!(emit.effects.iter().any(|effect| matches!(effect, HostEffect::LoadDocument { .. })));
        assert!(!emit.effects.iter().any(|effect| matches!(effect, HostEffect::Navigate { .. })));
    }

    fn load_document_projection(emit: &Emit<OsOperation, SpaceConfigOperation>) -> (OsProjection, String) {
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
        let empty = default_os_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&empty, &config, SpaceCommand::OpenSpace { space_id: "unknown-studio-id".into() });
        let (projection, id) = load_document_projection(&emit);
        assert_eq!(id, "unknown-studio-id");
        assert!(projection.workflow.nodes.is_empty());
        assert_ne!(id, "demo");
    }

    #[test]
    fn open_studio_demo_explicit_loads_demo_fixture() {
        let empty = default_os_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&empty, &config, SpaceCommand::OpenSpace { space_id: "demo".into() });
        let (projection, id) = load_document_projection(&emit);
        assert!(id.contains("demo-studio"));
        assert!(!projection.workflow.nodes.is_empty());
    }

    #[test]
    fn open_studio_loads_ephemeral_created_studio() {
        let home = home_ui::HomeApp;
        let home_projection = home.initial_projection();
        let history = empty_history();
        let doc = DocumentView { projection: &home_projection, history: &history };
        let home_config = home_engine::HomeConfig::default();
        let home_cfg = ConfigView { projection: &home_config };
        let create = home.handle(&home_protocol::HomeCommand::CreateStudio { name: "Ephemeral Open".into(), kind: "catalog".into(), folder_path: None }, &doc, &home_cfg);
        let space_id = create
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::Navigate { uri } => Some(uri.trim_start_matches("/spaces/").to_string()),
                _ => None,
            })
            .expect("navigate");
        let empty = default_os_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&empty, &config, SpaceCommand::OpenSpace { space_id: space_id.clone() });
        let (projection, id) = load_document_projection(&emit);
        assert_eq!(id, space_id);
        assert!(projection.workflow.nodes.is_empty());
    }

    #[test]
    fn demo_document_has_instances_and_edges() {
        let projection = demo_space_projection();
        assert!(projection.workflow.nodes.len() >= 5);
        assert!(projection.workflow.edges.len() >= 1);
        assert!(validate_workflow(&projection.workflow).ok);
    }

    #[test]
    fn renders_workflow_scene() {
        let mut app = VcsDocumentApp::new(SpaceApp);
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
        let mut app = VcsDocumentApp::new(SpaceApp);
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
    }

    #[test]
    fn move_media_node_emits_coalesced_move_operation() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node_id = projection.workflow.nodes.first().expect("node").id.clone();
        let emit = studio_emit(&projection, &config, SpaceCommand::MoveMediaNode { node_id: node_id.clone(), x: 120.0, y: 160.0 });
        assert_eq!(emit.coalesce_key.as_deref(), Some(format!("moveMediaNode:{node_id}").as_str()));
        let node = apply_operations(&projection, &emit.document_operations)
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
        let src_out = test_port("contract-src", "out", MediaPortDirection::Out, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, "test.contract.2d");
        let dst_in = test_port("contract-dst", "in", MediaPortDirection::In, MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, "test.contract.3d");
        projection.workflow.nodes.push(test_node("contract-src", vec![], vec![src_out]));
        projection.workflow.nodes.push(test_node("contract-dst", vec![dst_in], vec![]));
        let config = SpaceConfig::default();
        let emit = studio_emit(
            &projection,
            &config,
            SpaceCommand::ConnectMediaPorts { source_node_id: "contract-src".into(), source_port_id: "contract-src:out:out".into(), target_node_id: "contract-dst".into(), target_port_id: "contract-dst:in:in".into() },
        );
        assert!(emit.document_operations.is_empty(), "an incompatible connect must not push OsOperation::ConnectWorkflowPorts");
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
        let src_out = test_port("contract-src-2", "out", MediaPortDirection::Out, MediaType { class: MediaClass::Data, form: MediaForm::Value }, "test.contract.doc-a");
        let dst_in = test_port("contract-dst-2", "in", MediaPortDirection::In, MediaType { class: MediaClass::Data, form: MediaForm::Value }, "test.contract.doc-b");
        projection.workflow.nodes.push(test_node("contract-src-2", vec![], vec![src_out]));
        projection.workflow.nodes.push(test_node("contract-dst-2", vec![dst_in], vec![]));
        let config = SpaceConfig::default();
        let emit = studio_emit(
            &projection,
            &config,
            SpaceCommand::ConnectMediaPorts { source_node_id: "contract-src-2".into(), source_port_id: "contract-src-2:out:out".into(), target_node_id: "contract-dst-2".into(), target_port_id: "contract-dst-2:in:in".into() },
        );
        let edge = emit
            .document_operations
            .iter()
            .find_map(|operation| match operation {
                OsOperation::ConnectWorkflowPorts { edge } if edge.source_node_id == "contract-src-2" => Some(edge.clone()),
                _ => None,
            })
            .expect("a compatible connect must push OsOperation::ConnectWorkflowPorts with a negotiated contract");
        assert_eq!(edge.contract.kind_id, "test.contract.doc-b");
        assert_eq!(edge.contract.wire, MediaWireFormat::Document { schema: "test.contract.doc.schema".into() });
        assert!(edge.contract.conversion.is_none());
        let next = apply_operations(&projection, &emit.document_operations);
        assert!(validate_workflow(&next.workflow).ok, "a freshly negotiated edge must pass validate_workflow's contract-consistency check");
    }
    //#endregion 🔖️MediaContractConnect

    #[test]
    fn spawns_draw_app_instance() {
        seed_draw_plugin();
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, SpaceCommand::SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 80.0, y: 80.0 });
        assert!(!emit.document_operations.is_empty());
        let next = apply_operations(&projection, &emit.document_operations);
        assert_eq!(next.workflow.nodes.len(), projection.workflow.nodes.len() + 1);
        let expected_active = next.workflow.nodes.last().map(|node| node.id.clone());
        assert_eq!(emit.config_operations, vec![SpaceConfigOperation::SetActiveNode { node_id: expected_active }]);
    }

    #[test]
    fn spawns_draw_app_instance_at_drop_position() {
        seed_draw_plugin();
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let existing: HashSet<String> = projection.workflow.nodes.iter().map(|node| node.id.clone()).collect();
        let emit = studio_emit(&projection, &config, SpaceCommand::SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 321.0, y: 654.0 });
        let next = apply_operations(&projection, &emit.document_operations);
        let node = next.workflow.nodes.iter().find(|node| node.plugin_id == "draw" && !existing.contains(&node.id)).expect("newly spawned draw node");
        assert!((node.x - 321.0).abs() < 0.01);
        assert!((node.y - 654.0).abs() < 0.01);
    }

    #[test]
    fn open_instance_emits_open_plugin_instance_effect_matching_instance() {
        seed_draw_plugin();
        let projection = demo_space_projection();
        let node = projection.workflow.nodes.iter().find(|node| node.plugin_id == "draw").expect("draw node").clone();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, SpaceCommand::OpenInstance { node_id: Some(node.id.clone()) });
        assert!(emit.document_operations.is_empty(), "opening an instance is a host effect, not a document operation");
        let opened = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::OpenPluginInstance { plugin_id, app_id, os_instance_id } => Some((plugin_id.clone(), app_id.clone(), os_instance_id.clone())),
                _ => None,
            })
            .expect("OpenPluginInstance effect");
        assert_eq!(opened.0, "draw");
        assert_eq!(opened.1, "draw");
        assert_eq!(opened.2.as_deref(), Some(node.id.as_str()));
    }

    #[test]
    fn export_media_emits_download_effect_and_import_requests_file_open() {
        use base64::Engine;
        seed_draw_plugin();
        semio_framework_os::register_os_media_export_handler("2d.drawing", OsMediaFormat::Dwg, |_doc| {
            let drawing = semio_framework_os::DwgDrawing::default();
            let bytes = semio_framework_os::dwg_to_bytes(&drawing)?;
            Ok(semio_framework_os::OsMediaExportResult {
                data: base64::engine::general_purpose::STANDARD.encode(bytes),
                mime_type: OsMediaFormat::Dwg.mime_type().into(),
                file_name: "draw.dwg".into(),
                encoding: Some("base64".into()),
            })
        });
        semio_framework_os::register_dwg_import_handler("2d.drawing", |_drawing| Ok(json!({ "schema": "draw.document", "imported": true })));

        let projection = demo_space_projection();
        let node = projection.workflow.nodes.iter().find(|node| node.plugin_id == "draw").expect("draw node").clone();
        let config = SpaceConfig::default();

        let export = studio_emit(&projection, &config, SpaceCommand::ExportMedia { node_id: node.id.clone(), format: "dwg".into() });
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

        let import = studio_emit(&projection, &config, SpaceCommand::ImportMedia { node_id: node.id.clone(), format: "dwg".into() });
        assert!(import.effects.iter().any(|effect| matches!(effect, HostEffect::RequestFileOpen { import_action, .. } if import_action == "importMediaPayload")));
        assert_eq!(import.config_operations, vec![SpaceConfigOperation::SetPendingImport { node_id: Some(node.id.clone()), format: Some("dwg".into()) }]);

        // Decoding is exercised here; the decoded content is applied to the node's own document-ref
        // document by the host, so this arm emits no studio document operation.
        let pending_config = apply_config(&config, &import.config_operations);
        let payload = studio_emit(&projection, &pending_config, SpaceCommand::ImportMediaPayload { payload: format!("data:image/vnd.dwg;base64,{data}") });
        assert!(payload.document_operations.is_empty());
    }

    #[test]
    fn commit_checkpoint_round_trips_projection() {
        let mut app = VcsDocumentApp::new(SpaceApp);
        let before = app.projection().expect("projection").workflow.nodes.len();
        app.handle_action("commitCheckpoint", Some(&json!({ "message": "snapshot" })), &testkit::meta("local")).expect("commit");
        assert_eq!(app.projection().expect("projection").workflow.nodes.len(), before);
    }

    #[test]
    fn patch_parameter_action_updates_value() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, SpaceCommand::PatchParameter { parameter_id: "param-brush-size".into(), field: "value".into(), value: "48".into() });
        assert_eq!(emit.document_operations.len(), 1);
        let next = apply_operations(&projection, &emit.document_operations);
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
        let mut app = VcsDocumentApp::new(SpaceApp);
        let before = app.projection().expect("projection").workflow.nodes.len();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            SpaceCommand::SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 80.0, y: 80.0 },
            |app| app.projection().expect("projection").workflow.nodes.len(),
            before,
            before + 1,
        );
    }

    #[test]
    fn catalogue_tree_nests_apps_by_canonical_document() {
        seed_catalogue_apps();
        let config = SpaceConfig::default();
        let tree = build_catalogue_tree(resolve_labels::<SStudioLabels>(&config));
        let json = serde_json::to_string(&tree).unwrap();
        assert!(json.contains("s-play-catalogue.document.semio.puzzle.2d"));
        assert!(json.contains("s-play-catalogue.document.semio.puzzle.3d"));
        assert_eq!(json.matches("\"label\":\"puzzle\"").count(), 1);
    }

    #[test]
    fn patch_app_instances_updates_labels() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let ids: Vec<String> = projection.workflow.nodes.iter().take(2).map(|node| node.id.clone()).collect();
        let emit = studio_emit(&projection, &config, SpaceCommand::PatchAppInstances { node_ids: ids.clone(), field: "label".into(), value: "Batch Label".into() });
        let next = apply_operations(&projection, &emit.document_operations);
        let labels: Vec<String> = next.workflow.nodes.iter().filter(|node| ids.contains(&node.id)).map(|node| node.label.clone()).collect();
        assert!(labels.iter().all(|label| label == "Batch Label"));
    }

    #[test]
    fn open_and_close_focused_instance() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node_id = projection.workflow.nodes.first().expect("node").id.clone();
        let open_emit = studio_emit(&projection, &config, SpaceCommand::OpenInstance { node_id: Some(node_id.clone()) });
        assert!(open_emit.config_operations.contains(&SpaceConfigOperation::SetFocusedNode { node_id: Some(node_id.clone()) }));
        let config_after_open = apply_config(&config, &open_emit.config_operations);
        assert_eq!(config_after_open.focused_node_id.as_deref(), Some(node_id.as_str()));
        let close_emit = studio_emit(&projection, &config_after_open, SpaceCommand::CloseFocusedInstance);
        assert_eq!(close_emit.config_operations, vec![SpaceConfigOperation::SetFocusedNode { node_id: None }]);
    }

    #[test]
    fn inspector_tree_exposes_label_field() {
        let projection = demo_space_projection();
        let ids: Vec<String> = projection.workflow.nodes.iter().take(2).map(|node| node.id.clone()).collect();
        let config = SpaceConfig { selected_node_ids: ids, ..SpaceConfig::default() };
        let tree = build_inspector_tree(&projection, &config, resolve_labels::<SStudioLabels>(&config));
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

    #[test]
    fn spawns_puzzle5d_and_shooting_with_multi_port_registrations() {
        seed_multi_port_plugins();
        let mut projection = demo_space_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, SpaceCommand::SpawnApp { plugin_id: "puzzle.5d".into(), app_id: "puzzle5d".into(), x: 200.0, y: 100.0 });
        projection = apply_operations(&projection, &emit.document_operations);
        let emit = studio_emit(&projection, &config, SpaceCommand::SpawnApp { plugin_id: "shooting".into(), app_id: "shooting".into(), x: 300.0, y: 100.0 });
        projection = apply_operations(&projection, &emit.document_operations);
        let puzzle_node = projection.workflow.nodes.iter().find(|node| node.plugin_id == "puzzle.5d").expect("puzzle node");
        let shooting_node = projection.workflow.nodes.iter().find(|node| node.plugin_id == "shooting").expect("shooting node");
        // 🔌️ `AppIo::all_ports()` prepends the implicit `document:in`/`document:out` pair to every
        // app's declared ports now (the pre-merge `OsAppResourceSpec` had no such implicit pair).
        assert_eq!(puzzle_node.outputs.len(), 3, "document:out + out-a + out-b");
        assert_eq!(shooting_node.inputs.len(), 2, "document:in + scene-in");
    }

    #[test]
    fn unbind_parameter_field_removes_binding() {
        let mut projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node = projection.workflow.nodes.first().expect("node").clone();
        let parameter_id = parameter_entity_id(projection.parameters.first().expect("parameter")).to_string();
        let emit = studio_emit(&projection, &config, SpaceCommand::BindParameterField { node_id: node.id.clone(), field_path: "label".into(), parameter_id: parameter_id.clone() });
        projection = apply_operations(&projection, &emit.document_operations);
        assert!(projection.parameter_bindings.iter().any(|row| row.node_id == node.id && row.field_path == "label"));
        let emit = studio_emit(&projection, &config, SpaceCommand::UnbindParameterField { node_id: node.id.clone(), field_path: "label".into() });
        projection = apply_operations(&projection, &emit.document_operations);
        assert!(!projection.parameter_bindings.iter().any(|row| row.node_id == node.id && row.field_path == "label"));
    }

    #[test]
    fn checkout_checkpoint_restores_projection() {
        seed_draw_plugin();
        let mut app = VcsDocumentApp::new(SpaceApp);
        let before = app.projection().expect("projection").workflow.nodes.len();
        app.dispatch_typed(SpaceCommand::SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 80.0, y: 80.0 }, &testkit::meta("local")).expect("spawn");
        app.handle_action("commitCheckpoint", Some(&json!({ "message": "after-first-spawn" })), &testkit::meta("local")).expect("commit");
        let after_first = app.projection().expect("projection").workflow.nodes.len();
        assert!(after_first > before);
        let files = app.document_pack().expect("document pack");
        let parsed: store::ParsedDocumentText<OsProjection, OsOperation> = store::parse_document_pack(&files.pack, &files.spr).expect("parse document pack");
        let checkpoint_id = parsed.envelope.vcs.checkpoints[0].id.clone();
        app.dispatch_typed(SpaceCommand::SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 80.0, y: 80.0 }, &testkit::meta("local")).expect("spawn2");
        assert!(app.projection().expect("projection").workflow.nodes.len() > after_first);
        app.handle_action("checkoutCheckpoint", Some(&json!({ "checkpointId": checkpoint_id })), &testkit::meta("local")).expect("checkout");
        assert_eq!(app.projection().expect("projection").workflow.nodes.len(), after_first);
    }

    /// 🧪️ The definitional proof: two independent instances start from the same deterministic demo
    /// projection, apply DISJOINT edits (A spawns a new draw instance, B renames an existing
    /// instance), and exchanging operations over a backbone converges both sides onto the same
    /// projection — impossible under whole-document `setDocument` snapshots, where one side's write
    /// would clobber the other's.
    #[test]
    fn two_instances_converge_on_disjoint_edits_via_backbone() {
        seed_draw_plugin();
        let node_id = demo_space_projection().workflow.nodes.first().expect("node").id.clone();
        let rename_id = node_id.clone();
        testkit::assert_two_instances_converge::<SpaceApp, (usize, bool)>(
            "mem://s-studio-convergence",
            SpaceCommand::SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 80.0, y: 80.0 },
            SpaceCommand::PatchAppInstances { node_ids: vec![node_id.clone()], field: "label".into(), value: "Renamed".into() },
            move |app| {
                let projection = app.projection().expect("projection");
                let draw_count = projection.workflow.nodes.iter().filter(|node| node.plugin_id == "draw").count();
                let renamed = projection.workflow.nodes.iter().find(|node| node.id == rename_id).map(|node| node.label == "Renamed").unwrap_or(false);
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
        let mut app = VcsDocumentApp::new(SpaceApp);
        let node = app.render(S_PLAY_BODY_WORKFLOW, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#"\"engine\":\"flow\""#));
        assert!(json.contains("fixtureJson"));
        assert!(json.contains(r#"\"schema\":\"flow.fixture\""#));
    }

    #[test]
    fn node_graph_edit_set_fixture_moves_node_and_persists_camera() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let node = projection.workflow.nodes.first().expect("node").clone();
        let camera = OsWorkflowCamera { x: 40.0, y: -20.0, zoom: 2.0 };
        let mut fixture = os_workflow_to_flow_fixture(&projection.workflow, &camera);
        fixture["layout"][&node.id] = json!({ "x": 500.0 + node.width / 2.0, "y": 300.0 + node.height / 2.0 });
        let operations_json = json!({ "operations": [{ "operation": "setFixture", "fixtureJson": fixture.to_string() }] }).to_string();
        let emit = studio_emit(&projection, &config, SpaceCommand::NodeGraphEdit { operations_json });
        let moved = apply_operations(&projection, &emit.document_operations)
            .workflow
            .nodes
            .into_iter()
            .find(|row| row.id == node.id)
            .expect("node");
        assert!((moved.x - 500.0).abs() < 0.01);
        assert!((moved.y - 300.0).abs() < 0.01);
        assert_eq!(emit.config_operations, vec![SpaceConfigOperation::SetCamera { window_id: S_PLAY_WINDOW_WORKFLOW.into(), camera: camera.into() }]);
    }

    #[test]
    fn node_graph_viewport_persists_camera() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, SpaceCommand::NodeGraphViewport { viewport_json: r#"{"x":7.0,"y":9.0,"zoom":0.5}"#.into() });
        assert_eq!(emit.config_operations, vec![SpaceConfigOperation::SetCamera { window_id: S_PLAY_WINDOW_WORKFLOW.into(), camera: OsWorkflowCamera { x: 7.0, y: 9.0, zoom: 0.5 }.into() }]);
    }

    #[test]
    fn presence_heartbeat_publishes_peer_for_other_clients() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let first_node_id = projection.workflow.nodes[0].id.clone();
        let select_emit = studio_emit(&projection, &config, SpaceCommand::NodeGraphSelect { node_ids: vec![first_node_id], select_all: false });
        let config_after_select = apply_config(&config, &select_emit.config_operations);
        let _ = studio_emit(&projection, &config_after_select, SpaceCommand::PresenceHeartbeat { client_id: "client-test-a".into(), name: "Ada".into() });
        let other_config = SpaceConfig { client_id: Some("client-test-b".into()), space_id: config_after_select.space_id.clone(), ..SpaceConfig::default() };
        let peers = presence_peers_json(&other_config);
        assert!(peers.contains("client-test-a"));
        assert!(peers.contains("Ada"));
        assert!(peers.contains(r#""selectionCount":1"#));
        let self_config = SpaceConfig { client_id: Some("client-test-a".into()), ..config_after_select };
        let self_view = presence_peers_json(&self_config);
        assert!(!self_view.contains("client-test-a"));
    }

    /// 🐢️ Perf round 3: a heartbeat only records this client's own identity for the presence broadcast
    /// — it must declare `None` so it never triggers a full-shell `refresh-ui` for the sending client.
    #[test]
    fn presence_heartbeat_declares_none_ui_scope() {
        use semio_framework_core::kernel::UiDirtyScope;
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let emit = studio_emit(&projection, &config, SpaceCommand::PresenceHeartbeat { client_id: "client-test-c".into(), name: "Cass".into() });
        assert!(matches!(emit.ui_scope, UiDirtyScope::None), "presenceHeartbeat must declare None, got {:?}", emit.ui_scope);
    }

    #[test]
    fn space_labels_resolve_native_english_by_default() {
        let projection = demo_space_projection();
        let history = empty_history();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = SpaceConfig::default();
        let cfg = ConfigView { projection: &config };
        let app = SpaceApp;
        let catalogue_json = serde_json::to_string(&app.render(S_PLAY_CATALOGUE_BODY_KEY, &doc, &cfg)).unwrap();
        assert!(catalogue_json.contains("\"Apps\""));

        let parameters_json = serde_json::to_string(&app.render(S_PLAY_PARAMETERS_BODY_KEY, &doc, &cfg)).unwrap();
        assert!(parameters_json.contains("Add Parameter"));
        assert!(parameters_json.contains("\"Name\""));
        assert!(parameters_json.contains("\"Remove\""));
        assert!(!parameters_json.contains("Parameter hinzufügen"));
    }

    #[test]
    fn space_labels_resolve_native_german_locale() {
        let projection = demo_space_projection();
        let history = empty_history();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = SpaceConfig { locale: "de".into(), ..SpaceConfig::default() };
        let cfg = ConfigView { projection: &config };
        let app = SpaceApp;
        let parameters_json = serde_json::to_string(&app.render(S_PLAY_PARAMETERS_BODY_KEY, &doc, &cfg)).unwrap();
        assert!(parameters_json.contains("Parameter hinzufügen"));
        assert!(parameters_json.contains("\"Entfernen\""));
        assert!(!parameters_json.contains("Add Parameter"));

        let inspector_json = serde_json::to_string(&app.render(S_PLAY_INSPECTOR_BODY_KEY, &doc, &cfg)).unwrap();
        assert!(inspector_json.contains("Wähle Workflow-Knoten im Arbeitsbereich aus."));
    }

    /// 🌉️ Exercises BOTH apps together (Home's `createStudio` followed by Space's `openSpace`) — this
    /// crate already regular-depends on `home_ui`, so the integration test lives here instead of
    /// requiring a new dev-dependency cycle back from `home_ui` onto this crate.
    #[test]
    fn create_space_navigates_without_download_and_opens_empty() {
        let home = home_ui::HomeApp;
        let home_projection = home.initial_projection();
        let history = empty_history();
        let doc = DocumentView { projection: &home_projection, history: &history };
        let home_config = home_engine::HomeConfig::default();
        let home_cfg = ConfigView { projection: &home_config };
        let emit = home.handle(&home_protocol::HomeCommand::CreateStudio { name: "Fresh Studio".into(), kind: "catalog".into(), folder_path: None }, &doc, &home_cfg);
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
        assert!(document.vcs.initial_projection.workflow.nodes.is_empty());

        let empty = default_os_projection();
        let studio_doc = DocumentView { projection: &empty, history: &history };
        let studio_config = SpaceConfig::default();
        let studio_cfg = ConfigView { projection: &studio_config };
        let studio = SpaceApp;
        let open = studio.handle(&SpaceCommand::OpenSpace { space_id: space_id.to_string() }, &studio_doc, &studio_cfg);
        assert!(open.effects.iter().any(|effect| matches!(effect, HostEffect::LoadDocument { .. })), "openSpace must load the created studio");
        assert!(!open.effects.iter().any(|effect| matches!(effect, HostEffect::Navigate { .. })));
        assert!(!open.effects.iter().any(|effect| matches!(effect, HostEffect::DownloadMediaExport { .. })));
    }
}
//#endregion 🧪️Tests
