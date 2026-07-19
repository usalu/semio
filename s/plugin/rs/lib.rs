//! 🎛️ S Studio plugin — designer OS shell bundled as a hot-swappable WASM component.

use semio_framework_os::{register_os_fixture_json, OsDocument, OsProjection};
use serde::Deserialize;
use std::sync::LazyLock;
use vcs::{create_document_vcs_envelope, DocumentBackboneRef};

//#region 🔖Constants
const DEMO_STUDIO_JSON: &str = include_str!("../../example/demo.s.json");
//#endregion 🔖Constants

//#region 🔖DocumentHelpers
/// 🧵 Registers the draw/writer fixture documents referenced by the demo studio's app instances —
/// shared by the Home launcher's catalog seed ({@link app_home}) and the Studio app's media export
/// path ({@link app_studio}), both of which need these fixtures resolvable before they touch a studio
/// document that references them.
fn ensure_studio_fixtures_registered() {
    static FIXTURES: LazyLock<()> = LazyLock::new(|| {
        register_os_fixture_json("semio.draw.json", include_str!("../../../draw/example/semio.draw.json"));
        register_os_fixture_json("jack.writer.json", include_str!("../../../writer/example/jack.writer.json"));
    });
    let _ = &*FIXTURES;
}

/// 🌱 Parses the packaged demo studio fixture into a full `OsDocument` envelope — shared by the Home
/// launcher's catalog seed ({@link app_home}) and the Studio app's `initial_projection` ({@link
/// app_studio}).
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
        backbone: Option<DocumentBackboneRef>,
    }
    let demo: DemoFile = serde_json::from_str(DEMO_STUDIO_JSON).expect("demo studio json");
    let envelope = create_document_vcs_envelope(
        &demo.schema,
        &demo.id,
        demo.vcs.initial_projection,
        demo.backbone.as_ref().cloned(),
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

/// @emoji 🌱 The demo studio's bare `OsProjection` — the studio app's `initial_projection`, parsed
/// straight out of the packaged fixture (no envelope/runtime wrapper).
fn demo_studio_projection() -> OsProjection {
    demo_os_document().vcs.initial_projection
}
//#endregion 🔖DocumentHelpers

//#region 🔖app_home
pub mod app_home {
    //! 🏠 S Home launcher — lists/creates/imports/deletes studios against the shared catalog backbone.

    use super::{ensure_studio_fixtures_registered, parse_demo_studio_document};
    use semio_framework_os::{
        create_os_studio, delete_os_studio, document_backbone_ref, import_os_studio_from_json,
        list_os_studio_catalog_entries, load_os_studio_document, os_document_to_json,
        seed_os_studio_catalog_if_empty, MemoryBackbonePort, OsBackbonePort, OsDocument, OS_HOME_VFS_ROOT_ID,
        OS_STUDIO_BACKBONE_URI_PREFIX, VcsError,
    };
    use semio_framework_plugin::{
        app_labels, build_virtual_file_system_scene, create_tab_stack_layout, is_de_locale, localized_label_map,
        resolve_labels, ui_text, ActionEmit, App, AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DocumentView,
        HostEffect, SurfaceKind, UiNode, ViewState, VirtualFileSystemScene,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, LazyLock, Mutex};
    use vcs::LocalStorageBackbonePort;

    //#region 🔖Constants
    const S_HOME_APP_ID: &str = "home";
    const S_HOME_CONTROLLER_ID: &str = "s-home";
    const S_HOME_WINDOW: &str = "s-home-main";
    const S_HOME_BODY: &str = "s.home.vfs";
    const S_HOME_SURFACE: &str = "vfs:home:main";
    const OS_BOOT_STUDIO_ID: &str = "default";
    //#endregion 🔖Constants

    //#region 🔖Types
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SHomeDocument {
        schema: String,
        #[serde(default)]
        catalog_generation: u64,
    }

    /// @emoji 🔢 The Home launcher's only document op: pins the catalog-generation counter that forces a
    /// re-materialize of the studio list after a create/import/delete side-effect on the catalog port.
    /// It is its own {@link vcs::OperationDiff} (idempotent set), so forward/backward are symmetric.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "op", rename_all = "camelCase")]
    pub enum SHomeOp {
        /// 🫙 The identity op — an `OperationDiff` needs `Default`; never emitted by `handle_action`.
        #[default]
        Noop,
        SetCatalogGeneration { value: u64 },
    }

    impl vcs::OperationDiff<SHomeDocument> for SHomeOp {
        fn apply(&self, projection: &SHomeDocument) -> SHomeDocument {
            match self {
                SHomeOp::Noop => projection.clone(),
                SHomeOp::SetCatalogGeneration { value } => {
                    SHomeDocument { catalog_generation: *value, ..projection.clone() }
                }
            }
        }

        fn absorb(&mut self, other: Self) {
            if !matches!(other, SHomeOp::Noop) {
                *self = other;
            }
        }
    }

    impl vcs::Operation<SHomeDocument> for SHomeOp {
        type Diff = SHomeOp;

        fn diff(&self, _projection: &SHomeDocument) -> SHomeOp {
            self.clone()
        }

        fn backwards(&self, projection: &SHomeDocument) -> Vec<Self> {
            vec![SHomeOp::SetCatalogGeneration { value: projection.catalog_generation }]
        }
    }
    //#endregion 🔖Types

    //#region 🔖DocumentHelpers
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

    /// @emoji 🧭 Builds the typed emit for a freshly-created studio: bump the catalog counter (op) and
    /// navigate the shell to the new studio route (host effect).
    fn created_studio_emit(catalog_generation: u64, studio_id: &str) -> ActionEmit<SHomeOp> {
        ActionEmit {
            ops: vec![SHomeOp::SetCatalogGeneration { value: catalog_generation + 1 }],
            effects: vec![HostEffect::Navigate { uri: format!("/studios/{studio_id}") }],
            ..Default::default()
        }
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
        let uri = format!("file://{file_path}");
        let port = semio_framework_os::open_file_studio_backbone(file_path)?;
        register_studio_port(studio_id, port.clone());
        let mut document = load_os_studio_document(studio_id, catalog_port())?;
        document.backbone = Some(document_backbone_ref(&uri));
        port.write(&uri, &os_document_to_json(&document)?)?;
        let catalog_uri = format!("{OS_STUDIO_BACKBONE_URI_PREFIX}{studio_id}");
        sync_os_studio_document_helper(&document, &catalog_uri, &catalog_port())?;
        Ok(())
    }

    fn sync_os_studio_document_helper(
        document: &OsDocument,
        backbone_uri: &str,
        port: &Arc<dyn OsBackbonePort>,
    ) -> Result<(), VcsError> {
        let mut synced = document.clone();
        synced.backbone = Some(document_backbone_ref(backbone_uri));
        port.write(backbone_uri, &os_document_to_json(&synced)?)
    }

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
    //#endregion 🔖DocumentHelpers

    //#region 🔖Terminology
    app_labels! {
        /// 🗣️ Complete UI label set for the Home launcher; one field per label makes every locale combination compile-checked.
        struct SHomeLabels {
            vfs_empty_message: &'static str = en: "No studios yet. Create one from the toolbar.", de: "Noch keine Studios vorhanden. Erstelle eines ueber die Werkzeugleiste.";
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
            ("bindStudioFile", "Bind Studio File", "Studio-Datei verknuepfen"),
            ("importStudio", "Import Studio", "Studio importieren"),
            ("openStudio", "Open Studio", "Studio oeffnen"),
            ("navigateVirtualFileSystemNode", "Navigate File System Node", "Dateisystemknoten navigieren"),
            ("deleteVirtualFileSystemNode", "Delete File System Node", "Dateisystemknoten loeschen"),
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
        type Op = SHomeOp;

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
        ) -> ActionEmit<SHomeOp> {
            let generation = doc.projection.catalog_generation;
            let bump = |value: u64| ActionEmit::ops(vec![SHomeOp::SetCatalogGeneration { value }]);
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
                        .unwrap_or("file");
                    match kind {
                        "temporary" => {
                            if let Ok(entry) = create_os_studio(name, temp_catalog_port()) {
                                return created_studio_emit(generation, &entry.id);
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
                            if let Ok(entry) = create_os_studio(name, port.clone()) {
                                let mut emit = created_studio_emit(generation, &entry.id);
                                if let Ok(studio_document) = load_os_studio_document(&entry.id, port.clone()) {
                                    if let Ok(json) = os_document_to_json(&studio_document) {
                                        emit.effects.insert(
                                            0,
                                            HostEffect::DownloadMediaExport {
                                                filename: format!("{}.studio.json", entry.name.replace(' ', "-")),
                                                mime_type: "application/json".into(),
                                                data: json,
                                                encoding: None,
                                            },
                                        );
                                    }
                                }
                                return emit;
                            }
                        }
                    }
                    ActionEmit::default()
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
                    ActionEmit::default()
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
                    match json {
                        Some(json) => {
                            if import_os_studio_from_json(&json, port.clone()).is_ok() {
                                bump(generation + 1)
                            } else {
                                ActionEmit::default()
                            }
                        }
                        None => ActionEmit::effect(HostEffect::RequestFileOpen {
                            accept: ".json".into(),
                            read_as: None,
                            import_action: "importStudio".into(),
                            multiple: false,
                        }),
                    }
                }
                "openStudio" | "navigateVirtualFileSystemNode" => {
                    let studio_id = args
                        .and_then(|value| value.get("studioId").or_else(|| value.get("nodeId")))
                        .and_then(|value| value.as_str())
                        .and_then(|value| value.strip_prefix("studio:").or(Some(value)));
                    match studio_id {
                        Some(studio_id) => ActionEmit::effect(HostEffect::Navigate {
                            uri: format!("/studios/{studio_id}"),
                        }),
                        None => ActionEmit::default(),
                    }
                }
                "deleteVirtualFileSystemNode" => {
                    let studio_id = args
                        .and_then(|value| value.get("nodeId"))
                        .and_then(|value| value.as_str())
                        .and_then(|value| value.strip_prefix("studio:"));
                    match studio_id {
                        Some(studio_id) => {
                            let _ = delete_os_studio(studio_id, port.clone());
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
                .window_kind(S_HOME_WINDOW, "Studios", S_HOME_BODY, SurfaceKind::Canvas2d)
                .default_layout(create_tab_stack_layout(
                    &[S_HOME_WINDOW.into()],
                    Some(&["Studios".into()]),
                ))
                .operation("createStudio", "Create Studio")
                .shell_action("bindStudioFile", "Bind Studio File")
                .operation("importStudio", "Import Studio")
                .shell_action("openStudio", "Open Studio")
                .shell_action("navigateVirtualFileSystemNode", "Navigate File System Node")
                .operation("deleteVirtualFileSystemNode", "Delete File System Node")
                .shell_action("goHome", "Go Home")
                .view_action("setActivePanelTab", "Set Active Panel Tab")
                .keybinding("mod+n", "createStudio")
                .keybinding("mod+o", "importStudio"),
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
            assert!(rows.iter().any(|row| row.get("navigateUri").and_then(|v| v.as_str()).unwrap_or("").starts_with("/studios/")));
        }

        #[test]
        fn creates_studio_via_home_action() {
            let port = catalog_port();
            let before = list_os_studio_catalog_entries(port.clone()).expect("list").len();
            let mut home = VcsDocumentApp::new(HomeApp);
            home.handle_action("createStudio", Some(&json!({ "name": "Test Studio" })), &ViewState::default(), &testkit::meta("local"))
                .expect("create");
            let after = list_os_studio_catalog_entries(port).expect("list").len();
            assert!(after >= before);
        }

        #[test]
        fn home_declares_create_studio_action() {
            let app = create_home_app();
            assert!(app.definition.actions.iter().any(|action| action.id == "createStudio"));
        }

        #[test]
        fn temporary_studio_uses_ephemeral_port() {
            let mut home = HomeApp;
            let projection = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
            let history = empty_history();
            let doc = DocumentView { projection: &projection, history: &history };
            let emit = home.handle_action("createStudio", Some(&json!({ "name": "Temp Studio", "kind": "temporary" })), &doc, &ViewState::default());
            assert!(emit.effects.iter().any(|effect| matches!(effect, HostEffect::Navigate { .. })));
            let persistent = list_os_studio_catalog_entries(catalog_port()).expect("list");
            assert!(!persistent.iter().any(|entry| entry.name == "Temp Studio"));
            let ephemeral = list_os_studio_catalog_entries(temp_catalog_port()).expect("list");
            assert!(ephemeral.iter().any(|entry| entry.name == "Temp Studio"));
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
        fn home_labels_resolve_native_english_by_default() {
            let history = empty_history();
            let home = HomeApp;
            let home_doc = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
            let home_view = DocumentView { projection: &home_doc, history: &history };
            let home_node = home.render(S_HOME_BODY, &home_view, &ViewState::default());
            assert!(serde_json::to_string(&home_node).unwrap().contains("No studios yet. Create one from the toolbar."));
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
}
//#endregion 🔖app_home

//#region 🔖app_studio
pub mod app_studio {
    //! 🎛️ S Studio — the media-graph composition app hosting spawned app instances, parameters, and
    //! their compiled DAG.

    use super::{demo_studio_projection, ensure_studio_fixtures_registered};
    use semio_framework_os::{
        apply_flow_fixture_to_os_media_graph, build_os_media_flow_operator_infos, create_default_os_parameter,
        create_os_document_id, create_os_id, list_os_media_graph_vfs_children, list_os_programs,
        materialize_os_app_instance_document_json, media_port_spec_id, negotiate_media_contract, os_app_primary_output_kind,
        os_app_registration, os_media_graph_to_flow_fixture, os_media_graph_to_node_graph_payload,
        os_media_graph_vfs_schema, os_parameter_types_compatible, os_parameter_value, parameter_id_from_port_id,
        patch_os_parameter, MediaGraphPosition, OsAppInstance, OsDocumentRef, OsMediaGraphCamera,
        OsMediaGraphVfsNodeRecord, OsMediaPort, OsOp, OsParameter, OsParameterFieldBinding, OsParameterType, OsProjection,
        OS_MEDIA_GRAPH_VFS_ROOT_ID, OS_STUDIO_SCHEMA,
    };
    use semio_framework_plugin::{
        app_labels, build_node_graph_scene, build_text_editor_scene, build_virtual_file_system_scene,
        create_default_layout, host_now_ms, is_de_locale, localized_label_map, resolve_labels, tree_item_desc,
        ui_declarative_sections_to_tree, ui_inspector_all_equal, ui_text, MeasureSelectItem, WindowEngagementStatus,
        ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionEmit, ActionKind, App,
        AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DocumentView, HostEffect, NodeGraphScene, PanelGroup,
        PanelTreeBuilder, SurfaceKind, TextEditorScene, UiButtonNode, UiFieldNode, UiInputNode, UiNode,
        UiNumberStepperNode, UiSectionNode, UiSelectItem, UiSelectNode, UiToggleNode, UiTreeItemNode, ViewState,
        VirtualFileSystemScene, WindowEngagement, WindowEngagementInput, WindowEngagementSlot, WindowLayout,
        WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
        FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{BTreeMap, HashMap, HashSet};
    use std::sync::{LazyLock, Mutex};
    use infinite_board_port_directed_dag::{
        dag_fixture_to_wire_literal, DagCamera, DagFixture, DagFixtureEdge, DagNodeKind, DagNodeSpec, IoPortSpec,
    };

    //#region 🔖Constants
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

    const S_STUDIO_EXAMPLES: &[(&str, &str, &str)] = &[("demo", "Demo Studio", super::DEMO_STUDIO_JSON)];
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
        #[serde(skip_serializing_if = "Option::is_none")]
        media_graph_camera: Option<OsMediaGraphCamera>,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pending_import_instance_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pending_import_format: Option<String>,
    }
    //#endregion 🔖Types

    //#region 🔖DocumentHelpers
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

    /// @emoji 🗂️ Serializes a panel state for a typed {@link HostEffect::SetPanel} effect.
    fn panel_json(panel: &StudioPanelState) -> String {
        serde_json::to_string(panel).unwrap_or_else(|_| "{}".into())
    }

    fn s_play_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor {
            controller_id: S_PLAY_CONTROLLER_ID.into(),
            action: action.into(),
            args,
        }
    }

    fn parameter_entity_id(parameter: &OsParameter) -> &str {
        match parameter {
            OsParameter::Numeric { id, .. }
            | OsParameter::Categorical { id, .. }
            | OsParameter::Toggle { id, .. }
            | OsParameter::Text { id, .. } => id,
        }
    }

    /// @emoji ✨ Builds the `SpawnAppInstance` op (minting a deterministic instance id + app-document id
    /// embedded in the op, so replay never re-mints) plus the new instance id for the caller to focus.
    /// The store-free op-builder the plugin uses in place of os-core's `OsStore::spawn_app_instance`
    /// (a `DocumentApp` owns no store — its wrapper does).
    fn spawn_app_instance_op(
        program_id: &str,
        app_id: &str,
        label: Option<&str>,
        position: MediaGraphPosition,
    ) -> Option<(OsOp, String)> {
        let registration = os_app_registration(program_id, app_id)?;
        let instance_id = create_os_id("app");
        let instance = OsAppInstance {
            id: instance_id.clone(),
            program_id: program_id.into(),
            app_id: app_id.into(),
            label: label.map(str::to_string).unwrap_or_else(|| registration.label.clone()),
            yields: os_app_primary_output_kind(&registration),
            document: OsDocumentRef {
                document_id: create_os_document_id(),
                schema: registration.source_format.clone(),
            },
        };
        Some((OsOp::SpawnAppInstance { instance, position }, instance_id))
    }

    /// @emoji ➕ Builds an `AddParameter` op with a fresh default parameter of the requested type.
    fn add_parameter_op(parameter_type: &OsParameterType, name: &str) -> OsOp {
        OsOp::AddParameter {
            parameter: create_default_os_parameter(parameter_type, name, None),
        }
    }

    /// @emoji 🩹 Builds a `PatchParameter` op by folding `patch` (a `{field: value}` object) into the
    /// current parameter — the store-free op-builder used in place of os-core's `OsStore::patch_parameter`.
    fn patch_parameter_op(projection: &OsProjection, parameter_id: &str, patch: &Value) -> Option<OsOp> {
        let current = projection
            .parameters
            .iter()
            .find(|parameter| parameter_entity_id(parameter) == parameter_id)?;
        Some(OsOp::PatchParameter {
            parameter_id: parameter_id.into(),
            parameter: patch_os_parameter(current, patch),
        })
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

    /// @emoji 🤝 Resolves the source/target `OsMediaPort`s for a proposed connect from the live projection
    /// and negotiates their wire contract — shared by both connect entry points (`"connectMediaPorts"` and
    /// the `nodeGraphEdit`/`"connect"` fixture edit) so neither can push an `OsOp::ConnectMediaPorts` for an
    /// incompatible or unresolved pair of ports.
    fn negotiate_media_connect(projection: &OsProjection, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<semio_framework_os::MediaContract, String> {
        let source_port: &OsMediaPort = projection
            .media_graph
            .nodes
            .iter()
            .find(|node| node.id == source_node_id)
            .and_then(|node| node.outputs.iter().find(|port| port.id == source_port_id))
            .ok_or_else(|| format!("unknown source port {source_node_id}:{source_port_id}"))?;
        let target_port: &OsMediaPort = projection
            .media_graph
            .nodes
            .iter()
            .find(|node| node.id == target_node_id)
            .and_then(|node| node.inputs.iter().find(|port| port.id == target_port_id))
            .ok_or_else(|| format!("unknown target port {target_node_id}:{target_port_id}"))?;
        negotiate_media_contract(source_port, target_port)
    }

    fn media_graph_context_menu_json(labels: &SStudioLabels) -> String {
        json!([
            { "id": "open-instance", "label": labels.context_open_instance, "action": "openInstance" },
            { "id": "duplicate-instance", "label": labels.context_duplicate, "action": "duplicateAppInstance" },
            { "id": "copy-instance", "label": labels.context_copy, "action": "copyAppInstance" },
            { "id": "paste-instance", "label": labels.context_paste, "action": "pasteAppInstance" },
            { "id": "rename-instance", "label": labels.context_rename_label, "action": "renameAppInstance" },
            { "id": "remove-instance", "label": labels.context_remove, "action": "removeAppInstance" },
            { "id": "select-all", "label": labels.context_select_all, "action": "setMediaNodeSelection", "args": { "selectAll": true } },
            { "id": "clear-selection", "label": labels.context_clear_selection, "action": "setMediaNodeSelection", "args": { "nodeIds": [] } },
            { "id": "reorganize", "label": labels.context_reorganize, "action": "reorganizeMediaGraph" }
        ])
        .to_string()
    }

    // 🫀 The shared `presence:` backbone-URI hack (`read_os_presence_peers`/`write_os_presence`/
    // `OsPresencePeer`) was deleted from os-core — presence now flows through the hub's duplex
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

    fn runtime_studio_id(runtime: &StudioRuntimeState) -> String {
        runtime.studio_id.clone().unwrap_or_else(|| "default".into())
    }

    fn presence_peers_json(runtime: &StudioRuntimeState) -> String {
        let studio_id = runtime_studio_id(runtime);
        let self_client_id = runtime.client_id.clone().unwrap_or_default();
        let now_ms = host_now_ms();
        let peers: Vec<Value> = PRESENCE_PEERS
            .lock()
            .ok()
            .and_then(|registry| registry.get(&studio_id).cloned())
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
        let studio_id = runtime_studio_id(runtime);
        let now_ms = host_now_ms();
        if let Ok(mut registry) = PRESENCE_PEERS.lock() {
            let peers = registry.entry(studio_id).or_default();
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

    fn compiled_dag_wire_literal(projection: &OsProjection) -> String {
        let fixture = media_graph_to_dag_fixture(projection);
        dag_fixture_to_wire_literal(&fixture)
    }
    //#endregion 🔖DocumentHelpers

    //#region 🔖Terminology
    app_labels! {
        /// 🗣️ Complete UI label set for the Studio app; one field per label makes every locale combination compile-checked.
        struct SStudioLabels {
            apps_section: &'static str = en: "Apps", de: "Apps";
            media_vfs_empty_message: &'static str = en: "No app instances in the media graph.", de: "Keine App-Instanzen im Mediengraphen.";
            add_parameter: &'static str = en: "Add Parameter", de: "Parameter hinzufuegen";
            name: &'static str = en: "Name", de: "Name";
            value: &'static str = en: "Value", de: "Wert";
            min: &'static str = en: "Min", de: "Min";
            max: &'static str = en: "Max", de: "Max";
            step: &'static str = en: "Step", de: "Schritt";
            add_option: &'static str = en: "Add option", de: "Option hinzufuegen";
            new_option_placeholder: &'static str = en: "New option", de: "Neue Option";
            remove: &'static str = en: "Remove", de: "Entfernen";
            node_id: &'static str = en: "Node id", de: "Knoten-ID";
            label: &'static str = en: "Label", de: "Beschriftung";
            direct_value: &'static str = en: "Direct value", de: "Direkter Wert";
            media_graph_node: &'static str = en: "Media graph node", de: "Mediengraph-Knoten";
            media_graph_nodes: &'static str = en: "Media graph nodes", de: "Mediengraph-Knoten";
            app_instance: &'static str = en: "App instance", de: "App-Instanz";
            app_instances: &'static str = en: "App instances", de: "App-Instanzen";
            select_hint: &'static str = en: "Select media graph nodes or app instances in the canvas.", de: "Waehle Mediengraph-Knoten oder App-Instanzen im Arbeitsbereich aus.";
            program_prefix: &'static str = en: "Program", de: "Programm";
            app_prefix: &'static str = en: "App", de: "App";
            instance_id_prefix: &'static str = en: "Instance id", de: "Instanz-ID";
            bound_value_prefix: &'static str = en: "Bound value", de: "Gebundener Wert";
            active_app: &'static str = en: "Active app", de: "Aktive App";
            window_media_graph: &'static str = en: "Media Graph", de: "Mediengraph";
            window_media_vfs: &'static str = en: "Media VFS", de: "Media-VFS";
            window_compiled_dag: &'static str = en: "Compiled DAG", de: "Kompilierter DAG";
            toggle_on: &'static str = en: "On", de: "An";
            toggle_off: &'static str = en: "Off", de: "Aus";
            mixed_placeholder: &'static str = en: "Mixed", de: "Gemischt";
            parameter_count_suffix: &'static str = en: "parameter(s)", de: "Parameter";
            media_node_count_label: &'static str = en: "media node(s)", de: "Medienknoten";
            app_instance_count_label: &'static str = en: "app instance(s)", de: "App-Instanz(en)";
            context_open_instance: &'static str = en: "Open instance", de: "Instanz oeffnen";
            context_duplicate: &'static str = en: "Duplicate", de: "Duplizieren";
            context_copy: &'static str = en: "Copy", de: "Kopieren";
            context_paste: &'static str = en: "Paste", de: "Einfuegen";
            context_rename_label: &'static str = en: "Rename label…", de: "Bezeichnung umbenennen…";
            context_remove: &'static str = en: "Remove", de: "Entfernen";
            context_select_all: &'static str = en: "Select all", de: "Alle auswaehlen";
            context_clear_selection: &'static str = en: "Clear selection", de: "Auswahl aufheben";
            context_reorganize: &'static str = en: "Reorganize", de: "Neu anordnen";
        }
    }
    //#endregion 🔖Terminology

    //#region 🔖CommandLabels
    /// 🗣️ (action id) -> localized label for every operation/view-action/shell-action declared in
    /// `create_studio_app`'s static manifest — same rationale as `app_home`'s `s_home_action_labels`.
    fn s_studio_action_labels(is_de: bool) -> HashMap<String, String> {
        localized_label_map(is_de, &[
            // 🔧 Document-mutating operations
            ("setParameter", "Set Parameter", "Parameter festlegen"),
            ("patchParameter", "Patch Parameter", "Parameter aktualisieren"),
            ("addParameter", "Add Parameter", "Parameter hinzufuegen"),
            ("removeParameter", "Remove Parameter", "Parameter entfernen"),
            ("spawnApp", "Spawn App", "App erzeugen"),
            ("moveMediaNode", "Move Media Node", "Medienknoten verschieben"),
            ("connectMediaPorts", "Connect Media Ports", "Medien-Ports verbinden"),
            ("disconnectMediaEdge", "Disconnect Media Edge", "Medienverbindung trennen"),
            ("removeAppInstance", "Remove App Instance", "App-Instanz entfernen"),
            ("deleteSelection", "Delete Selection", "Auswahl loeschen"),
            ("copyAppInstance", "Copy App Instance", "App-Instanz kopieren"),
            ("duplicateAppInstance", "Duplicate App Instance", "App-Instanz duplizieren"),
            ("pasteAppInstance", "Paste App Instance", "App-Instanz einfuegen"),
            ("renameAppInstance", "Rename App Instance", "App-Instanz umbenennen"),
            ("patchMediaNodes", "Patch Media Nodes", "Medienknoten aktualisieren"),
            ("patchAppInstances", "Patch App Instances", "App-Instanzen aktualisieren"),
            ("bindParameterField", "Bind Parameter Field", "Parameterfeld verknuepfen"),
            ("unbindParameterField", "Unbind Parameter Field", "Parameterfeld loesen"),
            ("reorganizeMediaGraph", "Reorganize Media Graph", "Mediengraph neu anordnen"),
            ("mediaGraphEngagementSubmit", "Media Graph Engagement Submit", "Mediengraph-Eingabe bestaetigen"),
            ("compiledDagEngagementSubmit", "Compiled DAG Engagement Submit", "Kompilierter-DAG-Eingabe bestaetigen"),
            ("nodeGraphEdit", "Edit Media Graph", "Mediengraph bearbeiten"),
            // 👁️ Ephemeral view state
            ("setActivePanelTab", "Set Active Panel Tab", "Aktiven Panel-Tab festlegen"),
            ("selectInstance", "Select Instance", "Instanz auswaehlen"),
            ("nodeGraphSelect", "Select Graph Node", "Graphknoten auswaehlen"),
            ("setMediaNodeSelection", "Set Media Node Selection", "Medienknoten-Auswahl festlegen"),
            ("nodeGraphHover", "Hover Graph Node", "Graphknoten hovern"),
            ("textHover", "Text Hover", "Text-Hover"),
            ("nodeGraphViewport", "Set Graph Viewport", "Graph-Ansichtsfenster festlegen"),
            ("presenceHeartbeat", "Presence Heartbeat", "Anwesenheits-Heartbeat"),
            ("setAppInstanceSelection", "Set App Instance Selection", "App-Instanz-Auswahl festlegen"),
            ("mediaGraphEngagementInput", "Media Graph Engagement Input", "Mediengraph-Eingabe"),
            ("compiledDagEngagementInput", "Compiled DAG Engagement Input", "Kompilierter-DAG-Eingabe"),
            // 🗨️ Shell-only effects
            ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
            ("exportMedia", "Export Media", "Medien exportieren"),
            ("importMedia", "Import Media", "Medien importieren"),
            ("importMediaPayload", "Import Media Payload", "Medien-Payload importieren"),
            ("openStudio", "Open Studio", "Studio oeffnen"),
            ("openInstance", "Open Instance", "Instanz oeffnen"),
            ("closeFocusedInstance", "Close Focused Instance", "Fokussierte Instanz schliessen"),
            ("goHome", "Go Home", "Zur Startseite"),
            ("navigateVirtualFileSystemNode", "Navigate File System Node", "Dateisystemknoten navigieren"),
        ])
    }
    //#endregion 🔖CommandLabels

    //#region 🔖Panels
    #[derive(Default)]
    struct AppCatalogueNode {
        children: BTreeMap<String, AppCatalogueNode>,
        app: Option<StudioProgramEntry>,
    }

    /// 🌳 Builds a catalogue tree item on top of the SDK's `tree_item_desc` skeleton — only the
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
        item.icon_id = app.as_ref().map(|entry| entry.app_id.clone());
        item.default_open = (!children.is_empty()).then_some(true);
        if let Some(app) = &app {
            let mut drag_data = HashMap::new();
            drag_data.insert(
                S_PLAY_CATALOGUE_DRAG_MIME.into(),
                json!({ "programId": app.program_id, "appId": app.app_id, "label": app.label }).to_string(),
            );
            item.draggable = Some(true);
            item.drag_data = Some(drag_data);
        }
        item.items = (!children.is_empty()).then_some(children);
        item
    }

    fn build_catalogue_tree(panel: &StudioPanelState, labels: &SStudioLabels) -> UiNode {
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
            OsParameter::Numeric { id, value, step, .. } => UiNode::NumberStepper(UiNumberStepperNode {
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
                on_change: s_play_action(
                    "patchParameter",
                    Some(json!({ "parameterId": id, "field": "value" })),
                ),
            }),
            OsParameter::Toggle { id, value, .. } => UiNode::Toggle(UiToggleNode {
                id: format!("s-play-parameters.{id}.value"),
                icon_id: "toggle-left".into(),
                pressed: *value,
                text: Some(if *value { labels.toggle_on.into() } else { labels.toggle_off.into() }),
                on_change: s_play_action(
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
                on_change: s_play_action(
                    "patchParameter",
                    Some(json!({ "parameterId": id, "field": "value" })),
                ),
                min: None,
                max: None,
                step: None,
                accept: None,
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
                UiNode::Field(UiFieldNode {
                    id: format!("s-play-parameters.{id}.min"),
                    label: labels.min.into(),
                    child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
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
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
                UiNode::Field(UiFieldNode {
                    id: format!("s-play-parameters.{id}.max"),
                    label: labels.max.into(),
                    child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
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
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
                UiNode::Field(UiFieldNode {
                    id: format!("s-play-parameters.{id}.step"),
                    label: labels.step.into(),
                    child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
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
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
            ],
            OsParameter::Categorical { id, options, .. } => {
                let mut fields: Vec<UiNode> = options
                    .iter()
                    .map(|option| {
                        UiNode::Field(UiFieldNode {
                            id: format!("s-play-parameters.{id}.option.{option}"),
                            label: option.clone(),
                            child: Box::new(UiNode::Button(UiButtonNode {
                                id: Some(format!("s-play-parameters.{id}.option.{option}.remove")),
                                icon_id: "trash-2".into(),
                                label: labels.remove.into(),
                                action: s_play_action(
                                    "patchParameter",
                                    Some(json!({ "parameterId": id, "field": "removeOption", "value": option })),
                                ),
                                style: None,
                                disabled: None,
                                loading: None,
                            })),
                            description: None,
                            required: None,
                            error: None,
                        })
                    })
                    .collect();
                fields.push(UiNode::Field(UiFieldNode {
                    id: format!("s-play-parameters.{id}.add-option"),
                    label: labels.add_option.into(),
                    child: Box::new(UiNode::Input(UiInputNode {
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
                    })),
                    description: None,
                    required: None,
                    error: None,
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
            loading: None,
            children: vec![
                UiNode::Button(UiButtonNode {
                    id: Some("s-play-parameters.add".into()),
                    icon_id: "plus".into(),
                    label: labels.add_parameter.into(),
                    action: s_play_action("addParameter", Some(json!({ "type": "numeric" }))),
                    style: None,
                    disabled: None,
                    loading: None,
                }),
                ui_text(format!("{} {}", projection.parameters.len(), labels.parameter_count_suffix)),
            ],
        }];
        for parameter in &projection.parameters {
            let parameter_id = parameter_entity_id(parameter).to_string();
            let mut parameter_children = vec![
                UiNode::Field(UiFieldNode {
                    id: format!("s-play-parameters.{parameter_id}.name"),
                    label: labels.name.into(),
                    child: Box::new(UiNode::Input(UiInputNode {
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
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
                UiNode::Field(UiFieldNode {
                    id: format!("s-play-parameters.{parameter_id}.value-field"),
                    label: labels.value.into(),
                    child: Box::new(parameter_value_control(parameter, labels)),
                    description: None,
                    required: None,
                    error: None,
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
                disabled: None,
                loading: None,
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
                loading: None,
                children: parameter_children,
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
            loading: None,
            children: vec![ui_text(format!(
                "{} {} · {} {}",
                media_node_ids.len(),
                term_labels.media_node_count_label,
                instance_ids.len(),
                term_labels.app_instance_count_label
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
                    label: term_labels.node_id.into(),
                    child: Box::new(UiNode::Input(UiInputNode {
                        id: "s-play-inspector.media-node.id.input".into(),
                        input_kind: "text".into(),
                        value: media_node_ids[0].clone(),
                        placeholder: None,
                        commit: None,
                        on_change: s_play_action("noop", None),
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                }));
            }
            node_fields.push(UiNode::Field(UiFieldNode {
                id: "s-play-inspector.media-node.x".into(),
                label: "X".into(),
                child: Box::new(UiNode::Input(UiInputNode {
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
                })),
                description: None,
                required: None,
                error: None,
            }));
            node_fields.push(UiNode::Field(UiFieldNode {
                id: "s-play-inspector.media-node.y".into(),
                label: "Y".into(),
                child: Box::new(UiNode::Input(UiInputNode {
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
                })),
                description: None,
                required: None,
                error: None,
            }));
            children.push(UiSectionNode {
                id: "s-play-inspector.media-nodes".into(),
                label: Some(if media_node_ids.len() == 1 {
                    term_labels.media_graph_node.into()
                } else {
                    format!("{} ({})", term_labels.media_graph_nodes, media_node_ids.len())
                }),
                default_open: Some(true),
                loading: None,
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
                UiNode::Field(UiFieldNode {
                    id: "s-play-inspector.app-instance.label".into(),
                    label: term_labels.label.into(),
                    child: Box::new(UiNode::Input(UiInputNode {
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
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
            ];
            if instance_ids.len() == 1 {
                instance_fields.insert(2, ui_text(format!("{}: {}", term_labels.instance_id_prefix, instance_ids[0])));
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
                            instance_fields.push(UiNode::Field(UiFieldNode {
                                id: format!("s-play-inspector.app-parameter.{}", field_spec.field_path),
                                label: field_spec.label.clone(),
                                child: Box::new(UiNode::Select(UiSelectNode {
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
                                })),
                                description: None,
                                required: None,
                                error: None,
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
                loading: None,
                children: instance_fields,
            });
        }
        if media_node_ids.is_empty() && instance_ids.is_empty() {
            children[0].children.push(ui_text(term_labels.select_hint));
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
    //#endregion 🔖Panels

    //#region 🔖Render
    fn render_media_graph(projection: &OsProjection, runtime: &StudioRuntimeState, labels: &SStudioLabels) -> UiNode {
        let graph_payload = os_media_graph_to_node_graph_payload(&projection.media_graph, &projection.app_instances);
        let camera = runtime.media_graph_camera.clone().unwrap_or_default();
        let fixture = os_media_graph_to_flow_fixture(&projection.media_graph, &projection.app_instances, &camera);
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
                context_menu_json: Some(media_graph_context_menu_json(labels)),
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
                empty_message: Some(labels.media_vfs_empty_message.into()),
                drag_drop_enabled: Some(true),
            },
            Some(S_PLAY_WINDOW_MEDIA_VFS.into()),
            None,
        )
    }
    //#endregion 🔖Render

    //#region 🔖StudioApp
    #[derive(Default)]
    pub struct StudioApp {
        runtime: StudioRuntimeState,
    }

    impl StudioApp {
        /// @emoji 🎬 Seeds the studio app with the demo's first instance pre-selected (matching the old
        /// `initial_studio_envelope` runtime) so the media-graph measure dropdown opens on a live app.
        pub fn new() -> Self {
            Self {
                runtime: StudioRuntimeState {
                    active_instance_id: demo_studio_projection()
                        .app_instances
                        .first()
                        .map(|instance| instance.id.clone()),
                    ..StudioRuntimeState::default()
                },
            }
        }
    }

    impl DocumentApp for StudioApp {
        type Projection = OsProjection;
        type Op = OsOp;

        fn app_id(&self) -> &str {
            S_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            OS_STUDIO_SCHEMA
        }

        fn initial_projection(&self) -> OsProjection {
            demo_studio_projection()
        }

        fn handle_action(
            &mut self,
            action: &str,
            args: Option<&Value>,
            doc: &DocumentView<'_, OsProjection>,
            view_state: &ViewState,
        ) -> ActionEmit<OsOp> {
            let projection = doc.projection;
            let mut ops: Vec<OsOp> = Vec::new();
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
                    if let Some(studio_id) = args.and_then(|value| value.get("studioId")).and_then(|value| value.as_str()) {
                        return ActionEmit::effect(HostEffect::Navigate { uri: format!("/studios/{studio_id}") });
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
                            if let Some(op) = patch_parameter_op(projection, parameter_id, &patch) {
                                ops.push(op);
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
                    ops.push(add_parameter_op(&parameter_type, name));
                }
                "removeParameter" => {
                    if let Some(parameter_id) = args
                        .and_then(|value| value.get("parameterId"))
                        .and_then(|value| value.as_str())
                    {
                        ops.push(OsOp::RemoveParameter {
                            parameter_id: parameter_id.into(),
                        });
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
                        if let Some((op, instance_id)) = spawn_app_instance_op(program_id, app_id, None, position) {
                            self.runtime.active_instance_id = Some(instance_id);
                            ops.push(op);
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
                        ops.push(OsOp::MoveMediaNode {
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
                            Ok(contract) => ops.push(OsOp::ConnectMediaPorts {
                                edge: semio_framework_os::OsMediaGraphEdge {
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
                        ops.push(OsOp::DisconnectMediaEdge {
                            edge_id: edge_id.into(),
                        });
                    }
                }
                "removeAppInstance" => {
                    let instance_id = args
                        .and_then(|value| value.get("instanceId"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string)
                        .or_else(|| primary_selected_instance_id(&self.runtime, projection));
                    if let Some(instance_id) = instance_id {
                        ops.push(OsOp::RemoveAppInstance {
                            instance_id: instance_id.clone(),
                        });
                        if self.runtime.active_instance_id.as_deref() == Some(instance_id.as_str()) {
                            self.runtime.active_instance_id = None;
                        }
                        if self.runtime.focused_instance_id.as_deref() == Some(instance_id.as_str()) {
                            self.runtime.focused_instance_id = None;
                        }
                    }
                }
                "deleteSelection" => {
                    let instance_ids = selected_instance_ids(&self.runtime, projection);
                    for instance_id in instance_ids {
                        ops.push(OsOp::RemoveAppInstance {
                            instance_id: instance_id.clone(),
                        });
                    }
                    self.runtime.selected_app_instance_ids.clear();
                    self.runtime.selected_media_node_ids.clear();
                    self.runtime.active_instance_id = None;
                    self.runtime.focused_instance_id = None;
                }
                "copyAppInstance" => {
                    self.runtime.clipboard_instance_ids = selected_instance_ids(&self.runtime, projection);
                }
                "duplicateAppInstance" | "pasteAppInstance" => {
                    let source_ids = if action == "pasteAppInstance" {
                        self.runtime.clipboard_instance_ids.clone()
                    } else {
                        selected_instance_ids(&self.runtime, projection)
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
                        if let Some((op, new_id)) = spawn_app_instance_op(
                            &instance.program_id,
                            &instance.app_id,
                            Some(&label),
                            position,
                        ) {
                            self.runtime.active_instance_id = Some(new_id);
                            ops.push(op);
                        }
                    }
                }
                "renameAppInstance" => {
                    if let Some(instance_id) = primary_selected_instance_id(&self.runtime, projection) {
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
                            ops.push(OsOp::PatchAppInstance {
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
                        // 🧭 Examples are catalog documents in the new topology — selecting one navigates
                        // the shell to that studio route; the host's `openDocument(ref)` loads it (no
                        // in-place whole-document swap on the plugin side anymore); an empty id is the
                        // shell's "no example" reset and keeps the current studio route.
                        return ActionEmit::effect(HostEffect::Navigate { uri: format!("/studios/{example_id}") });
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
                            ensure_studio_fixtures_registered();
                            // 📤 `s` is a shell: the instance's live content lives in its own
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
                        self.runtime.pending_import_instance_id = Some(instance_id.to_string());
                        self.runtime.pending_import_format = Some(format.to_string());
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
                    if let (Some(instance_id), Some(format_name)) = (self.runtime.pending_import_instance_id.take(), self.runtime.pending_import_format.take()) {
                        let payload = args.and_then(|value| value.get("payload")).and_then(|value| value.as_str());
                        let format = semio_framework_os::OsMediaFormat::parse(&format_name);
                        if let (Some(payload), Some(format)) = (payload, format) {
                            use base64::Engine;
                            let base64_part = payload.split_once(',').map(|(_, data)| data).unwrap_or(payload);
                            if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(base64_part) {
                                if let Some(instance) = projection.app_instances.iter().find(|row| row.id == instance_id) {
                                    // 📥 Decoding/validation happens here; the decoded content is applied
                                    // to the instance's own `OsDocumentRef` document by the host (a
                                    // cross-document op the shell can't author from its own store), so
                                    // this arm emits no studio op.
                                    let _ = semio_framework_os::import_os_app_instance_media(instance, &bytes, format);
                                }
                            }
                        }
                    }
                    return ActionEmit::default();
                }
                "selectInstance" => {
                    self.runtime.active_instance_id = args
                        .and_then(|value| value.get("instanceId"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string);
                    if let Some(instance_id) = self.runtime.active_instance_id.clone() {
                        let node_id = projection
                            .media_graph
                            .nodes
                            .iter()
                            .find(|node| node.instance_id == instance_id)
                            .map(|node| node.id.clone());
                        self.runtime.selected_app_instance_ids = vec![instance_id];
                        self.runtime.selected_media_node_ids = node_id.into_iter().collect();
                    }
                }
                "nodeGraphSelect" | "setMediaNodeSelection" => {
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
                    self.runtime.selected_media_node_ids = node_ids.clone();
                    self.runtime.selected_app_instance_ids = node_ids
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
                    if self.runtime.selected_app_instance_ids.len() == 1 {
                        self.runtime.active_instance_id = self.runtime.selected_app_instance_ids.first().cloned();
                    }
                }
                "reorganizeMediaGraph" => {
                    let node_ids: Vec<String> = if self.runtime.selected_media_node_ids.is_empty() {
                        projection.media_graph.nodes.iter().map(|node| node.id.clone()).collect()
                    } else {
                        self.runtime.selected_media_node_ids.clone()
                    };
                    for (index, node_id) in node_ids.iter().enumerate() {
                        let col = (index % 4) as f64;
                        let row = (index / 4) as f64;
                        ops.push(OsOp::MoveMediaNode {
                            node_id: node_id.clone(),
                            x: 80.0 + col * 220.0,
                            y: 80.0 + row * 160.0,
                        });
                    }
                }
                "nodeGraphHover" | "textHover" => {
                    self.runtime.hovered_media_node_id = args
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
                        .and_then(|viewport_json| serde_json::from_str::<OsMediaGraphCamera>(viewport_json).ok())
                    {
                        self.runtime.media_graph_camera = Some(camera);
                    }
                }
                "nodeGraphEdit" => {
                    let edit_ops = args
                        .and_then(|value| value.get("ops"))
                        .and_then(|value| value.as_array())
                        .cloned()
                        .unwrap_or_default();
                    for edit in &edit_ops {
                        match edit.get("op").and_then(|value| value.as_str()).unwrap_or("") {
                            "setFixture" => {
                                if let Some(fixture_json) = edit.get("fixtureJson").and_then(|value| value.as_str()) {
                                    if let Some(camera) = serde_json::from_str::<Value>(fixture_json)
                                        .ok()
                                        .and_then(|fixture| fixture.get("camera").cloned())
                                        .and_then(|camera| serde_json::from_value::<OsMediaGraphCamera>(camera).ok())
                                    {
                                        self.runtime.media_graph_camera = Some(camera);
                                    }
                                    ops.extend(apply_flow_fixture_to_os_media_graph(&projection.media_graph, fixture_json));
                                }
                            }
                            "move" => {
                                if let (Some(node_id), Some(x), Some(y)) = (
                                    edit.get("nodeId").and_then(|value| value.as_str()),
                                    edit.get("x").and_then(|value| value.as_f64()),
                                    edit.get("y").and_then(|value| value.as_f64()),
                                ) {
                                    ops.push(OsOp::MoveMediaNode { node_id: node_id.into(), x, y });
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
                                        Ok(contract) => ops.push(OsOp::ConnectMediaPorts {
                                            edge: semio_framework_os::OsMediaGraphEdge {
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
                                for node_id in &self.runtime.selected_media_node_ids {
                                    if let Some(node) = projection.media_graph.nodes.iter().find(|node| node.id == *node_id) {
                                        ops.push(OsOp::RemoveAppInstance { instance_id: node.instance_id.clone() });
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                "presenceHeartbeat" => {
                    if let Some(client_id) = args.and_then(|value| value.get("clientId")).and_then(|value| value.as_str()) {
                        self.runtime.client_id = Some(client_id.into());
                        self.runtime.client_name = Some(
                            args.and_then(|value| value.get("name"))
                                .and_then(|value| value.as_str())
                                .unwrap_or("Guest")
                                .into(),
                        );
                    }
                    // 🐢 A heartbeat only records this client's own identity for the presence broadcast below
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
                    self.runtime.selected_app_instance_ids = instance_ids.clone();
                    self.runtime.selected_media_node_ids = instance_ids
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
                        self.runtime.active_instance_id = Some(instance_ids[0].clone());
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
                                ops.push(OsOp::MoveMediaNode {
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
                                ops.push(OsOp::PatchAppInstance {
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
                            ops.push(OsOp::UnbindParameterField {
                                instance_id: instance_id.into(),
                                field_path: field_path.into(),
                            });
                        } else {
                            ops.push(OsOp::BindParameterField {
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
                        ops.push(OsOp::UnbindParameterField {
                            instance_id: instance_id.into(),
                            field_path: field_path.into(),
                        });
                    }
                }
                "openStudio" => {
                    if let Some(studio_id) = args
                        .and_then(|value| value.get("studioId"))
                        .and_then(|value| value.as_str())
                    {
                        // 🧭 Switching studios navigates the shell; the host loads the target document by
                        // its `OsDocumentRef` (no in-place envelope swap on the plugin side).
                        return ActionEmit::effect(HostEffect::Navigate { uri: format!("/studios/{studio_id}") });
                    }
                    return ActionEmit::default();
                }
                "openInstance" => {
                    let instance_id = args
                        .and_then(|value| value.get("instanceId"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string)
                        .or_else(|| primary_selected_instance_id(&self.runtime, projection));
                    if let Some(instance_id) = instance_id {
                        self.runtime.focused_instance_id = Some(instance_id.clone());
                        self.runtime.active_instance_id = Some(instance_id.clone());
                        self.runtime.selected_app_instance_ids = vec![instance_id.clone()];
                        if let Some(node) = projection
                            .media_graph
                            .nodes
                            .iter()
                            .find(|row| row.instance_id == instance_id)
                        {
                            self.runtime.selected_media_node_ids = vec![node.id.clone()];
                        }
                        if let Some(instance) = projection
                            .app_instances
                            .iter()
                            .find(|row| row.id == instance_id)
                        {
                            effects.push(HostEffect::OpenPluginInstance {
                                program_id: instance.program_id.clone(),
                                app_id: instance.app_id.clone(),
                                os_instance_id: Some(instance.id.clone()),
                            });
                        }
                    }
                }
                "closeFocusedInstance" => {
                    self.runtime.focused_instance_id = None;
                    let mut panel = parse_panel_state(view_state);
                    panel.active_spawned_id = None;
                    return ActionEmit::effect(HostEffect::SetPanel { panel_json: panel_json(&panel) });
                }
                "goHome" => return ActionEmit::effect(HostEffect::Navigate { uri: "/".into() }),
                "mediaGraphEngagementInput" => {
                    self.runtime.media_graph_engagement_input = args
                        .and_then(|value| value.get("value"))
                        .and_then(|value| value.as_str())
                        .unwrap_or("")
                        .into();
                }
                "mediaGraphEngagementSubmit" => {
                    let raw = args
                        .and_then(|value| value.get("value"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string)
                        .unwrap_or_else(|| self.runtime.media_graph_engagement_input.clone());
                    let mut parts = raw.split_whitespace();
                    if let (Some(program_id), Some(app_id)) = (parts.next(), parts.next()) {
                        if let Some((op, instance_id)) = spawn_app_instance_op(
                            program_id,
                            app_id,
                            None,
                            MediaGraphPosition { x: 80.0, y: 80.0 },
                        ) {
                            self.runtime.active_instance_id = Some(instance_id);
                            ops.push(op);
                        }
                    }
                }
                "compiledDagEngagementInput" => {
                    self.runtime.compiled_dag_engagement_input = args
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
                publish_presence(&self.runtime);
            }
            ActionEmit {
                ops,
                coalesce_key,
                effects,
                ui_scope,
                ..Default::default()
            }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, OsProjection>, view_state: &ViewState) -> UiNode {
            let projection = doc.projection;
            let panel = parse_panel_state(view_state);
            let labels = resolve_labels::<SStudioLabels>(view_state);
            match body_key {
                S_PLAY_BODY_MEDIA_GRAPH => render_media_graph(projection, &self.runtime, labels),
                S_PLAY_BODY_MEDIA_VFS => render_media_vfs(projection, labels),
                S_PLAY_BODY_COMPILED_DAG => render_compiled_dag(projection),
                S_PLAY_CATALOGUE_BODY_KEY => build_catalogue_tree(&panel, labels),
                S_PLAY_PARAMETERS_BODY_KEY => build_parameters_tree(projection, labels),
                S_PLAY_INSPECTOR_BODY_KEY => build_inspector_tree(projection, &self.runtime, labels),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn window_measures(&self, doc: &DocumentView<'_, OsProjection>, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
            let labels = resolve_labels::<SStudioLabels>(view_state);
            HashMap::from([(
                S_PLAY_WINDOW_MEDIA_GRAPH.into(),
                media_graph_measures(&self.runtime, &doc.projection.app_instances, labels),
            )])
        }

        fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
            let labels = resolve_labels::<SStudioLabels>(view_state);
            let is_de = is_de_locale(view_state);
            AppLabelsOverlay::default()
                .window_kind_label(S_PLAY_WINDOW_MEDIA_GRAPH, labels.window_media_graph)
                .window_kind_label(S_PLAY_WINDOW_MEDIA_VFS, labels.window_media_vfs)
                .window_kind_label(S_PLAY_WINDOW_COMPILED_DAG, labels.window_compiled_dag)
                .action_labels(s_studio_action_labels(is_de))
        }
    }
    //#endregion 🔖StudioApp

    //#region 🔖StudioManifest
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
                on_change: Some(s_play_action("mediaGraphEngagementInput", None)),
                on_submit: Some(s_play_action("mediaGraphEngagementSubmit", None)),
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

    fn media_graph_measures(runtime: &StudioRuntimeState, instances: &[OsAppInstance], labels: &SStudioLabels) -> Vec<WindowMeasure> {
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

    pub fn create_studio_app() -> App {
        let projection = demo_studio_projection();
        let runtime = StudioRuntimeState {
            active_instance_id: projection.app_instances.first().map(|instance| instance.id.clone()),
            ..StudioRuntimeState::default()
        };
        let engagement = media_graph_engagement(
            &runtime,
            projection.media_graph.nodes.len(),
            projection.app_instances.len(),
        );
        let measures = media_graph_measures(&runtime, &projection.app_instances, resolve_labels::<SStudioLabels>(&ViewState::default()));
        let builder = App::builder(S_PLAY_APP_ID, "Studio").document(["semio", "s", "studio"])
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
            .operation("reorganizeMediaGraph", "Reorganize Media Graph")
            .operation("mediaGraphEngagementSubmit", "Media Graph Engagement Submit")
            .operation("compiledDagEngagementSubmit", "Compiled DAG Engagement Submit")
            .operation("nodeGraphEdit", "Edit Media Graph")
            .view_action("setActivePanelTab", "Set Active Panel Tab")
            .view_action("selectInstance", "Select Instance")
            .view_action("nodeGraphSelect", "Select Graph Node")
            .view_action("setMediaNodeSelection", "Set Media Node Selection")
            .view_action("nodeGraphHover", "Hover Graph Node")
            .view_action("textHover", "Text Hover")
            .view_action("nodeGraphViewport", "Set Graph Viewport")
            .view_action("presenceHeartbeat", "Presence Heartbeat")
            .view_action("setAppInstanceSelection", "Set App Instance Selection")
            .view_action("mediaGraphEngagementInput", "Media Graph Engagement Input")
            .view_action("compiledDagEngagementInput", "Compiled DAG Engagement Input")
            .shell_action("setActiveExample", "Set Active Example")
            .shell_action("exportMedia", "Export Media")
            .shell_action("importMedia", "Import Media")
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("importMediaPayload", "Import Media Payload", ActionKind::Shell) })
            .shell_action("openStudio", "Open Studio")
            .shell_action("openInstance", "Open Instance")
            .shell_action("closeFocusedInstance", "Close Focused Instance")
            .shell_action("goHome", "Go Home")
            .shell_action("navigateVirtualFileSystemNode", "Navigate File System Node")
            // 📝 Staged argument form for parameter creation (spawnApp/exportMedia stay context/registry-driven).
            .action_args("addParameter", vec![
                ActionArgDef::text("name", "Name").default_value("Parameter"),
                ActionArgDef::select("type", "Type", vec![
                    ActionArgOption::new("numeric", "Numeric"),
                    ActionArgOption::new("categorical", "Categorical"),
                    ActionArgOption::new("toggle", "Toggle"),
                    ActionArgOption::new("text", "Text"),
                ]).default_value("numeric"),
            ])
            // 📇 Per-window action scoping — the Media Graph (NodeGraph) window owns all graph/instance/
            // parameter editing plus the per-instance media import/export; the Media VFS
            // (VirtualFileSystem) window only navigates the media file tree; the read-only Compiled DAG
            // window only drives its own engagement. Navigation, panel-tab, presence, example and generic
            // node-graph view actions stay unscoped orphans and appear on every window.
            .window_kind_actions(S_PLAY_WINDOW_MEDIA_GRAPH, vec![
                "setParameter".into(), "patchParameter".into(), "addParameter".into(), "removeParameter".into(),
                "spawnApp".into(), "moveMediaNode".into(), "connectMediaPorts".into(), "disconnectMediaEdge".into(),
                "removeAppInstance".into(), "deleteSelection".into(), "copyAppInstance".into(),
                "duplicateAppInstance".into(), "pasteAppInstance".into(), "renameAppInstance".into(),
                "patchMediaNodes".into(), "patchAppInstances".into(), "bindParameterField".into(),
                "unbindParameterField".into(), "reorganizeMediaGraph".into(), "mediaGraphEngagementSubmit".into(),
                "mediaGraphEngagementInput".into(), "nodeGraphEdit".into(), "selectInstance".into(),
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
            .find(|window| window.id == S_PLAY_WINDOW_MEDIA_GRAPH)
        {
            window.options.measures = measures;
            window.options.engagement = WindowEngagementSlot::Some(engagement);
        }
        let compiled_engagement = compiled_dag_engagement(&demo_studio_projection());
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
            program: None,
        };
        app.definition.controller_id = S_PLAY_CONTROLLER_ID.into();
        let mut app = app.program("s", "S Studio", "studio");
        for (id, label, json) in S_STUDIO_EXAMPLES {
            app = app.example(*id, *label, (*json).to_string());
        }
        app
    }
    //#endregion 🔖StudioManifest

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_os::{
            apply_os_operation, merge_os_program_definition, os_baseline_resource, os_in_port, os_out_port, register_resource_descriptor, validate_media_graph, MediaClass, MediaForm, MediaType, MediaWireFormat, OsAppResourceSpec,
            OsMediaFormat, OsMediaGraphNode, OsMediaPort, OsPlatformAppInput, OsPlatformInput, ResourceKindSpec,
        };
        use semio_framework_plugin::{testkit, HistoryView, ModeDefinition, PluginApp, UiControlNode, UiNode, VcsDocumentApp};

        //#region 🔧Harness
        fn empty_history() -> HistoryView {
            HistoryView {
                columns: Vec::new(),
                can_undo: false,
                can_redo: false,
                active_alternative_id: None,
                current_checkpoint_id: None,
            }
        }

        /// 🎛️ Drives the typed `StudioApp::handle_action` against a projection snapshot, returning its emit.
        fn studio_emit(app: &mut StudioApp, projection: &OsProjection, action: &str, args: Value) -> ActionEmit<OsOp> {
            let history = empty_history();
            let doc = DocumentView { projection, history: &history };
            app.handle_action(action, Some(&args), &doc, &ViewState::default())
        }

        /// 📽️ Folds studio ops onto a projection the way the store would (minus history), for op-application asserts.
        fn apply_ops(projection: &OsProjection, ops: &[OsOp]) -> OsProjection {
            ops.iter().fold(projection.clone(), |current, op| apply_os_operation(&current, op))
        }
        //#endregion 🔧Harness

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
                            utilities: vec![],
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
            let projection = demo_studio_projection();
            assert!(projection.app_instances.len() >= 5);
            assert!(projection.media_graph.nodes.len() >= 2);
            assert!(projection.media_graph.edges.len() >= 1);
            assert!(validate_media_graph(&projection.media_graph).ok);
        }

        #[test]
        fn renders_media_graph_scene() {
            let mut app = VcsDocumentApp::new(StudioApp::new());
            let node = app.render(S_PLAY_BODY_MEDIA_GRAPH, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
        }

        #[test]
        fn studio_window_kind_actions_scope_editing_to_media_graph() {
            let definition = create_studio_app().definition;
            let resolve = |window_id: &str| -> Vec<String> {
                let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
                semio_framework_plugin::resolve_window_actions(&definition, window)
                    .into_iter()
                    .map(|action| action.id.clone())
                    .collect()
            };
            let graph = resolve(S_PLAY_WINDOW_MEDIA_GRAPH);
            let vfs = resolve(S_PLAY_WINDOW_MEDIA_VFS);
            let dag = resolve(S_PLAY_WINDOW_COMPILED_DAG);
            for graph_op in ["spawnApp", "connectMediaPorts", "removeAppInstance", "exportMedia", "addParameter"] {
                assert!(graph.contains(&graph_op.to_string()), "Media Graph must expose {graph_op}");
                assert!(!vfs.contains(&graph_op.to_string()), "Media VFS must NOT expose {graph_op}");
                assert!(!dag.contains(&graph_op.to_string()), "Compiled DAG must NOT expose {graph_op}");
            }
            assert!(vfs.contains(&"navigateVirtualFileSystemNode".to_string()));
            assert!(!graph.contains(&"navigateVirtualFileSystemNode".to_string()));
            assert!(dag.contains(&"compiledDagEngagementSubmit".to_string()));
            assert!(!graph.contains(&"compiledDagEngagementSubmit".to_string()));
            // 🌐 Global navigation/utility actions stay orphans on every window.
            for shared in ["setActiveExample", "goHome"] {
                assert!(graph.contains(&shared.to_string()) && vfs.contains(&shared.to_string()) && dag.contains(&shared.to_string()), "{shared} stays global");
            }
        }

        #[test]
        fn renders_compiled_dag_editor() {
            let mut app = VcsDocumentApp::new(StudioApp::new());
            let node = app.render(S_PLAY_BODY_COMPILED_DAG, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("text-editor"));
            let wire = compiled_dag_wire_literal(&demo_studio_projection());
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
        fn move_media_node_emits_coalesced_move_op() {
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
            let node_id = projection.media_graph.nodes.first().expect("node").id.clone();
            let emit = studio_emit(&mut app, &projection, "moveMediaNode", json!({ "nodeId": node_id, "x": 120.0, "y": 160.0 }));
            assert_eq!(emit.coalesce_key.as_deref(), Some(format!("moveMediaNode:{node_id}").as_str()));
            let node = apply_ops(&projection, &emit.ops)
                .media_graph
                .nodes
                .into_iter()
                .find(|row| row.id == node_id)
                .expect("node");
            assert!((node.x - 120.0).abs() < 0.01);
            assert!((node.y - 160.0).abs() < 0.01);
        }

        //#region 🔖MediaContractConnect
        #[test]
        fn connect_media_ports_rejects_incompatible_types_via_notice() {
            register_resource_descriptor(&ResourceKindSpec {
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
            register_resource_descriptor(&ResourceKindSpec {
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
            let mut projection = demo_studio_projection();
            projection.media_graph.nodes.push(OsMediaGraphNode {
                id: "contract-src".into(),
                instance_id: "contract-src".into(),
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
                inputs: vec![],
                outputs: vec![OsMediaPort { id: "contract-src:out".into(), resource_kind: "test.contract.2d".into(), direction: "out".into() }],
            });
            projection.media_graph.nodes.push(OsMediaGraphNode {
                id: "contract-dst".into(),
                instance_id: "contract-dst".into(),
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
                inputs: vec![OsMediaPort { id: "contract-dst:in".into(), resource_kind: "test.contract.3d".into(), direction: "in".into() }],
                outputs: vec![],
            });
            let mut app = StudioApp::new();
            let emit = studio_emit(
                &mut app,
                &projection,
                "connectMediaPorts",
                json!({ "sourceNodeId": "contract-src", "sourcePortId": "contract-src:out", "targetNodeId": "contract-dst", "targetPortId": "contract-dst:in" }),
            );
            assert!(emit.ops.is_empty(), "an incompatible connect must not push OsOp::ConnectMediaPorts");
            assert!(matches!(emit.effects.first(), Some(HostEffect::Notify { .. })), "an incompatible connect must surface a Notify effect instead");
        }

        #[test]
        fn connect_media_ports_negotiates_a_contract_for_compatible_types() {
            register_resource_descriptor(&ResourceKindSpec {
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
            register_resource_descriptor(&ResourceKindSpec {
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
            let mut projection = demo_studio_projection();
            projection.media_graph.nodes.push(OsMediaGraphNode {
                id: "contract-src-2".into(),
                instance_id: "contract-src-2".into(),
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
                inputs: vec![],
                outputs: vec![OsMediaPort { id: "contract-src-2:out".into(), resource_kind: "test.contract.doc-a".into(), direction: "out".into() }],
            });
            projection.media_graph.nodes.push(OsMediaGraphNode {
                id: "contract-dst-2".into(),
                instance_id: "contract-dst-2".into(),
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
                inputs: vec![OsMediaPort { id: "contract-dst-2:in".into(), resource_kind: "test.contract.doc-b".into(), direction: "in".into() }],
                outputs: vec![],
            });
            let mut app = StudioApp::new();
            let emit = studio_emit(
                &mut app,
                &projection,
                "connectMediaPorts",
                json!({ "sourceNodeId": "contract-src-2", "sourcePortId": "contract-src-2:out", "targetNodeId": "contract-dst-2", "targetPortId": "contract-dst-2:in" }),
            );
            let edge = emit
                .ops
                .iter()
                .find_map(|op| match op {
                    OsOp::ConnectMediaPorts { edge } if edge.source_node_id == "contract-src-2" => Some(edge.clone()),
                    _ => None,
                })
                .expect("a compatible connect must push OsOp::ConnectMediaPorts with a negotiated contract");
            assert_eq!(edge.contract.kind_id, "test.contract.doc-b");
            assert_eq!(edge.contract.wire, MediaWireFormat::Document { schema: "test.contract.doc.schema".into() });
            assert!(edge.contract.conversion.is_none());
            let next = apply_ops(&projection, &emit.ops);
            assert!(validate_media_graph(&next.media_graph).ok, "a freshly negotiated edge must pass validate_media_graph's contract-consistency check");
        }
        //#endregion 🔖MediaContractConnect

        #[test]
        fn spawns_draw_app_instance() {
            seed_draw_program();
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
            let emit = studio_emit(&mut app, &projection, "spawnApp", json!({ "programId": "draw", "appId": "draw" }));
            assert!(!emit.ops.is_empty());
            let next = apply_ops(&projection, &emit.ops);
            assert_eq!(next.app_instances.len(), projection.app_instances.len() + 1);
            assert_eq!(app.runtime.active_instance_id, next.app_instances.last().map(|i| i.id.clone()));
        }

        #[test]
        fn spawns_draw_app_instance_at_drop_position() {
            seed_draw_program();
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
            let existing: HashSet<String> = projection.app_instances.iter().map(|i| i.id.clone()).collect();
            let emit = studio_emit(
                &mut app,
                &projection,
                "spawnApp",
                json!({ "programId": "draw", "appId": "draw", "position": { "x": 321.0, "y": 654.0 } }),
            );
            let next = apply_ops(&projection, &emit.ops);
            let instance = next
                .app_instances
                .iter()
                .find(|i| i.program_id == "draw" && !existing.contains(&i.id))
                .expect("newly spawned draw instance");
            let node = next
                .media_graph
                .nodes
                .iter()
                .find(|n| n.instance_id == instance.id)
                .expect("media node for spawned instance");
            assert!((node.x - 321.0).abs() < 0.01);
            assert!((node.y - 654.0).abs() < 0.01);
        }

        #[test]
        fn open_instance_emits_open_plugin_instance_effect_matching_instance() {
            seed_draw_program();
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
            let instance = projection.app_instances.iter().find(|i| i.program_id == "draw").expect("draw instance").clone();
            let emit = studio_emit(&mut app, &projection, "openInstance", json!({ "instanceId": instance.id }));
            assert!(emit.ops.is_empty(), "opening an instance is a host effect, not a document op");
            let opened = emit
                .effects
                .iter()
                .find_map(|effect| match effect {
                    HostEffect::OpenPluginInstance { program_id, app_id, os_instance_id } => {
                        Some((program_id.clone(), app_id.clone(), os_instance_id.clone()))
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
            seed_draw_program();
            semio_framework_os::register_os_media_export_handler("2d.drawing", semio_framework_os::OsMediaFormat::Dwg, |_doc| {
                let drawing = semio_framework_os::DwgDrawing::default();
                let bytes = semio_framework_os::dwg_to_bytes(&drawing).map_err(|error| error)?;
                Ok(semio_framework_os::OsMediaExportResult {
                    data: base64::engine::general_purpose::STANDARD.encode(bytes),
                    mime_type: semio_framework_os::OsMediaFormat::Dwg.mime_type().into(),
                    file_name: "draw.dwg".into(),
                    encoding: Some("base64".into()),
                })
            });
            semio_framework_os::register_dwg_import_handler("2d.drawing", |_drawing| Ok(json!({ "schema": "draw.document", "imported": true })));

            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
            let instance = projection.app_instances.iter().find(|i| i.program_id == "draw").expect("draw instance").clone();

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
            // `OsDocumentRef` document by the host, so this arm emits no studio op.
            let payload = studio_emit(&mut app, &projection, "importMediaPayload", json!({ "payload": format!("data:image/vnd.dwg;base64,{data}") }));
            assert!(payload.ops.is_empty());
        }

        #[test]
        fn commit_checkpoint_round_trips_projection() {
            let mut app = VcsDocumentApp::new(StudioApp::new());
            let before = app.projection().expect("projection").app_instances.len();
            app.handle_action("commitCheckpoint", Some(&json!({ "message": "snapshot" })), &ViewState::default(), &testkit::meta("local"))
                .expect("commit");
            assert_eq!(app.projection().expect("projection").app_instances.len(), before);
        }

        #[test]
        fn patch_parameter_op_updates_numeric_value() {
            let projection = demo_studio_projection();
            let op = patch_parameter_op(&projection, "param-brush-size", &json!({ "value": 48.0 })).expect("op");
            let next = apply_os_operation(&projection, &op);
            match next.parameters.iter().find(|entry| entry.id() == "param-brush-size").expect("parameter") {
                OsParameter::Numeric { value, .. } => assert_eq!(*value, 48.0),
                _ => panic!("expected numeric"),
            }
        }

        #[test]
        fn patch_parameter_action_updates_value() {
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
            let emit = studio_emit(
                &mut app,
                &projection,
                "patchParameter",
                json!({ "parameterId": "param-brush-size", "field": "value", "value": 48.0 }),
            );
            assert_eq!(emit.ops.len(), 1);
            let next = apply_ops(&projection, &emit.ops);
            match next.parameters.iter().find(|entry| entry.id() == "param-brush-size").expect("parameter") {
                OsParameter::Numeric { value, .. } => assert_eq!(*value, 48.0),
                _ => panic!("expected numeric"),
            }
        }

        /// 🧪 Undo/redo round trip on a real operation, driven through the shared testkit harness
        /// instead of a hand-rolled `meta()`/repeated assert body.
        #[test]
        fn undo_redo_round_trip_on_spawn() {
            seed_draw_program();
            let mut app = VcsDocumentApp::new(StudioApp::new());
            let before = app.projection().expect("projection").app_instances.len();
            testkit::assert_undo_redo_round_trip(
                &mut app,
                "spawnApp",
                Some(&json!({ "programId": "draw", "appId": "draw" })),
                |app| app.projection().expect("projection").app_instances.len(),
                before,
                before + 1,
            );
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
            let tree = build_catalogue_tree(&panel, resolve_labels::<SStudioLabels>(&ViewState::default()));
            let json = serde_json::to_string(&tree).unwrap();
            assert!(json.contains("s-play-catalogue.document.semio.puzzle.2d"));
            assert!(json.contains("s-play-catalogue.document.semio.puzzle.3d"));
            assert_eq!(json.matches("\"label\":\"puzzle\"").count(), 1);
        }

        #[test]
        fn patch_app_instances_updates_labels() {
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
            let ids: Vec<String> = projection.app_instances.iter().take(2).map(|i| i.id.clone()).collect();
            let emit = studio_emit(
                &mut app,
                &projection,
                "patchAppInstances",
                json!({ "instanceIds": ids, "field": "label", "value": "Batch Label" }),
            );
            let next = apply_ops(&projection, &emit.ops);
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
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
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
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
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
                    component_kind: "world-3d".into(),
                    modes: vec![ModeDefinition {
                        id: "edit".into(),
                        label: "Edit".into(),
                        utilities: vec![],
                        layout_id: None,
                        commands: vec![],
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
                            utilities: vec![],
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
                        utilities: vec![],
                        layout_id: None,
                        commands: vec![],
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
                            utilities: vec![],
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
            seed_multi_port_programs();
            let mut app = StudioApp::new();
            let mut projection = demo_studio_projection();
            let emit = studio_emit(
                &mut app,
                &projection,
                "spawnApp",
                json!({ "programId": "puzzle.5d", "appId": "puzzle5d", "position": { "x": 200, "y": 100 } }),
            );
            projection = apply_ops(&projection, &emit.ops);
            let emit = studio_emit(
                &mut app,
                &projection,
                "spawnApp",
                json!({ "programId": "shooting", "appId": "shooting", "position": { "x": 300, "y": 100 } }),
            );
            projection = apply_ops(&projection, &emit.ops);
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
            let mut app = StudioApp::new();
            let mut projection = demo_studio_projection();
            let instance = projection.app_instances.first().expect("instance").clone();
            let parameter_id = parameter_entity_id(projection.parameters.first().expect("parameter")).to_string();
            let emit = studio_emit(
                &mut app,
                &projection,
                "bindParameterField",
                json!({ "instanceId": instance.id, "fieldPath": "label", "parameterId": parameter_id }),
            );
            projection = apply_ops(&projection, &emit.ops);
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
            projection = apply_ops(&projection, &emit.ops);
            assert!(!projection
                .parameter_bindings
                .iter()
                .any(|row| row.instance_id == instance.id && row.field_path == "label"));
        }

        #[test]
        fn checkout_checkpoint_restores_projection() {
            seed_draw_program();
            let mut app = VcsDocumentApp::new(StudioApp::new());
            let before = app.projection().expect("projection").app_instances.len();
            app.handle_action("spawnApp", Some(&json!({ "programId": "draw", "appId": "draw" })), &ViewState::default(), &testkit::meta("local"))
                .expect("spawn");
            app.handle_action("commitCheckpoint", Some(&json!({ "message": "after-first-spawn" })), &ViewState::default(), &testkit::meta("local"))
                .expect("commit");
            let after_first = app.projection().expect("projection").app_instances.len();
            assert!(after_first > before);
            let envelope: Value = serde_json::from_str(&app.document_json().expect("document json")).expect("envelope json");
            let checkpoint_id = envelope["vcs"]["checkpoints"][0]["id"].as_str().expect("checkpoint id").to_string();
            app.handle_action("spawnApp", Some(&json!({ "programId": "draw", "appId": "draw" })), &ViewState::default(), &testkit::meta("local"))
                .expect("spawn2");
            assert!(app.projection().expect("projection").app_instances.len() > after_first);
            app.handle_action("checkoutCheckpoint", Some(&json!({ "checkpointId": checkpoint_id })), &ViewState::default(), &testkit::meta("local"))
                .expect("checkout");
            assert_eq!(app.projection().expect("projection").app_instances.len(), after_first);
        }

        /// 🧪 The definitional proof: two independent instances start from the same deterministic demo
        /// projection, apply DISJOINT edits (A spawns a new draw instance, B renames an existing
        /// instance), and exchanging ops over a backbone converges both sides onto the same
        /// projection — impossible under whole-document `setDocument` snapshots, where one side's write
        /// would clobber the other's. Driven through the shared testkit convergence harness instead of
        /// a hand-rolled `MemoryBackbone::pair` + manual drain/ingest.
        #[test]
        fn two_instances_converge_on_disjoint_edits_via_backbone() {
            seed_draw_program();
            let instance_id = demo_studio_projection().app_instances.first().expect("instance").id.clone();
            let rename_args = json!({ "instanceIds": [instance_id.clone()], "field": "label", "value": "Renamed" });
            testkit::assert_two_instances_converge::<StudioApp, (usize, bool)>(
                "mem://s-studio-convergence",
                ("spawnApp", Some(&json!({ "programId": "draw", "appId": "draw" }))),
                ("patchAppInstances", Some(&rename_args)),
                move |app| {
                    let projection = app.projection().expect("projection");
                    let draw_count = projection.app_instances.iter().filter(|i| i.program_id == "draw").count();
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
        fn studio_declares_expected_actions_and_examples() {
            let studio = create_studio_app();
            assert!(studio.definition.actions.iter().any(|action| action.id == "spawnApp"));
            assert!(studio.definition.actions.iter().any(|action| action.id == "reorganizeMediaGraph"));
            assert_eq!(studio.examples.len(), S_STUDIO_EXAMPLES.len());
        }

        #[test]
        fn media_graph_scene_uses_flow_engine_with_fixture() {
            let mut app = VcsDocumentApp::new(StudioApp::new());
            let node = app.render(S_PLAY_BODY_MEDIA_GRAPH, None, &ViewState::default()).expect("render");
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains(r#"\"engine\":\"flow\""#));
            assert!(json.contains("fixtureJson"));
            assert!(json.contains(r#"\"schema\":\"flow.fixture\""#));
        }

        #[test]
        fn node_graph_edit_set_fixture_moves_node_and_persists_camera() {
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
            let node = projection.media_graph.nodes.first().expect("node").clone();
            let camera = OsMediaGraphCamera { x: 40.0, y: -20.0, zoom: 2.0 };
            let mut fixture = os_media_graph_to_flow_fixture(&projection.media_graph, &projection.app_instances, &camera);
            fixture["layout"][&node.id] = json!({ "x": 500.0 + node.width / 2.0, "y": 300.0 + node.height / 2.0 });
            let emit = studio_emit(
                &mut app,
                &projection,
                "nodeGraphEdit",
                json!({ "ops": [{ "op": "setFixture", "fixtureJson": fixture.to_string() }] }),
            );
            let moved = apply_ops(&projection, &emit.ops)
                .media_graph
                .nodes
                .into_iter()
                .find(|row| row.id == node.id)
                .expect("node");
            assert!((moved.x - 500.0).abs() < 0.01);
            assert!((moved.y - 300.0).abs() < 0.01);
            assert_eq!(app.runtime.media_graph_camera, Some(camera));
        }

        #[test]
        fn node_graph_viewport_persists_camera() {
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
            studio_emit(&mut app, &projection, "nodeGraphViewport", json!({ "viewportJson": r#"{"x":7.0,"y":9.0,"zoom":0.5}"# }));
            assert_eq!(app.runtime.media_graph_camera, Some(OsMediaGraphCamera { x: 7.0, y: 9.0, zoom: 0.5 }));
        }

        #[test]
        fn presence_heartbeat_publishes_peer_for_other_clients() {
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
            let first_node_id = projection.media_graph.nodes[0].id.clone();
            studio_emit(&mut app, &projection, "nodeGraphSelect", json!({ "nodeIds": [first_node_id] }));
            studio_emit(&mut app, &projection, "presenceHeartbeat", json!({ "clientId": "client-test-a", "name": "Ada" }));
            let other_runtime = StudioRuntimeState {
                client_id: Some("client-test-b".into()),
                studio_id: app.runtime.studio_id.clone(),
                ..StudioRuntimeState::default()
            };
            let peers = presence_peers_json(&other_runtime);
            assert!(peers.contains("client-test-a"));
            assert!(peers.contains("Ada"));
            assert!(peers.contains(r#""selectionCount":1"#));
            let self_view = presence_peers_json(&app.runtime);
            assert!(!self_view.contains("client-test-a"));
        }

        /// 🐢 Perf round 3: a heartbeat only records this client's own identity for the presence broadcast
        /// — it must declare `None` so it never triggers a full-shell `refresh-ui` for the sending client.
        #[test]
        fn presence_heartbeat_declares_none_ui_scope() {
            use semio_framework_core::kernel::UiDirtyScope;
            let mut app = StudioApp::new();
            let projection = demo_studio_projection();
            let emit = studio_emit(&mut app, &projection, "presenceHeartbeat", json!({ "clientId": "client-test-c", "name": "Cass" }));
            assert!(matches!(emit.ui_scope, UiDirtyScope::None), "presenceHeartbeat must declare None, got {:?}", emit.ui_scope);
        }

        #[test]
        fn studio_labels_resolve_native_english_by_default() {
            let history = empty_history();
            let app = StudioApp::new();
            let projection = demo_studio_projection();
            let doc = DocumentView { projection: &projection, history: &history };
            let catalogue_json = serde_json::to_string(&app.render(S_PLAY_CATALOGUE_BODY_KEY, &doc, &ViewState::default())).unwrap();
            assert!(catalogue_json.contains("\"Apps\""));

            let parameters_json = serde_json::to_string(&app.render(S_PLAY_PARAMETERS_BODY_KEY, &doc, &ViewState::default())).unwrap();
            assert!(parameters_json.contains("Add Parameter"));
            assert!(parameters_json.contains("\"Name\""));
            assert!(parameters_json.contains("\"Remove\""));
            assert!(!parameters_json.contains("Parameter hinzufuegen"));
        }

        #[test]
        fn studio_labels_resolve_native_german_locale() {
            let history = empty_history();
            let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
            let app = StudioApp::new();
            let projection = demo_studio_projection();
            let doc = DocumentView { projection: &projection, history: &history };
            let parameters_json = serde_json::to_string(&app.render(S_PLAY_PARAMETERS_BODY_KEY, &doc, &view_state)).unwrap();
            assert!(parameters_json.contains("Parameter hinzufuegen"));
            assert!(parameters_json.contains("\"Entfernen\""));
            assert!(!parameters_json.contains("Add Parameter"));

            let inspector_json = serde_json::to_string(&app.render(S_PLAY_INSPECTOR_BODY_KEY, &doc, &view_state)).unwrap();
            assert!(inspector_json.contains("Waehle Mediengraph-Knoten oder App-Instanzen im Arbeitsbereich aus."));
        }
    }
    //#endregion 🧪Tests
}
//#endregion 🔖app_studio

//#region 🔖Manifest
fn bundle() -> semio_framework_plugin::PluginBundle {
    semio_framework_plugin::PluginBundle::new("s", "S Studio", "0.1.0")
        .local_backbone_storage()
        .register_document_app(app_home::create_home_app(), || app_home::HomeApp)
        .register_document_app(app_studio::create_studio_app(), app_studio::StudioApp::new)
}

semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖Manifest
