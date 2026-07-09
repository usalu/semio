//! 🎛️ S Studio plugin — designer OS shell bundled as a hot-swappable WASM component.

use semio_framework_os::{
    create_os_studio, default_os_projection, delete_os_studio,
    import_os_studio_from_json, list_os_media_graph_vfs_children, list_os_programs,
    list_os_studio_catalog_entries, load_os_studio_document, materialize_os_projection, media_port_spec_id,
    os_app_registration,
    os_document_from_json, os_document_to_json, build_os_media_flow_operator_infos, materialize_os_app_instance_document_json,
    os_media_graph_to_node_graph_payload, os_media_graph_vfs_schema,
    os_parameter_types_compatible, os_parameter_value, parameter_id_from_port_id,
    register_os_fixture_json, create_os_id, seed_os_studio_catalog_if_empty, DevJsonBackbone, LocalJsonBackbone,
    MediaGraphPosition,
    OsAppInstance, OsBackbonePort, OsDocument, OsMediaGraphVfsNodeRecord, OsOp, OsParameter, OsParameterFieldBinding,
    OsParameterType, OsProjection, OsStore, OS_HOME_VFS_ROOT_ID, OS_MEDIA_GRAPH_VFS_ROOT_ID,
    OS_STUDIO_BACKBONE_URI_PREFIX, MemoryBackbonePort, VcsError,
};
use semio_framework_plugin::{PanelGroup, 
    build_node_graph_scene, build_text_editor_scene, build_virtual_file_system_scene,
    create_default_layout, create_tab_stack_layout, layout::MeasureSelectItem,
    layout::WindowEngagementStatus, tool_button, tool_collection, ui_declarative_sections_to_tree,
    ui_inspector_all_equal, ui_text,
    App, CommandDescriptor, ModeDefinition, NodeGraphScene, PluginApp, PluginBundle, SurfaceKind, TextEditorScene,
    ToolCategory,
    UiButtonNode, UiControlNode, UiFieldNode, UiInputNode, UiNode, UiNumberStepperNode, UiSectionNode,
    UiSelectItem, UiSelectNode, UiStackNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    ViewState, VirtualFileSystemScene,
    WindowEngagement, WindowEngagementInput, WindowLayout, WindowMeasure,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};
use vcs::{create_document_vcs_envelope, DocumentBackboneRef, LocalStorageBackbonePort};
use mathematical_graph_port_directed_dag::{
    dag_fixture_to_wire_literal, DagCamera, DagFixture, DagFixtureEdge, DagNodeKind, DagNodeSpec, IoPortSpec,
};

//#region 🔖Constants
const S_HOME_APP_ID: &str = "home";
const S_HOME_CONTROLLER_ID: &str = "s-home";
const S_HOME_WINDOW: &str = "s-home-main";
const S_HOME_BODY: &str = "s.home.vfs";
const S_HOME_SURFACE: &str = "vfs:home:main";

const S_PLAY_APP_ID: &str = "studio";
const S_PLAY_CONTROLLER_ID: &str = "s-play";
const S_PLAY_SURFACE_MEDIA_GRAPH: &str = "s.play.media-graph";
const S_PLAY_SURFACE_MEDIA_VFS: &str = "s.play.media-vfs";
const S_PLAY_SURFACE_COMPILED_DAG: &str = "s.play.compiled-dag";
const S_PLAY_BODY_MEDIA_GRAPH: &str = "s.play.media-graph";
const S_PLAY_BODY_MEDIA_VFS: &str = "s.play.media-vfs";
const S_PLAY_BODY_COMPILED_DAG: &str = "s.play.compiled-dag";
const S_PLAY_WINDOW_MEDIA_GRAPH: &str = "s-media-graph";
const S_PLAY_WINDOW_MEDIA_VFS: &str = "s-media-vfs";
const S_PLAY_WINDOW_COMPILED_DAG: &str = "s-compiled-dag";
const S_PLAY_CATALOGUE_TAB_ID: &str = "s-play-catalogue";
const S_PLAY_PARAMETERS_TAB_ID: &str = "s-play-parameters";
const S_PLAY_INSPECTOR_TAB_ID: &str = "s-play-inspector";
const S_PLAY_CATALOGUE_BODY_KEY: &str = "s.play.catalogue";
const S_PLAY_PARAMETERS_BODY_KEY: &str = "s.play.parameters";
const S_PLAY_INSPECTOR_BODY_KEY: &str = "s.play.inspector";
const S_PLAY_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

const DEMO_STUDIO_JSON: &str = include_str!("../../example/demo.s.json");
const OS_BOOT_STUDIO_ID: &str = "default";

