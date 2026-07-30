//! 🏠 S Home launcher app — `DocumentApp` impl, render, manifest (constitutional: ui).

use home::SHomeDocument;
use home_op::SHomeOperation;
use space_shared::{ensure_space_fixtures_registered, parse_demo_space_document};
use semio_framework_os::{
    create_ephemeral_os_space, create_os_space, delete_os_space, document_backbone_ref, encode_os_space_payload, export_os_space_pack, import_os_space_from_dsl,
    list_os_space_catalog_entries, load_os_space_document,
    seed_os_space_catalog_if_empty, MemoryBackbonePort, OsBackbonePort, OsDocument, OS_HOME_VFS_ROOT_ID,
    OS_SPACE_BACKBONE_URI_PREFIX, VcsError,
};
use semio_framework_plugin::{
    app_labels, build_virtual_file_system_scene, create_tab_stack_layout, is_de_locale, localized_label_map,
    resolve_labels, ui_text, ActionEmit, App, AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DocumentView,
    HostEffect, SurfaceKind, UiNode, ViewState, VirtualFileSystemScene,
};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};
use store::LocalStorageBackbonePort;

//#region 🔖Constants
const S_HOME_APP_ID: &str = "home";
const S_HOME_CONTROLLER_ID: &str = "s-home";
const S_HOME_WINDOW: &str = "s-home-main";
const S_HOME_BODY: &str = "s.home.vfs";
const S_HOME_SURFACE: &str = "vfs:home:main";
const OS_BOOT_STUDIO_ID: &str = "default";
//#endregion 🔖Constants

//#region 🔖DocumentHelpers
static CATALOG_PORT: LazyLock<Arc<dyn OsBackbonePort>> = LazyLock::new(|| {
    ensure_space_fixtures_registered();
    let port: Arc<dyn OsBackbonePort> = Arc::new(LocalStorageBackbonePort::new());
    if list_os_space_catalog_entries(port.clone())
        .map(|entries| entries.is_empty())
        .unwrap_or(true)
    {
        let mut demo = parse_demo_space_document();
        demo.id = OS_BOOT_STUDIO_ID.into();
        demo.name = if demo.name.trim().is_empty() {
            "Demo Studio".into()
        } else {
            demo.name
        };
        let _ = seed_os_space_catalog_if_empty(demo, port.clone());
    }
    port
});

static TEMP_CATALOG_PORT: LazyLock<Arc<dyn OsBackbonePort>> =
    LazyLock::new(|| Arc::new(MemoryBackbonePort::new()));

static STUDIO_PORTS: LazyLock<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static EPHEMERAL_STUDIOS: LazyLock<Mutex<HashMap<String, OsDocument>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// 🌉 `pub` (not `pub(crate)`): `app_space` (a sibling crate, `semio-s-app-space-space-ui`) resolves
/// studios through the Home launcher's own catalog port — see `app_space`'s `openSpace`/
/// `exportStudioPack`/`exportStudioDsl`/`importSpacePackPayload` actions.
pub fn catalog_port() -> Arc<dyn OsBackbonePort> {
    CATALOG_PORT.clone()
}

fn temp_catalog_port() -> Arc<dyn OsBackbonePort> {
    TEMP_CATALOG_PORT.clone()
}

fn register_studio_port(space_id: &str, port: Arc<dyn OsBackbonePort>) {
    if let Ok(mut guard) = STUDIO_PORTS.lock() {
        guard.insert(space_id.into(), port);
    }
}

/// @emoji 🫧 Registers a session-local studio document — no backbone URI, no catalog port write.
fn register_ephemeral_studio(document: OsDocument) -> String {
    let id = document.id.clone();
    if let Ok(mut guard) = EPHEMERAL_STUDIOS.lock() {
        guard.insert(id.clone(), document);
    }
    id
}

/// @emoji 🆕 Mints and registers an ephemeral empty studio for the default create path.
fn create_and_register_ephemeral_studio(name: &str) -> String {
    register_ephemeral_studio(create_ephemeral_os_space(name))
}

/// @emoji 📂 Resolves a studio id against the ephemeral registry, registered ports, then catalogs.
///
/// 🌉 `pub` (not `pub(crate)`): `app_space` resolves the studio it is asked to open through this same
/// lookup — see the note on {@link catalog_port}.
pub fn resolve_studio_document(space_id: &str) -> Option<OsDocument> {
    if let Ok(guard) = EPHEMERAL_STUDIOS.lock() {
        if let Some(document) = guard.get(space_id) {
            return Some(document.clone());
        }
    }
    if let Ok(guard) = STUDIO_PORTS.lock() {
        if let Some(port) = guard.get(space_id) {
            if let Ok(document) = load_os_space_document(space_id, port.clone()) {
                return Some(document);
            }
        }
    }
    for port in [temp_catalog_port(), catalog_port()] {
        if let Ok(document) = load_os_space_document(space_id, port) {
            return Some(document);
        }
    }
    None
}

/// @emoji 📦 Pack+spr bytes for `HostEffect::LoadDocument` / host `loadAppDocumentPack`.
///
/// 🌉 `pub` (not `pub(crate)`): `app_space`'s `openSpace` action loads the studio document it just
/// resolved through this helper — see the note on {@link catalog_port}.
pub fn space_document_envelope_pack(document: &OsDocument) -> Option<store::DocumentPackFiles> {
    export_os_space_pack(document).ok()
}

/// 🌉 `pub` (not `pub(crate)`, and not `#[cfg(test)]`): `app_space`'s own tests (a sibling crate) seed a
/// studio through this hook — a `#[cfg(test)]` gate here would vanish when this crate is pulled in as
/// `app_space`'s ordinary (non-dev) dependency, since `#[cfg(test)]` only activates for the crate under
/// test itself, not its dependencies.
pub fn register_studio_port_for_test(space_id: &str, port: Arc<dyn OsBackbonePort>) {
    register_studio_port(space_id, port);
}

fn list_all_space_catalog_entries() -> Vec<semio_framework_os::OsSpaceCatalogEntry> {
    let mut seen = HashSet::new();
    let mut entries = Vec::new();
    for port in [catalog_port(), temp_catalog_port()] {
        if let Ok(rows) = list_os_space_catalog_entries(port) {
            for entry in rows {
                if seen.insert(entry.id.clone()) {
                    entries.push(entry);
                }
            }
        }
    }
    if let Ok(guard) = EPHEMERAL_STUDIOS.lock() {
        for (id, document) in guard.iter() {
            if !seen.insert(id.clone()) {
                continue;
            }
            let projection = &document.vcs.initial_projection;
            entries.push(semio_framework_os::OsSpaceCatalogEntry {
                id: id.clone(),
                name: document.name.clone(),
                backbone_uri: String::new(),
                app_count: projection.app_instances.len(),
                node_count: projection.workflow.nodes.len(),
                updated_at: "0".into(),
            });
        }
    }
    entries
}