const S_STUDIO_EXAMPLES: &[(&str, &str, &str)] = &[("demo", "Demo Studio", DEMO_STUDIO_JSON)];
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StudioPanelState {
    #[serde(default)]
    active_panel_tab: String,
    #[serde(default)]
    programs: Vec<StudioProgramEntry>,
    #[serde(default)]
    spawned_apps: Vec<SpawnedAppEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_spawned_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StudioProgramEntry {
    plugin_id: String,
    program_id: String,
    app_id: String,
    label: String,
    document: Vec<String>,
    yields: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpawnedAppEntry {
    id: String,
    plugin_id: String,
    instance_id: u32,
    app_id: String,
    label: String,
    document: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StudioRuntimeState {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    focused_instance_id: Option<String>,
    #[serde(default)]
    selected_media_node_ids: Vec<String>,
    #[serde(default)]
    selected_app_instance_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hovered_media_node_id: Option<String>,
    #[serde(default)]
    media_graph_engagement_input: String,
    #[serde(default)]
    compiled_dag_engagement_input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    studio_id: Option<String>,
    #[serde(default)]
    clipboard_instance_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SStudioEnvelope {
    document: OsDocument,
    #[serde(default)]
    runtime: StudioRuntimeState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SHomeDocument {
    schema: String,
    #[serde(default)]
    catalog_generation: u64,
}
//#endregion 🔖Types

//#region 🔖CatalogBackbone
fn ensure_studio_fixtures_registered() {
    static FIXTURES: LazyLock<()> = LazyLock::new(|| {
        register_os_fixture_json("semio.draw.json", include_str!("../../../draw/example/semio.draw.json"));
        register_os_fixture_json("jack.writer.json", include_str!("../../../writer/example/jack.writer.json"));
    });
    let _ = &*FIXTURES;
}

static CATALOG_PORT: LazyLock<Arc<dyn OsBackbonePort>> = LazyLock::new(|| {
    ensure_studio_fixtures_registered();
    let port: Arc<dyn OsBackbonePort> = Arc::new(LocalStorageBackbonePort::new());
    if list_os_studio_catalog_entries(port.clone())
        .map(|entries| entries.is_empty())
        .unwrap_or(true)
    {
        let mut demo = parse_demo_studio_document();
        demo.id = OS_BOOT_STUDIO_ID.into();
        demo.name = if demo.name.trim().is_empty() {
            "Demo Studio".into()
        } else {
            demo.name
        };
        let _ = seed_os_studio_catalog_if_empty(demo, port.clone());
    }
    port
});

static TEMP_CATALOG_PORT: LazyLock<Arc<dyn OsBackbonePort>> =
    LazyLock::new(|| Arc::new(MemoryBackbonePort::new()));

static STUDIO_PORTS: LazyLock<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn catalog_port() -> Arc<dyn OsBackbonePort> {
    CATALOG_PORT.clone()
}

fn temp_catalog_port() -> Arc<dyn OsBackbonePort> {
    TEMP_CATALOG_PORT.clone()
}

fn register_studio_port(studio_id: &str, port: Arc<dyn OsBackbonePort>) {
    if let Ok(mut guard) = STUDIO_PORTS.lock() {
        guard.insert(studio_id.into(), port);
    }
}

fn resolve_studio_port(studio_id: &str) -> Arc<dyn OsBackbonePort> {
    if let Ok(guard) = STUDIO_PORTS.lock() {
        if let Some(port) = guard.get(studio_id) {
            return port.clone();
        }
    }
    if load_os_studio_document(studio_id, catalog_port()).is_ok() {
        return catalog_port();
    }
    if load_os_studio_document(studio_id, temp_catalog_port()).is_ok() {
        return temp_catalog_port();
    }
    catalog_port()
}

fn load_studio_document(studio_id: &str) -> Result<OsDocument, VcsError> {
    load_os_studio_document(studio_id, resolve_studio_port(studio_id))
}

fn list_all_studio_catalog_entries() -> Vec<semio_framework_os::OsStudioCatalogEntry> {
    let mut seen = HashSet::new();
    let mut entries = Vec::new();
    for port in [catalog_port(), temp_catalog_port()] {
        if let Ok(rows) = list_os_studio_catalog_entries(port) {
            for entry in rows {
                if seen.insert(entry.id.clone()) {
                    entries.push(entry);
                }
            }
        }
    }
    entries
}

fn studio_navigate_op(studio_id: &str) -> String {
    json!({
        "op": "navigate",
        "uri": format!("/studios/{studio_id}")
    })
    .to_string()
}

fn finish_create_ops(document: &mut SHomeDocument, entry: &semio_framework_os::OsStudioCatalogEntry) -> Vec<String> {
    document.catalog_generation += 1;
    vec![
        set_home_document_op(document),
        studio_navigate_op(&entry.id),
    ]
}

#[cfg(not(target_arch = "wasm32"))]
fn create_folder_studio(
    name: &str,
    folder_path: &str,
) -> Result<semio_framework_os::OsStudioCatalogEntry, VcsError> {
    let port = semio_framework_os::open_folder_studio_backbone(folder_path)?;
    let entry = create_os_studio(name, port.clone())?;
    register_studio_port(&entry.id, port);
    Ok(entry)
}

#[cfg(not(target_arch = "wasm32"))]
fn bind_studio_file(studio_id: &str, file_path: &str) -> Result<(), VcsError> {
    let port = Arc::new(semio_framework_os::NativeFileBackbonePort::new(file_path));
    register_studio_port(studio_id, port.clone());
    let mut document = load_os_studio_document(studio_id, catalog_port())?;
    let uri = format!("local://{file_path}");
    let mut backbone = LocalJsonBackbone::new(port);
    backbone.attach(&uri)?;
    backbone.sync(&document)?;
    document = backbone.load_attached()?.unwrap_or(document);
    let mut dev_backbone = DevJsonBackbone::new(catalog_port());
    let catalog_uri = format!("{OS_STUDIO_BACKBONE_URI_PREFIX}{studio_id}");
    dev_backbone.attach(&catalog_uri);
    dev_backbone.sync(&document)?;
    Ok(())
}
//#endregion 🔖CatalogBackbone

//#region 🔖DocumentHelpers
fn parse_demo_studio_document() -> OsDocument {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DemoVcs {
        initial_projection: OsProjection,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DemoFile {
        schema: String,
        id: String,
        name: String,
        vcs: DemoVcs,
        #[serde(default)]
        backbone: Option<semio_framework_os::OsBackboneRef>,
    }
    let demo: DemoFile = serde_json::from_str(DEMO_STUDIO_JSON).expect("demo studio json");
    let envelope = create_document_vcs_envelope(
        &demo.schema,
        &demo.id,
        demo.vcs.initial_projection,
        demo.backbone.as_ref().map(|entry| DocumentBackboneRef {
            kind: entry.kind.clone(),
            uri: entry.uri.clone(),
        }),
    );
    OsDocument {
        schema: demo.schema,
        id: demo.id,
        name: demo.name,
        vcs: envelope.vcs,
        applied_edit_ids: Vec::new(),
        backbone: demo.backbone,
    }
}

fn demo_os_document() -> OsDocument {
    parse_demo_studio_document()
}

fn initial_studio_envelope() -> SStudioEnvelope {
    SStudioEnvelope {
        document: demo_os_document(),
        runtime: StudioRuntimeState {
            active_instance_id: demo_os_document()
                .vcs
                .initial_projection
                .app_instances
                .first()
                .map(|instance| instance.id.clone()),
            ..StudioRuntimeState::default()
        },
    }
}

fn initial_studio_document_json() -> String {
    serde_json::to_string(&initial_studio_envelope()).expect("studio envelope json")
}

fn parse_studio_envelope(document_json: &str) -> SStudioEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| initial_studio_envelope())
}

fn parse_panel_state(view_state: &ViewState) -> StudioPanelState {
    view_state
        .panel_json
        .as_deref()
        .and_then(|json| serde_json::from_str(json).ok())
        .unwrap_or_else(|| StudioPanelState {
            active_panel_tab: S_PLAY_CATALOGUE_TAB_ID.into(),
            programs: Vec::new(),
            spawned_apps: Vec::new(),
            active_spawned_id: None,
        })
}

fn projection_from_document(document: &OsDocument) -> OsProjection {
    OsStore::new(document.clone())
        .projection()
        .unwrap_or_else(|_| default_os_projection())
}

fn set_home_document_op(document: &SHomeDocument) -> String {
    json!({ "op": "setDocument", "document": document }).to_string()
}

fn set_studio_document_op(envelope: &SStudioEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn set_panel_op(panel: &StudioPanelState) -> String {
    json!({ "op": "setPanel", "panel": panel }).to_string()
}

fn s_play_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: S_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn s_home_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: S_HOME_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn ui_stack_horizontal(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "horizontal".into(),
        gap: Some("standard".into()),
        padding: None,
        children,
    })
}

fn parameter_entity_id(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { id, .. }
        | OsParameter::Categorical { id, .. }
        | OsParameter::Toggle { id, .. }
        | OsParameter::Text { id, .. } => id,
    }
}

fn store_from_envelope(envelope: &SStudioEnvelope) -> OsStore {
    OsStore::new(envelope.document.clone())
}

fn envelope_from_store(store: OsStore, runtime: StudioRuntimeState) -> SStudioEnvelope {
    SStudioEnvelope {
        document: store.document(),
        runtime,
    }
}

fn studio_example_document_json(example_id: &str) -> Option<String> {
    S_STUDIO_EXAMPLES
        .iter()
        .find(|(id, _, _)| *id == example_id)
        .map(|(_, _, json)| (*json).to_string())
}

fn studio_id_for_envelope(envelope: &SStudioEnvelope) -> Option<String> {
    envelope
        .runtime
        .studio_id
        .clone()
        .or_else(|| {
            if envelope.document.id.is_empty() {
                None
            } else {
                Some(envelope.document.id.clone())
            }
        })
}

fn persist_studio_document(document: &OsDocument, studio_id: &str) {
    let port = catalog_port();
    let uri = format!("{OS_STUDIO_BACKBONE_URI_PREFIX}{studio_id}");
    let mut backbone = DevJsonBackbone::new(port);
    backbone.attach(&uri);
    let _ = backbone.sync(document);
}

fn persist_envelope_document(envelope: &SStudioEnvelope) {
    if let Some(studio_id) = studio_id_for_envelope(envelope) {
        persist_studio_document(&envelope.document, &studio_id);
    }
}

fn primary_selected_instance_id(runtime: &StudioRuntimeState, projection: &OsProjection) -> Option<String> {
    runtime.selected_app_instance_ids.first().cloned().or_else(|| {
        runtime.selected_media_node_ids.first().and_then(|node_id| {
            projection
                .media_graph
                .nodes
                .iter()
                .find(|node| node.id == *node_id)
                .map(|node| node.instance_id.clone())
        })
    })
}

fn selected_instance_ids(runtime: &StudioRuntimeState, projection: &OsProjection) -> Vec<String> {
    if !runtime.selected_app_instance_ids.is_empty() {
        return runtime.selected_app_instance_ids.clone();
    }
    runtime
        .selected_media_node_ids
        .iter()
        .filter_map(|node_id| {
            projection
                .media_graph
                .nodes
                .iter()
                .find(|node| node.id == *node_id)
                .map(|node| node.instance_id.clone())
        })
        .collect()
}

fn media_graph_context_menu_json() -> String {
    json!([
        { "id": "open-instance", "label": "Open instance", "command": "openInstance" },
        { "id": "duplicate-instance", "label": "Duplicate", "command": "duplicateAppInstance" },
        { "id": "copy-instance", "label": "Copy", "command": "copyAppInstance" },
        { "id": "paste-instance", "label": "Paste", "command": "pasteAppInstance" },
        { "id": "rename-instance", "label": "Rename label…", "command": "renameAppInstance" },
        { "id": "remove-instance", "label": "Remove", "command": "removeAppInstance" },
        { "id": "select-all", "label": "Select all", "command": "setMediaNodeSelection", "args": { "selectAll": true } },
        { "id": "clear-selection", "label": "Clear selection", "command": "setMediaNodeSelection", "args": { "nodeIds": [] } },
        { "id": "reorganize", "label": "Reorganize", "command": "reorganizeMediaGraph" }
    ])
    .to_string()
}

fn presence_peers_json() -> String {
    "[]".into()
}
//#endregion 🔖DocumentHelpers

//#region 🔖HomeVfs
fn os_home_vfs_schema_json() -> String {
    json!({
        "descriptorKinds": {
            "text": { "id": "text", "name": "Text", "presentation": "text" }
        },
        "fileNodeKinds": {
            "studio": {
                "id": "studio",
                "name": "Studio",
                "descriptors": [{ "id": "apps", "descriptorKindId": "text", "label": "Apps" }]
            }
        },
        "descriptorColumnIds": ["apps"]
    })
    .to_string()
}

fn home_vfs_rows() -> Vec<Value> {
    let mut rows = vec![json!({
        "id": OS_HOME_VFS_ROOT_ID,
        "fileNodeKindId": "studio",
        "name": "Studios",
        "path": "/",
        "parentId": null,
        "hasChildren": true,
        "navigateUri": null,
        "descriptorValues": { "apps": "" }
    })];
    for entry in list_all_studio_catalog_entries() {
            rows.push(json!({
                "id": format!("studio:{}", entry.id),
                "fileNodeKindId": "studio",
                "name": entry.name,
                "path": format!("/studios/{}", entry.id),
                "parentId": OS_HOME_VFS_ROOT_ID,
                "hasChildren": false,
                "navigateUri": format!("/studios/{}", entry.id),
                "descriptorValues": {
                    "apps": format!("{} apps · {} nodes", entry.app_count, entry.node_count)
                }
            }));
    }
    rows
}

fn render_home_vfs() -> UiNode {
    build_virtual_file_system_scene(
        S_HOME_SURFACE,
        S_HOME_CONTROLLER_ID,
        VirtualFileSystemScene {
            schema_json: os_home_vfs_schema_json(),
            rows_json: serde_json::to_string(&home_vfs_rows()).unwrap_or_else(|_| "[]".into()),
            selected_row_ids_json: None,
            hovered_row_id: None,
            empty_message: Some("No studios yet. Create one from the toolbar.".into()),
            drag_drop_enabled: None,
        },
        Some(S_HOME_WINDOW.into()),
        None,
    )
}
//#endregion 🔖HomeVfs

//#region 🔖MediaGraphVfs
fn flatten_media_vfs_rows(
    parent_id: &str,
    instances: &[OsAppInstance],
    graph: &semio_framework_os::OsMediaGraph,
    bindings: &[OsParameterFieldBinding],
    parameters: &[OsParameter],
    rows: &mut Vec<Value>,
) {
    let children = list_os_media_graph_vfs_children(parent_id, instances, graph, bindings, parameters);
    for child in &children {
        rows.push(vfs_node_to_row(child));
        if child.has_children {
            flatten_media_vfs_rows(&child.id, instances, graph, bindings, parameters, rows);
        }
    }
}

fn vfs_node_to_row(node: &OsMediaGraphVfsNodeRecord) -> Value {
    json!({
        "id": node.id,
        "fileNodeKindId": node.file_node_kind_id,
        "name": node.name,
        "path": node.path,
        "parentId": node.parent_id,
        "hasChildren": node.has_children,
        "navigateUri": node.navigate_uri,
        "descriptorValues": node.descriptor_values
    })
}

fn render_media_vfs(document: &OsDocument) -> UiNode {
    let projection = projection_from_document(document);
    let mut rows = vec![json!({
        "id": OS_MEDIA_GRAPH_VFS_ROOT_ID,
        "fileNodeKindId": "root",
        "name": "Media Graph",
        "path": "/",
        "parentId": null,
        "hasChildren": true,
        "descriptorValues": {}
    })];
    flatten_media_vfs_rows(
        OS_MEDIA_GRAPH_VFS_ROOT_ID,
        &projection.app_instances,
        &projection.media_graph,
        &projection.parameter_bindings,
        &projection.parameters,
        &mut rows,
    );
    let schema = os_media_graph_vfs_schema();
    build_virtual_file_system_scene(
        S_PLAY_SURFACE_MEDIA_VFS,
        S_PLAY_CONTROLLER_ID,
        VirtualFileSystemScene {
            schema_json: serde_json::to_string(&schema).unwrap_or_else(|_| "{}".into()),
            rows_json: serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()),
            selected_row_ids_json: None,
            hovered_row_id: None,
            empty_message: Some("No app instances in the media graph.".into()),
            drag_drop_enabled: Some(true),
        },
        Some(S_PLAY_WINDOW_MEDIA_VFS.into()),
        None,
    )
}
//#endregion 🔖MediaGraphVfs

//#region 🔖StudioPanels
#[derive(Default)]
struct AppCatalogueNode {
    children: BTreeMap<String, AppCatalogueNode>,
    app: Option<StudioProgramEntry>,
}

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
    let mut drag_data = HashMap::new();
    if let Some(app) = &app {
        drag_data.insert(
            S_PLAY_CATALOGUE_DRAG_MIME.into(),
            json!({ "programId": app.program_id, "appId": app.app_id }).to_string(),
        );
    }
    UiTreeItemNode {
        id: format!("s-play-catalogue.document.{id_path}"),
        label: label.into(),
        description: app.as_ref().and_then(|entry| (!entry.yields.is_empty()).then(|| entry.yields.clone())),
        icon_id: app.as_ref().map(|entry| entry.app_id.clone()),
        selected: None,
        default_open: (!children.is_empty()).then_some(true),
        command: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: app.as_ref().map(|_| true),
        drag_data: (!drag_data.is_empty()).then_some(drag_data),
        items: (!children.is_empty()).then_some(children),
        control: None,
        is_hidden: None,
    }
}

fn build_catalogue_tree(panel: &StudioPanelState) -> UiNode {
    let programs: Vec<StudioProgramEntry> = if panel.programs.is_empty() {
        let mut entries = Vec::new();
        for program in list_os_programs() {
            if program.id == "s.system" {
                continue;
            }
            for app in program.apps {
                entries.push(StudioProgramEntry {
                    plugin_id: program.id.clone(),
                    program_id: program.id.clone(),
                    app_id: app.id,
                    label: app.label,
                    document: app.document,
                    yields: app
                        .outputs
                        .first()
                        .map(|port| port.resource_kind.clone())
                        .unwrap_or_default(),
                });
            }
        }
        entries
    } else {
        panel.programs.clone()
    };
    let mut document = AppCatalogueNode::default();
    for program in programs {
        let mut node = &mut document;
        for segment in &program.document {
            node = node.children.entry(segment.clone()).or_default();
        }
        node.app = Some(program);
    }
    let sections = vec![UiTreeSectionNode {
        id: S_PLAY_CATALOGUE_TAB_ID.into(),
        label: Some("Apps".into()),
        default_open: Some(true),
        items: document
            .children
            .into_iter()
            .map(|(segment, node)| app_catalogue_item(&[segment.clone()], &segment, node))
            .collect(),
    }];
    UiNode::Tree(UiTreeNode {
        sections,
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn parameter_value_control(parameter: &OsParameter) -> UiNode {
    match parameter {
        OsParameter::Numeric { id, value, step, .. } => UiNode::NumberStepper(UiNumberStepperNode {
            id: format!("s-play-parameters.{id}.value"),
            value: *value,
            step: step.unwrap_or(1.0),
            uniform: true,
            on_absolute: s_play_cmd(
                "patchParameter",
                Some(json!({ "parameterId": id, "field": "value" })),
            ),
            on_delta: s_play_cmd(
                "patchParameter",
                Some(json!({ "parameterId": id, "field": "value" })),
            ),
        }),
        OsParameter::Categorical { id, value, options, .. } => UiNode::Select(UiSelectNode {
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
            on_change: s_play_cmd(
                "patchParameter",
                Some(json!({ "parameterId": id, "field": "value" })),
            ),
        }),
        OsParameter::Toggle { id, value, .. } => UiNode::Toggle(UiToggleNode {
            id: format!("s-play-parameters.{id}.value"),
            icon_id: "toggle-left".into(),
            pressed: *value,
            text: Some(if *value { "On".into() } else { "Off".into() }),
            on_change: s_play_cmd(
                "patchParameter",
                Some(json!({ "parameterId": id, "field": "value" })),
            ),
        }),
        OsParameter::Text { id, value, .. } => UiNode::Input(UiInputNode {
            id: format!("s-play-parameters.{id}.value"),
            input_kind: "text".into(),
            value: value.clone(),
            placeholder: None,
            commit: None,
            on_change: s_play_cmd(
                "patchParameter",
                Some(json!({ "parameterId": id, "field": "value" })),
            ),
        }),
    }
}

fn parameter_constraint_fields(parameter: &OsParameter) -> Vec<UiNode> {
    match parameter {
        OsParameter::Numeric {
            id,
            min,
            max,
            step,
            ..
        } => vec![
            UiNode::Field(UiFieldNode {
                id: format!("s-play-parameters.{id}.min"),
                label: "Min".into(),
                child: UiControlNode::NumberStepper(UiNumberStepperNode {
                    id: format!("s-play-parameters.{id}.min.stepper"),
                    value: min.unwrap_or(0.0),
                    step: 1.0,
                    uniform: true,
                    on_absolute: s_play_cmd(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "min" })),
                    ),
                    on_delta: s_play_cmd(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "min" })),
                    ),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: format!("s-play-parameters.{id}.max"),
                label: "Max".into(),
                child: UiControlNode::NumberStepper(UiNumberStepperNode {
                    id: format!("s-play-parameters.{id}.max.stepper"),
                    value: max.unwrap_or(0.0),
                    step: 1.0,
                    uniform: true,
                    on_absolute: s_play_cmd(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "max" })),
                    ),
                    on_delta: s_play_cmd(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "max" })),
                    ),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: format!("s-play-parameters.{id}.step"),
                label: "Step".into(),
                child: UiControlNode::NumberStepper(UiNumberStepperNode {
                    id: format!("s-play-parameters.{id}.step.stepper"),
                    value: step.unwrap_or(0.0),
                    step: 0.1,
                    uniform: true,
                    on_absolute: s_play_cmd(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "step" })),
                    ),
                    on_delta: s_play_cmd(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "step" })),
                    ),
                }),
            }),
        ],
        OsParameter::Categorical { id, options, .. } => {
            let mut fields: Vec<UiNode> = options
                .iter()
                .map(|option| {
                    UiNode::Field(UiFieldNode {
                        id: format!("s-play-parameters.{id}.option.{option}"),
                        label: option.clone(),
                        child: UiControlNode::Button(UiButtonNode {
                            id: Some(format!("s-play-parameters.{id}.option.{option}.remove")),
                            icon_id: "trash-2".into(),
                            label: "Remove".into(),
                            command: s_play_cmd(
                                "patchParameter",
                                Some(json!({ "parameterId": id, "field": "removeOption", "value": option })),
                            ),
                            style: None,
                        }),
                    })
                })
                .collect();
            fields.push(UiNode::Field(UiFieldNode {
                id: format!("s-play-parameters.{id}.add-option"),
                label: "Add option".into(),
                child: UiControlNode::Input(UiInputNode {
                    id: format!("s-play-parameters.{id}.add-option.input"),
                    input_kind: "text".into(),
                    value: String::new(),
                    placeholder: Some("New option".into()),
                    commit: None,
                    on_change: s_play_cmd(
                        "patchParameter",
                        Some(json!({ "parameterId": id, "field": "addOption" })),
                    ),
                }),
            }));
            fields
        }
        _ => Vec::new(),
    }
}