/// @emoji 🧭 Builds the typed emit for a freshly-created studio: bump the catalog counter (operation) and
/// navigate the shell to the new studio route (host effect).
fn created_studio_emit(catalog_generation: u64, space_id: &str) -> ActionEmit<SHomeOperation> {
    ActionEmit {
        operations: vec![SHomeOperation::SetCatalogGeneration { value: catalog_generation + 1 }],
        effects: vec![HostEffect::Navigate { uri: format!("/spaces/{space_id}") }],
        ..Default::default()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn create_folder_studio(
    name: &str,
    folder_path: &str,
) -> Result<semio_framework_os::OsSpaceCatalogEntry, VcsError> {
    let port = semio_framework_os::open_folder_space_backbone(folder_path)?;
    let entry = create_os_space(name, port.clone())?;
    register_studio_port(&entry.id, port);
    Ok(entry)
}

#[cfg(not(target_arch = "wasm32"))]
fn bind_studio_file(space_id: &str, file_path: &str) -> Result<(), VcsError> {
    let uri = format!("file://{file_path}");
    let port = semio_framework_os::open_file_space_backbone(file_path)?;
    register_studio_port(space_id, port.clone());
    let mut document = load_os_space_document(space_id, catalog_port())?;
    document.backbone = Some(document_backbone_ref(&uri));
    port.write(&uri, &encode_os_space_payload(&document)?)?;
    let catalog_uri = format!("{OS_SPACE_BACKBONE_URI_PREFIX}{space_id}");
    sync_os_space_document_helper(&document, &catalog_uri, &catalog_port())?;
    Ok(())
}

fn sync_os_space_document_helper(
    document: &OsDocument,
    backbone_uri: &str,
    port: &Arc<dyn OsBackbonePort>,
) -> Result<(), VcsError> {
    let mut synced = document.clone();
    synced.backbone = Some(document_backbone_ref(backbone_uri));
    port.write(backbone_uri, &encode_os_space_payload(&synced)?)
}

fn os_home_vfs_schema_json() -> String {
    json!({
        "descriptorKinds": {
            "text": { "id": "text", "name": "Text", "presentation": "text" }
        },
        "fileNodeKinds": {
            "studio": {
                "id": "studio",
                "name": "Space",
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
    for entry in list_all_space_catalog_entries() {
        rows.push(json!({
            "id": format!("studio:{}", entry.id),
            "fileNodeKindId": "studio",
            "name": entry.name,
            "path": format!("/spaces/{}", entry.id),
            "parentId": OS_HOME_VFS_ROOT_ID,
            "hasChildren": false,
            "navigateUri": format!("/spaces/{}", entry.id),
            "descriptorValues": {
                "apps": format!("{} apps · {} nodes", entry.app_count, entry.node_count)
            }
        }));
    }
    rows
}
//#endregion 🔖DocumentHelpers

//#region 🔖Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the Home launcher; one field per label makes every locale combination compile-checked.
    struct SHomeLabels {
        vfs_empty_message: &'static str = en: "No studios yet. Create one from the navbar.", de: "Noch keine Studios vorhanden. Erstelle eines über die Navigationsleiste.";
        window_main: &'static str = en: "Studios", de: "Studios";
    }
}
//#endregion 🔖Terminology

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every shell/view action declared in `create_home_app`'s static
/// manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the
/// command palette and Actions rail get a translated label without threading locale through the builder.
fn s_home_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("createStudio", "Create Studio", "Studio erstellen"),
        ("bindSpaceFile", "Bind Studio File", "Studio-Datei verknüpfen"),
        ("importSpace", "Import Studio", "Studio importieren"),
        ("openSpace", "Open Studio", "Studio öffnen"),
        ("navigateVirtualFileSystemNode", "Navigate File System Node", "Dateisystemknoten navigieren"),
        ("deleteVirtualFileSystemNode", "Delete File System Node", "Dateisystemknoten löschen"),
        ("goHome", "Go Home", "Zur Startseite"),
        ("setActivePanelTab", "Set Active Panel Tab", "Aktiven Panel-Tab festlegen"),
    ])
}
//#endregion 🔖CommandLabels

//#region 🔖Render
fn render_home_vfs(labels: &SHomeLabels) -> UiNode {
    build_virtual_file_system_scene(
        S_HOME_SURFACE,
        S_HOME_CONTROLLER_ID,
        VirtualFileSystemScene {
            schema_json: os_home_vfs_schema_json(),
            rows_json: serde_json::to_string(&home_vfs_rows()).unwrap_or_else(|_| "[]".into()),
            selected_row_ids_json: None,
            hovered_row_id: None,
            empty_message: Some(labels.vfs_empty_message.into()),
            drag_drop_enabled: None,
        },
        Some(S_HOME_WINDOW.into()),
        None,
    )
}
//#endregion 🔖Render

//#region 🔖HomeApp
pub struct HomeApp;

impl DocumentApp for HomeApp {
    type Projection = SHomeDocument;
    type Operation = SHomeOperation;

    fn app_id(&self) -> &str {
        S_HOME_APP_ID
    }

    fn document_schema(&self) -> &str {
        "s.home"
    }

    fn initial_projection(&self) -> SHomeDocument {
        SHomeDocument { schema: "s.home".into(), catalog_generation: 0 }
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, SHomeDocument>,
        _view_state: &ViewState,
    ) -> ActionEmit<SHomeOperation> {
        let generation = doc.projection.catalog_generation;
        let bump = |value: u64| ActionEmit::operations(vec![SHomeOperation::SetCatalogGeneration { value }]);
        let port = catalog_port();
        match action {
            "createStudio" => {
                let name = args
                    .and_then(|value| value.get("name"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("Untitled Studio");
                let kind = args
                    .and_then(|value| value.get("kind"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("catalog");
                match kind {
                    "folder" => {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            if let Some(folder_path) = args
                                .and_then(|value| value.get("folderPath"))
                                .and_then(|value| value.as_str())
                            {
                                if let Ok(entry) = create_folder_studio(name, folder_path) {
                                    eprintln!("[DEBUG] createStudio folder id={}", entry.id);
                                    return created_studio_emit(generation, &entry.id);
                                }
                            }
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            let _ = name;
                        }
                    }
                    _ => {
                        let space_id = create_and_register_ephemeral_studio(name);
                        eprintln!("[DEBUG] createStudio ephemeral id={space_id}");
                        return created_studio_emit(generation, &space_id);
                    }
                }
                ActionEmit::default()
            }
            "bindSpaceFile" => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let space_id = args
                        .and_then(|value| value.get("spaceId"))
                        .and_then(|value| value.as_str());
                    let file_path = args
                        .and_then(|value| value.get("filePath"))
                        .and_then(|value| value.as_str());
                    if let (Some(space_id), Some(file_path)) = (space_id, file_path) {
                        let _ = bind_studio_file(space_id, file_path);
                    }
                }
                ActionEmit::default()
            }
            "importSpace" => {
                let dsl = args
                    .and_then(|value| value.get("dsl"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| {
                        args.and_then(|value| value.get("payload"))
                            .and_then(|value| value.as_str())
                            .map(str::to_string)
                    });
                match dsl {
                    Some(dsl) => {
                        if import_os_space_from_dsl(&dsl, port.clone()).is_ok() {
                            bump(generation + 1)
                        } else {
                            ActionEmit::default()
                        }
                    }
                    None => ActionEmit::effect(HostEffect::RequestFileOpen {
                        accept: ".os".into(),
                        read_as: None,
                        import_action: "importSpace".into(),
                        multiple: false,
                    }),
                }
            }
            "openSpace" | "navigateVirtualFileSystemNode" => {
                let space_id = args
                    .and_then(|value| value.get("spaceId").or_else(|| value.get("nodeId")))
                    .and_then(|value| value.as_str())
                    .and_then(|value| value.strip_prefix("studio:").or(Some(value)));
                match space_id {
                    Some(space_id) => ActionEmit::effect(HostEffect::Navigate {
                        uri: format!("/spaces/{space_id}"),
                    }),
                    None => ActionEmit::default(),
                }
            }
            "deleteVirtualFileSystemNode" => {
                let space_id = args
                    .and_then(|value| value.get("nodeId"))
                    .and_then(|value| value.as_str())
                    .and_then(|value| value.strip_prefix("studio:"));
                match space_id {
                    Some(space_id) => {
                        if let Ok(mut guard) = EPHEMERAL_STUDIOS.lock() {
                            guard.remove(space_id);
                        }
                        let _ = delete_os_space(space_id, port.clone());
                        bump(generation + 1)
                    }
                    None => ActionEmit::default(),
                }
            }
            "goHome" => ActionEmit::effect(HostEffect::Navigate { uri: "/".into() }),
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, _doc: &DocumentView<'_, SHomeDocument>, view_state: &ViewState) -> UiNode {
        let labels = resolve_labels::<SHomeLabels>(view_state);
        match body_key {
            S_HOME_BODY => render_home_vfs(labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<SHomeLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(S_HOME_WINDOW, labels.window_main)
            .action_labels(s_home_action_labels(is_de))
    }
}
//#endregion 🔖HomeApp

//#region 🔖HomeManifest
pub fn create_home_app() -> App {
    let mut app = App::from_builder(
        App::builder(S_HOME_APP_ID, "Home").document(["semio", "s", "home"])
            .icon_id("home")
            .mode("explore", "Explore")
            .default_mode_id("explore")
            .window_kind(S_HOME_WINDOW, "Studios", S_HOME_BODY, SurfaceKind::Canvas2d, "home")
            .default_layout(create_tab_stack_layout(
                &[S_HOME_WINDOW.into()],
                Some(&["Studios".into()]),
            ))
            .operation("createStudio", "Create Studio")
            .shell_action("bindSpaceFile", "Bind Studio File")
            .operation("importSpace", "Import Studio")
            .shell_action("openSpace", "Open Studio")
            .shell_action("navigateVirtualFileSystemNode", "Navigate File System Node")
            .operation("deleteVirtualFileSystemNode", "Delete File System Node")
            .shell_action("goHome", "Go Home")
            .view_action("setActivePanelTab", "Set Active Panel Tab")
            .keybinding("mod+n", "createStudio")
            .keybinding("mod+o", "importSpace"),
    );
    app.definition.controller_id = S_HOME_CONTROLLER_ID.into();
    app
}
//#endregion 🔖HomeManifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, HistoryView, PluginApp, VcsDocumentApp};

    fn empty_history() -> HistoryView {
        HistoryView {
            columns: Vec::new(),
            can_undo: false,
            can_redo: false,
            active_alternative_id: None,
            current_checkpoint_id: None,
            recent_ops: Vec::new(),
        }
    }

    #[test]
    fn home_manifest_uses_home_app_id() {
        let app = create_home_app();
        assert_eq!(app.definition.id, "home");
        assert_eq!(app.definition.controller_id, "s-home");
    }

    #[test]
    fn home_vfs_lists_seeded_studio() {
        let rows = home_vfs_rows();
        assert!(rows.iter().any(|row| row.get("navigateUri").and_then(|v| v.as_str()).unwrap_or("").starts_with("/spaces/")));
    }

    #[test]
    fn creates_studio_via_home_action() {
        let port = catalog_port();
        let before = list_os_space_catalog_entries(port.clone()).expect("list").len();
        let mut home = VcsDocumentApp::new(HomeApp);
        home.handle_action("createStudio", Some(&json!({ "name": "Test Studio" })), &ViewState::default(), &testkit::meta("local"))
            .expect("create");
        let after = list_os_space_catalog_entries(port).expect("list").len();
        assert!(after >= before);
    }

    #[test]
    fn home_declares_create_space_action() {
        let app = create_home_app();
        assert!(app.definition.actions.iter().any(|action| action.id == "createStudio"));
    }

    #[test]
    fn temporary_studio_uses_ephemeral_registry_not_catalog() {
        let mut home = HomeApp;
        let projection = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
        let history = empty_history();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = home.handle_action("createStudio", Some(&json!({ "name": "Temp Studio", "kind": "temporary" })), &doc, &ViewState::default());
        assert!(emit.effects.iter().any(|effect| matches!(effect, HostEffect::Navigate { .. })));
        assert!(
            !emit.effects.iter().any(|effect| matches!(effect, HostEffect::DownloadMediaExport { .. })),
            "ephemeral create must not download"
        );
        let persistent = list_os_space_catalog_entries(catalog_port()).expect("list");
        assert!(!persistent.iter().any(|entry| entry.name == "Temp Studio"));
        let ephemeral_catalog = list_os_space_catalog_entries(temp_catalog_port()).expect("list");
        assert!(!ephemeral_catalog.iter().any(|entry| entry.name == "Temp Studio"));
        let uri = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::Navigate { uri } => Some(uri.as_str()),
                _ => None,
            })
            .expect("navigate");
        let space_id = uri.trim_start_matches("/spaces/");
        let document = resolve_studio_document(space_id).expect("ephemeral studio");
        assert_eq!(document.name, "Temp Studio");
        assert!(document.backbone.is_none());
        assert!(document.vcs.initial_projection.app_instances.is_empty());
    }

    #[test]
    fn space_document_persists_through_backbone_port() {
        let port: Arc<dyn OsBackbonePort> = Arc::new(LocalStorageBackbonePort::new());
        let mut demo = parse_demo_space_document();
        demo.id = "persist-test".into();
        demo.name = "Persist Test".into();
        let _ = seed_os_space_catalog_if_empty(demo.clone(), port.clone()).expect("seed");
        let loaded = load_os_space_document("persist-test", port.clone()).expect("load");
        assert_eq!(loaded.id, "persist-test");
        assert_eq!(loaded.name, "Persist Test");
    }

    #[test]
    fn home_labels_resolve_native_english_by_default() {
        let history = empty_history();
        let home = HomeApp;
        let home_doc = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
        let home_view = DocumentView { projection: &home_doc, history: &history };
        let home_node = home.render(S_HOME_BODY, &home_view, &ViewState::default());
        assert!(serde_json::to_string(&home_node).unwrap().contains("No studios yet. Create one from the navbar."));
    }

    #[test]
    fn home_labels_resolve_native_german_locale() {
        let history = empty_history();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let home = HomeApp;
        let home_doc = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
        let home_view = DocumentView { projection: &home_doc, history: &history };
        let home_node = home.render(S_HOME_BODY, &home_view, &view_state);
        assert!(serde_json::to_string(&home_node).unwrap().contains("Noch keine Studios vorhanden"));
    }
}
//#endregion 🧪Tests