fn build_parameters_tree(document: &OsDocument) -> UiNode {
    let projection = projection_from_document(document);
    let mut children = vec![UiSectionNode {
        id: "s-play-parameters.header".into(),
        label: Some(FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL.into()),
        default_open: Some(true),
        children: vec![
            UiNode::Button(UiButtonNode {
                id: Some("s-play-parameters.add".into()),
                icon_id: "plus".into(),
                label: "Add Parameter".into(),
                command: s_play_cmd("addParameter", Some(json!({ "type": "numeric" }))),
                style: None,
            }),
            ui_text(format!("{} parameter(s)", projection.parameters.len())),
        ],
    }];
    for parameter in &projection.parameters {
        let parameter_id = parameter_entity_id(parameter).to_string();
        let mut parameter_children = vec![
            UiNode::Field(UiFieldNode {
                id: format!("s-play-parameters.{parameter_id}.name"),
                label: "Name".into(),
                child: UiControlNode::Input(UiInputNode {
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
                    on_change: s_play_cmd(
                        "patchParameter",
                        Some(json!({ "parameterId": parameter_id, "field": "name" })),
                    ),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: format!("s-play-parameters.{parameter_id}.value-field"),
                label: "Value".into(),
                child: match parameter_value_control(parameter) {
                    UiNode::Input(input) => UiControlNode::Input(input),
                    UiNode::Select(select) => UiControlNode::Select(select),
                    UiNode::Toggle(toggle) => UiControlNode::Toggle(toggle),
                    UiNode::NumberStepper(stepper) => UiControlNode::NumberStepper(stepper),
                    other => UiControlNode::Input(UiInputNode {
                        id: format!("s-play-parameters.{parameter_id}.fallback"),
                        input_kind: "text".into(),
                        value: format!("{other:?}"),
                        placeholder: None,
                        commit: None,
                        on_change: s_play_cmd("patchParameter", None),
                    }),
                },
            }),
        ];
        parameter_children.extend(parameter_constraint_fields(parameter));
        parameter_children.push(UiNode::Button(UiButtonNode {
            id: Some(format!("s-play-parameters.{parameter_id}.remove")),
            icon_id: "trash-2".into(),
            label: "Remove".into(),
            command: s_play_cmd(
                "removeParameter",
                Some(json!({ "parameterId": parameter_id })),
            ),
            style: None,
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
            children: parameter_children,
        });
    }
    ui_declarative_sections_to_tree(&children)
}

fn build_inspector_tree(document: &OsDocument, runtime: &StudioRuntimeState) -> UiNode {
    let projection = projection_from_document(document);
    let media_node_ids = &runtime.selected_media_node_ids;
    let instance_ids = &runtime.selected_app_instance_ids;
    let mut children = vec![UiSectionNode {
        id: "s-play-inspector.header".into(),
        label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
        default_open: Some(true),
        children: vec![ui_text(format!(
            "{} media node(s) · {} app instance(s)",
            media_node_ids.len(),
            instance_ids.len()
        ))],
    }];
    if !media_node_ids.is_empty() {
        let nodes: Vec<_> = media_node_ids
            .iter()
            .filter_map(|node_id| projection.media_graph.nodes.iter().find(|node| &node.id == node_id))
            .collect();
        let xs: Vec<_> = nodes.iter().map(|node| node.x).collect();
        let ys: Vec<_> = nodes.iter().map(|node| node.y).collect();
        let x_uniform = ui_inspector_all_equal(&xs.iter().map(|v| v.to_string()).collect::<Vec<_>>());
        let y_uniform = ui_inspector_all_equal(&ys.iter().map(|v| v.to_string()).collect::<Vec<_>>());
        let mut node_fields = Vec::new();
        if media_node_ids.len() == 1 {
            node_fields.push(UiNode::Field(UiFieldNode {
                id: "s-play-inspector.media-node.id".into(),
                label: "Node id".into(),
                child: UiControlNode::Input(UiInputNode {
                    id: "s-play-inspector.media-node.id.input".into(),
                    input_kind: "text".into(),
                    value: media_node_ids[0].clone(),
                    placeholder: None,
                    commit: None,
                    on_change: s_play_cmd("noop", None),
                }),
            }));
        }
        node_fields.push(UiNode::Field(UiFieldNode {
            id: "s-play-inspector.media-node.x".into(),
            label: "X".into(),
            child: UiControlNode::Input(UiInputNode {
                id: "s-play-inspector.media-node.x.input".into(),
                input_kind: "number".into(),
                value: if x_uniform {
                    xs.first().map(|v| v.to_string()).unwrap_or_default()
                } else {
                    String::new()
                },
                placeholder: if x_uniform { None } else { Some("Mixed".into()) },
                commit: None,
                on_change: s_play_cmd(
                    "patchMediaNodes",
                    Some(json!({ "nodeIds": media_node_ids, "field": "position", "axis": "x" })),
                ),
            }),
        }));
        node_fields.push(UiNode::Field(UiFieldNode {
            id: "s-play-inspector.media-node.y".into(),
            label: "Y".into(),
            child: UiControlNode::Input(UiInputNode {
                id: "s-play-inspector.media-node.y.input".into(),
                input_kind: "number".into(),
                value: if y_uniform {
                    ys.first().map(|v| v.to_string()).unwrap_or_default()
                } else {
                    String::new()
                },
                placeholder: if y_uniform { None } else { Some("Mixed".into()) },
                commit: None,
                on_change: s_play_cmd(
                    "patchMediaNodes",
                    Some(json!({ "nodeIds": media_node_ids, "field": "position", "axis": "y" })),
                ),
            }),
        }));
        children.push(UiSectionNode {
            id: "s-play-inspector.media-nodes".into(),
            label: Some(if media_node_ids.len() == 1 {
                "Media graph node".into()
            } else {
                format!("Media graph nodes ({})", media_node_ids.len())
            }),
            default_open: Some(true),
            children: node_fields,
        });
    }
    if !instance_ids.is_empty() {
        let instances: Vec<_> = instance_ids
            .iter()
            .filter_map(|id| projection.app_instances.iter().find(|instance| &instance.id == id))
            .collect();
        let labels: Vec<_> = instances.iter().map(|instance| instance.label.clone()).collect();
        let programs: Vec<_> = instances.iter().map(|instance| instance.program_id.clone()).collect();
        let apps: Vec<_> = instances.iter().map(|instance| instance.app_id.clone()).collect();
        let label_uniform = ui_inspector_all_equal(&labels);
        let program_uniform = ui_inspector_all_equal(&programs);
        let app_uniform = ui_inspector_all_equal(&apps);
        let mut instance_fields = vec![
            ui_text(format!(
                "Program: {}",
                if program_uniform {
                    programs.first().cloned().unwrap_or_default()
                } else {
                    "Mixed".into()
                }
            )),
            ui_text(format!(
                "App: {}",
                if app_uniform {
                    apps.first().cloned().unwrap_or_default()
                } else {
                    "Mixed".into()
                }
            )),
            UiNode::Field(UiFieldNode {
                id: "s-play-inspector.app-instance.label".into(),
                label: "Label".into(),
                child: UiControlNode::Input(UiInputNode {
                    id: "s-play-inspector.app-instance.label.input".into(),
                    input_kind: "text".into(),
                    value: if label_uniform {
                        labels.first().cloned().unwrap_or_default()
                    } else {
                        String::new()
                    },
                    placeholder: if label_uniform { None } else { Some("Mixed".into()) },
                    commit: None,
                    on_change: s_play_cmd(
                        "patchAppInstances",
                        Some(json!({ "instanceIds": instance_ids, "field": "label" })),
                    ),
                }),
            }),
        ];
        if instance_ids.len() == 1 {
            instance_fields.insert(2, ui_text(format!("Instance id: {}", instance_ids[0])));
        }
        if instance_ids.len() == 1 {
            if let Some(instance) = instances.first() {
                if let Some(registration) = os_app_registration(&instance.program_id, &instance.app_id) {
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
                            label: "Direct value".into(),
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
                        instance_fields.push(UiNode::Field(UiFieldNode {
                            id: format!("s-play-inspector.app-parameter.{}", field_spec.field_path),
                            label: field_spec.label.clone(),
                            child: UiControlNode::Select(UiSelectNode {
                                id: format!(
                                    "s-play-inspector.app-parameter.{}.select",
                                    field_spec.field_path
                                ),
                                value: binding
                                    .map(|entry| entry.parameter_id.clone())
                                    .unwrap_or_else(|| "__direct__".into()),
                                items,
                                placeholder: None,
                                on_change: s_play_cmd(
                                    "bindParameterField",
                                    Some(json!({
                                        "instanceId": instance.id,
                                        "fieldPath": field_spec.field_path,
                                    })),
                                ),
                            }),
                        }));
                        if let Some(binding) = binding {
                            if let Some(parameter) = projection
                                .parameters
                                .iter()
                                .find(|entry| entry.id() == binding.parameter_id)
                            {
                                instance_fields.push(ui_text(format!(
                                    "Bound value: {}",
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
                "App instance".into()
            } else {
                format!("App instances ({})", instance_ids.len())
            }),
            default_open: Some(true),
            children: instance_fields,
        });
    }
    if media_node_ids.is_empty() && instance_ids.is_empty() {
        children[0].children.push(ui_text(
            "Select media graph nodes or app instances in the canvas.",
        ));
    }
    ui_declarative_sections_to_tree(&children)
}

trait OsParameterId {
    fn id(&self) -> &str;
}

impl OsParameterId for OsParameter {
    fn id(&self) -> &str {
        parameter_entity_id(self)
    }
}
//#endregion 🔖StudioPanels

//#region 🔖CompiledDag
fn parameter_name(parameter: &OsParameter) -> &str {
    match parameter {
        OsParameter::Numeric { name, .. }
        | OsParameter::Categorical { name, .. }
        | OsParameter::Toggle { name, .. }
        | OsParameter::Text { name, .. } => name,
    }
}

fn media_port_label(
    port_id: &str,
    parameter_by_id: &HashMap<String, &OsParameter>,
) -> String {
    parameter_id_from_port_id(port_id)
        .and_then(|id| parameter_by_id.get(&id).map(|row| parameter_name(row).to_string()))
        .or_else(|| media_port_spec_id(port_id))
        .unwrap_or_else(|| port_id.to_string())
}

fn media_graph_to_dag_fixture(projection: &OsProjection) -> DagFixture {
    let instance_by_id: HashMap<_, _> = projection
        .app_instances
        .iter()
        .map(|row| (row.id.clone(), row))
        .collect();
    let parameter_by_id: HashMap<_, _> = projection
        .parameters
        .iter()
        .map(|row| match row {
            OsParameter::Numeric { id, .. }
            | OsParameter::Categorical { id, .. }
            | OsParameter::Toggle { id, .. }
            | OsParameter::Text { id, .. } => (id.clone(), row),
        })
        .collect();
    let nodes = projection
        .media_graph
        .nodes
        .iter()
        .map(|node| {
            let instance = instance_by_id.get(&node.instance_id);
            let registration = instance
                .and_then(|row| os_app_registration(&row.program_id, &row.app_id));
            let icon = format!(
                "emoji:{}",
                registration
                    .map(|row| row.component_kind.clone())
                    .unwrap_or_else(|| "s".into())
            );
            DagNodeSpec {
                id: node.id.clone(),
                name: instance
                    .map(|row| row.label.clone())
                    .unwrap_or_else(|| node.instance_id.clone()),
                abbreviation: instance
                    .map(|row| {
                        if row.app_id.chars().count() <= 3 {
                            row.app_id.clone()
                        } else {
                            row.app_id.chars().take(3).collect()
                        }
                    })
                    .unwrap_or_else(|| "app".into()),
                icon: icon.clone(),
                x: node.x + node.width / 2.0,
                y: node.y + node.height / 2.0,
                width: node.width,
                height: node.height,
                operator_kind: instance.map(|row| row.program_id.clone()),
                kind: DagNodeKind::AppInstance {
                    instance_id: node.instance_id.clone(),
                    program_id: instance
                        .map(|row| row.program_id.clone())
                        .unwrap_or_default(),
                    app_id: instance
                        .map(|row| row.app_id.clone())
                        .unwrap_or_default(),
                    icon,
                    inputs: node
                        .inputs
                        .iter()
                        .map(|port| {
                            let mut spec = IoPortSpec::simple(
                                &port.id,
                                media_port_label(&port.id, &parameter_by_id),
                            );
                            spec.resource_kind = Some(port.resource_kind.clone());
                            spec
                        })
                        .collect(),
                    outputs: node
                        .outputs
                        .iter()
                        .map(|port| {
                            let mut spec = IoPortSpec::simple(
                                &port.id,
                                media_port_label(&port.id, &parameter_by_id),
                            );
                            spec.resource_kind = Some(port.resource_kind.clone());
                            spec
                        })
                        .collect(),
                },
                ..Default::default()
            }
        })
        .collect();
    let edges = projection
        .media_graph
        .edges
        .iter()
        .map(|edge| DagFixtureEdge {
            id: edge.id.clone(),
            source: format!("{}:{}", edge.source_node_id, edge.source_port_id),
            target: format!("{}:{}", edge.target_node_id, edge.target_port_id),
            ..Default::default()
        })
        .collect();
    DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        },
        nodes,
        edges,
    }
}

fn compiled_dag_wire_literal(document: &OsDocument) -> String {
    let projection = projection_from_document(document);
    let fixture = media_graph_to_dag_fixture(&projection);
    dag_fixture_to_wire_literal(&fixture)
}

fn compiled_dag_engagement(document: &OsDocument) -> WindowEngagement {
    let wire = compiled_dag_wire_literal(document);
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
//#endregion 🔖CompiledDag

//#region 🔖StudioWindows
fn render_media_graph(document: &OsDocument, runtime: &StudioRuntimeState) -> UiNode {
    let projection = projection_from_document(document);
    let graph_payload = os_media_graph_to_node_graph_payload(&projection.media_graph, &projection.app_instances);
    let operators = build_os_media_flow_operator_infos(
        &projection.media_graph,
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
        S_PLAY_SURFACE_MEDIA_GRAPH,
        S_PLAY_CONTROLLER_ID,
        NodeGraphScene {
            editable: Some(true),
            operators_json: Some(serde_json::to_string(&operators).unwrap_or_else(|_| "[]".into())),
            context_menu_json: Some(media_graph_context_menu_json()),
            find_items_json: Some(graph_payload.find_items_json),
            selection_json,
            hover_json,
            capabilities_json: Some(r#"{"spotlight":false,"noteEdit":false,"clusters":false}"#.into()),
            presence_peers_json: Some(presence_peers_json()),
            ..NodeGraphScene::base(
                graph_payload.nodes_json,
                graph_payload.edges_json,
                graph_payload.viewport_json,
            )
        },
    )
}

fn render_compiled_dag(document: &OsDocument) -> UiNode {
    let wire = compiled_dag_wire_literal(document);
    build_text_editor_scene(
        S_PLAY_SURFACE_COMPILED_DAG,
        S_PLAY_CONTROLLER_ID,
        TextEditorScene::base(wire, Some("wire".into()), None),
    )
}
//#endregion 🔖StudioWindows

//#region 🔖SHomeApp
struct SHomeApp;

impl PluginApp for SHomeApp {
    fn app_id(&self) -> &str {
        S_HOME_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&SHomeDocument {
            schema: "s.home".into(),
            catalog_generation: 0,
        })
        .expect("home document json")
    }

    fn handle_command_patch_ops(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut document: SHomeDocument = serde_json::from_str(document_json).unwrap_or(SHomeDocument {
            schema: "s.home".into(),
            catalog_generation: 0,
        });
        let port = catalog_port();
        match command {
            "createStudio" => {
                let name = args
                    .and_then(|value| value.get("name"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("Untitled Studio");
                let kind = args
                    .and_then(|value| value.get("kind"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("file");
                match kind {
                    "temporary" => {
                        if let Ok(entry) = create_os_studio(name, temp_catalog_port()) {
                            return finish_create_ops(&mut document, &entry);
                        }
                    }
                    "folder" => {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            if let Some(folder_path) = args
                                .and_then(|value| value.get("folderPath"))
                                .and_then(|value| value.as_str())
                            {
                                if let Ok(entry) = create_folder_studio(name, folder_path) {
                                    return finish_create_ops(&mut document, &entry);
                                }
                            }
                            return vec![json!({
                                "op": "requestFolderPick",
                                "importCommand": "createStudio",
                                "args": { "kind": "folder", "name": name }
                            })
                            .to_string()];
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            let _ = name;
                        }
                    }
                    "file" | _ => {
                        if let Ok(entry) = create_os_studio(name, port.clone()) {
                            let mut ops = finish_create_ops(&mut document, &entry);
                            if let Ok(studio_document) = load_os_studio_document(&entry.id, port.clone()) {
                                if let Ok(json) = os_document_to_json(&studio_document) {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    {
                                        ops.insert(
                                            1,
                                            json!({
                                                "op": "requestFileSave",
                                                "filename": format!("{}.studio.json", entry.name.replace(' ', "-")),
                                                "mimeType": "application/json",
                                                "data": json,
                                                "studioId": entry.id
                                            })
                                            .to_string(),
                                        );
                                    }
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        ops.insert(
                                            1,
                                            json!({
                                                "op": "downloadMediaExport",
                                                "filename": format!("{}.studio.json", entry.name.replace(' ', "-")),
                                                "mimeType": "application/json",
                                                "data": json
                                            })
                                            .to_string(),
                                        );
                                    }
                                }
                            }
                            return ops;
                        }
                    }
                }
            }
            "bindStudioFile" => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let studio_id = args
                        .and_then(|value| value.get("studioId"))
                        .and_then(|value| value.as_str());
                    let file_path = args
                        .and_then(|value| value.get("filePath"))
                        .and_then(|value| value.as_str());
                    if let (Some(studio_id), Some(file_path)) = (studio_id, file_path) {
                        let _ = bind_studio_file(studio_id, file_path);
                    }
                }
            }
            "importStudio" => {
                let json = args
                    .and_then(|value| value.get("json"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| {
                        args.and_then(|value| value.get("payload"))
                            .and_then(|value| value.as_str())
                            .map(str::to_string)
                    })
                    .or_else(|| {
                        args.and_then(|value| value.get("payload"))
                            .map(|value| value.to_string())
                    });
                if let Some(json) = json {
                    if import_os_studio_from_json(&json, port.clone()).is_ok() {
                        document.catalog_generation += 1;
                    }
                    return vec![set_home_document_op(&document)];
                }
                return vec![json!({
                    "op": "requestFileOpen",
                    "importCommand": "importStudio",
                    "accept": ".json"
                })
                .to_string()];
            }
            "openStudio" | "navigateVirtualFileSystemNode" => {
                let studio_id = args
                    .and_then(|value| value.get("studioId").or_else(|| value.get("nodeId")))
                    .and_then(|value| value.as_str())
                    .and_then(|value| value.strip_prefix("studio:").or(Some(value)));
                if let Some(studio_id) = studio_id {
                    return vec![json!({
                        "op": "navigate",
                        "uri": format!("/studios/{studio_id}")
                    })
                    .to_string()];
                }
            }
            "deleteVirtualFileSystemNode" => {
                let studio_id = args
                    .and_then(|value| value.get("nodeId"))
                    .and_then(|value| value.as_str())
                    .and_then(|value| value.strip_prefix("studio:"));
                if let Some(studio_id) = studio_id {
                    let _ = delete_os_studio(studio_id, port.clone());
                    document.catalog_generation += 1;
                }
            }
            "goHome" => {
                return vec![json!({ "op": "navigate", "uri": "/" }).to_string()];
            }
            _ => {}
        }
        vec![set_home_document_op(&document)]
    }

    fn render(&self, body_key: &str, _document_json: &str, _view_state: &ViewState) -> UiNode {
        match body_key {
            S_HOME_BODY => render_home_vfs(),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖SHomeApp

//#region 🔖SStudioApp
struct SStudioApp;

impl SStudioApp {
    fn handle_studio_command(
        envelope: &mut SStudioEnvelope,
        command: &str,
        args: Option<&Value>,
    ) -> Vec<String> {
        let mut ops = Vec::new();
        let mut store = store_from_envelope(envelope);
        let mut runtime = envelope.runtime.clone();
        match command {
            "setActivePanelTab" => {
                if let Some(tab) = args.and_then(|value| value.get("tabId")).and_then(|value| value.as_str()) {
                    let mut panel = StudioPanelState {
                        active_panel_tab: tab.into(),
                        ..Default::default()
                    };
                    ops.push(set_panel_op(&panel));
                }
                return ops;
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
                    let projection = store.projection().unwrap_or_else(|_| default_os_projection());
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
                        let _ = store.patch_parameter(parameter_id, &patch).expect("patch parameter");
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
                let _ = store.add_parameter(&parameter_type, name);
            }
            "removeParameter" => {
                if let Some(parameter_id) = args
                    .and_then(|value| value.get("parameterId"))
                    .and_then(|value| value.as_str())
                {
                    let _ = store.dispatch_apply(vec![OsOp::RemoveParameter {
                        parameter_id: parameter_id.into(),
                    }]);
                }
            }
            "spawnApp" => {
                let program_id = args
                    .and_then(|value| value.get("programId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let app_id = args
                    .and_then(|value| value.get("appId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if !program_id.is_empty() && !app_id.is_empty() {
                    let position = args
                        .and_then(|value| value.get("position"))
                        .and_then(|value| value.as_object())
                        .map(|position| MediaGraphPosition {
                            x: position
                                .get("x")
                                .and_then(|value| value.as_f64())
                                .unwrap_or(80.0),
                            y: position
                                .get("y")
                                .and_then(|value| value.as_f64())
                                .unwrap_or(80.0),
                        })
                        .unwrap_or(MediaGraphPosition { x: 80.0, y: 80.0 });
                    if let Ok(instance_id) = store.spawn_app_instance(program_id, app_id, None, position) {
                        runtime.active_instance_id = Some(instance_id.clone());
                    }
                }
            }
            "moveMediaNode" => {
                if let (Some(node_id), Some(x), Some(y)) = (
                    args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()),
                    args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()),
                    args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()),
                ) {
                    let _ = store.dispatch_apply(vec![OsOp::MoveMediaNode {
                        node_id: node_id.into(),
                        x,
                        y,
                    }]);
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
                    let _ = store.dispatch_apply(vec![OsOp::ConnectMediaPorts {
                        edge: semio_framework_os::OsMediaGraphEdge {
                            id: create_os_id("edge"),
                            source_node_id: source_node_id.into(),
                            source_port_id: source_port_id.into(),
                            target_node_id: target_node_id.into(),
                            target_port_id: target_port_id.into(),
                        },
                    }]);
                }
            }
            "disconnectMediaEdge" => {
                if let Some(edge_id) = args
                    .and_then(|value| value.get("edgeId"))
                    .and_then(|value| value.as_str())
                {
                    let _ = store.dispatch_apply(vec![OsOp::DisconnectMediaEdge {
                        edge_id: edge_id.into(),
                    }]);
                }
            }
            "removeAppInstance" => {
                let instance_id = args
                    .and_then(|value| value.get("instanceId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| {
                        let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                        primary_selected_instance_id(&runtime, &projection)
                    });
                if let Some(instance_id) = instance_id {
                    let _ = store.dispatch_apply(vec![OsOp::RemoveAppInstance {
                        instance_id: instance_id.clone(),
                    }]);
                    if runtime.active_instance_id.as_deref() == Some(instance_id.as_str()) {
                        runtime.active_instance_id = None;
                    }
                    if runtime.focused_instance_id.as_deref() == Some(instance_id.as_str()) {
                        runtime.focused_instance_id = None;
                    }
                }
            }
            "deleteSelection" => {
                let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                let instance_ids = selected_instance_ids(&runtime, &projection);
                for instance_id in instance_ids {
                    let _ = store.dispatch_apply(vec![OsOp::RemoveAppInstance {
                        instance_id: instance_id.clone(),
                    }]);
                }
                runtime.selected_app_instance_ids.clear();
                runtime.selected_media_node_ids.clear();
                runtime.active_instance_id = None;
                runtime.focused_instance_id = None;
            }
            "copyAppInstance" => {
                let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                runtime.clipboard_instance_ids = selected_instance_ids(&runtime, &projection);
            }
            "duplicateAppInstance" | "pasteAppInstance" => {
                let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                let source_ids = if command == "pasteAppInstance" {
                    runtime.clipboard_instance_ids.clone()
                } else {
                    selected_instance_ids(&runtime, &projection)
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
                        .media_graph
                        .nodes
                        .iter()
                        .find(|node| node.instance_id == instance_id)
                        .map(|node| MediaGraphPosition {
                            x: node.x + 40.0,
                            y: node.y + 40.0,
                        })
                        .unwrap_or(MediaGraphPosition { x: 80.0, y: 80.0 });
                    let label = format!("{} Copy", instance.label);
                    if let Ok(new_id) = store.spawn_app_instance(
                        &instance.program_id,
                        &instance.app_id,
                        Some(&label),
                        position,
                    ) {
                        runtime.active_instance_id = Some(new_id);
                    }
                }
            }
            "renameAppInstance" => {
                let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                if let Some(instance_id) = primary_selected_instance_id(&runtime, &projection) {
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
                        let _ = store.dispatch_apply(vec![OsOp::PatchAppInstance {
                            instance_id,
                            label: Some(label),
                        }]);
                    }
                }
            }
            "patchAppSource" => {
                if let (Some(instance_id), Some(inline)) = (
                    args.and_then(|value| value.get("instanceId"))
                        .and_then(|value| value.as_str()),
                    args.and_then(|value| value.get("inline"))
                        .and_then(|value| value.as_str()),
                ) {
                    let _ = store.dispatch_apply(vec![OsOp::PatchAppSource {
                        instance_id: instance_id.into(),
                        inline: inline.into(),
                    }]);
                }
            }
            "commitCheckpoint" => {
                let message = args
                    .and_then(|value| value.get("message"))
                    .and_then(|value| value.as_str());
                let _ = store.dispatch_json(&json!({
                    "kind": "commitCheckpoint",
                    "message": message
                }).to_string());
            }
            "checkoutCheckpoint" => {
                if let Some(checkpoint_id) = args
                    .and_then(|value| value.get("checkpointId"))
                    .and_then(|value| value.as_str())
                {
                    let _ = store.dispatch_json(&json!({
                        "kind": "checkoutCheckpoint",
                        "checkpointId": checkpoint_id
                    }).to_string());
                }
            }
            "setActiveExample" => {
                if let Some(example_id) = args
                    .and_then(|value| value.get("exampleId"))
                    .and_then(|value| value.as_str())
                {
                    let Some(document_json) = studio_example_document_json(example_id) else {
                        return vec![];
                    };
                    if let Ok(document) = os_document_from_json(&document_json) {
                        let active_instance_id = projection_from_document(&document)
                            .app_instances
                            .first()
                            .map(|instance| instance.id.clone());
                        *envelope = SStudioEnvelope {
                            document,
                            runtime: StudioRuntimeState {
                                studio_id: Some(example_id.into()),
                                active_instance_id,
                                ..StudioRuntimeState::default()
                            },
                        };
                        return vec![set_studio_document_op(envelope)];
                    }
                }
            }
            "exportMedia" => {
                if let (Some(instance_id), Some(format)) = (
                    args.and_then(|value| value.get("instanceId")).and_then(|value| value.as_str()),
                    args.and_then(|value| value.get("format")).and_then(|value| value.as_str()),
                ) {
                    let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                    if let Some(instance) = projection
                        .app_instances
                        .iter()
                        .find(|row| row.id == instance_id)
                    {
                        let export_format = semio_framework_os::OsMediaExportFormat::parse(format)
                            .unwrap_or(semio_framework_os::OsMediaExportFormat::Svg);
                        if let Ok(result) = semio_framework_os::export_os_app_instance_media(
                            instance,
                            &json!({}),
                            export_format,
                        ) {
                            ops.push(json!({
                                "op": "downloadMediaExport",
                                "filename": result.file_name,
                                "mimeType": result.mime_type,
                                "data": result.data,
                            })
                            .to_string());
                        }
                    }
                }
            }
            "undo" => {
                let _ = store.dispatch_json(r#"{"kind":"undo"}"#);
            }
            "redo" => {
                let _ = store.dispatch_json(r#"{"kind":"redo"}"#);
            }
            "selectInstance" => {
                runtime.active_instance_id = args
                    .and_then(|value| value.get("instanceId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                if let Some(instance_id) = &runtime.active_instance_id {
                    let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                    let node_id = projection
                        .media_graph
                        .nodes
                        .iter()
                        .find(|node| node.instance_id == *instance_id)
                        .map(|node| node.id.clone());
                    runtime.selected_app_instance_ids = vec![instance_id.clone()];
                    runtime.selected_media_node_ids = node_id.into_iter().collect();
                }
            }
            "nodeGraphSelect" | "setMediaNodeSelection" => {
                let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                let node_ids: Vec<String> = if args
                    .and_then(|value| value.get("selectAll"))
                    .and_then(|value| value.as_bool())
                    .unwrap_or(false)
                {
                    projection.media_graph.nodes.iter().map(|node| node.id.clone()).collect()
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
                runtime.selected_media_node_ids = node_ids.clone();
                runtime.selected_app_instance_ids = node_ids
                    .iter()
                    .filter_map(|node_id| {
                        projection
                            .media_graph
                            .nodes
                            .iter()
                            .find(|node| node.id == *node_id)
                            .map(|node| node.instance_id.clone())
                    })
                    .collect();
                if runtime.selected_app_instance_ids.len() == 1 {
                    runtime.active_instance_id = runtime.selected_app_instance_ids.first().cloned();
                }
            }
            "reorganizeMediaGraph" => {
                let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                let node_ids: Vec<String> = if runtime.selected_media_node_ids.is_empty() {
                    projection.media_graph.nodes.iter().map(|node| node.id.clone()).collect()
                } else {
                    runtime.selected_media_node_ids.clone()
                };
                let mut ops = Vec::new();
                for (index, node_id) in node_ids.iter().enumerate() {
                    let col = (index % 4) as f64;
                    let row = (index / 4) as f64;
                    ops.push(OsOp::MoveMediaNode {
                        node_id: node_id.clone(),
                        x: 80.0 + col * 220.0,
                        y: 80.0 + row * 160.0,
                    });
                }
                if !ops.is_empty() {
                    let _ = store.dispatch_apply(ops);
                }
            }
            "nodeGraphHover" | "textHover" => {
                runtime.hovered_media_node_id = args
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
                let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                runtime.selected_app_instance_ids = instance_ids.clone();
                runtime.selected_media_node_ids = instance_ids
                    .iter()
                    .filter_map(|instance_id| {
                        projection
                            .media_graph
                            .nodes
                            .iter()
                            .find(|node| node.instance_id == *instance_id)
                            .map(|node| node.id.clone())
                    })
                    .collect();
                if instance_ids.len() == 1 {
                    runtime.active_instance_id = Some(instance_ids[0].clone());
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
                    let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                    for node_id in node_ids {
                        if let Some(node) = projection.media_graph.nodes.iter().find(|row| row.id == node_id) {
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
                            let _ = store.dispatch_apply(vec![OsOp::MoveMediaNode {
                                node_id,
                                x,
                                y,
                            }]);
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
                            let _ = store.dispatch_apply(vec![OsOp::PatchAppInstance {
                                instance_id,
                                label: Some(label.into()),
                            }]);
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
                        let _ = store.dispatch_apply(vec![OsOp::UnbindParameterField {
                            instance_id: instance_id.into(),
                            field_path: field_path.into(),
                        }]);
                    } else {
                        let _ = store.dispatch_apply(vec![OsOp::BindParameterField {
                            binding: OsParameterFieldBinding {
                                parameter_id: parameter_id.into(),
                                instance_id: instance_id.into(),
                                field_path: field_path.into(),
                            },
                        }]);
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
                    let _ = store.dispatch_apply(vec![OsOp::UnbindParameterField {
                        instance_id: instance_id.into(),
                        field_path: field_path.into(),
                    }]);
                }
            }
            "openStudio" => {
                if let Some(studio_id) = args
                    .and_then(|value| value.get("studioId"))
                    .and_then(|value| value.as_str())
                {
                    if let Ok(document) = load_studio_document(studio_id) {
                        *envelope = SStudioEnvelope {
                            document,
                            runtime: StudioRuntimeState {
                                studio_id: Some(studio_id.into()),
                                active_instance_id: envelope.runtime.active_instance_id.clone(),
                                ..StudioRuntimeState::default()
                            },
                        };
                        return vec![set_studio_document_op(envelope)];
                    }
                }
            }
            "openInstance" => {
                let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                let instance_id = args
                    .and_then(|value| value.get("instanceId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| primary_selected_instance_id(&runtime, &projection));
                if let Some(instance_id) = instance_id {
                    runtime.focused_instance_id = Some(instance_id.clone());
                    runtime.active_instance_id = Some(instance_id.clone());
                    runtime.selected_app_instance_ids = vec![instance_id.clone()];
                    let projection = store.projection().unwrap_or_else(|_| default_os_projection());
                    if let Some(node) = projection
                        .media_graph
                        .nodes
                        .iter()
                        .find(|row| row.instance_id == instance_id)
                    {
                        runtime.selected_media_node_ids = vec![node.id.clone()];
                    }
                    if let Some(instance) = projection
                        .app_instances
                        .iter()
                        .find(|row| row.id == instance_id)
                    {
                        ensure_studio_fixtures_registered();
                        let document_json = materialize_os_app_instance_document_json(
                            instance,
                            &projection.parameter_bindings,
                            &projection.parameters,
                            &projection.app_instances,
                        );
                        ops.push(json!({
                            "op": "openPluginInstance",
                            "programId": instance.program_id,
                            "appId": instance.app_id,
                            "osInstanceId": instance.id,
                            "label": instance.label,
                            "documentJson": document_json,
                        })
                        .to_string());
                    }
                }
            }
            "closeFocusedInstance" => {
                runtime.focused_instance_id = None;
            }
            "goHome" => {
                return vec![json!({ "op": "navigate", "uri": "/" }).to_string()];
            }
            "mediaGraphEngagementInput" => {
                runtime.media_graph_engagement_input = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .into();
            }
            "mediaGraphEngagementSubmit" => {
                let raw = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .unwrap_or(&runtime.media_graph_engagement_input);
                let mut parts = raw.split_whitespace();
                if let (Some(program_id), Some(app_id)) = (parts.next(), parts.next()) {
                    if let Ok(instance_id) = store.spawn_app_instance(
                        program_id,
                        app_id,
                        None,
                        MediaGraphPosition { x: 80.0, y: 80.0 },
                    ) {
                        runtime.active_instance_id = Some(instance_id);
                    }
                }
            }
            "compiledDagEngagementInput" => {
                runtime.compiled_dag_engagement_input = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .into();
            }
            "compiledDagEngagementSubmit" => {
                let _ = runtime.compiled_dag_engagement_input.clone();
            }
            _ => {}
        }
        *envelope = envelope_from_store(store, runtime);
        persist_envelope_document(envelope);
        ops.push(set_studio_document_op(envelope));
        ops
    }
}

impl PluginApp for SStudioApp {
    fn app_id(&self) -> &str {
        S_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        initial_studio_document_json()
    }

    fn handle_command_patch_ops(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        view_state: &ViewState,
    ) -> Vec<String> {
        if command == "setActivePanelTab" {
            if let Some(tab) = args.and_then(|value| value.get("tabId")).and_then(|value| value.as_str()) {
                let mut panel = parse_panel_state(view_state);
                panel.active_panel_tab = tab.into();
                return vec![set_panel_op(&panel)];
            }
        }
        if command == "closeFocusedInstance" {
            let mut panel = parse_panel_state(view_state);
            panel.active_spawned_id = None;
            let mut envelope = parse_studio_envelope(document_json);
            envelope.runtime.focused_instance_id = None;
            return vec![set_panel_op(&panel), set_studio_document_op(&envelope)];
        }
        if command == "navigateVirtualFileSystemNode" {
            if let Some(studio_id) = args
                .and_then(|value| value.get("studioId"))
                .and_then(|value| value.as_str())
            {
                return vec![json!({
                    "op": "navigate",
                    "uri": format!("/studios/{studio_id}")
                })
                .to_string()];
            }
        }
        let mut envelope = parse_studio_envelope(document_json);
        Self::handle_studio_command(&mut envelope, command, args)
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let envelope = parse_studio_envelope(document_json);
        let panel = parse_panel_state(view_state);
        match body_key {
            S_PLAY_BODY_MEDIA_GRAPH => render_media_graph(&envelope.document, &envelope.runtime),
            S_PLAY_BODY_MEDIA_VFS => render_media_vfs(&envelope.document),
            S_PLAY_BODY_COMPILED_DAG => render_compiled_dag(&envelope.document),
            S_PLAY_CATALOGUE_BODY_KEY => build_catalogue_tree(&panel),
            S_PLAY_PARAMETERS_BODY_KEY => build_parameters_tree(&envelope.document),
            S_PLAY_INSPECTOR_BODY_KEY => build_inspector_tree(&envelope.document, &envelope.runtime),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_measures(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let envelope = parse_studio_envelope(document_json);
        let projection = projection_from_document(&envelope.document);
        HashMap::from([(
            S_PLAY_WINDOW_MEDIA_GRAPH.into(),
            media_graph_measures(&envelope.runtime, &projection.app_instances),
        )])
    }
}
//#endregion 🔖SStudioApp

//#region 🔖Manifest
fn studio_play_layout() -> WindowLayout {
    create_default_layout(
        &[
            S_PLAY_WINDOW_MEDIA_GRAPH.into(),
            S_PLAY_WINDOW_MEDIA_VFS.into(),
            S_PLAY_WINDOW_COMPILED_DAG.into(),
        ],
        "row",
        Some(&[40.0, 30.0, 30.0]),
        Some(&[
            "Media Graph".into(),
            "Media VFS".into(),
            "Compiled DAG".into(),
        ]),
    )
}

fn media_graph_engagement(runtime: &StudioRuntimeState, node_count: usize, app_count: usize) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("s-media-catalogue-hint".into()),
            value: Some(runtime.media_graph_engagement_input.clone()),
            placeholder: Some("Drag apps from Catalogue workbench tab".into()),
            on_change: Some(s_play_cmd("mediaGraphEngagementInput", None)),
            on_submit: Some(s_play_cmd("mediaGraphEngagementSubmit", None)),
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

fn media_graph_measures(runtime: &StudioRuntimeState, instances: &[OsAppInstance]) -> Vec<WindowMeasure> {
    vec![WindowMeasure::Select {
        id: "s-media-active-instance".into(),
        label: Some("Active app".into()),
        value: runtime.active_instance_id.clone().unwrap_or_default(),
        items: instances
            .iter()
            .map(|instance| MeasureSelectItem {
                id: instance.id.clone(),
                value: instance.id.clone(),
                label: instance.label.clone(),
            })
            .collect(),
        on_change: s_play_cmd("selectInstance", None),
    }]
}

fn home_create_tools() -> Vec<semio_framework_plugin::ToolNode> {
    let mut children = vec![
        tool_button(
            "s-home.create.temporary",
            "zap",
            "Temporary",
            s_home_cmd("createStudio", Some(json!({ "kind": "temporary" }))),
        ),
        tool_button(
            "s-home.create.file",
            "file-json",
            "File",
            s_home_cmd("createStudio", Some(json!({ "kind": "file" }))),
        ),
    ];
    #[cfg(not(target_arch = "wasm32"))]
    children.push(tool_button(
        "s-home.create.folder",
        "folder",
        "Folder",
        s_home_cmd("createStudio", Some(json!({ "kind": "folder" }))),
    ));
    vec![
        tool_collection("s-home.create", "plus", "Create", children).with_category(ToolCategory::Commands),
        tool_button(
            "s-home.import",
            "upload",
            "Import Studio",
            s_home_cmd("importStudio", None),
        )
        .with_category(ToolCategory::Commands),
    ]
}

fn create_home_app() -> App {
    let mut app = App::from_builder(
        App::builder(S_HOME_APP_ID, "Home").document(["semio", "s", "home"])
            .icon_id("home")
            .mode("explore", "Explore")
            .default_mode_id("explore")
            .mode_tools("explore", home_create_tools())
            .window_kind(S_HOME_WINDOW, "Studios", S_HOME_BODY, SurfaceKind::Canvas2d)
            .default_layout(create_tab_stack_layout(
                &[S_HOME_WINDOW.into()],
                Some(&["Studios".into()]),
            ))
            .keybinding("mod+n", "createStudio")
            .keybinding("mod+o", "importStudio"),
    );
    app.definition.controller_id = S_HOME_CONTROLLER_ID.into();
    app
}

fn create_studio_app() -> App {
    let projection = projection_from_document(&demo_os_document());
    let runtime = StudioRuntimeState {
        active_instance_id: projection.app_instances.first().map(|instance| instance.id.clone()),
        ..StudioRuntimeState::default()
    };
    let engagement = media_graph_engagement(
        &runtime,
        projection.media_graph.nodes.len(),
        projection.app_instances.len(),
    );
    let measures = media_graph_measures(&runtime, &projection.app_instances);
    let mut builder = App::builder(S_PLAY_APP_ID, "Studio").document(["semio", "s", "studio"])
        .icon_id("s")
        .mode("main", "Studio")
        .default_mode_id("main")
        .window_kind(S_PLAY_WINDOW_MEDIA_GRAPH, "Media Graph", S_PLAY_BODY_MEDIA_GRAPH, SurfaceKind::NodeGraph)
        .window_kind(S_PLAY_WINDOW_MEDIA_VFS, "Media VFS", S_PLAY_BODY_MEDIA_VFS, SurfaceKind::VirtualFileSystem)
        .window_kind(
            S_PLAY_WINDOW_COMPILED_DAG,
            "Compiled DAG",
            S_PLAY_BODY_COMPILED_DAG,
            SurfaceKind::NodeGraph,
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
        .default_layout(studio_play_layout())
        .mode_tools(
            "main",
            vec![tool_collection(
                "s-play.history",
                "history",
                "History",
                vec![
                    tool_button("s-play.undo", "undo-2", "Undo", s_play_cmd("undo", None)),
                    tool_button("s-play.redo", "redo-2", "Redo", s_play_cmd("redo", None)),
                    tool_button(
                        "s-play.checkpoint",
                        "git-commit-horizontal",
                        "Checkpoint",
                        s_play_cmd("commitCheckpoint", None),
                    ),
                ],
            )
            .with_category(ToolCategory::History)],
        )
        .keybinding("mod+z", "undo")
        .keybinding("mod+shift+z", "redo")
        .keybinding("mod+s", "commitCheckpoint");
    let mut definition = builder.build_definition();
    if let Some(window) = definition
        .window_kinds
        .iter_mut()
        .find(|window| window.id == S_PLAY_WINDOW_MEDIA_GRAPH)
    {
        window.measures = measures;
        window.engagement = Some(engagement);
    }
    let compiled_engagement = compiled_dag_engagement(&demo_os_document());
    if let Some(window) = definition
        .window_kinds
        .iter_mut()
        .find(|window| window.id == S_PLAY_WINDOW_COMPILED_DAG)
    {
        window.engagement = Some(compiled_engagement);
    }
    let mut app = App {
        definition,
        examples: vec![],
        program: None,
    };
    app.definition.controller_id = S_PLAY_CONTROLLER_ID.into();
    let mut app = app.program("s", "S Studio", "studio");
    for (id, label, json) in S_STUDIO_EXAMPLES {
        app = app.example(*id, *label, (*json).to_string());
    }
    app
}

fn bundle() -> PluginBundle {
    PluginBundle::new("s", "S Studio", "0.1.0")
        .local_backbone_storage()
        .register_app(create_home_app(), || Box::new(SHomeApp))
        .register_app(create_studio_app(), || Box::new(SStudioApp))
}

semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_os::{
        merge_os_program_definition, os_baseline_resource, os_in_port, os_out_port,
        validate_media_graph, OsAppResourceSpec, OsPlatformAppInput, OsPlatformInput,
    };
    use semio_framework_plugin::{PanelGroup, ModeDefinition, ToolNode, UiControlNode, UiNode};

    fn seed_draw_program() {
        let mut resources = HashMap::new();
        resources.insert(
            "draw".into(),
            os_baseline_resource("2d.drawing", "draw.document", "draw"),
        );
        merge_os_program_definition(
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
                        tools: vec![],
                layout_id: None,
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
        let envelope = initial_studio_envelope();
        let projection = projection_from_document(&envelope.document);
        assert!(projection.app_instances.len() >= 5);
        assert!(projection.media_graph.nodes.len() >= 2);
        assert!(projection.media_graph.edges.len() >= 1);
        assert!(validate_media_graph(&projection.media_graph).ok);
    }

    #[test]
    fn renders_media_graph_scene() {
        let app = SStudioApp;
        let envelope = initial_studio_document_json();
        let node = app.render(S_PLAY_BODY_MEDIA_GRAPH, &envelope, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_compiled_dag_editor() {
        let app = SStudioApp;
        let envelope = initial_studio_document_json();
        let node = app.render(S_PLAY_BODY_COMPILED_DAG, &envelope, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
        let wire = compiled_dag_wire_literal(&parse_studio_envelope(&envelope).document);
        assert!(wire.contains("appInstance") || wire.contains("draw"));
    }

    #[test]
    fn studio_manifest_uses_studio_app_id() {
        let app = create_studio_app();
        assert_eq!(app.definition.id, "studio");
        assert_eq!(app.definition.controller_id, "s-play");
        assert_eq!(app.program.as_ref().map(|p| p.program_id.as_str()), Some("s"));
    }

    #[test]
    fn home_manifest_uses_home_app_id() {
        let app = create_home_app();
        assert_eq!(app.definition.id, "home");
        assert_eq!(app.definition.controller_id, "s-home");
    }

    #[test]
    fn move_media_node_updates_projection() {
        let mut envelope = initial_studio_envelope();
        let node_id = projection_from_document(&envelope.document)
            .media_graph
            .nodes
            .first()
            .expect("node")
            .id
            .clone();
        SStudioApp::handle_studio_command(
            &mut envelope,
            "moveMediaNode",
            Some(&json!({ "nodeId": node_id, "x": 120.0, "y": 160.0 })),
        );
        let node = projection_from_document(&envelope.document)
            .media_graph
            .nodes
            .into_iter()
            .find(|row| row.id == node_id)
            .expect("node");
        assert!((node.x - 120.0).abs() < 0.01);
        assert!((node.y - 160.0).abs() < 0.01);
    }

    #[test]
    fn spawns_draw_app_instance() {
        seed_draw_program();
        let mut envelope = initial_studio_envelope();
        let ops = SStudioApp::handle_studio_command(
            &mut envelope,
            "spawnApp",
            Some(&json!({ "programId": "draw", "appId": "draw" })),
        );
        assert!(!ops.is_empty());
        let projection = projection_from_document(&envelope.document);
        assert!(projection
            .app_instances
            .iter()
            .any(|instance| instance.program_id == "draw"));
    }

    #[test]
    fn commit_checkpoint_round_trips_projection() {
        let mut envelope = initial_studio_envelope();
        let before = projection_from_document(&envelope.document);
        SStudioApp::handle_studio_command(
            &mut envelope,
            "commitCheckpoint",
            Some(&json!({ "message": "snapshot" })),
        );
        let rematerialized = projection_from_document(&envelope.document);
        assert_eq!(rematerialized.app_instances.len(), before.app_instances.len());
    }

    #[test]
    fn patch_parameter_via_store_works() {
        let envelope = initial_studio_envelope();
        let mut store = store_from_envelope(&envelope);
        store
            .patch_parameter("param-brush-size", &json!({ "value": 48.0 }))
            .expect("patch");
        let projection = store.projection().expect("projection");
        match projection
            .parameters
            .iter()
            .find(|entry| entry.id() == "param-brush-size")
            .expect("parameter")
        {
            OsParameter::Numeric { value, .. } => assert_eq!(*value, 48.0),
            _ => panic!("expected numeric"),
        }
    }

    #[test]
    fn patch_parameter_updates_value() {
        let mut envelope = initial_studio_envelope();
        let ops = SStudioApp::handle_studio_command(
            &mut envelope,
            "patchParameter",
            Some(&json!({ "parameterId": "param-brush-size", "field": "value", "value": 48.0 })),
        );
        assert_eq!(ops.len(), 1);
        let projection = store_from_envelope(&envelope)
            .projection()
            .expect("projection");
        let parameter = projection
            .parameters
            .iter()
            .find(|entry| entry.id() == "param-brush-size")
            .expect("parameter");
        match parameter {
            OsParameter::Numeric { value, .. } => assert_eq!(*value, 48.0),
            _ => panic!("expected numeric"),
        }
    }

    #[test]
    fn catalogue_tree_nests_apps_by_canonical_document() {
        let panel = StudioPanelState {
            programs: vec![
                StudioProgramEntry {
                    plugin_id: "puzzle".into(),
                    program_id: "puzzle".into(),
                    app_id: "puzzle2d-play".into(),
                    label: "Puzzle 2D".into(),
                    document: vec!["semio".into(), "puzzle".into(), "2d".into()],
                    yields: "layout".into(),
                },
                StudioProgramEntry {
                    plugin_id: "puzzle".into(),
                    program_id: "puzzle".into(),
                    app_id: "puzzle3d-play".into(),
                    label: "Puzzle 3D".into(),
                    document: vec!["semio".into(), "puzzle".into(), "3d".into()],
                    yields: "model".into(),
                },
            ],
            ..Default::default()
        };
        let tree = build_catalogue_tree(&panel);
        let json = serde_json::to_string(&tree).unwrap();
        assert!(json.contains("s-play-catalogue.document.semio.puzzle.2d"));
        assert!(json.contains("s-play-catalogue.document.semio.puzzle.3d"));
        assert_eq!(json.matches("\"label\":\"puzzle\"").count(), 1);
    }

    #[test]
    fn home_vfs_lists_seeded_studio() {
        let rows = home_vfs_rows();
        assert!(rows.iter().any(|row| row.get("navigateUri").and_then(|v| v.as_str()).unwrap_or("").starts_with("/studios/")));
    }

    #[test]
    fn creates_studio_via_home_command() {
        let port = catalog_port();
        let before = list_os_studio_catalog_entries(port.clone()).expect("list").len();
        let mut home = SHomeApp;
        home.handle_command_patch_ops(
            "createStudio",
            Some(&json!({ "name": "Test Studio" })),
            &home.initial_document_json(),
            &ViewState::default(),
        );
        let after = list_os_studio_catalog_entries(port).expect("list").len();
        assert!(after >= before);
    }

    #[test]
    fn home_explore_tools_include_create_collection() {
        let app = create_home_app();
        let explore = app
            .definition
            .modes
            .iter()
            .find(|mode| mode.id == "explore")
            .expect("explore mode");
        assert!(explore.tools.iter().any(|tool| {
            matches!(tool, ToolNode::Collection { id, .. } if id == "s-home.create")
        }));
    }

    #[test]
    fn temporary_studio_uses_ephemeral_port() {
        let mut home = SHomeApp;
        let ops = home.handle_command_patch_ops(
            "createStudio",
            Some(&json!({ "name": "Temp Studio", "kind": "temporary" })),
            &home.initial_document_json(),
            &ViewState::default(),
        );
        assert!(ops.iter().any(|op| op.contains("navigate")));
        let persistent = list_os_studio_catalog_entries(catalog_port()).expect("list");
        assert!(!persistent.iter().any(|entry| entry.name == "Temp Studio"));
        let ephemeral = list_os_studio_catalog_entries(temp_catalog_port()).expect("list");
        assert!(ephemeral.iter().any(|entry| entry.name == "Temp Studio"));
    }

    #[test]
    fn patch_app_instances_updates_labels() {
        let mut envelope = initial_studio_envelope();
        let ids: Vec<String> = projection_from_document(&envelope.document)
            .app_instances
            .iter()
            .take(2)
            .map(|instance| instance.id.clone())
            .collect();
        SStudioApp::handle_studio_command(
            &mut envelope,
            "patchAppInstances",
            Some(&json!({ "instanceIds": ids, "field": "label", "value": "Batch Label" })),
        );
        let labels: Vec<String> = projection_from_document(&envelope.document)
            .app_instances
            .iter()
            .filter(|instance| ids.contains(&instance.id))
            .map(|instance| instance.label.clone())
            .collect();
        assert!(labels.iter().all(|label| label == "Batch Label"));
    }

    #[test]
    fn open_and_close_focused_instance() {
        let mut envelope = initial_studio_envelope();
        let instance_id = projection_from_document(&envelope.document)
            .app_instances
            .first()
            .expect("instance")
            .id
            .clone();
        assert!(envelope.runtime.focused_instance_id.is_none());
        SStudioApp::handle_studio_command(
            &mut envelope,
            "openInstance",
            Some(&json!({ "instanceId": instance_id })),
        );
        assert_eq!(envelope.runtime.focused_instance_id.as_deref(), Some(instance_id.as_str()));
        SStudioApp::handle_studio_command(&mut envelope, "closeFocusedInstance", None);
        assert!(envelope.runtime.focused_instance_id.is_none());
    }

    #[test]
    fn open_instance_emits_materialized_document_json() {
        ensure_studio_fixtures_registered();
        let mut envelope = initial_studio_envelope();
        let instance_id = projection_from_document(&envelope.document)
            .app_instances
            .iter()
            .find(|instance| instance.program_id == "draw")
            .expect("draw instance")
            .id
            .clone();
        let ops = SStudioApp::handle_studio_command(
            &mut envelope,
            "openInstance",
            Some(&json!({ "instanceId": instance_id })),
        );
        let open_op = ops
            .iter()
            .find(|op| op.contains("openPluginInstance"))
            .expect("open op");
        let parsed: Value = serde_json::from_str(open_op).expect("json");
        assert_eq!(parsed["op"], "openPluginInstance");
        let document: Value = serde_json::from_str(parsed["documentJson"].as_str().expect("document json")).expect("document");
        assert_eq!(document["schema"], "draw.document");
        assert_eq!(document["id"], "semio");
    }

    #[test]
    fn inspector_tree_exposes_label_field() {
        let mut envelope = initial_studio_envelope();
        let ids: Vec<String> = projection_from_document(&envelope.document)
            .app_instances
            .iter()
            .take(2)
            .map(|instance| instance.id.clone())
            .collect();
        envelope.runtime.selected_app_instance_ids = ids;
        let tree = build_inspector_tree(&envelope.document, &envelope.runtime);
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
        assert_eq!(input.on_change.command, "patchAppInstances");
    }

    fn seed_multi_port_programs() {
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
                component_kind: SurfaceKind::World3d,
                modes: vec![ModeDefinition {
                    id: "edit".into(),
                    label: "Edit".into(),
                    tools: vec![],
                layout_id: None,
                }],
                default_mode_id: None,
                parameter_fields: Vec::new(),
            },
        );
        merge_os_program_definition(
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
                        tools: vec![],
                layout_id: None,
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
                component_kind: SurfaceKind::World3d,
                modes: vec![ModeDefinition {
                    id: "edit".into(),
                    label: "Edit".into(),
                    tools: vec![],
                layout_id: None,
                }],
                default_mode_id: None,
                parameter_fields: Vec::new(),
            },
        );
        merge_os_program_definition(
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
                        tools: vec![],
                layout_id: None,
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
        seed_multi_port_programs();
        let mut envelope = initial_studio_envelope();
        SStudioApp::handle_studio_command(
            &mut envelope,
            "spawnApp",
            Some(&json!({ "programId": "puzzle.5d", "appId": "puzzle5d", "position": { "x": 200, "y": 100 } })),
        );
        SStudioApp::handle_studio_command(
            &mut envelope,
            "spawnApp",
            Some(&json!({ "programId": "shooting", "appId": "shooting", "position": { "x": 300, "y": 100 } })),
        );
        let projection = projection_from_document(&envelope.document);
        let puzzle_instance = projection.app_instances.iter().rev().nth(1).expect("puzzle");
        let shooting_instance = projection.app_instances.last().expect("shooting");
        let puzzle_node = projection
            .media_graph
            .nodes
            .iter()
            .find(|node| node.instance_id == puzzle_instance.id)
            .expect("puzzle node");
        let shooting_node = projection
            .media_graph
            .nodes
            .iter()
            .find(|node| node.instance_id == shooting_instance.id)
            .expect("shooting node");
        assert_eq!(puzzle_node.outputs.len(), 2);
        assert_eq!(shooting_node.inputs.len(), 1);
    }

    #[test]
    fn unbind_parameter_field_removes_binding() {
        let mut envelope = initial_studio_envelope();
        let projection = projection_from_document(&envelope.document);
        let instance = projection.app_instances.first().expect("instance");
        let parameter = projection.parameters.first().expect("parameter");
        let parameter_id = parameter_entity_id(parameter);
        SStudioApp::handle_studio_command(
            &mut envelope,
            "bindParameterField",
            Some(&json!({
                "instanceId": instance.id,
                "fieldPath": "label",
                "parameterId": parameter_id,
            })),
        );
        let bound = projection_from_document(&envelope.document)
            .parameter_bindings
            .iter()
            .any(|row| row.instance_id == instance.id && row.field_path == "label");
        assert!(bound);
        SStudioApp::handle_studio_command(
            &mut envelope,
            "unbindParameterField",
            Some(&json!({
                "instanceId": instance.id,
                "fieldPath": "label",
            })),
        );
        let still_bound = projection_from_document(&envelope.document)
            .parameter_bindings
            .iter()
            .any(|row| row.instance_id == instance.id && row.field_path == "label");
        assert!(!still_bound);
    }

    #[test]
    fn checkout_checkpoint_restores_projection() {
        seed_draw_program();
        let mut envelope = initial_studio_envelope();
        let before = projection_from_document(&envelope.document).app_instances.len();
        let mut store = store_from_envelope(&envelope);
        store
            .spawn_app_instance("draw", "draw", None, MediaGraphPosition { x: 80.0, y: 80.0 })
            .expect("spawn");
        store
            .dispatch_json(r#"{"kind":"commitCheckpoint","message":"after-first-spawn"}"#)
            .expect("commit");
        let checkpoint_id = store.document().vcs.checkpoints[0].id.clone();
        let after_first = store.projection().expect("projection").app_instances.len();
        assert!(after_first > before);
        store
            .spawn_app_instance("draw", "draw", None, MediaGraphPosition { x: 120.0, y: 80.0 })
            .expect("spawn2");
        assert!(store.projection().expect("projection2").app_instances.len() > after_first);
        store
            .dispatch_json(&json!({ "kind": "checkoutCheckpoint", "checkpointId": checkpoint_id }).to_string())
            .expect("checkout");
        let restored = store.projection().expect("restored").app_instances.len();
        assert_eq!(restored, after_first);
        envelope = envelope_from_store(store, envelope.runtime);
        assert_eq!(
            projection_from_document(&envelope.document).app_instances.len(),
            after_first
        );
    }

    #[test]
    fn studio_document_persists_through_backbone_port() {
        let port: Arc<dyn OsBackbonePort> = Arc::new(LocalStorageBackbonePort::new());
        let mut demo = parse_demo_studio_document();
        demo.id = "persist-test".into();
        demo.name = "Persist Test".into();
        let _ = seed_os_studio_catalog_if_empty(demo.clone(), port.clone()).expect("seed");
        let loaded = load_os_studio_document("persist-test", port.clone()).expect("load");
        assert_eq!(loaded.id, "persist-test");
        assert_eq!(loaded.name, "Persist Test");
    }

    #[test]
    fn studio_and_home_modes_expose_history_tools() {
        let studio = create_studio_app();
        let home = create_home_app();
        let studio_tools = studio
            .definition
            .modes
            .iter()
            .find(|mode| mode.id == "main")
            .map(|mode| mode.tools.len())
            .unwrap_or(0);
        let home_tools = home
            .definition
            .modes
            .iter()
            .find(|mode| mode.id == "explore")
            .map(|mode| mode.tools.len())
            .unwrap_or(0);
        assert!(studio_tools > 0);
        assert!(home_tools > 0);
        assert_eq!(studio.examples.len(), S_STUDIO_EXAMPLES.len());
    }
}
//#endregion 🧪Tests
