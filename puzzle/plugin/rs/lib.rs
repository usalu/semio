//! 🧩 Puzzle plugin — 2D, 3D, and 5D play apps in one hot-swappable WASM component.

pub mod d2 {
    //! 🧩 Puzzle 2D plugin — declarative puzzle 2d play app bundled as a hot-swappable WASM component.

    use puzzle_2d::{handle_position_on_circle, handle_position_on_rectangle, puzzle2d_document_delta_ops, puzzle_2d_lod_scale_json, puzzle_board_host, BoardHost, Point, Puzzle2dExtension, Puzzle2dOp, BOARD_CAMERA_ZOOM_MAX, BOARD_CAMERA_ZOOM_MIN};
    use semio_framework_plugin::{
        build_canvas_2d_scene, build_puzzle2d_board_scene, create_default_layout,
        MeasureSelectItem, WindowEngagementStatus,
        ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, ActionDescriptor, DocumentApp, DocumentView, PanelGroup, PluginBundle, Puzzle2dBoardScene, SurfaceKind, ToolCategory, ToolDefinition, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement,
        WindowEngagementInput, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, SET_ACTIVE_TOOL_ACTION_ID,
    };
    use semio_framework_plugin::kernel::HostEffect;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::LazyLock;

    //#region 🔖Constants
    const PUZZLE2D_PLAY_APP_ID: &str = "puzzle2d-play";
    const PUZZLE2D_PLAY_CONTROLLER_ID: &str = "puzzle2d-play";
    const PUZZLE2D_PLAY_SURFACE_ID: &str = "puzzle2d.play.composite";
    const PUZZLE2D_PLAY_BODY_OVERVIEW: &str = "puzzle2d.play.overview";
    const PUZZLE2D_PLAY_BODY_DETAIL: &str = "puzzle2d.play.detail";
    const PUZZLE2D_PLAY_BODY_SELECTION: &str = "puzzle2d.play.selection";
    const PUZZLE2D_PLAY_BODY_LAYERS: &str = "puzzle2d.play.layers";
    const PUZZLE2D_PLAY_BODY_CATALOGUE: &str = "puzzle2d.play.catalogue";
    const PUZZLE2D_PLAY_BODY_PROPERTIES: &str = "puzzle2d.play.properties";
    const PUZZLE2D_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
    const PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID: &str = "concrete-forest";
    const PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID: &str = "nakagin-capsule-tower";
    const CONCRETE_FOREST_EXAMPLE_JSON: &str = include_str!("../../2d/example/concrete-forest.2d.json");
    const NAKAGIN_EXAMPLE_JSON: &str = include_str!("../../2d/example/nakagin-capsule-tower.2d.json");
    /// 🧰 The three canvas tools declared to the framework toolbar (host-owned active tool, never a doc field).
    const PUZZLE2D_TOOL_SELECT: &str = "select";
    const PUZZLE2D_TOOL_BRUSH: &str = "brush";
    const PUZZLE2D_TOOL_FILL: &str = "fill";
    const BOARD_DEFAULT_WIDTH: u32 = 1024;
    const BOARD_DEFAULT_HEIGHT: u32 = 768;

    //#region 🔖PaneConstants
    const PUZZLE2D_PANE_OVERVIEW: &str = "2d-overview";
    const PUZZLE2D_PANE_DETAIL: &str = "2d-detail";
    const PUZZLE2D_PANE_SELECTION: &str = "2d-selection";
    const PUZZLE2D_PANES: [&str; 3] = [PUZZLE2D_PANE_OVERVIEW, PUZZLE2D_PANE_DETAIL, PUZZLE2D_PANE_SELECTION];
    const PUZZLE2D_LOD_MODE_AUTOMATIC: &str = "automatic";
    const PUZZLE2D_VIEWPORT_REF_SHORT_PX: f64 = 640.0;
    const PUZZLE2D_VIEWPORT_MARGIN: f64 = 0.18;
    const PUZZLE2D_VIEWPORT_FRAMING_HALF_SPAN_SCALE: f64 = 2.25;
    const PUZZLE2D_VIEWPORT_ZOOM_BOOST: f64 = 2.5;
    const PUZZLE2D_PANE_ZOOM_SCALE_OVERVIEW: f64 = 0.68;
    const PUZZLE2D_PANE_ZOOM_SCALE_DETAIL: f64 = 2.15;
    const PUZZLE2D_PANE_ZOOM_SCALE_SELECTION: f64 = 0.36;
    //#endregion 🔖PaneConstants

    //#region 🔖EngagementConstants
    const PUZZLE2D_SUGGESTION_OFFSET_MIN: f64 = 0.0;
    const PUZZLE2D_SUGGESTION_OFFSET_MAX: f64 = 160.0;
    const PUZZLE2D_SUGGESTION_OFFSET_STEP: f64 = 4.0;
    const PUZZLE2D_FILL_COUNT_MAX: u32 = 1000;
    /// 📶 Mirrors `ui_styling::metrics::board::SUGGESTION_OFFSET`; kept local since the plugin crate has no styling dependency.
    const PUZZLE2D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;
    //#endregion 🔖EngagementConstants

    static NODE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
    //#endregion 🔖Constants

    //#region 🔖Envelope
    fn default_selection_method() -> String {
        "rectangle".into()
    }

    fn default_grid_factor() -> f64 {
        1.0
    }

    fn default_suggestion_offset() -> f64 {
        PUZZLE2D_DEFAULT_SUGGESTION_OFFSET
    }

    /// 📶 Overview/selection default to automatic LOD; detail defaults to a fixed "detail" tier, matching the pre-migration triptych.
    fn default_lod_mode_by_pane() -> BTreeMap<String, String> {
        BTreeMap::from([(PUZZLE2D_PANE_OVERVIEW.to_string(), PUZZLE2D_LOD_MODE_AUTOMATIC.to_string()), (PUZZLE2D_PANE_DETAIL.to_string(), "detail".to_string()), (PUZZLE2D_PANE_SELECTION.to_string(), PUZZLE2D_LOD_MODE_AUTOMATIC.to_string())])
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle2dPlayRuntime {
        #[serde(default)]
        selected_ids: Vec<String>,
        #[serde(default = "default_lod_mode_by_pane")]
        lod_mode_by_pane: BTreeMap<String, String>,
        #[serde(default)]
        engagement_input_by_pane: BTreeMap<String, String>,
        #[serde(default)]
        brush_candidate_index: usize,
        #[serde(default)]
        brush_candidates: Vec<Value>,
        #[serde(default)]
        brush_candidate_source_handle_id: String,
        #[serde(default)]
        fill_count: u32,
        #[serde(default = "default_selection_method")]
        selection_method: String,
        #[serde(default)]
        grid_snap_enabled: bool,
        #[serde(default = "default_grid_factor")]
        grid_factor: f64,
        #[serde(default = "default_suggestion_offset")]
        suggestion_offset: f64,
        #[serde(default)]
        node_kind_weights: BTreeMap<String, f64>,
        #[serde(default)]
        handle_kind_weights: BTreeMap<String, f64>,
    }

    /// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
    impl Default for Puzzle2dPlayRuntime {
        fn default() -> Self {
            Self {
                selected_ids: Vec::new(),
                lod_mode_by_pane: default_lod_mode_by_pane(),
                engagement_input_by_pane: BTreeMap::new(),
                brush_candidate_index: 0,
                brush_candidates: Vec::new(),
                brush_candidate_source_handle_id: String::new(),
                fill_count: 0,
                selection_method: default_selection_method(),
                grid_snap_enabled: false,
                grid_factor: default_grid_factor(),
                suggestion_offset: default_suggestion_offset(),
                node_kind_weights: BTreeMap::new(),
                handle_kind_weights: BTreeMap::new(),
            }
        }
    }

    /// 🧾 Transient render/mutation bundle pairing the persisted projection (the bare fixture json)
    /// with the app's ephemeral view state. It is never persisted — the {@link VcsDocumentApp} store
    /// owns the fixture as its projection and {@link Puzzle2dPlayApp} owns the runtime — but rebuilding
    /// it per call lets the panel/canvas/engagement helpers keep their existing `&scene` signatures.
    struct Puzzle2dScene {
        fixture: Value,
        runtime: Puzzle2dPlayRuntime,
        /// 🧰 The host-owned active tool for this render/mutation, sourced from `ViewState.active_tool_id`
        /// (defaulting to `select`) — never persisted in the runtime and never a document field.
        active_tool: String,
    }

    fn default_empty_fixture() -> Value {
        json!({
            "schema": PUZZLE2D_FIXTURE_SCHEMA,
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [],
            "edges": [],
            "wires": []
        })
    }

    fn example_fixture(json_text: &str) -> Value {
        serde_json::from_str(json_text).unwrap_or_else(|_| default_empty_fixture())
    }

    fn puzzle2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor { controller_id: PUZZLE2D_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
    }

    /// 🧰 The host-owned active tool for this view, read from `ViewState.active_tool_id` (defaulting to
    /// `select`) — the single source of truth for the canvas tool, never a document field or runtime cache.
    fn puzzle2d_active_tool(view_state: &ViewState) -> String {
        view_state.active_tool_id.clone().unwrap_or_else(|| PUZZLE2D_TOOL_SELECT.into())
    }

    fn selection_ids(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).or_else(|| args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(|id| vec![id.to_string()])).unwrap_or_default()
    }

    fn fixture_camera(fixture: &Value) -> (f64, f64, f64) {
        let camera = fixture.get("camera");
        (
            camera.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0),
            camera.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0),
            camera.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()).unwrap_or(1.0),
        )
    }

    fn fixture_nodes(fixture: &Value) -> &[Value] {
        fixture.get("nodes").and_then(|value| value.as_array()).map(|values| values.as_slice()).unwrap_or(&[])
    }

    fn fixture_edges(fixture: &Value) -> &[Value] {
        fixture.get("edges").and_then(|value| value.as_array()).map(|values| values.as_slice()).unwrap_or(&[])
    }

    fn kind_catalog_entries<'a>(fixture: &'a Value, key: &str) -> Option<&'a [Value]> {
        fixture.get("meta").and_then(|value| value.get("kindCatalogs")).and_then(|value| value.get(key)).and_then(|value| value.as_array()).map(|values| values.as_slice())
    }

    fn new_node_id(prefix: &str) -> String {
        let serial = NODE_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        format!("{prefix}-{serial}")
    }

    fn add_node_to_fixture(fixture: &mut Value, kind: Option<&str>, args: Option<&Value>) {
        let Some(obj) = fixture.as_object_mut() else {
            return;
        };
        let nodes = obj.entry("nodes".to_string()).or_insert_with(|| json!([]));
        let Some(nodes) = nodes.as_array_mut() else {
            return;
        };
        let node_kind = kind.unwrap_or("node");
        let id = new_node_id("node");
        let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
        let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
        let shape = args.and_then(|value| value.get("shape")).and_then(|value| value.as_str()).unwrap_or("circle");
        let mut node = json!({
            "id": id,
            "nodeKind": node_kind,
            "shape": shape,
            "x": x,
            "y": y,
            "text": id,
            "handles": []
        });
        if shape == "rectangle" {
            node["width"] = json!(args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()).unwrap_or(48.0));
            node["height"] = json!(args.and_then(|value| value.get("height")).and_then(|value| value.as_f64()).unwrap_or(48.0));
        } else {
            node["radius"] = json!(args.and_then(|value| value.get("radius")).and_then(|value| value.as_f64()).unwrap_or(24.0));
        }
        if let Some(icon_kind) = args.and_then(|value| value.get("iconKind")) {
            node["iconKind"] = icon_kind.clone();
        }
        nodes.push(node);
    }

    fn delete_selection_from_fixture(fixture: &mut Value, selected: &[String]) {
        if selected.is_empty() {
            return;
        }
        let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
        let node_ids: HashSet<String> = fixture_nodes(fixture).iter().filter_map(|node| node.get("id").and_then(|value| value.as_str())).filter(|id| selected.contains(id)).map(str::to_string).collect();
        let handle_ids: HashSet<String> = fixture_nodes(fixture)
            .iter()
            .flat_map(|node| node.get("handles").and_then(|value| value.as_array()).into_iter().flatten().filter_map(|handle| handle.get("id").and_then(|value| value.as_str())))
            .filter(|id| selected.contains(id))
            .map(str::to_string)
            .collect();
        if let Some(nodes) = fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
            *nodes = nodes
                .iter()
                .filter(|node| node.get("id").and_then(|value| value.as_str()).is_none_or(|id| !node_ids.contains(id)))
                .map(|node| {
                    let mut next = node.clone();
                    if let Some(handles) = next.get_mut("handles").and_then(|value| value.as_array_mut()) {
                        handles.retain(|handle| handle.get("id").and_then(|value| value.as_str()).is_none_or(|id| !handle_ids.contains(id)));
                    }
                    next
                })
                .collect();
        }
        if let Some(edges) = fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
            edges.retain(|edge| {
                let id_ok = edge.get("id").and_then(|value| value.as_str()).is_none_or(|id| !selected.contains(id));
                let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("");
                let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("");
                id_ok && !node_ids.contains(source) && !node_ids.contains(target) && !handle_ids.contains(source) && !handle_ids.contains(target)
            });
        }
    }

    /// 🙈 Patches `hidden`/`locked` onto every selected node, handle, and edge in the fixture.
    fn apply_selection_flag(fixture: &mut Value, selected: &[String], flag: &str, value: bool) {
        if selected.is_empty() {
            return;
        }
        let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
        let key = if flag == "locked" { "locked" } else { "hidden" };
        if let Some(nodes) = fixture.get_mut("nodes").and_then(|entry| entry.as_array_mut()) {
            for node in nodes.iter_mut() {
                let node_selected = node.get("id").and_then(|entry| entry.as_str()).is_some_and(|id| selected.contains(id));
                if let Some(handles) = node.get_mut("handles").and_then(|entry| entry.as_array_mut()) {
                    for handle in handles.iter_mut() {
                        let handle_selected = handle.get("id").and_then(|entry| entry.as_str()).is_some_and(|id| selected.contains(id));
                        if handle_selected {
                            if let Some(obj) = handle.as_object_mut() {
                                obj.insert(key.to_string(), json!(value));
                            }
                        }
                    }
                }
                if node_selected {
                    if let Some(obj) = node.as_object_mut() {
                        obj.insert(key.to_string(), json!(value));
                    }
                }
            }
        }
        if let Some(edges) = fixture.get_mut("edges").and_then(|entry| entry.as_array_mut()) {
            for edge in edges.iter_mut() {
                let edge_selected = edge.get("id").and_then(|entry| entry.as_str()).is_some_and(|id| selected.contains(id));
                if edge_selected {
                    if let Some(obj) = edge.as_object_mut() {
                        obj.insert(key.to_string(), json!(value));
                    }
                }
            }
        }
    }

    /// 📋 Clones every selected node (+24/+24 offset, fresh node+handle ids) and any edge whose both endpoints were cloned; returns the new node ids.
    fn duplicate_selection_in_fixture(fixture: &mut Value, selected: &[String]) -> Vec<String> {
        if selected.is_empty() {
            return Vec::new();
        }
        let selected_set: HashSet<&str> = selected.iter().map(String::as_str).collect();
        let mut id_remap: HashMap<String, String> = HashMap::new();
        let mut new_ids: Vec<String> = Vec::new();

        let source_nodes: Vec<Value> = fixture_nodes(fixture).iter().filter(|node| node.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id))).cloned().collect();

        let new_nodes: Vec<Value> = source_nodes
            .into_iter()
            .map(|mut node| {
                let old_id = node.get("id").and_then(|value| value.as_str()).unwrap_or_default().to_string();
                let new_id = new_node_id("node");
                id_remap.insert(old_id, new_id.clone());
                if let Some(obj) = node.as_object_mut() {
                    obj.insert("id".into(), json!(new_id));
                    if let Some(x) = obj.get("x").and_then(|value| value.as_f64()) {
                        obj.insert("x".into(), json!(x + 24.0));
                    }
                    if let Some(y) = obj.get("y").and_then(|value| value.as_f64()) {
                        obj.insert("y".into(), json!(y + 24.0));
                    }
                    if let Some(handles) = obj.get_mut("handles").and_then(|value| value.as_array_mut()) {
                        for handle in handles.iter_mut() {
                            let old_handle_id = handle.get("id").and_then(|value| value.as_str()).unwrap_or_default().to_string();
                            let suffix = old_handle_id.rsplit(':').next().unwrap_or(old_handle_id.as_str());
                            let new_handle_id = format!("{new_id}:{suffix}");
                            id_remap.insert(old_handle_id, new_handle_id.clone());
                            if let Some(hobj) = handle.as_object_mut() {
                                hobj.insert("id".into(), json!(new_handle_id));
                            }
                        }
                    }
                }
                new_ids.push(new_id);
                node
            })
            .collect();

        if let Some(nodes) = fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
            nodes.extend(new_nodes);
        }

        let new_edges: Vec<Value> = fixture_edges(fixture)
            .iter()
            .filter_map(|edge| {
                let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("");
                let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("");
                let (new_source, new_target) = (id_remap.get(source)?, id_remap.get(target)?);
                let mut clone = edge.clone();
                if let Some(obj) = clone.as_object_mut() {
                    obj.insert("id".into(), json!(new_node_id("edge")));
                    obj.insert("source".into(), json!(new_source));
                    obj.insert("target".into(), json!(new_target));
                }
                Some(clone)
            })
            .collect();
        if !new_edges.is_empty() {
            if let Some(edges) = fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
                edges.extend(new_edges);
            }
        }

        new_ids
    }

    /// 🎯 Every node/handle id sharing a `nodeKind`/`handleKind` with anything currently selected.
    fn select_same_kind_ids(fixture: &Value, selected: &[String]) -> Vec<String> {
        let selected_set: HashSet<&str> = selected.iter().map(String::as_str).collect();
        let mut node_kinds: HashSet<&str> = HashSet::new();
        let mut handle_kinds: HashSet<&str> = HashSet::new();
        for node in fixture_nodes(fixture) {
            if node.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id)) {
                if let Some(kind) = node.get("nodeKind").and_then(|value| value.as_str()) {
                    node_kinds.insert(kind);
                }
            }
            for handle in node.get("handles").and_then(|value| value.as_array()).into_iter().flatten() {
                if handle.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id)) {
                    if let Some(kind) = handle.get("handleKind").and_then(|value| value.as_str()) {
                        handle_kinds.insert(kind);
                    }
                }
            }
        }
        let mut ids: Vec<String> = Vec::new();
        for node in fixture_nodes(fixture) {
            if node.get("nodeKind").and_then(|value| value.as_str()).is_some_and(|kind| node_kinds.contains(kind)) {
                if let Some(id) = node.get("id").and_then(|value| value.as_str()) {
                    ids.push(id.to_string());
                }
            }
            for handle in node.get("handles").and_then(|value| value.as_array()).into_iter().flatten() {
                if handle.get("handleKind").and_then(|value| value.as_str()).is_some_and(|kind| handle_kinds.contains(kind)) {
                    if let Some(id) = handle.get("id").and_then(|value| value.as_str()) {
                        ids.push(id.to_string());
                    }
                }
            }
        }
        ids
    }

    fn set_fixture_camera(fixture: &mut Value, camera: &Value) {
        if let Some(obj) = fixture.as_object_mut() {
            obj.insert("camera".to_string(), camera.clone());
        }
    }

    fn puzzle_extension_id() -> &'static str {
        let _extension = Puzzle2dExtension;
        "puzzle.2d"
    }
    //#endregion 🔖Envelope

    //#region 🔖BoardHost
    /// 🧱 The expensive half of syncing `host` from `envelope`: a full `clear_scene()` + rebuild of
    /// every node/handle/edge plus the kind-catalog/kind-compat re-push. Only needed when
    /// `envelope.fixture` content actually changed — gated by `last_synced_fixture` in `handle_action`.
    fn sync_host_fixture_content(host: &mut BoardHost, envelope: &Puzzle2dScene) {
        let _ = host.parse_fixture_v1(&envelope.fixture);
        if let Some(catalogs) = envelope.fixture.get("meta").and_then(|value| value.get("kindCatalogs")) {
            if let Ok(json) = serde_json::to_string(catalogs) {
                let _ = host.set_board_kind_catalogs_from_json(&json);
            }
        }
        if let Some(compat) = envelope.fixture.get("meta").and_then(|value| value.get("kindCompatibility")).or_else(|| envelope.fixture.get("kindCompatibility")) {
            if let Ok(json) = serde_json::to_string(compat) {
                let _ = host.set_handle_link_compat_from_json(&json);
            }
        }
    }

    /// 🪶 The cheap half of syncing `host` from `envelope`: plain setters mirroring ephemeral runtime
    /// view state (selection/tool/grid/LOD/…) — must run on every action regardless of whether the
    /// fixture content changed, since this state itself changes every action.
    fn sync_host_runtime_state(host: &mut BoardHost, envelope: &Puzzle2dScene) {
        host.set_size(BOARD_DEFAULT_WIDTH, BOARD_DEFAULT_HEIGHT, 1.0);
        host.set_selection_ids(&envelope.runtime.selected_ids);
        host.set_active_tool(&envelope.active_tool);
        let overview_lod_mode = envelope.runtime.lod_mode_by_pane.get(PUZZLE2D_PANE_OVERVIEW).map(String::as_str).unwrap_or(PUZZLE2D_LOD_MODE_AUTOMATIC);
        if overview_lod_mode == PUZZLE2D_LOD_MODE_AUTOMATIC {
            host.set_automatic_lod(true);
        } else {
            host.set_automatic_lod(false);
            host.set_forced_draw_lod_label(overview_lod_mode);
        }
        host.set_grid_snap_enabled(envelope.runtime.grid_snap_enabled);
        let _ = host.set_grid_factor(envelope.runtime.grid_factor);
        host.set_suggestion_offset(envelope.runtime.suggestion_offset);
        if let Ok(weights_json) = serde_json::to_string(&json!({
            "nodeWeights": envelope.runtime.node_kind_weights,
            "handleWeights": envelope.runtime.handle_kind_weights,
        })) {
            host.set_brush_kind_weights(&weights_json);
        }
        host.set_selection_options(&envelope.runtime.selection_method, "replace", true, true, true);
    }

    fn sync_host_from_envelope(host: &mut BoardHost, envelope: &Puzzle2dScene) {
        sync_host_fixture_content(host, envelope);
        sync_host_runtime_state(host, envelope);
    }

    fn apply_board_events_from_json(events_json: &str, envelope: &mut Puzzle2dScene) {
        let Ok(events) = serde_json::from_str::<Vec<Value>>(events_json) else {
            return;
        };
        for event in events {
            let Some(name) = event.get("name").and_then(|value| value.as_str()) else {
                continue;
            };
            let payload = event.get("payload").cloned().unwrap_or(Value::Null);
            match name {
                "camera" => {
                    if let Some(obj) = envelope.fixture.as_object_mut() {
                        obj.insert("camera".into(), payload);
                    }
                }
                "select" => {
                    if let Some(ids) = payload.get("ids").and_then(|value| serde_json::from_value(value.clone()).ok()) {
                        envelope.runtime.selected_ids = ids;
                    }
                }
                "nodeDragEnd" => {
                    if let Some(moves) = payload.get("moves").and_then(|value| value.as_array()) {
                        for entry in moves {
                            let Some(id) = entry.get("id").and_then(|value| value.as_str()) else {
                                continue;
                            };
                            if let Some(x) = entry.get("x").and_then(|value| value.as_f64()) {
                                patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "x", &json!(x));
                            }
                            if let Some(y) = entry.get("y").and_then(|value| value.as_f64()) {
                                patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "y", &json!(y));
                            }
                        }
                    }
                }
                "nodeMove" => {
                    let Some(id) = payload.get("id").and_then(|value| value.as_str()) else {
                        continue;
                    };
                    if let Some(x) = payload.get("x").and_then(|value| value.as_f64()) {
                        patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "x", &json!(x));
                    }
                    if let Some(y) = payload.get("y").and_then(|value| value.as_f64()) {
                        patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "y", &json!(y));
                    }
                }
                "brushPlace" => {
                    apply_brush_place_payload(&mut envelope.fixture, &payload);
                }
                "edgeCreate" => {
                    if let Some(edges) = envelope.fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
                        edges.push(payload);
                    }
                }
                "nodeDelete" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        envelope.runtime.selected_ids = vec![id.to_string()];
                        delete_selection_from_fixture(&mut envelope.fixture, &envelope.runtime.selected_ids);
                        envelope.runtime.selected_ids.clear();
                    }
                }
                "edgeDelete" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        if let Some(edges) = envelope.fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
                            edges.retain(|edge| edge.get("id").and_then(|value| value.as_str()) != Some(id));
                        }
                    }
                }
                "brushCandidates" => {
                    if let Some(candidates) = payload.get("candidates").and_then(|value| value.as_array()) {
                        envelope.runtime.brush_candidates = candidates.clone();
                    }
                    if let Some(source) = payload.get("sourceHandleId").and_then(|value| value.as_str()) {
                        envelope.runtime.brush_candidate_source_handle_id = source.to_string();
                    }
                    if let Some(index) = payload.get("index").and_then(|value| value.as_u64()) {
                        envelope.runtime.brush_candidate_index = index as usize;
                    }
                }
                _ => {}
            }
        }
    }

    /// 🐢 `UiDirtyScope.windowBodies`/`.panelBodies` are matched against `AppDefinition.windowKinds[].bodyKey`
    /// on the shell side (`buildUiRefreshRequest`'s `uiRefreshWantsWindow`), so these must be the body-key
    /// constants (`puzzle2d.play.overview`, …) — *not* the pane/kind-id constants (`PUZZLE2D_PANES`,
    /// `2d-overview`, …), which are a different id space used to key tools/engagements/measures.
    const PUZZLE2D_WINDOW_BODY_KEYS: [&str; 3] = [PUZZLE2D_PLAY_BODY_OVERVIEW, PUZZLE2D_PLAY_BODY_DETAIL, PUZZLE2D_PLAY_BODY_SELECTION];

    /// 🐢 Classifies a batch of board events into the narrowest `UiDirtyScope` that covers all of them —
    /// `applyBoardEvents` fires on every select/drag/zoom, so getting this right is most of the
    /// perf-round-3 win. Unrecognized/empty event batches fall back to `Full` (safe default).
    fn puzzle2d_board_events_scope(events: &[Value]) -> semio_framework_core::kernel::UiDirtyScope {
        use semio_framework_core::kernel::UiDirtyScope;
        if events.is_empty() {
            return UiDirtyScope::None;
        }
        let panes: Vec<String> = PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect();
        let mut window_bodies = false;
        let mut panel_layers = false;
        let mut panel_properties = false;
        let mut engagements = false;
        let mut measures = false;
        let mut recognized_all = true;
        for event in events {
            let Some(name) = event.get("name").and_then(|value| value.as_str()) else {
                recognized_all = false;
                continue;
            };
            match name {
                "camera" => {
                    window_bodies = true;
                }
                "select" => {
                    window_bodies = true;
                    panel_layers = true;
                    panel_properties = true;
                    engagements = true;
                }
                "nodeMove" | "nodeDragEnd" => {
                    window_bodies = true;
                    panel_properties = true;
                }
                "brushPlace" | "edgeCreate" | "edgeDelete" | "nodeDelete" => {
                    window_bodies = true;
                    panel_layers = true;
                    panel_properties = true;
                    engagements = true;
                    measures = true;
                }
                "brushCandidates" => {
                    window_bodies = true;
                    engagements = true;
                }
                _ => recognized_all = false,
            }
        }
        if !recognized_all {
            return UiDirtyScope::Full;
        }
        let mut panel_bodies = Vec::new();
        if panel_layers {
            panel_bodies.push(PUZZLE2D_PLAY_BODY_LAYERS.to_string());
        }
        if panel_properties {
            panel_bodies.push(PUZZLE2D_PLAY_BODY_PROPERTIES.to_string());
        }
        UiDirtyScope::Partial {
            window_bodies: if window_bodies { panes } else { Vec::new() },
            panel_bodies,
            tools: false,
            engagements,
            measures,
            labels: false,
        }
    }

    /// 🐢 Narrow `UiDirtyScope` shared by pure view/selection/camera actions that only touch the 3
    /// canvas panes (never a panel or engagement/measure/tool refresh).
    fn puzzle2d_window_only_scope() -> semio_framework_core::kernel::UiDirtyScope {
        semio_framework_core::kernel::UiDirtyScope::Partial {
            window_bodies: PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
            panel_bodies: Vec::new(),
            tools: false,
            engagements: false,
            measures: false,
            labels: false,
        }
    }

    /// 🐢 Narrow `UiDirtyScope` for actions that additionally change the engagement bar (active tool,
    /// brush weights, LOD/grid settings, engagement text input) but never touch document content.
    fn puzzle2d_window_and_engagements_scope() -> semio_framework_core::kernel::UiDirtyScope {
        semio_framework_core::kernel::UiDirtyScope::Partial {
            window_bodies: PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
            panel_bodies: Vec::new(),
            tools: false,
            engagements: true,
            measures: false,
            labels: false,
        }
    }

    /// 🐢 Narrow `UiDirtyScope` for settings surfaced in the measures sidebar (LOD mode, grid, brush
    /// weights, suggestion offset — see `puzzle2d_window_measures`) but that never touch document
    /// content or the engagement bar.
    fn puzzle2d_window_and_measures_scope() -> semio_framework_core::kernel::UiDirtyScope {
        semio_framework_core::kernel::UiDirtyScope::Partial {
            window_bodies: PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
            panel_bodies: Vec::new(),
            tools: false,
            engagements: false,
            measures: true,
            labels: false,
        }
    }

    /// 🐢 Narrow `UiDirtyScope` for a runtime-only selection change: the 3 canvas panes plus the
    /// layers/properties panels (which highlight the selection) and the engagement bar.
    fn puzzle2d_select_scope() -> semio_framework_core::kernel::UiDirtyScope {
        semio_framework_core::kernel::UiDirtyScope::Partial {
            window_bodies: PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
            panel_bodies: vec![PUZZLE2D_PLAY_BODY_LAYERS.to_string(), PUZZLE2D_PLAY_BODY_PROPERTIES.to_string()],
            tools: false,
            engagements: true,
            measures: false,
            labels: false,
        }
    }

    /// 🪞 Re-syncs `envelope.runtime.selected_ids` from `self.host` for engine-driven selection changes
    /// (e.g. `delete_selection`, brush commit). Camera is deliberately NOT mirrored here: every action
    /// that moves the camera (`setCamera`, `focusSelection`, the `camera` board event) already writes
    /// `envelope.fixture`'s camera directly — re-deriving it from `host.camera` here used to blindly
    /// overwrite that write with the *pre-action* host camera (since nothing had told `self.host` about
    /// the new value yet), silently reverting every plain `camera` echo from the client.
    fn apply_host_events(host: &mut BoardHost, envelope: &mut Puzzle2dScene) {
        let events_raw = host.drain_events_json();
        apply_board_events_from_json(&events_raw, envelope);
        envelope.runtime.selected_ids = host.selection.iter().cloned().collect();
    }

    /// 🎲 Re-mints a node id when it collides with an existing one — client-side brush serials restart every session.
    fn unique_node_id(fixture: &Value, candidate: String) -> String {
        if fixture_nodes(fixture).iter().any(|node| node.get("id").and_then(|value| value.as_str()) == Some(candidate.as_str())) {
            new_node_id("node")
        } else {
            candidate
        }
    }

    fn unique_edge_id(fixture: &Value, candidate: String) -> String {
        if fixture_edges(fixture).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(candidate.as_str())) {
            new_node_id("edge")
        } else {
            candidate
        }
    }

    fn apply_brush_place_payload(fixture: &mut Value, payload: &Value) {
        let node_id = unique_node_id(fixture, payload.get("nodeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(|| new_node_id("node")));
        let edge_id = unique_edge_id(fixture, payload.get("edgeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(|| new_node_id("edge")));
        let node_kind = payload.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("node");
        let x = payload.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let y = payload.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let shape = payload.get("shape").and_then(|value| value.as_str()).unwrap_or("circle");
        let mut node = json!({
            "id": node_id,
            "nodeKind": node_kind,
            "shape": shape,
            "x": x,
            "y": y,
            "text": node_kind,
            "handles": payload.get("handles").cloned().unwrap_or_else(|| json!([])),
        });
        if shape == "rectangle" {
            node["width"] = json!(payload.get("width").and_then(|value| value.as_f64()).unwrap_or(48.0));
            node["height"] = json!(payload.get("height").and_then(|value| value.as_f64()).unwrap_or(48.0));
        } else {
            node["radius"] = json!(payload.get("radius").and_then(|value| value.as_f64()).unwrap_or(24.0));
        }
        if let Some(icon) = payload.get("iconKind") {
            node["iconKind"] = icon.clone();
        }
        if let Some(nodes) = fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
            nodes.push(node);
        }
        let source = payload.get("sourceHandleId").and_then(|value| value.as_str()).unwrap_or("");
        if !source.is_empty() {
            if let Some(edges) = fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
                edges.push(json!({
                    "id": edge_id,
                    "edgeKind": "link",
                    "source": source,
                    "target": format!("{node_id}:v{}", payload.get("targetHandleIndex").and_then(|value| value.as_u64()).unwrap_or(0)),
                }));
            }
        }
    }

    /// 🖌️ Tool Options group for the brush tool — the brush candidate picker (`{nodeKind, targetHandleIndex}`
    /// rows drained from the host), tagged `active_tool_id: Some("brush")` so [`partition_window_measures`]
    /// surfaces it in the Tool Options rail only while the brush tool is active. `None` when the host has no
    /// candidates yet (nothing to place), so the rail stays empty just like the old gated engagement control.
    fn puzzle2d_brush_tool_options(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> Option<WindowMeasure> {
        if envelope.runtime.brush_candidates.is_empty() {
            return None;
        }
        let items: Vec<MeasureSelectItem> = envelope
            .runtime
            .brush_candidates
            .iter()
            .enumerate()
            .map(|(index, candidate)| {
                let node_kind = candidate.get("nodeKind").and_then(|value| value.as_str()).or_else(|| candidate.as_str()).unwrap_or("kind");
                let id = format!("puzzle2d.brush.candidate.{index}");
                MeasureSelectItem { id: id.clone(), value: id, label: node_kind.into() }
            })
            .collect();
        let selected_index = envelope.runtime.brush_candidate_index.min(items.len().saturating_sub(1));
        Some(WindowMeasure::Group {
            id: "puzzle2d-tool-options-brush".into(),
            label: labels.placement.into(),
            default_open: Some(true),
            active_tool_id: Some(PUZZLE2D_TOOL_BRUSH.into()),
            children: vec![WindowMeasure::Select {
                id: "puzzle2d-brush-placement".into(),
                label: Some(labels.placement.into()),
                value: format!("puzzle2d.brush.candidate.{selected_index}"),
                items,
                on_change: puzzle2d_action("engagementControlSelect", None),
            }],
        })
    }

    /// 🪣 Tool Options group for the fill tool — the fill-count slider, tagged `active_tool_id: Some("fill")`
    /// so it surfaces in the Tool Options rail only while the fill tool is active.
    fn puzzle2d_fill_tool_options(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
        WindowMeasure::Group {
            id: "puzzle2d-tool-options-fill".into(),
            label: labels.fill.into(),
            default_open: Some(true),
            active_tool_id: Some(PUZZLE2D_TOOL_FILL.into()),
            children: vec![WindowMeasure::Slider {
                id: "puzzle2d-fill-count".into(),
                label: Some(format!("{} {}", labels.fill, envelope.runtime.fill_count)),
                value: envelope.runtime.fill_count as f64,
                min: 0.0,
                max: PUZZLE2D_FILL_COUNT_MAX as f64,
                step: Some(1.0),
                on_change: puzzle2d_action("setFillCount", None),
            }],
        }
    }

    fn puzzle2d_engagement(envelope: &Puzzle2dScene, host: &BoardHost, pane: &str, labels: &Puzzle2dLabels) -> WindowEngagement {
        let overlay: Value = serde_json::from_str(&host.overlay_paint_state_json()).unwrap_or(Value::Null);
        let pane_lod_mode = envelope.runtime.lod_mode_by_pane.get(pane).map(String::as_str).unwrap_or(PUZZLE2D_LOD_MODE_AUTOMATIC);
        let lod = overlay.get("lod").and_then(|value| value.as_str()).unwrap_or(if pane_lod_mode == PUZZLE2D_LOD_MODE_AUTOMATIC { "auto" } else { pane_lod_mode });
        let node_count = fixture_nodes(&envelope.fixture).len();
        let edge_count = fixture_edges(&envelope.fixture).len();
        let input_value = envelope.runtime.engagement_input_by_pane.get(pane).cloned().unwrap_or_default();
        let placeholder = match envelope.active_tool.as_str() {
            "fill" => "Fill",
            "brush" => "Brush",
            _ => "select, brush, fill, clear",
        };
        WindowEngagement {
            session_active: Some(envelope.active_tool != "select"),
            input: Some(WindowEngagementInput {
                id: Some("puzzle2d-engagement".into()),
                value: Some(input_value),
                placeholder: Some(placeholder.into()),
                disabled: None,
                on_change: Some(puzzle2d_action("engagementInput", Some(json!({ "pane": pane })))),
                on_submit: Some(puzzle2d_action("engagementSubmit", Some(json!({ "pane": pane })))),
                on_repeat_last: None,
                on_abort: Some(puzzle2d_action("engagementAbort", Some(json!({ "pane": pane })))),
            }),
            control: None,
            controls: None,
            status: Some(vec![WindowEngagementStatus { id: "puzzle2d-board-status".into(), text: format!("{node_count} nodes · {edge_count} edges · LOD {lod}") }]),
            // 🧰 The select/brush/fill switcher now lives in the framework toolbar (declared via `.tool` +
            // `.window_kind_tools`), so the engagement no longer duplicates it as toggle options.
            options: None,
            possible_engagements: None,
        }
    }
    //#endregion 🔖BoardHost

    //#region 🔖Canvas
    fn fixture_wires(fixture: &Value) -> &[Value] {
        fixture.get("wires").and_then(|value| value.as_array()).map(|values| values.as_slice()).unwrap_or(&[])
    }

    fn fixture_handles(fixture: &Value) -> Vec<Value> {
        fixture_nodes(fixture).iter().flat_map(|node| node.get("handles").and_then(|value| value.as_array()).into_iter().flatten().cloned()).collect()
    }

    fn fixture_endpoint_xy(fixture: &Value, endpoint_id: &str) -> Option<(f64, f64)> {
        if let Some((node_id, handle_id)) = endpoint_id.split_once(':') {
            let node = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(node_id))?;
            let cx = node.get("x").and_then(|value| value.as_f64())?;
            let cy = node.get("y").and_then(|value| value.as_f64())?;
            let handle = node.get("handles").and_then(|value| value.as_array()).into_iter().flatten().find(|handle| handle.get("id").and_then(|value| value.as_str()) == Some(handle_id))?;
            let angle = handle.get("angle").and_then(|value| value.as_f64()).unwrap_or(0.0);
            let point = if node.get("shape").and_then(|value| value.as_str()) == Some("rectangle") {
                let width = node.get("width").and_then(|value| value.as_f64()).unwrap_or(48.0);
                let height = node.get("height").and_then(|value| value.as_f64()).unwrap_or(48.0);
                handle_position_on_rectangle(Point::new(cx, cy), width, height, angle)
            } else {
                let radius = node.get("radius").and_then(|value| value.as_f64()).unwrap_or(24.0);
                handle_position_on_circle(Point::new(cx, cy), radius, angle)
            };
            return Some((point.x, point.y));
        }
        let node = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(endpoint_id))?;
        Some((node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0), node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0)))
    }

    //#region 🔖PaneCamera
    fn puzzle2d_pane_zoom_scale(pane: &str) -> f64 {
        match pane {
            PUZZLE2D_PANE_DETAIL => PUZZLE2D_PANE_ZOOM_SCALE_DETAIL,
            PUZZLE2D_PANE_SELECTION => PUZZLE2D_PANE_ZOOM_SCALE_SELECTION,
            _ => PUZZLE2D_PANE_ZOOM_SCALE_OVERVIEW,
        }
    }

    fn puzzle2d_clamp_zoom(value: f64) -> f64 {
        value.clamp(BOARD_CAMERA_ZOOM_MIN, BOARD_CAMERA_ZOOM_MAX)
    }

    /// 📐 World-space center and half-span of every node's extent (circle radius or rectangle half-size), used to frame pane cameras.
    fn puzzle2d_fixture_world_bounds(fixture: &Value) -> (f64, f64, f64) {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for node in fixture_nodes(fixture) {
            let (Some(x), Some(y)) = (node.get("x").and_then(|value| value.as_f64()), node.get("y").and_then(|value| value.as_f64())) else {
                continue;
            };
            let (half_w, half_h) = if node.get("shape").and_then(|value| value.as_str()) == Some("rectangle") {
                (node.get("width").and_then(|value| value.as_f64()).unwrap_or(48.0) * 0.5, node.get("height").and_then(|value| value.as_f64()).unwrap_or(48.0) * 0.5)
            } else {
                let radius = node.get("radius").and_then(|value| value.as_f64()).unwrap_or(24.0);
                (radius, radius)
            };
            min_x = min_x.min(x - half_w);
            max_x = max_x.max(x + half_w);
            min_y = min_y.min(y - half_h);
            max_y = max_y.max(y + half_h);
        }
        if !min_x.is_finite() {
            return (0.0, 0.0, 400.0);
        }
        let half_span = (max_x - min_x).max(max_y - min_y).max(1.0) * 0.5;
        ((min_x + max_x) * 0.5, (min_y + max_y) * 0.5, half_span)
    }

    /// 📷 Triptych camera for a pane: overview is zoomed out and centered on the fixture, detail zooms into the last-placed node, selection frames a lower-left quadrant — mirrors the pre-migration `puzzle2dPlayTriptychCameraForPane`.
    fn puzzle2d_pane_camera(fixture: &Value, pane: &str) -> (f64, f64, f64) {
        let (camera_x, camera_y, camera_zoom) = fixture_camera(fixture);
        if pane == PUZZLE2D_PANE_OVERVIEW {
            return (camera_x, camera_y, puzzle2d_clamp_zoom(camera_zoom));
        }
        let (cx, cy, half_span) = puzzle2d_fixture_world_bounds(fixture);
        let usable = PUZZLE2D_VIEWPORT_REF_SHORT_PX * (1.0 - 2.0 * PUZZLE2D_VIEWPORT_MARGIN);
        let world_span = (2.0 * half_span * PUZZLE2D_VIEWPORT_FRAMING_HALF_SPAN_SCALE).max(1.0);
        let base_zoom = puzzle2d_clamp_zoom((usable / world_span) * PUZZLE2D_VIEWPORT_ZOOM_BOOST);
        let zoom = puzzle2d_clamp_zoom(base_zoom * puzzle2d_pane_zoom_scale(pane));
        match pane {
            PUZZLE2D_PANE_DETAIL => {
                let nodes = fixture_nodes(fixture);
                let detail_node = nodes.get(nodes.len().saturating_sub(1).min(42));
                let x = detail_node.and_then(|node| node.get("x")).and_then(|value| value.as_f64()).unwrap_or(cx) + camera_x * 0.02;
                let y = detail_node.and_then(|node| node.get("y")).and_then(|value| value.as_f64()).unwrap_or(cy) + camera_y * 0.02;
                (x, y, zoom)
            }
            PUZZLE2D_PANE_SELECTION => (cx - half_span * 0.28 + camera_x * 0.06, cy + half_span * 0.22 + camera_y * 0.05, zoom),
            _ => (cx + camera_x * 0.04, cy + camera_y * 0.03, zoom),
        }
    }

    /// 🖱️ Recovers the pane id from the `canvas-2d-host` surface id echoed back into pointer-action args.
    fn pane_from_surface_id(surface_id: &str) -> &'static str {
        if surface_id.ends_with(PUZZLE2D_PANE_DETAIL) {
            PUZZLE2D_PANE_DETAIL
        } else if surface_id.ends_with(PUZZLE2D_PANE_SELECTION) {
            PUZZLE2D_PANE_SELECTION
        } else {
            PUZZLE2D_PANE_OVERVIEW
        }
    }

    //#endregion 🔖PaneCamera

    /// 🗄️ Caches the last serialized fixture keyed by an fnv1a hash of the raw `document_json` it came from, so the overview/detail/selection panes of the same `refreshUi` tick reuse one `String` instead of each re-serializing the whole fixture graph.
    static PUZZLE2D_FIXTURE_JSON_CACHE: LazyLock<std::sync::Mutex<Option<(u64, String)>>> = LazyLock::new(|| std::sync::Mutex::new(None));

    fn fnv1a_hash(bytes: &[u8]) -> u64 {
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    fn cached_fixture_json(document_json: &str, fixture: &Value) -> String {
        let key = fnv1a_hash(document_json.as_bytes());
        let mut cache = PUZZLE2D_FIXTURE_JSON_CACHE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some((cached_key, cached_json)) = cache.as_ref() {
            if *cached_key == key {
                return cached_json.clone();
            }
        }
        let json = fixture.to_string();
        *cache = Some((key, json.clone()));
        json
    }

    fn puzzle2d_board_scene(document_json: &str, envelope: &Puzzle2dScene, pane: &str) -> Puzzle2dBoardScene {
        let fixture = &envelope.fixture;
        let (camera_x, camera_y, zoom) = puzzle2d_pane_camera(fixture, pane);
        let camera_json = json!({ "x": camera_x, "y": camera_y, "zoom": zoom }).to_string();
        let kind_catalogs_json = fixture.get("meta").and_then(|value| value.get("kindCatalogs")).map(|value| value.to_string()).unwrap_or_else(|| "{}".into());
        let selection_json = serde_json::to_string(&envelope.runtime.selected_ids).unwrap_or_else(|_| "[]".into());
        let brush_kind_weights_json = serde_json::to_string(&json!({
            "nodeWeights": envelope.runtime.node_kind_weights,
            "handleWeights": envelope.runtime.handle_kind_weights,
        }))
        .unwrap_or_else(|_| "{}".into());
        let kind_compatibility_json = fixture
            .get("meta")
            .and_then(|value| value.get("kindCompatibility"))
            .or_else(|| fixture.get("kindCompatibility"))
            .map(|value| value.to_string())
            .unwrap_or_else(|| "[]".into());
        let lod_mode = envelope.runtime.lod_mode_by_pane.get(pane).cloned().unwrap_or_else(|| PUZZLE2D_LOD_MODE_AUTOMATIC.to_string());
        Puzzle2dBoardScene {
            fixture_json: cached_fixture_json(document_json, fixture),
            camera_json,
            kind_catalogs_json,
            selection_json,
            interactive: pane == PUZZLE2D_PANE_OVERVIEW,
            hovered_id: None,
            active_tool: Some(envelope.active_tool.clone()),
            selection_method: envelope.runtime.selection_method.clone(),
            grid_snap_enabled: envelope.runtime.grid_snap_enabled,
            grid_factor: envelope.runtime.grid_factor,
            suggestion_offset: envelope.runtime.suggestion_offset,
            brush_kind_weights_json,
            kind_compatibility_json,
            lod_mode,
        }
    }

    fn render_canvas(document_json: &str, envelope: &Puzzle2dScene, pane: &str) -> UiNode {
        build_puzzle2d_board_scene(format!("{PUZZLE2D_PLAY_SURFACE_ID}.{pane}"), PUZZLE2D_PLAY_CONTROLLER_ID, puzzle2d_board_scene(document_json, envelope, pane))
    }

    fn force_layout_fixture(fixture: &mut Value) {
        let Ok(layout_json) = puzzle_2d::apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), r#"{"mode":"force-graph"}"#) else {
            return;
        };
        if let Ok(parsed) = serde_json::from_str(&layout_json) {
            *fixture = parsed;
        }
    }

    fn patch_inspector_nodes(fixture: &mut Value, ids: &[String], field: &str, value: &Value) {
        if let Some(nodes) = fixture.get_mut("nodes").and_then(|entry| entry.as_array_mut()) {
            for node in nodes {
                let Some(id) = node.get("id").and_then(|entry| entry.as_str()) else {
                    continue;
                };
                if !ids.is_empty() && !ids.contains(&id.to_string()) {
                    continue;
                }
                if let Some(obj) = node.as_object_mut() {
                    obj.insert(field.to_string(), value.clone());
                }
            }
        }
    }
    //#endregion 🔖Canvas

    //#region 🔖Terminology
    /// 🗣️ Complete UI label set for the 2d app; one field per label makes every terminology×locale combination compile-checked.
    struct Puzzle2dLabels {
        // entity nouns — remapped under the "reuse" terminology
        nodes: &'static str,
        handles: &'static str,
        // document tree / catalogue section labels
        edges: &'static str,
        none: &'static str,
        // window-kind titles (window headers / tab titles)
        window_overview: &'static str,
        window_detail: &'static str,
        window_selection: &'static str,
        // properties panel summary labels
        schema: &'static str,
        extension: &'static str,
        // inspector field labels
        id: &'static str,
        node_kind: &'static str,
        x: &'static str,
        y: &'static str,
        // measures
        automatic: &'static str,
        lod: &'static str,
        suggestion: &'static str,
        offset: &'static str,
        node_weights: &'static str,
        handle_weights: &'static str,
        // engagement
        select: &'static str,
        brush: &'static str,
        fill: &'static str,
        placement: &'static str,
    }

    const PUZZLE2D_LABELS_NATIVE_EN: Puzzle2dLabels = Puzzle2dLabels {
        nodes: "Nodes",
        handles: "Handles",
        edges: "Edges",
        none: "(none)",
        window_overview: "Overview",
        window_detail: "Detail",
        window_selection: "Selection",
        schema: "Schema",
        extension: "Extension",
        id: "Id",
        node_kind: "Node Kind",
        x: "X",
        y: "Y",
        automatic: "Automatic",
        lod: "LOD",
        suggestion: "Suggestion",
        offset: "Offset",
        node_weights: "Node Weights",
        handle_weights: "Handle Weights",
        select: "Select",
        brush: "Brush",
        fill: "Fill",
        placement: "Placement",
    };

    const PUZZLE2D_LABELS_NATIVE_DE: Puzzle2dLabels = Puzzle2dLabels {
        nodes: "Knoten",
        handles: "Anschlüsse",
        edges: "Kanten",
        none: "(keine)",
        window_overview: "Übersicht",
        window_detail: "Detail",
        window_selection: "Auswahl",
        schema: "Schema",
        extension: "Erweiterung",
        id: "Id",
        node_kind: "Knotenart",
        x: "X",
        y: "Y",
        automatic: "Automatisch",
        lod: "LOD",
        suggestion: "Vorschlag",
        offset: "Versatz",
        node_weights: "Knotengewichte",
        handle_weights: "Anschlussgewichte",
        select: "Auswählen",
        brush: "Pinsel",
        fill: "Füllen",
        placement: "Platzierung",
    };

    const PUZZLE2D_LABELS_REUSE_EN: Puzzle2dLabels = Puzzle2dLabels {
        nodes: "Building components",
        handles: "Connection points",
        window_overview: "Assembly",
        window_detail: "Connection Detail",
        window_selection: "Component Selection",
        ..PUZZLE2D_LABELS_NATIVE_EN
    };
    const PUZZLE2D_LABELS_REUSE_DE: Puzzle2dLabels = Puzzle2dLabels {
        nodes: "Baukomponenten",
        handles: "Verbindungspunkte",
        window_overview: "Baugruppe",
        window_detail: "Verbindungsdetail",
        window_selection: "Komponentenauswahl",
        ..PUZZLE2D_LABELS_NATIVE_DE
    };

    /// 🗣️ Resolves the active label set from the shell-provided locale/terminology; unknown terminology ids fall back to native.
    fn puzzle2d_labels(view_state: &ViewState) -> &'static Puzzle2dLabels {
        let terminology = view_state.terminology.as_deref().unwrap_or("native");
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        match (terminology, is_de) {
            ("reuse", true) => &PUZZLE2D_LABELS_REUSE_DE,
            ("reuse", false) => &PUZZLE2D_LABELS_REUSE_EN,
            (_, true) => &PUZZLE2D_LABELS_NATIVE_DE,
            (_, false) => &PUZZLE2D_LABELS_NATIVE_EN,
        }
    }
    //#endregion 🔖Terminology

    //#region 🔖DocumentPanel
    fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
        UiTreeItemNode {
            loading: None,
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

    fn node_label(node: &Value) -> String {
        node.get("text").and_then(|value| value.as_str()).filter(|value| !value.is_empty()).or_else(|| node.get("id").and_then(|value| value.as_str())).unwrap_or("node").into()
    }

    fn edge_label(edge: &Value, fixture: &Value) -> String {
        let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("?");
        let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("?");
        let source_label = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(source)).map(node_label).unwrap_or_else(|| source.into());
        let target_label = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(target)).map(node_label).unwrap_or_else(|| target.into());
        format!("{source_label} → {target_label}")
    }

    fn document_tree_selected_ids(fixture: &Value, selected: &[String]) -> Vec<String> {
        selected
            .iter()
            .filter_map(|id| {
                if fixture_nodes(fixture).iter().any(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
                    return Some(format!("puzzle2d-play-document.node.{id}"));
                }
                if fixture_edges(fixture).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
                    return Some(format!("puzzle2d-play-document.edge.{id}"));
                }
                None
            })
            .collect()
    }

    fn render_document_panel(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> UiNode {
        let fixture = &envelope.fixture;
        let node_items: Vec<UiTreeItemNode> = fixture_nodes(fixture)
            .iter()
            .filter_map(|node| {
                let id = node.get("id")?.as_str()?;
                Some(tree_item_with_action(format!("puzzle2d-play-document.node.{id}"), node_label(node), node.get("nodeKind").and_then(|value| value.as_str()).map(str::to_string), puzzle2d_action("setSelection", Some(json!({ "ids": [id] })))))
            })
            .collect();
        let edge_items: Vec<UiTreeItemNode> = fixture_edges(fixture)
            .iter()
            .filter_map(|edge| {
                let id = edge.get("id")?.as_str()?;
                Some(tree_item_with_action(format!("puzzle2d-play-document.edge.{id}"), edge_label(edge, fixture), edge.get("edgeKind").and_then(|value| value.as_str()).map(str::to_string), puzzle2d_action("setSelection", Some(json!({ "ids": [id] })))))
            })
            .collect();
        UiNode::Tree(UiTreeNode {
            loading: None,
            sections: vec![
                UiTreeSectionNode {
                    loading: None,
                    id: "puzzle2d-play-document.nodes".into(),
                    label: Some(labels.nodes.into()),
                    default_open: Some(true),
                    items: if node_items.is_empty() {
                        vec![UiTreeItemNode {
                            loading: None,
                            id: "puzzle2d-play-document.nodes.empty".into(),
                            label: labels.none.into(),
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
                        }]
                    } else {
                        node_items
                    },
                },
                UiTreeSectionNode {
                    loading: None,
                    id: "puzzle2d-play-document.edges".into(),
                    label: Some(labels.edges.into()),
                    default_open: Some(false),
                    items: if edge_items.is_empty() {
                        vec![UiTreeItemNode {
                            loading: None,
                            id: "puzzle2d-play-document.edges.empty".into(),
                            label: labels.none.into(),
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
                        }]
                    } else {
                        edge_items
                    },
                },
            ],
            selected_ids: Some(document_tree_selected_ids(fixture, &envelope.runtime.selected_ids)),
            highlighted_ids: None,
            selection_change: Some(puzzle2d_action("setSelection", None)),
            drop_action: None,
        })
    }
    //#endregion 🔖DocumentPanel

    //#region 🔖CataloguePanel
    fn catalog_kind_label(entry: &Value) -> String {
        entry.get("name").and_then(|value| value.as_str()).filter(|value| !value.is_empty()).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
    }

    fn inferred_kind_entries(fixture: &Value, field: &str) -> Vec<Value> {
        let mut ids = BTreeSet::new();
        match field {
            "nodes" => {
                for node in fixture_nodes(fixture) {
                    if let Some(kind) = node.get("nodeKind").and_then(|value| value.as_str()) {
                        ids.insert(kind.to_string());
                    }
                }
            }
            "handles" => {
                for node in fixture_nodes(fixture) {
                    if let Some(handles) = node.get("handles").and_then(|value| value.as_array()) {
                        for handle in handles {
                            if let Some(kind) = handle.get("handleKind").and_then(|value| value.as_str()) {
                                ids.insert(kind.to_string());
                            }
                        }
                    }
                }
            }
            "edges" => {
                for edge in fixture_edges(fixture) {
                    if let Some(kind) = edge.get("edgeKind").and_then(|value| value.as_str()) {
                        ids.insert(kind.to_string());
                    }
                }
            }
            _ => {}
        }
        ids.into_iter().map(|id| json!({ "id": id, "name": id })).collect()
    }

    /// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx) reads to auto-wire catalogue drag sources.
    const PUZZLE2D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

    fn puzzle2d_catalog_item_drag_data(slice: &str, kind_id: &str, entry: &Value) -> HashMap<String, String> {
        let mut payload = json!({ "kindId": kind_id, "catalogSlice": slice });
        if let Some(obj) = payload.as_object_mut() {
            if let Some(shape) = entry.get("shape") {
                obj.insert("shape".into(), shape.clone());
            }
            if let Some(radius) = entry.get("radius") {
                obj.insert("radius".into(), radius.clone());
            }
            if let Some(width) = entry.get("width") {
                obj.insert("width".into(), width.clone());
            }
            if let Some(height) = entry.get("height") {
                obj.insert("height".into(), height.clone());
            }
            if let Some(icon_kind) = entry.get("iconKind") {
                obj.insert("iconKind".into(), icon_kind.clone());
            }
        }
        HashMap::from([(PUZZLE2D_CATALOGUE_DRAG_MIME.to_string(), payload.to_string())])
    }

    fn kind_catalog_section(section_id: &str, slice: &str, label: &str, entries: &[Value], labels: &Puzzle2dLabels) -> UiTreeSectionNode {
        let items: Vec<UiTreeItemNode> = entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
                let draggable = slice == "nodes";
                UiTreeItemNode {
                    loading: None,
                    id: format!("{section_id}.{index}.{kind_id}"),
                    label: catalog_kind_label(entry),
                    description: Some(kind_id.into()),
                    icon_id: None,
                    selected: None,
                    default_open: None,
                    action: Some(puzzle2d_action("addNode", Some(json!({ "kind": kind_id })))),
                    hover_action: None,
                    unhover_action: None,
                    actions: None,
                    draggable: draggable.then_some(true),
                    drag_data: draggable.then(|| puzzle2d_catalog_item_drag_data(slice, kind_id, entry)),
                    items: None,
                    control: None,
                    is_hidden: None,
                }
            })
            .collect();
        UiTreeSectionNode {
            loading: None,
            id: section_id.into(),
            label: Some(label.into()),
            default_open: Some(true),
            items: if items.is_empty() {
                vec![UiTreeItemNode {
                    loading: None,
                    id: format!("{section_id}.empty"),
                    label: labels.none.into(),
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
                }]
            } else {
                items
            },
        }
    }

    fn render_catalogue_panel(fixture: &Value, labels: &Puzzle2dLabels) -> UiNode {
        let inferred_nodes = inferred_kind_entries(fixture, "nodes");
        let inferred_handles = inferred_kind_entries(fixture, "handles");
        let inferred_edges = inferred_kind_entries(fixture, "edges");
        let node_entries = kind_catalog_entries(fixture, "nodes").unwrap_or(inferred_nodes.as_slice());
        let handle_entries = kind_catalog_entries(fixture, "handles").unwrap_or(inferred_handles.as_slice());
        let edge_entries = kind_catalog_entries(fixture, "edges").unwrap_or(inferred_edges.as_slice());
        UiNode::Tree(UiTreeNode {
            loading: None,
            sections: vec![
                kind_catalog_section("puzzle2d-play-kinds.nodes", "nodes", labels.nodes, &node_entries, labels),
                kind_catalog_section("puzzle2d-play-kinds.handles", "handles", labels.handles, &handle_entries, labels),
                kind_catalog_section("puzzle2d-play-kinds.edges", "edges", labels.edges, &edge_entries, labels),
            ],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        })
    }
    //#endregion 🔖CataloguePanel

    //#region 🔖InspectorPanel
    fn render_properties_panel(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> UiNode {
        let selected_nodes: Vec<&Value> = envelope.runtime.selected_ids.iter().filter_map(|id| fixture_nodes(&envelope.fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))).collect();
        if selected_nodes.is_empty() {
            return ui_stack_vertical(vec![
                ui_text(format!("{}: {PUZZLE2D_FIXTURE_SCHEMA}", labels.schema)),
                ui_text(format!("{}: {}", labels.extension, puzzle_extension_id())),
                ui_text(format!("{}: {}", labels.nodes, fixture_nodes(&envelope.fixture).len())),
                ui_text(format!("{}: {}", labels.edges, fixture_edges(&envelope.fixture).len())),
            ]);
        }
        let node = selected_nodes[0];
        ui_stack_vertical(vec![
            ui_inspector_readonly_field("puzzle2d-play-inspector.id", labels.id, node.get("id").and_then(|value| value.as_str()).unwrap_or("").to_string()),
            ui_inspector_readonly_field("puzzle2d-play-inspector.node-kind", labels.node_kind, node.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("—").to_string()),
            ui_inspector_readonly_field("puzzle2d-play-inspector.x", labels.x, node.get("x").and_then(|value| value.as_f64()).map(|value| value.to_string()).unwrap_or_else(|| "—".into())),
            ui_inspector_readonly_field("puzzle2d-play-inspector.y", labels.y, node.get("y").and_then(|value| value.as_f64()).map(|value| value.to_string()).unwrap_or_else(|| "—".into())),
        ])
    }
    //#endregion 🔖InspectorPanel

    //#region 🔖Measures
    fn puzzle2d_lod_tier_ids() -> Vec<String> {
        serde_json::from_str::<Vec<Value>>(&puzzle_2d_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
    }

    fn puzzle2d_kind_ids(fixture: &Value, field: &str) -> Vec<String> {
        let inferred = inferred_kind_entries(fixture, field);
        let entries = kind_catalog_entries(fixture, field).unwrap_or(inferred.as_slice());
        entries.iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
    }

    /// 📶 Per-pane LOD select measure: "Automatic" plus every scale tier (minimap…micro), persisted via `setLodModeForPane`.
    fn puzzle2d_lod_measure(pane: &str, current_mode: &str, labels: &Puzzle2dLabels) -> WindowMeasure {
        let mut items = vec![MeasureSelectItem { id: PUZZLE2D_LOD_MODE_AUTOMATIC.into(), value: PUZZLE2D_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() }];
        items.extend(puzzle2d_lod_tier_ids().into_iter().map(|tier| MeasureSelectItem { id: tier.clone(), value: tier.clone(), label: tier }));
        WindowMeasure::Select { id: format!("{pane}-lod"), label: Some(labels.lod.into()), value: current_mode.into(), items, on_change: puzzle2d_action("setLodModeForPane", Some(json!({ "pane": pane }))) }
    }

    fn puzzle2d_kind_weight_measures(prefix: &str, ids: &[String], weights: &BTreeMap<String, f64>, catalog_slice: &str) -> Vec<WindowMeasure> {
        ids.iter()
            .map(|kind_id| {
                let weight = weights.get(kind_id).copied().unwrap_or(0.0);
                WindowMeasure::Slider {
                    id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-{prefix}-{kind_id}"),
                    label: Some(format!("{kind_id} {:.0}%", weight * 100.0)),
                    value: weight,
                    min: 0.0,
                    max: 1.0,
                    step: Some(0.01),
                    on_change: puzzle2d_action("setBrushKindWeights", Some(json!({ "kindId": kind_id, "catalogSlice": catalog_slice }))),
                }
            })
            .collect()
    }

    /// 🎚️ Suggestion offset slider plus node/handle kind-weight sliders, calling `setSuggestionOffset`/`setBrushKindWeights`.
    fn puzzle2d_suggestion_measures_group(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
        let node_ids = puzzle2d_kind_ids(&envelope.fixture, "nodes");
        let handle_ids = puzzle2d_kind_ids(&envelope.fixture, "handles");
        WindowMeasure::Group {
            id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion"),
            label: labels.suggestion.into(),
            default_open: Some(false),
            active_tool_id: None,
            children: vec![
                WindowMeasure::Slider {
                    id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-offset"),
                    label: Some(labels.offset.into()),
                    value: envelope.runtime.suggestion_offset,
                    min: PUZZLE2D_SUGGESTION_OFFSET_MIN,
                    max: PUZZLE2D_SUGGESTION_OFFSET_MAX,
                    step: Some(PUZZLE2D_SUGGESTION_OFFSET_STEP),
                    on_change: puzzle2d_action("setSuggestionOffset", None),
                },
                WindowMeasure::Group {
                    id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-distribution-nodes"),
                    label: labels.node_weights.into(),
                    default_open: Some(false),
                    active_tool_id: None,
                    children: puzzle2d_kind_weight_measures("node-kind", &node_ids, &envelope.runtime.node_kind_weights, "nodes"),
                },
                WindowMeasure::Group {
                    id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-distribution-handles"),
                    label: labels.handle_weights.into(),
                    default_open: Some(false),
                    active_tool_id: None,
                    children: puzzle2d_kind_weight_measures("handle-kind", &handle_ids, &envelope.runtime.handle_kind_weights, "handles"),
                },
            ],
        }
    }

    fn puzzle2d_window_measures(pane: &str, envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> Vec<WindowMeasure> {
        let mode = envelope.runtime.lod_mode_by_pane.get(pane).map(String::as_str).unwrap_or(PUZZLE2D_LOD_MODE_AUTOMATIC);
        let mut measures = vec![
            puzzle2d_lod_measure(pane, mode, labels),
            puzzle2d_suggestion_measures_group(envelope, labels),
            puzzle2d_fill_tool_options(envelope, labels),
        ];
        if let Some(brush) = puzzle2d_brush_tool_options(envelope, labels) {
            measures.push(brush);
        }
        measures
    }
    //#endregion 🔖Measures

    //#region 🔖Puzzle2dPlayApp
    /// 🧩 Puzzle-2d play app. Owns the `BoardHost` engine and ephemeral view `runtime`; the persisted
    /// document (the bare fixture json) lives in the wrapping `VcsDocumentApp`'s op store. Each action
    /// rehydrates the host from the projection, mutates a transient {@link Puzzle2dScene}, then emits
    /// the granular op delta (`puzzle2d_document_delta_ops`) turning the old fixture into the new.
    pub struct Puzzle2dPlayApp {
        host: BoardHost,
        runtime: Puzzle2dPlayRuntime,
        /// 🗄️ The fixture content last parsed into `host` via `parse_fixture_v1` — lets `handle_action`
        /// skip that full clear-scene-and-rebuild (and the kind-catalog/kind-compat re-push) on the
        /// large majority of actions (select/camera/tool/…) that never touch fixture content.
        last_synced_fixture: Option<Value>,
    }

    impl Default for Puzzle2dPlayApp {
        fn default() -> Self {
            Self { host: puzzle_board_host(), runtime: Puzzle2dPlayRuntime::default(), last_synced_fixture: None }
        }
    }

    impl DocumentApp for Puzzle2dPlayApp {
        type Projection = Value;
        type Op = Puzzle2dOp;

        fn app_id(&self) -> &str {
            PUZZLE2D_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            PUZZLE2D_FIXTURE_SCHEMA
        }

        fn initial_projection(&self) -> Value {
            default_empty_fixture()
        }

        fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> ActionEmit<Puzzle2dOp> {
            let before = doc.projection.clone();
            let active_tool = view_state.active_tool_id.as_deref().unwrap_or(PUZZLE2D_TOOL_SELECT).to_string();
            let mut envelope = Puzzle2dScene { fixture: before.clone(), runtime: self.runtime.clone(), active_tool: active_tool.clone() };
            // 🐢 `sync_host_fixture_content` (`parse_fixture_v1`) does a full `clear_scene()` + rebuild of
            // every node/handle/edge — skip it when the fixture content is byte-identical to what `host`
            // already has (the common case: select/camera/tool/… actions never touch fixture content).
            if self.last_synced_fixture.as_ref() != Some(&envelope.fixture) {
                sync_host_fixture_content(&mut self.host, &envelope);
                // 🧹 `parse_fixture_v1` always `clear_scene()`s then rebuilds, so it unconditionally emits
                // an `edgeCreate` for every edge as a side effect of parsing — not a real structural
                // change. Discard that parse-induced noise now so `apply_host_events` below only sees
                // events genuinely produced by *this* action's own engine calls (delete_selection, brush
                // ops, …); otherwise those spurious edgeCreate events get replayed into
                // `envelope.fixture.edges` on the *next* action, duplicating every edge every action.
                let _ = self.host.drain_events_json();
                self.last_synced_fixture = Some(envelope.fixture.clone());
            }
            sync_host_runtime_state(&mut self.host, &envelope);
            let mut coalesce_key: Option<String> = None;
            let mut effects: Vec<HostEffect> = Vec::new();
            // 🐢 Default to Full (safe: every unrecognized/rare action re-renders everything, same as
            // before this ticket); the narrow-tier arms below override it to the smallest scope that
            // actually covers what they touch.
            let mut ui_scope = semio_framework_core::kernel::UiDirtyScope::Full;
            match action {
                "setSelection" | "documentSelect" => {
                    envelope.runtime.selected_ids = selection_ids(args);
                    self.host.set_selection_ids(&envelope.runtime.selected_ids);
                    ui_scope = puzzle2d_select_scope();
                }
                "addNode" => {
                    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str());
                    add_node_to_fixture(&mut envelope.fixture, kind, args);
                    {}
                }
                "deleteSelection" => {
                    self.host.delete_selection();
                    delete_selection_from_fixture(&mut envelope.fixture, &envelope.runtime.selected_ids);
                    envelope.runtime.selected_ids.clear();
                    {}
                }
                "setSelectionFlag" => {
                    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
                    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
                    apply_selection_flag(&mut envelope.fixture, &envelope.runtime.selected_ids, flag, value);
                    {}
                }
                "duplicateSelection" => {
                    let new_ids = duplicate_selection_in_fixture(&mut envelope.fixture, &envelope.runtime.selected_ids);
                    if new_ids.is_empty() {
                        {}
                    } else {
                        envelope.runtime.selected_ids = new_ids;
                        self.host.set_selection_ids(&envelope.runtime.selected_ids);
                        {}
                    }
                }
                "selectSameKind" => {
                    let ids = select_same_kind_ids(&envelope.fixture, &envelope.runtime.selected_ids);
                    if ids.is_empty() {
                        {}
                    } else {
                        envelope.runtime.selected_ids = ids;
                        self.host.set_selection_ids(&envelope.runtime.selected_ids);
                        {}
                    }
                }
                "setCamera" => {
                    if let Some(camera) = args.and_then(|value| value.get("camera")) {
                        if let (Some(x), Some(y), Some(zoom)) = (camera.get("x").and_then(|value| value.as_f64()), camera.get("y").and_then(|value| value.as_f64()), camera.get("zoom").and_then(|value| value.as_f64())) {
                            self.host.set_camera(x, y, zoom);
                        }
                        set_fixture_camera(&mut envelope.fixture, camera);
                        coalesce_key = Some("camera".into());
                        ui_scope = puzzle2d_window_only_scope();
                    }
                }
                "setActiveExample" => {
                    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                    envelope.fixture = if example_id.is_empty() || example_id == "empty" {
                        default_empty_fixture()
                    } else if example_id == PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID || example_id == "concrete" {
                        serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_fixture())
                    } else if example_id == PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID || example_id == "nakagin" {
                        serde_json::from_str(NAKAGIN_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_fixture())
                    } else {
                        default_empty_fixture()
                    };
                    envelope.runtime = Puzzle2dPlayRuntime::default();
                    {}
                }
                SET_ACTIVE_TOOL_ACTION_ID => {
                    // 🧰 Host-owned tool switch (framework-injected View action): the new tool already lives in
                    // `view_state.active_tool_id`; here we only clear any in-progress brush/fill scratch and
                    // emit nothing. The host engine was re-pointed at the new tool by `sync_host_runtime_state`.
                    self.host.brush_fill_session_clear();
                    self.host.brush_cancel_slot();
                    let _ = self.host.drain_events_json();
                    self.runtime.fill_count = 0;
                    self.runtime.brush_candidates.clear();
                    self.runtime.brush_candidate_index = 0;
                    self.runtime.brush_candidate_source_handle_id = String::new();
                    for pane in PUZZLE2D_PANES {
                        self.runtime.engagement_input_by_pane.insert(pane.to_string(), String::new());
                    }
                    return ActionEmit::default();
                }
                "engagementInput" => {
                    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(PUZZLE2D_PANE_OVERVIEW);
                    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                    if PUZZLE2D_PANES.contains(&pane) {
                        envelope.runtime.engagement_input_by_pane.insert(pane.to_string(), value.to_string());
                        ui_scope = puzzle2d_window_and_engagements_scope();
                    }
                }
                "engagementSubmit" => {
                    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(PUZZLE2D_PANE_OVERVIEW).to_string();
                    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map(str::trim).unwrap_or("").to_lowercase();
                    let applied = match value.as_str() {
                        "select" | "brush" | "fill" => {
                            // 🧰 Reconcile the engagement text-command tool switch through the host-owned
                            // active tool: point the local engine now and let the framework persist the new
                            // `view_state.active_tool_id` for the pane via `HostEffect::SetActiveTool`.
                            self.host.set_active_tool(value.as_str());
                            effects.push(HostEffect::SetActiveTool { window_kind_id: pane.clone(), tool_id: value.clone() });
                            true
                        }
                        "clear" => {
                            envelope.runtime.selected_ids.clear();
                            self.host.set_selection_ids(&[]);
                            true
                        }
                        "rectangle" => {
                            envelope.runtime.selection_method = "rectangle".into();
                            self.host.set_selection_options("rectangle", "replace", true, true, true);
                            true
                        }
                        "lasso" => {
                            envelope.runtime.selection_method = "lasso".into();
                            self.host.set_selection_options("lasso", "replace", true, true, true);
                            true
                        }
                        _ => false,
                    };
                    if applied && PUZZLE2D_PANES.contains(&pane.as_str()) {
                        envelope.runtime.engagement_input_by_pane.insert(pane, String::new());
                    }
                    {}
                }
                "engagementAbort" => {
                    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(PUZZLE2D_PANE_OVERVIEW);
                    if PUZZLE2D_PANES.contains(&pane) {
                        envelope.runtime.engagement_input_by_pane.insert(pane.to_string(), String::new());
                    }
                    if active_tool != PUZZLE2D_TOOL_SELECT {
                        self.host.set_active_tool(PUZZLE2D_TOOL_SELECT);
                        effects.push(HostEffect::SetActiveTool { window_kind_id: pane.to_string(), tool_id: PUZZLE2D_TOOL_SELECT.into() });
                    }
                    {}
                }
                "engagementControlSelect" => {
                    let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
                    if let Some(index) = candidate_id.strip_prefix("puzzle2d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
                        self.host.brush_set_candidate_index(index);
                        envelope.runtime.brush_candidate_index = index;
                        {}
                    } else {
                        {}
                    }
                }
                "setLodModeForPane" => {
                    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or("");
                    let mode = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
                    if let (true, Some(mode)) = (PUZZLE2D_PANES.contains(&pane), mode) {
                        envelope.runtime.lod_mode_by_pane.insert(pane.to_string(), mode.to_string());
                        if pane == PUZZLE2D_PANE_OVERVIEW {
                            if mode == PUZZLE2D_LOD_MODE_AUTOMATIC {
                                self.host.set_automatic_lod(true);
                            } else {
                                self.host.set_automatic_lod(false);
                                self.host.set_forced_draw_lod_label(mode);
                            }
                        }
                        ui_scope = puzzle2d_window_and_measures_scope();
                    }
                }
                "setGridSnapEnabled" => {
                    let enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(false);
                    envelope.runtime.grid_snap_enabled = enabled;
                    self.host.set_grid_snap_enabled(enabled);
                    ui_scope = puzzle2d_window_and_measures_scope();
                }
                "setGridFactor" => {
                    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                        envelope.runtime.grid_factor = value;
                        let _ = self.host.set_grid_factor(value);
                        ui_scope = puzzle2d_window_and_measures_scope();
                    }
                }
                "setSelectionMethod" => {
                    let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
                    envelope.runtime.selection_method = method.into();
                    self.host.set_selection_options(method, "replace", true, true, true);
                    ui_scope = puzzle2d_window_only_scope();
                }
                "setBrushKindWeights" => {
                    if let Some(weights) = args.and_then(|value| value.get("weights")) {
                        envelope.runtime.node_kind_weights = weights.get("nodeWeights").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                        envelope.runtime.handle_kind_weights = weights.get("handleWeights").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                    } else if let Some(kind_id) = args.and_then(|value| value.get("kindId")).and_then(|value| value.as_str()) {
                        let weight = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()).unwrap_or(0.0).clamp(0.0, 1.0);
                        let slice = args.and_then(|value| value.get("catalogSlice")).and_then(|value| value.as_str()).unwrap_or("nodes");
                        if slice == "handles" {
                            envelope.runtime.handle_kind_weights.insert(kind_id.to_string(), weight);
                        } else {
                            envelope.runtime.node_kind_weights.insert(kind_id.to_string(), weight);
                        }
                    }
                    if let Ok(weights_json) = serde_json::to_string(&json!({
                        "nodeWeights": envelope.runtime.node_kind_weights,
                        "handleWeights": envelope.runtime.handle_kind_weights,
                    })) {
                        self.host.set_brush_kind_weights(&weights_json);
                    }
                    ui_scope = puzzle2d_window_and_measures_scope();
                }
                "setBrushNodeSize" => {
                    if let Some(size) = args.and_then(|value| value.get("size")).and_then(|value| value.as_f64()) {
                        self.host.set_brush_node_size(size);
                        ui_scope = puzzle2d_window_only_scope();
                    }
                }
                "setSuggestionOffset" => {
                    let distance = args.and_then(|value| value.get("distance").or_else(|| value.get("value"))).and_then(|value| value.as_f64());
                    if let Some(distance) = distance {
                        let clamped = distance.clamp(PUZZLE2D_SUGGESTION_OFFSET_MIN, PUZZLE2D_SUGGESTION_OFFSET_MAX);
                        envelope.runtime.suggestion_offset = clamped;
                        self.host.set_suggestion_offset(clamped);
                        ui_scope = puzzle2d_window_and_measures_scope();
                    }
                }
                "brushCycleCandidate" => {
                    let forward = args.and_then(|value| value.get("forward")).and_then(|value| value.as_bool()).unwrap_or(true);
                    self.host.brush_cycle_candidate(forward);
                    envelope.runtime.brush_candidate_index = envelope.runtime.brush_candidate_index.saturating_add(1);
                    ui_scope = puzzle2d_window_and_engagements_scope();
                }
                "brushSetCandidateIndex" => {
                    if let Some(index) = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()) {
                        self.host.brush_set_candidate_index(index as usize);
                        envelope.runtime.brush_candidate_index = index as usize;
                        ui_scope = puzzle2d_window_and_engagements_scope();
                    }
                }
                "brushOpenSlot" => {
                    if let Some(handle_id) = args.and_then(|value| value.get("handleId")).and_then(|value| value.as_str()) {
                        self.host.brush_open_slot(handle_id);
                    }
                    {}
                }
                "brushCommitSlot" => {
                    self.host.brush_commit_slot();
                    apply_host_events(&mut self.host, &mut envelope);
                    {}
                }
                "brushCancelSlot" => {
                    self.host.brush_cancel_slot();
                    {}
                }
                "setFillCount" => {
                    let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_f64()).map(|value| value.round().max(0.0) as u32).unwrap_or(0).min(PUZZLE2D_FILL_COUNT_MAX);
                    envelope.runtime.fill_count = count;
                    effects.push(HostEffect::SetActiveTool { window_kind_id: PUZZLE2D_PANE_OVERVIEW.into(), tool_id: PUZZLE2D_TOOL_FILL.into() });
                    self.host.set_active_tool("brush");
                    self.host.brush_fill_session_begin(count, 1);
                    let step = self.host.brush_fill_session_step(count.max(1));
                    if let Ok(progress) = serde_json::from_str::<Value>(&step) {
                        if let Some(placements) = progress.get("placements").and_then(|value| value.as_array()) {
                            for placement in placements {
                                apply_brush_place_payload(&mut envelope.fixture, placement);
                            }
                        }
                    }
                    {}
                }
                "brushFillSessionBegin" => {
                    let max_count = args.and_then(|value| value.get("maxCount")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
                    let seed = args.and_then(|value| value.get("seed")).and_then(|value| value.as_u64()).unwrap_or(1) as u32;
                    self.host.brush_fill_session_begin(max_count, u64::from(seed));
                    {}
                }
                "brushFillSessionStep" => {
                    let budget = args.and_then(|value| value.get("chunkBudget")).and_then(|value| value.as_u64()).unwrap_or(8) as u32;
                    let step = self.host.brush_fill_session_step(budget);
                    if let Ok(progress) = serde_json::from_str::<Value>(&step) {
                        if let Some(placements) = progress.get("placements").and_then(|value| value.as_array()) {
                            for placement in placements {
                                apply_brush_place_payload(&mut envelope.fixture, placement);
                            }
                        }
                    }
                    {}
                }
                "brushFillSessionClear" => {
                    self.host.brush_fill_session_clear();
                    envelope.runtime.fill_count = 0;
                    {}
                }
                "patchInspectorNodes" => {
                    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_else(|| envelope.runtime.selected_ids.clone());
                    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                    let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                    if !field.is_empty() {
                        patch_inspector_nodes(&mut envelope.fixture, &ids, field, &value);
                        {}
                    } else {
                        {}
                    }
                }
                "forceLayout" | "reorganize" => {
                    force_layout_fixture(&mut envelope.fixture);
                    {}
                }
                "redrawHandles" => {
                    if let Ok(next) = puzzle_2d::apply_edge_handle_snap_to_fixture_v1_json(&envelope.fixture.to_string()) {
                        if let Ok(parsed) = serde_json::from_str(&next) {
                            envelope.fixture = parsed;
                        }
                    }
                    {}
                }
                "selectAll" => {
                    let ids: Vec<String> = fixture_nodes(&envelope.fixture).iter().filter_map(|node| node.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect();
                    envelope.runtime.selected_ids = ids.clone();
                    self.host.set_selection_ids(&ids);
                    ui_scope = puzzle2d_select_scope();
                }
                "clearSelection" => {
                    envelope.runtime.selected_ids.clear();
                    self.host.set_selection_ids(&[]);
                    ui_scope = puzzle2d_select_scope();
                }
                "focusSelection" => {
                    if envelope.runtime.selected_ids.is_empty() {
                        {}
                    } else {
                        let mut min_x = f64::INFINITY;
                        let mut min_y = f64::INFINITY;
                        let mut max_x = f64::NEG_INFINITY;
                        let mut max_y = f64::NEG_INFINITY;
                        for node in fixture_nodes(&envelope.fixture) {
                            let Some(id) = node.get("id").and_then(|value| value.as_str()) else {
                                continue;
                            };
                            if !envelope.runtime.selected_ids.iter().any(|selected| selected == id) {
                                continue;
                            }
                            let x = node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
                            let y = node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
                            let radius = node.get("radius").and_then(|value| value.as_f64()).unwrap_or(24.0);
                            min_x = min_x.min(x - radius);
                            min_y = min_y.min(y - radius);
                            max_x = max_x.max(x + radius);
                            max_y = max_y.max(y + radius);
                        }
                        if min_x.is_finite() {
                            let camera = json!({
                                "x": (min_x + max_x) * 0.5,
                                "y": (min_y + max_y) * 0.5,
                                "zoom": 1.0,
                            });
                            set_fixture_camera(&mut envelope.fixture, &camera);
                            if let (Some(x), Some(y), Some(zoom)) = (camera.get("x").and_then(|value| value.as_f64()), camera.get("y").and_then(|value| value.as_f64()), camera.get("zoom").and_then(|value| value.as_f64())) {
                                self.host.set_camera(x, y, zoom);
                            }
                            {}
                        } else {
                            {}
                        }
                    }
                }
                "applyBoardEvents" => {
                    if let Some(events_json) = args.and_then(|value| value.get("eventsJson")).and_then(|value| value.as_str()) {
                        ui_scope = serde_json::from_str::<Vec<Value>>(events_json).map(|events| puzzle2d_board_events_scope(&events)).unwrap_or(semio_framework_core::kernel::UiDirtyScope::Full);
                        apply_board_events_from_json(events_json, &mut envelope);
                        // 🪞 `apply_host_events` below trusts `host.selection` as the post-action source of
                        // truth and overwrites `envelope.runtime.selected_ids` with it — mirror the new
                        // selection into the host now (as every other selection-setting arm already does)
                        // or the just-applied `select`/`brushCandidates` selection is silently reverted.
                        self.host.set_selection_ids(&envelope.runtime.selected_ids);
                    }
                }
                "lodScaleJson" => {
                    let _ = puzzle_2d_lod_scale_json();
                    ui_scope = semio_framework_core::kernel::UiDirtyScope::None;
                }
                _ => {}
            }
            apply_host_events(&mut self.host, &mut envelope);
            self.runtime = envelope.runtime;
            let ops = puzzle2d_document_delta_ops(&before, &envelope.fixture);
            // 🐢 Safety net: a `None` scope claims nothing needs re-rendering — never pair that with an
            // actual document mutation (would silently desync remote clients' UI from the committed op).
            if !ops.is_empty() && matches!(ui_scope, semio_framework_core::kernel::UiDirtyScope::None) {
                ui_scope = semio_framework_core::kernel::UiDirtyScope::Full;
            }
            ActionEmit { ops, coalesce_key, effects, ui_scope, ..Default::default() }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> UiNode {
            let document_json = doc.projection.to_string();
            let envelope = Puzzle2dScene { fixture: doc.projection.clone(), runtime: self.runtime.clone(), active_tool: puzzle2d_active_tool(view_state) };
            let labels = puzzle2d_labels(view_state);
            match body_key {
                PUZZLE2D_PLAY_BODY_OVERVIEW => render_canvas(&document_json, &envelope, PUZZLE2D_PANE_OVERVIEW),
                PUZZLE2D_PLAY_BODY_DETAIL => render_canvas(&document_json, &envelope, PUZZLE2D_PANE_DETAIL),
                PUZZLE2D_PLAY_BODY_SELECTION => render_canvas(&document_json, &envelope, PUZZLE2D_PANE_SELECTION),
                PUZZLE2D_PLAY_BODY_LAYERS => render_document_panel(&envelope, labels),
                PUZZLE2D_PLAY_BODY_CATALOGUE => render_catalogue_panel(&envelope.fixture, labels),
                PUZZLE2D_PLAY_BODY_PROPERTIES => render_properties_panel(&envelope, labels),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn window_engagements(&self, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
            let envelope = Puzzle2dScene { fixture: doc.projection.clone(), runtime: self.runtime.clone(), active_tool: puzzle2d_active_tool(view_state) };
            let labels = puzzle2d_labels(view_state);
            PUZZLE2D_PANES.iter().map(|pane| (pane.to_string(), puzzle2d_engagement(&envelope, &self.host, pane, labels))).collect()
        }

        fn window_measures(&self, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
            let envelope = Puzzle2dScene { fixture: doc.projection.clone(), runtime: self.runtime.clone(), active_tool: puzzle2d_active_tool(view_state) };
            let labels = puzzle2d_labels(view_state);
            PUZZLE2D_PANES.iter().map(|pane| (pane.to_string(), puzzle2d_window_measures(pane, &envelope, labels))).collect()
        }

        fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
            let labels = puzzle2d_labels(view_state);
            semio_framework_plugin::AppLabelsOverlay {
                app_label: None,
                window_kind_labels: std::collections::HashMap::from([
                    (PUZZLE2D_PANE_OVERVIEW.to_string(), labels.window_overview.to_string()),
                    (PUZZLE2D_PANE_DETAIL.to_string(), labels.window_detail.to_string()),
                    (PUZZLE2D_PANE_SELECTION.to_string(), labels.window_selection.to_string()),
                ]),
                panel_tab_labels: std::collections::HashMap::new(),
                mode_labels: std::collections::HashMap::new(),
            }
        }
    }
    //#endregion 🔖Puzzle2dPlayApp

    //#region 🔖AppFactory
    /// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/engagement-bound
    /// vocabulary dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
    fn puzzle2d_internal_action(id: &str, label: &str, kind: ActionKind) -> ActionDefinition {
        ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, kind) }
    }

    /// 🧰 One canvas tool declaration (host-owned active tool). Select/brush/fill are this window's entire
    /// top-level exclusive tool set — not a sub-collection — so each carries `group: None` and renders as
    /// its own flat toolbar icon (matching the `process` toolbar), never a collapsed dropdown.
    fn puzzle2d_tool(id: &str, label: &str, icon: &str, category: ToolCategory) -> ToolDefinition {
        ToolDefinition { category: Some(category), ..ToolDefinition::new(id, label, icon) }
    }

    pub fn create_puzzle2d_app() -> App {
        let mut host = puzzle_board_host();
        let envelope = Puzzle2dScene { fixture: default_empty_fixture(), runtime: Puzzle2dPlayRuntime::default(), active_tool: PUZZLE2D_TOOL_SELECT.into() };
        sync_host_from_envelope(&mut host, &envelope);
        let labels = puzzle2d_labels(&ViewState::default());
        let mut app = App::from_builder(
            App::builder(PUZZLE2D_PLAY_APP_ID, "Puzzle 2D")
                .document(["semio", "puzzle", "2d"])
                .icon_id("puzzle2d")
                .terminology("reuse")
                .mode("edit", "Edit")
                .default_mode_id("edit")
                .window_kind_with_engagement(PUZZLE2D_PANE_OVERVIEW, "Overview", PUZZLE2D_PLAY_BODY_OVERVIEW, SurfaceKind::Canvas2d, puzzle2d_engagement(&envelope, &host, PUZZLE2D_PANE_OVERVIEW, labels))
                .window_kind_with_engagement(PUZZLE2D_PANE_DETAIL, "Detail", PUZZLE2D_PLAY_BODY_DETAIL, SurfaceKind::Canvas2d, puzzle2d_engagement(&envelope, &host, PUZZLE2D_PANE_DETAIL, labels))
                .window_kind_with_engagement(PUZZLE2D_PANE_SELECTION, "Selection", PUZZLE2D_PLAY_BODY_SELECTION, SurfaceKind::Canvas2d, puzzle2d_engagement(&envelope, &host, PUZZLE2D_PANE_SELECTION, labels))
                .panel_tab("framework.panel.document", FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PUZZLE2D_PLAY_BODY_LAYERS)
                .panel_tab("framework.panel.catalogue", FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PUZZLE2D_PLAY_BODY_CATALOGUE)
                .panel_tab("framework.panel.inspection", FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PUZZLE2D_PLAY_BODY_PROPERTIES)
                // ✏️ Palette-visible content operations.
                .operation("addNode", "Add Node")
                .operation("setActiveExample", "Set Active Example")
                .operation("deleteSelection", "Delete Selection")
                .operation("duplicateSelection", "Duplicate Selection")
                .operation("forceLayout", "Force Layout")
                .operation("focusSelection", "Focus Selection")
                // 👁️ Palette-visible ephemeral view/selection commands.
                .view_action("selectAll", "Select All")
                .view_action("clearSelection", "Clear Selection")
                .view_action("selectSameKind", "Select Same Kind")
                // 🔧 Internal content operations — inspector/panel/board/import-bound, not palette commands.
                .action_with(puzzle2d_internal_action("setSelectionFlag", "Set Selection Flag", ActionKind::Operation))
                .action_with(puzzle2d_internal_action("setCamera", "Set Camera", ActionKind::Operation))
                .action_with(puzzle2d_internal_action("patchInspectorNodes", "Patch Inspector Nodes", ActionKind::Operation))
                .action_with(puzzle2d_internal_action("redrawHandles", "Redraw Handles", ActionKind::Operation))
                .action_with(puzzle2d_internal_action("reorganize", "Reorganize", ActionKind::Operation))
                .action_with(puzzle2d_internal_action("applyBoardEvents", "Apply Board Events", ActionKind::Operation))
                .action_with(puzzle2d_internal_action("setFillCount", "Set Fill Count", ActionKind::Operation))
                .action_with(puzzle2d_internal_action("brushFillSessionStep", "Brush Fill Session Step", ActionKind::Operation))
                .action_with(puzzle2d_internal_action("brushCommitSlot", "Brush Commit Slot", ActionKind::Operation))
                // 🖱️ Internal pointer/gesture/engagement view vocabulary — pure runtime/host state, emit no ops.
                .action_with(puzzle2d_internal_action("setSelection", "Set Selection", ActionKind::View))
                .action_with(puzzle2d_internal_action("documentSelect", "Document Select", ActionKind::View))
                .action_with(puzzle2d_internal_action("engagementInput", "Engagement Input", ActionKind::View))
                .action_with(puzzle2d_internal_action("engagementSubmit", "Engagement Submit", ActionKind::View))
                .action_with(puzzle2d_internal_action("engagementAbort", "Engagement Abort", ActionKind::View))
                .action_with(puzzle2d_internal_action("engagementControlSelect", "Engagement Control Select", ActionKind::View))
                .action_with(puzzle2d_internal_action("setLodModeForPane", "Set LOD Mode For Pane", ActionKind::View))
                .action_with(puzzle2d_internal_action("setGridSnapEnabled", "Set Grid Snap Enabled", ActionKind::View))
                .action_with(puzzle2d_internal_action("setGridFactor", "Set Grid Factor", ActionKind::View))
                .action_with(puzzle2d_internal_action("setSelectionMethod", "Set Selection Method", ActionKind::View))
                .action_with(puzzle2d_internal_action("setBrushKindWeights", "Set Brush Kind Weights", ActionKind::View))
                .action_with(puzzle2d_internal_action("setBrushNodeSize", "Set Brush Node Size", ActionKind::View))
                .action_with(puzzle2d_internal_action("setSuggestionOffset", "Set Suggestion Offset", ActionKind::View))
                .action_with(puzzle2d_internal_action("brushCycleCandidate", "Brush Cycle Candidate", ActionKind::View))
                .action_with(puzzle2d_internal_action("brushSetCandidateIndex", "Brush Set Candidate Index", ActionKind::View))
                .action_with(puzzle2d_internal_action("brushOpenSlot", "Brush Open Slot", ActionKind::View))
                .action_with(puzzle2d_internal_action("brushCancelSlot", "Brush Cancel Slot", ActionKind::View))
                .action_with(puzzle2d_internal_action("brushFillSessionBegin", "Brush Fill Session Begin", ActionKind::View))
                .action_with(puzzle2d_internal_action("brushFillSessionClear", "Brush Fill Session Clear", ActionKind::View))
                .action_with(puzzle2d_internal_action("lodScaleJson", "LOD Scale Json", ActionKind::View))
                // 📝 Staged palette args for the two content commands that need a target.
                .action_args("addNode", vec![
                    ActionArgDef::select("kind", "Kind", vec![ActionArgOption::new("node", "Node")]).required().default_value("node"),
                ])
                .action_args("setActiveExample", vec![
                    ActionArgDef::select("exampleId", "Example", vec![
                        ActionArgOption::new("empty", "Empty"),
                        ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, "Concrete Forest"),
                        ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, "Nakagin Capsule Tower"),
                    ]).required().default_value("empty"),
                ])
                // 🧰 Canvas tools — one exclusive set, active tool host-owned (never a document op). The
                // select/brush/fill switcher is rendered by the framework toolbar for the interactive pane.
                .tool(puzzle2d_tool(PUZZLE2D_TOOL_SELECT, "Select", "cursor", ToolCategory::Selection))
                .tool(puzzle2d_tool(PUZZLE2D_TOOL_BRUSH, "Brush", "brush", ToolCategory::Tools))
                .tool(puzzle2d_tool(PUZZLE2D_TOOL_FILL, "Fill", "fill", ToolCategory::Tools))
                .window_kind_tools(PUZZLE2D_PANE_OVERVIEW, vec![PUZZLE2D_TOOL_SELECT.into(), PUZZLE2D_TOOL_BRUSH.into(), PUZZLE2D_TOOL_FILL.into()])
                .default_layout(create_default_layout(&[PUZZLE2D_PANE_OVERVIEW.into(), PUZZLE2D_PANE_DETAIL.into(), PUZZLE2D_PANE_SELECTION.into()], "row", Some(&[50.0, 25.0, 25.0]), Some(&["Overview".into(), "Detail".into(), "Selection".into()]))),
        );
        for pane in PUZZLE2D_PANES {
            if let Some(window) = app.definition.window_kinds.iter_mut().find(|window| window.id == pane) {
                window.options.measures = puzzle2d_window_measures(pane, &envelope, labels);
            }
        }
        app.example("empty", "Empty", serde_json::to_string(&default_empty_fixture()).unwrap())
            .example(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, "Concrete Forest", serde_json::to_string(&example_fixture(CONCRETE_FOREST_EXAMPLE_JSON)).unwrap())
            .example(PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, "Nakagin Capsule Tower", serde_json::to_string(&example_fixture(NAKAGIN_EXAMPLE_JSON)).unwrap())
            .program("puzzle2d", "Puzzle 2D", "layout")
    }

    fn puzzle2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
        semio_framework_os::title_card_svg(value, "Puzzle 2D", 1024, 768)
    }

    /// 📥 Tier C DWG import — the puzzle-2d fixture only supports circle/rectangle nodes (no polygonal outlines), so this
    /// always returns an empty board whose camera is framed to the DWG's extents; never errors on a structurally valid DWG.
    fn puzzle2d_document_json_from_dwg(drawing: &semio_framework_os::DwgDrawing) -> Result<Value, String> {
        let mut fixture = default_empty_fixture();
        let center_x = (drawing.extmin[0] + drawing.extmax[0]) / 2.0;
        let center_y = (drawing.extmin[1] + drawing.extmax[1]) / 2.0;
        fixture["camera"] = json!({ "x": center_x, "y": center_y, "zoom": 1.0 });
        Ok(fixture)
    }

    pub fn register_puzzle2d_exports() {
        semio_framework_os::register_2d_export_handlers("2d.puzzle", "puzzle2d", puzzle2d_document_json_to_svg);
        semio_framework_os::register_dwg_import_handler("2d.puzzle", puzzle2d_document_json_from_dwg);
    }
    //#endregion 🔖AppFactory

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};
        use semio_framework_plugin::app::AppActionRegistry;
        use vcs::{Backbone, BackboneMessage, MemoryBackbone};

        fn meta(actor: &str) -> ActionMeta {
            ActionMeta { actor: actor.into(), instance_id: 1 }
        }

        fn new_app() -> VcsDocumentApp<Puzzle2dPlayApp> {
            VcsDocumentApp::new(Puzzle2dPlayApp::default())
        }

        /// 🧰 A registry-backed app so kind discipline (View/Shell actions must emit no ops) and the tool
        /// contract are enforced exactly as in production (`VcsDocumentApp::with_registry`).
        fn registry_app() -> VcsDocumentApp<Puzzle2dPlayApp> {
            VcsDocumentApp::with_registry(Puzzle2dPlayApp::default(), AppActionRegistry::from_definition(&create_puzzle2d_app().definition))
        }

        fn brush_view_state() -> ViewState {
            ViewState { active_tool_id: Some(PUZZLE2D_TOOL_BRUSH.into()), ..ViewState::default() }
        }

        fn concrete_forest_app() -> VcsDocumentApp<Puzzle2dPlayApp> {
            let mut app = new_app();
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), &ViewState::default(), &meta("local")).expect("load concrete forest");
            app
        }

        #[test]
        fn renders_puzzle2d_board_scene() {
            let mut app = new_app();
            let node = app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("puzzle2d-board"));
        }

        #[test]
        fn add_node_action_emits_upsert_op_and_appends_node() {
            let mut app = new_app();
            let result = app.handle_action("addNode", Some(&json!({ "kind": "node" })), &ViewState::default(), &meta("local")).expect("add node");
            assert_eq!(result.operations.len(), 1, "addNode must emit exactly one granular op");
            assert_eq!(fixture_nodes(&app.projection().expect("projection")).len(), 1);
        }

        #[test]
        fn set_active_example_loads_concrete_forest_via_ops() {
            let mut app = new_app();
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), &ViewState::default(), &meta("local")).expect("load example");
            assert!(!fixture_nodes(&app.projection().expect("projection")).is_empty());
        }

        #[test]
        fn select_then_delete_selection_removes_the_node() {
            let mut app = new_app();
            app.handle_action("addNode", Some(&json!({ "kind": "node" })), &ViewState::default(), &meta("local")).expect("add node");
            let node_id = fixture_nodes(&app.projection().expect("projection"))[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
            app.handle_action("setSelection", Some(&json!({ "ids": [node_id] })), &ViewState::default(), &meta("local")).expect("select");
            app.handle_action("deleteSelection", None, &ViewState::default(), &meta("local")).expect("delete");
            assert!(fixture_nodes(&app.projection().expect("projection")).is_empty());
        }

        #[test]
        fn undo_redo_round_trip_through_the_wrapper() {
            let mut app = new_app();
            app.handle_action("addNode", Some(&json!({ "kind": "node" })), &ViewState::default(), &meta("local")).expect("add");
            assert_eq!(fixture_nodes(&app.projection().expect("projection")).len(), 1);
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            assert_eq!(fixture_nodes(&app.projection().expect("projection")).len(), 0);
            app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
            assert_eq!(fixture_nodes(&app.projection().expect("projection")).len(), 1);
        }

        #[test]
        fn camera_drag_coalesces_into_one_undo_step() {
            let mut app = new_app();
            let camera_x = |app: &VcsDocumentApp<Puzzle2dPlayApp>| app.projection().expect("projection").get("camera").and_then(|camera| camera.get("x")).and_then(|value| value.as_f64()).unwrap_or(f64::NAN);
            let origin_x = camera_x(&app);
            for x in [1.0, 2.0, 3.0] {
                app.handle_action("setCamera", Some(&json!({ "camera": { "x": x, "y": 0.0, "zoom": 1.0 } })), &ViewState::default(), &meta("local")).expect("camera");
            }
            assert_eq!(camera_x(&app), 3.0);
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            assert_eq!(camera_x(&app), origin_x, "the coalesced drag is a single undo step back to the loaded camera");
        }

        /// 🐢 Regression test for a perf-round-2 bug: `sync_host_from_envelope`'s `parse_fixture_v1`
        /// always `clear_scene()`s then rebuilds, so every edge looked "new" and got re-`push_event`'d
        /// as `edgeCreate` — which `apply_host_events` then replayed into `envelope.fixture.edges` on
        /// the *next* action, duplicating every edge once per action forever. Repeated no-op actions
        /// (here: repeated selects) must leave the edge count untouched.
        #[test]
        fn repeated_actions_do_not_duplicate_edges() {
            let mut app = new_app();
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID })), &ViewState::default(), &meta("local")).expect("load nakagin");
            let edge_count = |app: &VcsDocumentApp<Puzzle2dPlayApp>| fixture_edges(&app.projection().expect("projection")).len();
            let before = edge_count(&app);
            assert!(before > 0, "fixture must have edges for this regression test to be meaningful");
            let node_id = fixture_nodes(&app.projection().expect("projection"))[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
            for _ in 0..5 {
                app.handle_action("applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), &ViewState::default(), &meta("local")).expect("select");
            }
            assert_eq!(edge_count(&app), before, "selecting repeatedly must not grow the edges array");
        }

        /// 🪞 Regression test: `applyBoardEvents`'s `select` case only mutated `envelope.runtime`, never
        /// `self.host`, so `apply_host_events`'s `host.selection`-is-truth re-sync silently reverted the
        /// selection to whatever `self.host` held before the action (empty, on a fresh sync).
        #[test]
        fn apply_board_events_select_persists_across_the_next_action() {
            let mut app = concrete_forest_app();
            let node_id = fixture_nodes(&app.projection().expect("projection"))[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
            app.handle_action("applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), &ViewState::default(), &meta("local")).expect("select");
            let rendered_once = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, None, &ViewState::default()).expect("render")).unwrap();
            assert!(rendered_once.contains(&node_id), "selection must be visible immediately after the select action");
            // A second, unrelated action used to silently clear the selection via the stale `host.selection` re-sync.
            app.handle_action("applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), &ViewState::default(), &meta("local")).expect("no-op");
            let rendered_twice = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, None, &ViewState::default()).expect("render")).unwrap();
            assert!(rendered_twice.contains(&node_id), "selection must survive a subsequent unrelated action");
        }

        /// 🪞 Regression test: `apply_host_events` used to epsilon-compare `host.camera` (still the
        /// *pre-action* value) against the fixture and blindly overwrite the fixture with it, reverting
        /// a plain `camera` board event (used for the live wheel-zoom echo) before it ever committed.
        #[test]
        fn apply_board_events_camera_event_commits() {
            let mut app = new_app();
            app.handle_action("applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 5.0, "y": 6.0, "zoom": 1.2 } }]).to_string() })), &ViewState::default(), &meta("local")).expect("camera event");
            let camera = app.projection().expect("projection").get("camera").cloned().expect("camera field");
            assert_eq!(camera.get("x").and_then(Value::as_f64), Some(5.0));
            assert_eq!(camera.get("y").and_then(Value::as_f64), Some(6.0));
            assert_eq!(camera.get("zoom").and_then(Value::as_f64), Some(1.2));
        }

        /// 🐢 A pure selection change is runtime state, not document state — it must not produce any
        /// `KernelOperation`s (previously it fell back to a whole-document `ReplaceDocument` once the
        /// edge-duplication bug made `before` and `after` genuinely diverge).
        #[test]
        fn select_action_emits_no_ops() {
            let mut app = concrete_forest_app();
            let node_id = fixture_nodes(&app.projection().expect("projection"))[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
            let result = app.handle_action("applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), &ViewState::default(), &meta("local")).expect("select");
            assert!(result.operations.is_empty(), "selection must not produce document ops");
        }

        /// 🐢 Perf round 3: a select event must declare a narrow `Partial` ui_scope (the 3 canvas panes +
        /// layers/properties panels + engagements) — never `Full`, or the shell's batched `refresh-ui`
        /// call degrades back to fetching everything on every select.
        #[test]
        fn select_action_declares_partial_ui_scope() {
            use semio_framework_core::kernel::UiDirtyScope;
            let mut app = concrete_forest_app();
            let node_id = fixture_nodes(&app.projection().expect("projection"))[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
            let result = app.handle_action("applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), &ViewState::default(), &meta("local")).expect("select");
            match result.ui_scope {
                UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, tools, labels } => {
                    // 🐢 Regression: `window_bodies` must list the window *body keys* (matched against
                    // `AppDefinition.windowKinds[].bodyKey` by the shell's `buildUiRefreshRequest`), not
                    // the pane/kind-id constants (`PUZZLE2D_PANES`) — those are a different id space.
                    assert_eq!(window_bodies, vec![PUZZLE2D_PLAY_BODY_OVERVIEW, PUZZLE2D_PLAY_BODY_DETAIL, PUZZLE2D_PLAY_BODY_SELECTION], "window_bodies must be body keys, not pane ids");
                    assert!(panel_bodies.contains(&PUZZLE2D_PLAY_BODY_LAYERS.to_string()));
                    assert!(panel_bodies.contains(&PUZZLE2D_PLAY_BODY_PROPERTIES.to_string()));
                    assert!(engagements, "select must refresh the engagement bar");
                    assert!(!measures, "select must not force a measures refresh");
                    assert!(!tools);
                    assert!(!labels);
                }
                other => panic!("expected a Partial ui_scope for select, got {other:?}"),
            }
        }

        /// 🐢 Perf round 3: a camera-only board event touches only the 3 canvas panes — no panels,
        /// engagements, measures, or tools.
        #[test]
        fn camera_event_declares_window_only_ui_scope() {
            use semio_framework_core::kernel::UiDirtyScope;
            let mut app = new_app();
            let result = app.handle_action("applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 1.0, "y": 2.0, "zoom": 1.0 } }]).to_string() })), &ViewState::default(), &meta("local")).expect("camera event");
            match result.ui_scope {
                UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, tools, labels } => {
                    assert_eq!(window_bodies.len(), 3);
                    assert!(panel_bodies.is_empty());
                    assert!(!engagements && !measures && !tools && !labels);
                }
                other => panic!("expected a Partial ui_scope for a camera event, got {other:?}"),
            }
        }

        /// 🐢 Perf round 3: an empty `applyBoardEvents` batch (no-op) must declare `None` — nothing for
        /// the shell to re-render at all.
        #[test]
        fn empty_board_events_declare_none_ui_scope() {
            use semio_framework_core::kernel::UiDirtyScope;
            let mut app = new_app();
            let result = app.handle_action("applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), &ViewState::default(), &meta("local")).expect("no-op");
            assert!(matches!(result.ui_scope, UiDirtyScope::None), "empty board events must declare None, got {:?}", result.ui_scope);
        }

        /// 🐢 Perf round 3: cold-tier structural actions (document ops) must keep the safe `Full`
        /// default — no puzzle2d scope helper narrows them.
        #[test]
        fn add_node_action_declares_full_ui_scope() {
            use semio_framework_core::kernel::UiDirtyScope;
            let mut app = new_app();
            let result = app.handle_action("addNode", Some(&json!({ "kind": "node" })), &ViewState::default(), &meta("local")).expect("add node");
            assert!(matches!(result.ui_scope, UiDirtyScope::Full), "addNode must stay Full, got {:?}", result.ui_scope);
        }

        #[test]
        fn document_panel_lists_nodes_section() {
            let mut app = concrete_forest_app();
            let json = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
            assert!(json.contains("puzzle2d-play-document.nodes"));
            assert!(json.contains("seed-left-001"));
        }

        #[test]
        fn labels_resolve_native_english_and_german_and_reuse() {
            let mut app = concrete_forest_app();
            let english = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
            assert!(english.contains("\"Nodes\"") && english.contains("\"Edges\""));
            let german = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_LAYERS, None, &ViewState { locale: Some("de".into()), ..ViewState::default() }).expect("render")).unwrap();
            assert!(german.contains("\"Knoten\"") && german.contains("\"Kanten\""));
            let reuse = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_LAYERS, None, &ViewState { terminology: Some("reuse".into()), locale: Some("en".into()), ..ViewState::default() }).expect("render")).unwrap();
            assert!(reuse.contains("Building components"));
        }

        #[test]
        fn app_definition_has_three_lod_pane_window_kinds() {
            let app = create_puzzle2d_app();
            let ids: Vec<&str> = app.definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
            assert_eq!(ids, vec![PUZZLE2D_PANE_OVERVIEW, PUZZLE2D_PANE_DETAIL, PUZZLE2D_PANE_SELECTION]);
            for window in &app.definition.window_kinds {
                assert!(window.options.engagement.as_option().is_some(), "pane {} must have engagement", window.id);
                assert!(!window.options.measures.is_empty(), "pane {} must have LOD/suggestion measures", window.id);
            }
        }

        #[test]
        fn dwg_import_returns_empty_board_framed_to_extents() {
            let mut drawing = semio_framework_os::DwgDrawing::default();
            drawing.extmin = [0.0, 0.0, 0.0];
            drawing.extmax = [100.0, 200.0, 0.0];
            let fixture = puzzle2d_document_json_from_dwg(&drawing).unwrap();
            assert_eq!(fixture.get("schema").and_then(|value| value.as_str()), Some(PUZZLE2D_FIXTURE_SCHEMA));
            assert!(fixture_nodes(&fixture).is_empty());
            assert_eq!(fixture_camera(&fixture), (50.0, 100.0, 1.0));
        }

        /// 🧪 Definitional convergence proof: two instances on one backbone make DISJOINT node edits
        /// (each adds its own node) and, after exchanging ops, both converge to contain BOTH nodes —
        /// impossible under whole-document `setDocument` snapshots, which would clobber one side.
        #[test]
        fn two_instances_converge_disjoint_node_edits_via_backbone() {
            let mut instance_a = new_app();
            let mut instance_b = new_app();
            let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle2d-convergence", "mem://puzzle2d-convergence");
            instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
            instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

            instance_a.handle_action("addNode", Some(&json!({ "kind": "seed" })), &ViewState::default(), &meta("actor-a")).expect("a adds node");
            instance_b.handle_action("addNode", Some(&json!({ "kind": "other" })), &ViewState::default(), &meta("actor-b")).expect("b adds node");

            // A neutral history action always calls store.dispatch(), which pumps inbound ops first.
            instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
            instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

            assert_eq!(fixture_nodes(&instance_a.projection().expect("projection")).len(), 2, "instance A must contain both nodes");
            assert_eq!(fixture_nodes(&instance_b.projection().expect("projection")).len(), 2, "instance B must contain both nodes");
        }

        #[test]
        fn ingest_operations_is_idempotent() {
            let mut sender = new_app();
            let (near, mut far) = MemoryBackbone::pair("mem://puzzle2d-doc", "mem://puzzle2d-doc");
            sender.attach_backbone(Box::new(near)).expect("attach");
            sender.handle_action("addNode", Some(&json!({ "kind": "seed" })), &ViewState::default(), &meta("local")).expect("add");

            let mut envelopes = Vec::new();
            for message in far.receive().expect("receive") {
                if let BackboneMessage::Ops { envelopes: ops } = message {
                    envelopes.extend(ops);
                }
            }
            assert!(!envelopes.is_empty(), "the applied op must flow onto the channel");
            let operations_json = serde_json::to_string(&envelopes).expect("serialize");

            let mut receiver = new_app();
            receiver.ingest_operations(&operations_json).expect("ingest once");
            receiver.ingest_operations(&operations_json).expect("ingest twice");
            assert_eq!(fixture_nodes(&receiver.projection().expect("projection")).len(), 1, "feeding the same op twice must not double-apply");
        }

        /// 🧰 The framework-injected `setActiveTool` View action is host-owned: switching tools must emit no
        /// document ops and add no undo history — the active tool lives in `ViewState.active_tool_id`.
        #[test]
        fn tool_switch_emits_no_ops_and_no_history() {
            let mut app = registry_app();
            let result = app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": PUZZLE2D_TOOL_BRUSH })), &brush_view_state(), &meta("local")).expect("switch tool");
            assert!(result.operations.is_empty(), "a tool switch must not produce document ops");
            let can_undo = app.handle_action("undo", None, &ViewState::default(), &meta("local"));
            assert!(can_undo.map(|r| r.operations.is_empty()).unwrap_or(true), "a tool switch must not have created an undo step");
        }

        /// 🧰 The app declares exactly the select/brush/fill canvas tools and binds them to the interactive
        /// overview pane; the retired `fn tools()` trait method is gone.
        #[test]
        fn tool_registry_declares_tools() {
            let definition = create_puzzle2d_app().definition;
            let ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
            assert_eq!(ids, vec![PUZZLE2D_TOOL_SELECT, PUZZLE2D_TOOL_BRUSH, PUZZLE2D_TOOL_FILL]);
            let overview = definition.window_kinds.iter().find(|window| window.id == PUZZLE2D_PANE_OVERVIEW).expect("overview pane");
            let overview_tools: Vec<&str> = overview.tools.iter().map(|tool| tool.as_str()).collect();
            assert_eq!(overview_tools, vec![PUZZLE2D_TOOL_SELECT, PUZZLE2D_TOOL_BRUSH, PUZZLE2D_TOOL_FILL]);
            assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
            // 🧰 D-1: select/brush/fill are this window's whole exclusive tool set, NOT a sub-collection, so
            // each carries `group: None` and renders as a flat toolbar icon (never one collapsed dropdown).
            for tool in &definition.tools {
                assert_eq!(tool.group, None, "tool {} must render flat (no shared group)", tool.id);
            }
        }

        #[test]
        fn fill_and_brush_params_are_tagged_tool_options_not_engagement_controls() {
            let labels = puzzle2d_labels(&ViewState::default());
            let host = puzzle_board_host();
            let group_tag = |measures: &[WindowMeasure], id: &str| {
                measures.iter().find_map(|measure| match measure {
                    WindowMeasure::Group { id: gid, active_tool_id, .. } if gid == id => Some(active_tool_id.clone()),
                    _ => None,
                })
            };
            // 🪣 Fill-count slider now lives in a general measure tagged for the fill tool (surfaced by
            // partition_window_measures only while "fill" is active), NOT the engagement command HUD.
            let mut fill_runtime = Puzzle2dPlayRuntime::default();
            fill_runtime.fill_count = 3;
            let fill_scene = Puzzle2dScene { fixture: default_empty_fixture(), runtime: fill_runtime, active_tool: PUZZLE2D_TOOL_FILL.into() };
            let fill_measures = puzzle2d_window_measures(PUZZLE2D_PANE_OVERVIEW, &fill_scene, labels);
            assert_eq!(group_tag(&fill_measures, "puzzle2d-tool-options-fill"), Some(Some(PUZZLE2D_TOOL_FILL.into())));
            assert!(puzzle2d_engagement(&fill_scene, &host, PUZZLE2D_PANE_OVERVIEW, labels).control.is_none(), "fill engagement HUD must no longer carry the relocated control");
            // 🖌️ Brush candidate picker becomes a fill-tool-sibling tagged group, present only once the host
            // has candidates to place (empty ⇒ absent, matching the old gated-control behaviour).
            let empty_brush = Puzzle2dScene { fixture: default_empty_fixture(), runtime: Puzzle2dPlayRuntime::default(), active_tool: PUZZLE2D_TOOL_BRUSH.into() };
            assert_eq!(group_tag(&puzzle2d_window_measures(PUZZLE2D_PANE_OVERVIEW, &empty_brush, labels), "puzzle2d-tool-options-brush"), None);
            let mut brush_runtime = Puzzle2dPlayRuntime::default();
            brush_runtime.brush_candidates = vec![json!({ "nodeKind": "node" })];
            let brush_scene = Puzzle2dScene { fixture: default_empty_fixture(), runtime: brush_runtime, active_tool: PUZZLE2D_TOOL_BRUSH.into() };
            let brush_measures = puzzle2d_window_measures(PUZZLE2D_PANE_OVERVIEW, &brush_scene, labels);
            assert_eq!(group_tag(&brush_measures, "puzzle2d-tool-options-brush"), Some(Some(PUZZLE2D_TOOL_BRUSH.into())));
            assert!(puzzle2d_engagement(&brush_scene, &host, PUZZLE2D_PANE_OVERVIEW, labels).control.is_none(), "brush engagement HUD must no longer carry the relocated control");
        }

        /// 🧭 Kind discipline: every View-declared runtime/host action must run through the registry
        /// without tripping the "must not emit operations" guard (proving each is correctly classified).
        #[test]
        fn view_actions_emit_no_ops_through_the_registry() {
            let mut app = registry_app();
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), &ViewState::default(), &meta("local")).expect("load example");
            let node_id = fixture_nodes(&app.projection().expect("projection"))[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
            let view_dispatches: Vec<(&str, Value)> = vec![
                ("setSelection", json!({ "ids": [node_id.clone()] })),
                ("selectAll", Value::Null),
                ("selectSameKind", Value::Null),
                ("clearSelection", Value::Null),
                ("setSelectionMethod", json!({ "method": "lasso" })),
                ("setGridSnapEnabled", json!({ "enabled": true })),
                ("setGridFactor", json!({ "value": 2.0 })),
                ("setLodModeForPane", json!({ "pane": PUZZLE2D_PANE_OVERVIEW, "value": "detail" })),
                ("setBrushKindWeights", json!({ "kindId": "node", "value": 0.5 })),
                ("setBrushNodeSize", json!({ "size": 12.0 })),
                ("setSuggestionOffset", json!({ "value": 40.0 })),
                ("engagementInput", json!({ "pane": PUZZLE2D_PANE_OVERVIEW, "value": "brush" })),
                ("engagementSubmit", json!({ "pane": PUZZLE2D_PANE_OVERVIEW, "value": "brush" })),
                ("engagementAbort", json!({ "pane": PUZZLE2D_PANE_OVERVIEW })),
                ("brushCycleCandidate", json!({ "forward": true })),
                ("brushSetCandidateIndex", json!({ "index": 0 })),
                ("brushFillSessionBegin", json!({ "maxCount": 4, "seed": 1 })),
                ("brushFillSessionClear", Value::Null),
                ("lodScaleJson", Value::Null),
            ];
            for (action, args) in view_dispatches {
                let args_ref = (!args.is_null()).then_some(&args);
                let result = app.handle_action(action, args_ref, &brush_view_state(), &meta("local")).unwrap_or_else(|error| panic!("view action '{action}' must not error: {error}"));
                assert!(result.operations.is_empty(), "view action '{action}' must not emit document ops");
            }
        }
    }
    //#endregion 🧪Tests
}
pub mod d3 {
    //! 🧊 Puzzle 3D plugin — 3D puzzle assembly play app bundled as a hot-swappable WASM component.

    use puzzle_3d::{puzzle3d_document_delta_ops, BrushPlacePayload, Puzzle3dOp, Puzzle3dPrecomputeSession};
    use semio_framework_plugin::{
        apply_world3d_sun_action, build_world_3d_scene, create_default_layout, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, DocumentApp, DocumentView, MeasureSelectItem, merge_world_selection_ids, mesh_from_kind, strip_engagement_prefix, ui_inspector_groups_to_tree, ui_inspector_readonly_field,
        ui_stack_vertical, ui_text, world3d_chunking_json, world3d_environment_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_meshes_json_from_urls, world3d_scene_extended, world3d_selection_json, world3d_sun_measures, App, ActionDescriptor, PanelGroup,
        SurfaceKind, ToolCategory, ToolDefinition, UiControlNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementInput, WindowMeasure, WorldSunConfig, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, SET_ACTIVE_TOOL_ACTION_ID,
    };
    use semio_framework_plugin::kernel::HostEffect;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{HashMap, HashSet};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::LazyLock;

    //#region 🔖Constants
    const PUZZLE3D_PLAY_APP_ID: &str = "puzzle3d-play";
    const PUZZLE3D_PLAY_CONTROLLER_ID: &str = "puzzle3d-play";
    const PUZZLE3D_PLAY_SURFACE_VIEWPORT: &str = "puzzle.3d.play.viewport";
    const PUZZLE3D_PLAY_BODY_COMPOSITE: &str = "puzzle3d.play.composite";
    const PUZZLE3D_PLAY_BODY_DOCUMENT: &str = "puzzle.3d.play.document";
    const PUZZLE3D_PLAY_BODY_KINDS: &str = "puzzle.3d.play.kinds";
    const PUZZLE3D_PLAY_BODY_INSPECTOR: &str = "puzzle.3d.play.inspector";
    const PUZZLE3D_PLAY_BODY_SETTINGS: &str = "puzzle.3d.play.settings";
    const PUZZLE3D_PLAY_BODY_JACK: &str = "puzzle.3d.play.jack";
    const PUZZLE3D_PLAY_WINDOW_MAIN: &str = "puzzle3d-main";
    const PUZZLE3D_FIXTURE_SCHEMA: &str = "puzzle.3d.fixture";
    const PUZZLE3D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
    const PUZZLE3D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";
    const PUZZLE3D_FALLBACK_MESH_KIND: &str = "box";
    /// 🧰 Host-owned active tool (`view_state.active_tool_id`) when the host hasn't set one yet — the first declared window tool.
    const PUZZLE3D_DEFAULT_TOOL: &str = "select";
    const PUZZLE3D_FILL_COUNT_MAX: u32 = 1000;

    const CONCRETE_FOREST_EXAMPLE_JSON: &str = include_str!("../../3d/example/concrete-forest.3d.json");
    const NAKAGIN_EXAMPLE_JSON: &str = include_str!("../../3d/example/nakagin-capsule-tower.3d.json");
    //#endregion 🔖Constants

    //#region 🔖Document
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dCamera {
        #[serde(default)]
        position: [f64; 3],
        #[serde(default)]
        target: [f64; 3],
        #[serde(default = "one_f64")]
        zoom: f64,
        #[serde(default)]
        up: Option<[f64; 3]>,
        #[serde(default)]
        projection: Option<String>,
    }

    fn one_f64() -> f64 {
        1.0
    }

    fn default_selection_method() -> String {
        "rectangle".into()
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dVortex {
        id: String,
        #[serde(default, rename = "vortexKind")]
        vortex_kind: Option<String>,
        #[serde(default)]
        position: [f64; 3],
        #[serde(default)]
        direction: Option<[f64; 3]>,
        #[serde(default)]
        radius: Option<f64>,
        #[serde(default)]
        hidden: bool,
        #[serde(default)]
        locked: bool,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dReferenceSource {
        #[serde(default)]
        url: String,
        #[serde(default, rename = "mediaKind")]
        media_kind: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dReference {
        id: String,
        #[serde(default)]
        source: Puzzle3dReferenceSource,
        #[serde(default)]
        origin: [f64; 3],
        #[serde(default, rename = "widthWorld")]
        width_world: f64,
        #[serde(default)]
        locked: bool,
        #[serde(default)]
        hidden: bool,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dObject {
        id: String,
        #[serde(default)]
        label: Option<String>,
        #[serde(default, rename = "objectKind")]
        object_kind: Option<String>,
        #[serde(default)]
        origin: [f64; 3],
        #[serde(default)]
        orientation: Option<[f64; 4]>,
        #[serde(default)]
        scale: Option<Value>,
        #[serde(default, rename = "meshUrl")]
        mesh_url: Option<String>,
        #[serde(default)]
        vortices: Vec<Puzzle3dVortex>,
        #[serde(default)]
        hidden: bool,
        #[serde(default)]
        locked: bool,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dFixtureMeta {
        #[serde(default, rename = "kindCatalogs")]
        kind_catalogs: Option<Value>,
        #[serde(default, rename = "kindCompatibility")]
        kind_compatibility: Option<Value>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dTargetVolume {
        id: String,
        #[serde(default)]
        origin: [f64; 3],
        #[serde(default)]
        orientation: Option<[f64; 4]>,
        #[serde(default)]
        scale: Option<Value>,
        #[serde(default)]
        hidden: bool,
        #[serde(default)]
        locked: bool,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dAttraction {
        #[serde(default)]
        id: String,
        attracting: String,
        attracted: String,
        #[serde(default)]
        gap: f64,
        #[serde(default)]
        shift: f64,
        #[serde(default)]
        rise: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default)]
        turn: f64,
        #[serde(default)]
        tilt: f64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dFixture {
        schema: String,
        #[serde(default)]
        domain: String,
        #[serde(default)]
        camera: Puzzle3dCamera,
        #[serde(default)]
        meta: Puzzle3dFixtureMeta,
        #[serde(default)]
        objects: Vec<Puzzle3dObject>,
        #[serde(default)]
        attractions: Vec<Puzzle3dAttraction>,
        #[serde(default, rename = "targetVolumes")]
        target_volumes: Vec<Puzzle3dTargetVolume>,
        #[serde(default)]
        references: Vec<Puzzle3dReference>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dSelection {
        #[serde(default)]
        object_ids: Vec<String>,
        #[serde(default)]
        vortex_ids: Vec<String>,
        #[serde(default)]
        attraction_ids: Vec<String>,
        #[serde(default)]
        target_volume_ids: Vec<String>,
        #[serde(default)]
        reference_ids: Vec<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dSelectableKinds {
        #[serde(default = "default_true")]
        objects: bool,
        #[serde(default = "default_true")]
        vortices: bool,
        #[serde(default = "default_true")]
        attractions: bool,
    }

    impl Default for Puzzle3dSelectableKinds {
        fn default() -> Self {
            Self { objects: true, vortices: true, attractions: true }
        }
    }

    fn default_true() -> bool {
        true
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle3dRuntime {
        #[serde(default)]
        selection: Puzzle3dSelection,
        #[serde(default = "default_selection_method")]
        selection_method: String,
        #[serde(default)]
        hovered_object_id: Option<String>,
        #[serde(default)]
        hovered_vortex_full_id: Option<String>,
        /// 🎯 Anchor position (client x/y) of the open per-vortex brush-candidate suggestion popup, or `None` when closed.
        #[serde(default)]
        suggestion_menu: Option<[f64; 2]>,
        #[serde(default = "default_overlap_budget")]
        overlap_budget: f64,
        #[serde(default)]
        fill_count: u32,
        #[serde(default)]
        brush_candidate_index: usize,
        #[serde(default)]
        object_kind_weights: HashMap<String, f64>,
        #[serde(default)]
        vortex_kind_weights: HashMap<String, f64>,
        #[serde(default = "default_true")]
        lod_automatic: bool,
        #[serde(default)]
        lod_depth_variable: bool,
        #[serde(default = "default_true")]
        lod_show_grid: bool,
        #[serde(default = "default_manual_lod")]
        lod_manual: f64,
        #[serde(default)]
        grid_snap_enabled: bool,
        #[serde(default = "default_grid_factor")]
        grid_factor: f64,
        #[serde(default)]
        selectable_kinds: Puzzle3dSelectableKinds,
        #[serde(default)]
        hovered_kind_id: Option<String>,
        #[serde(default)]
        engagement_input: String,
        #[serde(default = "default_selection_mode")]
        selection_mode_default: String,
        #[serde(default = "default_proximity_radius")]
        proximity_radius: f64,
        #[serde(default = "default_chunk_size")]
        chunk_size: f64,
        #[serde(default)]
        fill_edit_target_volumes: bool,
        #[serde(default = "default_voxel_dims")]
        voxel_dims: [u32; 3],
        #[serde(default = "default_jack_query")]
        jack_query: String,
        #[serde(default = "default_view_preset")]
        view_preset: String,
        #[serde(default)]
        sun: WorldSunConfig,
    }

    impl Default for Puzzle3dRuntime {
        /// 🎛️ Mirrors every `#[serde(default = "...")]` above — `#[derive(Default)]` would silently ignore
        /// them and zero out fields like `overlap_budget`/`selection_method`/`lod_automatic` in Rust-constructed runtimes.
        fn default() -> Self {
            Self {
                selection: Puzzle3dSelection::default(),
                selection_method: default_selection_method(),
                hovered_object_id: None,
                hovered_vortex_full_id: None,
                suggestion_menu: None,
                overlap_budget: default_overlap_budget(),
                fill_count: 0,
                brush_candidate_index: 0,
                object_kind_weights: HashMap::new(),
                vortex_kind_weights: HashMap::new(),
                lod_automatic: default_true(),
                lod_depth_variable: false,
                lod_show_grid: default_true(),
                lod_manual: default_manual_lod(),
                grid_snap_enabled: false,
                grid_factor: default_grid_factor(),
                selectable_kinds: Puzzle3dSelectableKinds::default(),
                hovered_kind_id: None,
                engagement_input: String::new(),
                selection_mode_default: default_selection_mode(),
                proximity_radius: default_proximity_radius(),
                chunk_size: default_chunk_size(),
                fill_edit_target_volumes: false,
                voxel_dims: default_voxel_dims(),
                jack_query: default_jack_query(),
                view_preset: default_view_preset(),
                sun: WorldSunConfig::default(),
            }
        }
    }

    fn default_view_preset() -> String {
        "perspective".into()
    }

    fn default_overlap_budget() -> f64 {
        0.02
    }

    fn default_manual_lod() -> f64 {
        100.0
    }

    fn default_grid_factor() -> f64 {
        10.0
    }

    fn default_selection_mode() -> String {
        "default".into()
    }

    fn default_proximity_radius() -> f64 {
        0.75
    }

    fn default_chunk_size() -> f64 {
        256.0
    }

    fn default_voxel_dims() -> [u32; 3] {
        [1, 1, 1]
    }

    fn default_jack_query() -> String {
        "MATCH (n:Object) RETURN n.name".into()
    }

    /// 🧾 Transient render/mutation bundle pairing the persisted projection (the bare `Puzzle3dFixture`
    /// json) with the app's ephemeral view state. Never persisted — the {@link VcsDocumentApp} store owns
    /// the fixture and {@link Puzzle3dPlayApp} owns the runtime — but rebuilt per call so the existing
    /// panel/world/engagement helpers keep their `&scene` signatures.
    #[derive(Clone)]
    struct Puzzle3dScene {
        fixture: Puzzle3dFixture,
        runtime: Puzzle3dRuntime,
        /// 🧰 Host-owned active tool mirrored from `view_state.active_tool_id` — transient, never persisted.
        active_tool: String,
    }

    /// 🧭 The select/brush/fill interaction mode the world engine reads, derived from the flat active tool
    /// (the transform gumball tools `move`/`rotate`/`scale` and `worldRelocate` all present as `select`).
    fn puzzle3d_scene_mode(active_tool: &str) -> &str {
        match active_tool {
            "brush" => "brush",
            "fill" => "fill",
            _ => "select",
        }
    }

    /// 🎚️ The gumball handle the world engine draws, derived from the flat active tool (non-transform tools fall back to `move`).
    fn puzzle3d_transform_handle(active_tool: &str) -> &str {
        match active_tool {
            "rotate" => "rotate",
            "scale" => "scale",
            _ => "move",
        }
    }

    static PUZZLE3D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

    fn empty_fixture() -> Puzzle3dFixture {
        Puzzle3dFixture {
            schema: PUZZLE3D_FIXTURE_SCHEMA.into(),
            domain: "architecture".into(),
            camera: Puzzle3dCamera::default(),
            meta: Puzzle3dFixtureMeta::default(),
            objects: Vec::new(),
            attractions: Vec::new(),
            target_volumes: Vec::new(),
            references: Vec::new(),
        }
    }

    fn default_fixture() -> Puzzle3dFixture {
        serde_json::from_str::<Puzzle3dFixture>(CONCRETE_FOREST_EXAMPLE_JSON).unwrap_or_else(|_| empty_fixture())
    }

    fn nakagin_fixture() -> Puzzle3dFixture {
        serde_json::from_str::<Puzzle3dFixture>(NAKAGIN_EXAMPLE_JSON).unwrap_or_else(|_| empty_fixture())
    }

    /// 🧾 Materializes the transient scene from the persisted projection (bare fixture json) and the
    /// app's current view state; an unparseable projection degrades to an empty board.
    fn scene_from_projection(projection: &Value, runtime: Puzzle3dRuntime, active_tool: &str) -> Puzzle3dScene {
        let fixture = serde_json::from_value::<Puzzle3dFixture>(projection.clone()).unwrap_or_else(|_| empty_fixture());
        Puzzle3dScene { fixture, runtime, active_tool: active_tool.to_string() }
    }

    fn puzzle3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor { controller_id: PUZZLE3D_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
    }

    fn camera_json(camera: &Puzzle3dCamera) -> String {
        let mut value = json!({
            "position": camera.position,
            "target": camera.target,
            "zoom": camera.zoom,
            "fov": 45.0,
        });
        if let Some(object) = value.as_object_mut() {
            if let Some(up) = camera.up {
                object.insert("up".into(), json!(up));
            }
            if let Some(projection) = &camera.projection {
                object.insert("projection".into(), json!(projection));
            }
        }
        value.to_string()
    }

    fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
        args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).filter(|ids: &Vec<String>| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
    }

    /// 🎥 Named orbit-camera rigs — top/front/right use an orthographic projection with a non-Z `up` to avoid gimbal lock when looking straight down/along the Z-up axis.
    fn puzzle3d_camera_view_preset(preset: &str) -> Puzzle3dCamera {
        match preset {
            "top" => Puzzle3dCamera { position: [0.0, 0.0, 30.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: Some([0.0, 1.0, 0.0]), projection: Some("orthographic".into()) },
            "front" => Puzzle3dCamera { position: [0.0, -30.0, 0.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: Some([0.0, 0.0, 1.0]), projection: Some("orthographic".into()) },
            "right" => Puzzle3dCamera { position: [30.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: Some([0.0, 0.0, 1.0]), projection: Some("orthographic".into()) },
            "perspective" => Puzzle3dCamera { position: [12.0, -12.0, 9.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: Some([0.0, 0.0, 1.0]), projection: Some("perspective".into()) },
            _ => Puzzle3dCamera { position: [12.0, -12.0, 9.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: None, projection: None },
        }
    }

    fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
        [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
    }

    fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
        let len = (ax * ax + ay * ay + az * az).sqrt();
        if len < 1e-8 {
            return [0.0, 0.0, 0.0, 1.0];
        }
        let half = angle * 0.5;
        let s = half.sin();
        [ax / len * s, ay / len * s, az / len * s, half.cos()]
    }

    fn scale_value_mul(scale: &Option<Value>, sx: f64, sy: f64, sz: f64) -> Value {
        match scale {
            Some(Value::Array(values)) if values.len() >= 3 => json!([values[0].as_f64().unwrap_or(1.0) * sx, values[1].as_f64().unwrap_or(1.0) * sy, values[2].as_f64().unwrap_or(1.0) * sz,]),
            Some(Value::Number(value)) => {
                let factor = value.as_f64().unwrap_or(1.0);
                json!([factor * sx, factor * sy, factor * sz])
            }
            _ => json!([sx, sy, sz]),
        }
    }

    fn resolve_object_mesh_url(object: &Puzzle3dObject, meta: &Puzzle3dFixtureMeta) -> Option<String> {
        if let Some(url) = object.mesh_url.as_ref().filter(|url| !url.is_empty()) {
            return Some(url.clone());
        }
        let kind_id = object.object_kind.as_deref()?;
        let catalogs = meta.kind_catalogs.as_ref()?;
        let objects = catalogs.get("objects")?.as_array()?;
        for entry in objects {
            if entry.get("id").and_then(|v| v.as_str()) == Some(kind_id) {
                return entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string);
            }
        }
        None
    }

    fn collect_mesh_urls(fixture: &Puzzle3dFixture) -> Vec<String> {
        let mut urls = HashSet::new();
        for object in &fixture.objects {
            if let Some(url) = resolve_object_mesh_url(object, &fixture.meta) {
                urls.insert(url);
            }
        }
        if let Some(catalogs) = fixture.meta.kind_catalogs.as_ref() {
            if let Some(objects) = catalogs.get("objects").and_then(|v| v.as_array()) {
                for entry in objects {
                    if let Some(url) = entry.get("meshUrl").and_then(|v| v.as_str()) {
                        urls.insert(url.to_string());
                    }
                }
            }
        }
        urls.into_iter().collect()
    }

    fn object_scale_json(object: &Puzzle3dObject) -> [f64; 3] {
        match &object.scale {
            Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
            _ => [1.0, 1.0, 1.0],
        }
    }

    /// 🙈 Hidden objects stay in the emitted array — `worldPick`'s `id` arg is the array index into it — but render at zero scale so they're effectively invisible without shifting any other object's index.
    fn world_instances_json(fixture: &Puzzle3dFixture, runtime: &Puzzle3dRuntime) -> String {
        let selection = &runtime.selection;
        let instances: Vec<Value> = fixture
            .objects
            .iter()
            .map(|object| {
                let selected = selection.object_ids.contains(&object.id);
                let hovered = runtime.hovered_object_id.as_deref() == Some(object.id.as_str());
                let kind_highlighted = runtime.hovered_kind_id.is_some() && runtime.hovered_kind_id.as_deref() == object.object_kind.as_deref();
                let mesh_id = resolve_object_mesh_url(object, &fixture.meta).map(|url| world3d_mesh_id_from_url(&url)).unwrap_or_else(|| PUZZLE3D_FALLBACK_MESH_KIND.into());
                let scale = if object.hidden { json!([0.0, 0.0, 0.0]) } else { json!(object_scale_json(object)) };
                json!({
                    "id": object.id,
                    "meshId": mesh_id,
                    "position": [
                        object.origin.first().copied().unwrap_or(0.0),
                        object.origin.get(1).copied().unwrap_or(0.0),
                        object.origin.get(2).copied().unwrap_or(0.0),
                    ],
                    "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                    "scale": scale,
                    "label": object.label.clone().or_else(|| object.object_kind.clone()).unwrap_or_else(|| object.id.clone()),
                    "selected": selected,
                    "hovered": hovered,
                    "highlighted": kind_highlighted,
                })
            })
            .collect();
        serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
    }

    fn world_meshes_json(fixture: &Puzzle3dFixture) -> String {
        let urls = collect_mesh_urls(fixture);
        let kinds = vec![PUZZLE3D_FALLBACK_MESH_KIND.into(), "vortex-marker".into()];
        if urls.is_empty() {
            return world3d_meshes_json_from_kinds_and_urls(&kinds, &[]);
        }
        let mut meshes_json = world3d_meshes_json_from_kinds_and_urls(&kinds, &urls);
        if !meshes_json.contains(PUZZLE3D_FALLBACK_MESH_KIND) {
            let fallback = world3d_meshes_json_from_kinds_and_urls(&[PUZZLE3D_FALLBACK_MESH_KIND.into()], &[]);
            let mut merged: Vec<Value> = serde_json::from_str(&meshes_json).unwrap_or_default();
            let fallback_meshes: Vec<Value> = serde_json::from_str(&fallback).unwrap_or_default();
            merged.extend(fallback_meshes);
            meshes_json = serde_json::to_string(&merged).unwrap_or(meshes_json);
        }
        meshes_json
    }

    fn quat_rotate_vector(quat: [f64; 4], vector: [f64; 3]) -> [f64; 3] {
        let [x, y, z, w] = quat;
        let vx = vector[0];
        let vy = vector[1];
        let vz = vector[2];
        let ix = w * vx + y * vz - z * vy;
        let iy = w * vy + z * vx - x * vz;
        let iz = w * vz + x * vy - y * vx;
        let iw = -x * vx - y * vy - z * vz;
        [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x]
    }

    fn world_vortex_position(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
        let orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let rotated = quat_rotate_vector(orientation, vortex.position);
        [object.origin.first().copied().unwrap_or(0.0) + rotated[0], object.origin.get(1).copied().unwrap_or(0.0) + rotated[1], object.origin.get(2).copied().unwrap_or(0.0) + rotated[2]]
    }

    fn world_vortex_direction(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
        let direction = vortex.direction.unwrap_or([0.0, 0.0, -1.0]);
        quat_rotate_vector(object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), direction)
    }

    fn vortex_color(meta: &Puzzle3dFixtureMeta, vortex_kind: Option<&str>) -> String {
        let Some(kind_id) = vortex_kind else {
            return "#38bdf8".into();
        };
        let Some(catalogs) = meta.kind_catalogs.as_ref() else {
            return "#38bdf8".into();
        };
        let Some(entries) = catalogs.get("vortices").and_then(|value| value.as_array()) else {
            return "#38bdf8".into();
        };
        for entry in entries {
            if entry.get("id").and_then(|value| value.as_str()) == Some(kind_id) {
                return entry.get("color").and_then(|value| value.as_str()).unwrap_or("#38bdf8").to_string();
            }
        }
        "#38bdf8".into()
    }

    fn puzzle3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
        if vortex_id.contains(':') {
            vortex_id.to_string()
        } else {
            format!("{object_id}:{vortex_id}")
        }
    }

    fn resolve_vortex_world_position(fixture: &Puzzle3dFixture, full_id: &str) -> Option<[f64; 3]> {
        for object in &fixture.objects {
            for vortex in &object.vortices {
                if puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id {
                    return Some(world_vortex_position(object, vortex));
                }
            }
        }
        None
    }

    fn resolve_vortex_kind(fixture: &Puzzle3dFixture, full_id: &str) -> Option<String> {
        fixture.objects.iter().find_map(|object| object.vortices.iter().find(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id).and_then(|vortex| vortex.vortex_kind.clone()))
    }

    /// 🧲 Permissive when the fixture declares no `kindCompatibility` rules at all — otherwise requires an explicit (or bidirectional) entry.
    fn puzzle3d_kinds_compatible(fixture: &Puzzle3dFixture, source_kind: &str, target_kind: &str) -> bool {
        let Some(entries) = fixture.meta.kind_compatibility.as_ref().and_then(|value| value.as_array()) else {
            return true;
        };
        if entries.is_empty() {
            return true;
        }
        entries.iter().any(|entry| {
            let source = entry.get("source").and_then(|value| value.as_str()).unwrap_or("");
            let target = entry.get("target").and_then(|value| value.as_str()).unwrap_or("");
            let bidirectional = entry.get("bidirectional").and_then(|value| value.as_bool()).unwrap_or(false);
            (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind)
        })
    }

    fn world_vortices_json(fixture: &Puzzle3dFixture, runtime: &Puzzle3dRuntime) -> String {
        let mut records = Vec::new();
        for object in &fixture.objects {
            for vortex in &object.vortices {
                let position = world_vortex_position(object, vortex);
                let direction = world_vortex_direction(object, vortex);
                let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                let selected = runtime.selection.vortex_ids.contains(&full_id);
                let hovered = runtime.hovered_vortex_full_id.as_deref() == Some(full_id.as_str());
                records.push(json!({
                    "fullId": full_id,
                    "objectId": object.id,
                    "vortexKind": vortex.vortex_kind,
                    "position": position,
                    "direction": direction,
                    "radius": vortex.radius.unwrap_or(0.36),
                    "color": vortex_color(&fixture.meta, vortex.vortex_kind.as_deref()),
                    "selected": selected,
                    "hovered": hovered,
                }));
            }
        }
        serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
    }

    fn world_attractions_json(fixture: &Puzzle3dFixture) -> String {
        let records: Vec<Value> = fixture
            .attractions
            .iter()
            .filter_map(|attraction| {
                let from = resolve_vortex_world_position(fixture, &attraction.attracting)?;
                let to = resolve_vortex_world_position(fixture, &attraction.attracted)?;
                Some(json!({
                    "id": attraction.id,
                    "from": from,
                    "to": to,
                    "color": "#60a5fa",
                }))
            })
            .collect();
        serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
    }

    fn target_volume_scale_json(volume: &Puzzle3dTargetVolume) -> [f64; 3] {
        match &volume.scale {
            Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
            _ => [1.0, 1.0, 1.0],
        }
    }

    fn world_target_volumes_json(fixture: &Puzzle3dFixture) -> String {
        let records: Vec<Value> = fixture
            .target_volumes
            .iter()
            .map(|volume| {
                json!({
                    "id": volume.id,
                    "origin": volume.origin,
                    "orientation": volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                    "scale": target_volume_scale_json(volume),
                    "color": "#f472b6",
                })
            })
            .collect();
        serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
    }

    fn world_references_json(fixture: &Puzzle3dFixture) -> String {
        let records: Vec<Value> = fixture
            .references
            .iter()
            .map(|reference| {
                json!({
                    "id": reference.id,
                    "url": reference.source.url,
                    "origin": reference.origin,
                    "widthWorld": if reference.width_world > 0.0 { reference.width_world } else { 1.0 },
                    "locked": reference.locked,
                    "hidden": reference.hidden,
                })
            })
            .collect();
        serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
    }

    fn world_interaction_json(envelope: &Puzzle3dScene, session: &Puzzle3dPrecomputeSession) -> String {
        let runtime = &envelope.runtime;
        let suggestion_menu = runtime.suggestion_menu.map(|[x, y]| {
            let (pending, candidates) = puzzle3d_brush_target_vortex(envelope)
                .map(|target| {
                    let raw = session.brush_candidates(&target);
                    let free = parse_brush_candidates_free(&raw);
                    let pending: Value = serde_json::from_str(&raw).unwrap_or(Value::Null);
                    let pending = pending.get("unknownPending").and_then(Value::as_bool).unwrap_or(false);
                    let candidates: Vec<Value> = free
                        .iter()
                        .enumerate()
                        .map(|(index, candidate)| {
                            let object_label = candidate.get("objectKind").and_then(Value::as_str).or_else(|| candidate.get("objectKindId").and_then(Value::as_str)).unwrap_or("kind");
                            let source_vortex_index = candidate.get("sourceVortexIndex").and_then(Value::as_u64).unwrap_or(0);
                            json!({ "index": index, "objectLabel": object_label, "vortexLabel": format!("vortex {source_vortex_index}") })
                        })
                        .collect();
                    (pending, candidates)
                })
                .unwrap_or((false, Vec::new()));
            json!({ "open": true, "x": x, "y": y, "pending": pending, "candidates": candidates })
        });
        let fill_build: Value = serde_json::from_str(&session.fill_progress()).unwrap_or(Value::Null);
        let fill_build = json!({
            "count": fill_build.get("count").cloned().unwrap_or(json!(0)),
            "maxCount": fill_build.get("maxCount").cloned().unwrap_or(json!(PUZZLE3D_FILL_COUNT_MAX)),
            "done": fill_build.get("done").cloned().unwrap_or(json!(true)),
        });
        json!({
            "activeTool": puzzle3d_scene_mode(&envelope.active_tool),
            "brushCandidateIndex": runtime.brush_candidate_index,
            "hoveredVortexFullId": runtime.hovered_vortex_full_id.clone(),
            "fillEditTargetVolumes": runtime.fill_edit_target_volumes,
            "voxelDims": runtime.voxel_dims,
            "gridFactor": runtime.grid_factor,
            "suggestionMenu": suggestion_menu,
            "fillBuild": fill_build,
        })
        .to_string()
    }

    fn world3d_lod_json(runtime: &Puzzle3dRuntime) -> String {
        json!({
            "gridFactor": runtime.grid_factor,
            "gridSnapEnabled": runtime.grid_snap_enabled,
            "showLodGrid": runtime.lod_show_grid,
            "automaticLod": runtime.lod_automatic,
            "depthVariableLod": runtime.lod_depth_variable,
            "manualLod": runtime.lod_manual,
        })
        .to_string()
    }

    fn world_brush_preview_json(session: &Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) -> Option<String> {
        if envelope.active_tool != "brush" {
            return None;
        }
        let vortex_id = puzzle3d_brush_target_vortex(envelope)?;
        session.brush_preview_json(&vortex_id, envelope.runtime.brush_candidate_index)
    }

    /// ⏱️ Bounded to one small chunk per call (matches puzzle5d's drive path and the premigration idle
    /// worker's chunk budget) — `handle_action` runs synchronously on the UI thread, and the host redrives
    /// this via 120ms `suggestionsTick`/`fillBuildTick` ticks, so a large per-call budget here is exactly
    /// what froze the UI for minutes: 128×32 Monte-Carlo collision task units, blocking, every single tick.
    fn drive_precompute(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
        sync_precompute_session(session, envelope);
        session.precompute_step(8);
    }

    fn scene_config_json(envelope: &Puzzle3dScene) -> String {
        json!({
            "fixture": {
                "objects": envelope.fixture.objects,
                "attractions": envelope.fixture.attractions,
                "targetVolumes": envelope.fixture.target_volumes,
            },
            "kindCatalogs": envelope.fixture.meta.kind_catalogs,
            "kindCompatibility": envelope.fixture.meta.kind_compatibility.clone().unwrap_or(json!([])),
            "overlapBudget": envelope.runtime.overlap_budget,
            "seed": 1,
            "weights": {
                "objectWeights": envelope.runtime.object_kind_weights,
                "vortexWeights": envelope.runtime.vortex_kind_weights,
            }
        })
        .to_string()
    }

    /// 🧊 Scales the unit box fallback (`mesh_from_kind` extent 1.0) past `BRUSH_COLLISION_MESH_MIN_EXTENT` (2.0) in `puzzle_3d`'s collision engine, otherwise its registration is a silent no-op and brush candidates never populate before a real GLB arrives.
    const PUZZLE3D_FALLBACK_MESH_SCALE: f32 = 4.0;

    fn scaled_mesh_positions(positions: &[f32], scale: f32) -> Vec<f32> {
        positions.iter().map(|value| value * scale).collect()
    }

    /// 🧊 Only seeds the box fallback for URLs with no mesh yet, so a real GLB registered earlier via `registerBrushMesh` survives every subsequent resync.
    fn sync_precompute_session(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
        let _ = session.set_scene(&scene_config_json(envelope));
        let fallback = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
        let fallback_positions = scaled_mesh_positions(&fallback.positions, PUZZLE3D_FALLBACK_MESH_SCALE);
        if !session.has_mesh(PUZZLE3D_FALLBACK_MESH_KIND) {
            session.register_mesh(PUZZLE3D_FALLBACK_MESH_KIND, &fallback_positions, &fallback.indices);
        }
        for url in collect_mesh_urls(&envelope.fixture) {
            if !session.has_mesh(&url) {
                session.register_mesh(&url, &fallback_positions, &fallback.indices);
            }
        }
    }

    fn world_selection_json(envelope: &Puzzle3dScene) -> String {
        let runtime = &envelope.runtime;
        let mut value: Value = serde_json::from_str(&world3d_selection_json(&runtime.selection_method, &runtime.selection.object_ids, runtime.hovered_object_id.as_deref())).unwrap_or_else(|_| json!({}));
        if let Some(object) = value.as_object_mut() {
            object.insert("granularity".into(), json!("mesh"));
            object.insert("selectionMode".into(), json!("mesh"));
            object.insert(
                "targets".into(),
                json!({
                    "mesh": true,
                    "vertex": false,
                    "edge": false,
                    "face": false,
                }),
            );
            object.insert("transformTool".into(), json!(puzzle3d_transform_handle(&envelope.active_tool)));
            if let Some(active_id) = runtime.selection.object_ids.first() {
                object.insert("activeObjectId".into(), json!(active_id));
            }
            let gumball_active = !runtime.selection.object_ids.is_empty();
            object.insert("gumballActive".into(), json!(gumball_active));
            if gumball_active {
                if let Some(target) = gumball_target_world(envelope) {
                    object.insert("gumballTarget".into(), json!(target));
                }
            }
        }
        value.to_string()
    }

    fn gumball_target_world(envelope: &Puzzle3dScene) -> Option<[f64; 3]> {
        let selected: Vec<&Puzzle3dObject> = envelope.fixture.objects.iter().filter(|object| envelope.runtime.selection.object_ids.contains(&object.id)).collect();
        if selected.is_empty() {
            return None;
        }
        let mut sum = [0.0, 0.0, 0.0];
        for object in &selected {
            sum[0] += object.origin.first().copied().unwrap_or(0.0);
            sum[1] += object.origin.get(1).copied().unwrap_or(0.0);
            sum[2] += object.origin.get(2).copied().unwrap_or(0.0);
        }
        let count = selected.len() as f64;
        Some([sum[0] / count, sum[1] / count, sum[2] / count])
    }

    fn fixture_from_engine_json(envelope: &Puzzle3dScene, fixture_json: &str) -> Option<Puzzle3dScene> {
        let parsed: Value = serde_json::from_str(fixture_json).ok()?;
        let mut next = envelope.clone();
        next.fixture.objects = serde_json::from_value(parsed.get("objects")?.clone()).ok()?;
        next.fixture.attractions = parsed.get("attractions").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
        next.fixture.target_volumes = parsed.get("targetVolumes").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
        Some(next)
    }

    fn apply_puzzle3d_fill_count(precompute: &mut Puzzle3dPrecomputeSession, mut envelope: Puzzle3dScene, count: u32) -> Puzzle3dScene {
        envelope.runtime.fill_count = count;
        if count > 0 {
            envelope.active_tool = "fill".into();
            if let Ok(fixture_json) = precompute.apply_fill_count_rust(count) {
                if let Some(next) = fixture_from_engine_json(&envelope, &fixture_json) {
                    envelope = next;
                    puzzle3d_rederive_all_attractions(&mut envelope.fixture);
                    resolve_puzzle3d_attractions(&mut envelope.fixture);
                }
            }
        }
        envelope
    }

    /// 🎯 Mirrors the host's client-side `handleZoomToSelection` framing math so a keybinding/engagement-token
    /// driven focus (which bypasses that host interception) still produces a sensible camera.
    fn apply_puzzle3d_focus_selection(envelope: &mut Puzzle3dScene) {
        let selected_origins: Vec<[f64; 3]> = envelope.fixture.objects.iter().filter(|object| envelope.runtime.selection.object_ids.contains(&object.id)).map(|object| object.origin).collect();
        if selected_origins.is_empty() {
            return;
        }
        let count = selected_origins.len() as f64;
        let mut center = [0.0, 0.0, 0.0];
        for origin in &selected_origins {
            center[0] += origin[0];
            center[1] += origin[1];
            center[2] += origin[2];
        }
        center = [center[0] / count, center[1] / count, center[2] / count];
        let max_distance = selected_origins
            .iter()
            .map(|origin| {
                let dx = origin[0] - center[0];
                let dy = origin[1] - center[1];
                let dz = origin[2] - center[2];
                (dx * dx + dy * dy + dz * dz).sqrt()
            })
            .fold(1.0_f64, f64::max);
        let distance = max_distance * 3.0 + 2.0;
        envelope.fixture.camera.position = [center[0] + distance * 0.6, center[1] - distance * 0.6, center[2] + distance * 0.5];
        envelope.fixture.camera.target = center;
    }

    fn next_object_id() -> String {
        let next = PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        format!("object-{next}")
    }

    /// 🧊 Seeds real vortices for a freshly placed object from its kind catalog's `vortices` templates, so it is immediately brushable instead of connector-less.
    fn puzzle3d_vortices_from_kind_template(catalog_entry: &Value) -> Vec<Puzzle3dVortex> {
        catalog_entry
            .get("vortices")
            .and_then(|value| value.as_array())
            .map(|templates| {
                templates
                    .iter()
                    .enumerate()
                    .map(|(index, template)| {
                        let position = template.get("position").and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]);
                        let direction = template.get("direction").and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok());
                        let radius = template.get("radius").and_then(|value| value.as_f64());
                        Puzzle3dVortex { id: format!("v{index}"), vortex_kind: template.get("vortexKind").and_then(|value| value.as_str()).map(str::to_string), position, direction, radius, hidden: false, locked: false }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 🙈 Applies `hidden`/`locked` to the given ids of one entity kind — `"vortex"` ids are full ids (`objectId:vortexId`).
    fn apply_puzzle3d_selection_flag(fixture: &mut Puzzle3dFixture, entity: &str, ids: &[String], flag: &str, value: bool) {
        if ids.is_empty() {
            return;
        }
        let ids: HashSet<&str> = ids.iter().map(String::as_str).collect();
        match entity {
            "object" => {
                for object in fixture.objects.iter_mut().filter(|object| ids.contains(object.id.as_str())) {
                    if flag == "locked" { object.locked = value } else { object.hidden = value }
                }
            }
            "vortex" => {
                for object in fixture.objects.iter_mut() {
                    for vortex in object.vortices.iter_mut() {
                        if ids.contains(puzzle3d_vortex_full_id(&object.id, &vortex.id).as_str()) {
                            if flag == "locked" { vortex.locked = value } else { vortex.hidden = value }
                        }
                    }
                }
            }
            "reference" => {
                for reference in fixture.references.iter_mut().filter(|reference| ids.contains(reference.id.as_str())) {
                    if flag == "locked" { reference.locked = value } else { reference.hidden = value }
                }
            }
            "targetVolume" => {
                for volume in fixture.target_volumes.iter_mut().filter(|volume| ids.contains(volume.id.as_str())) {
                    if flag == "locked" { volume.locked = value } else { volume.hidden = value }
                }
            }
            _ => {}
        }
    }

    fn value_as_vec3(value: &Value) -> Option<[f64; 3]> {
        let array = value.as_array()?;
        Some([array.first()?.as_f64()?, array.get(1)?.as_f64()?, array.get(2)?.as_f64()?])
    }

    /// 🔎 Generic inspector edit dispatcher — `entity`/`field` select the target, `ids` scope it (full ids for vortices, `objectId:vortexId`).
    fn apply_puzzle3d_inspector_patch(fixture: &mut Puzzle3dFixture, entity: &str, ids: &[String], field: &str, value: &Value) {
        if ids.is_empty() {
            return;
        }
        let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
        match entity {
            "object" => {
                for object in fixture.objects.iter_mut().filter(|object| id_set.contains(object.id.as_str())) {
                    match field {
                        "label" => object.label = value.as_str().map(str::to_string),
                        "origin" => {
                            if let Some(origin) = value_as_vec3(value) {
                                object.origin = origin;
                            }
                        }
                        _ => {}
                    }
                }
            }
            "vortex" => {
                for object in fixture.objects.iter_mut() {
                    for vortex in object.vortices.iter_mut() {
                        if !id_set.contains(puzzle3d_vortex_full_id(&object.id, &vortex.id).as_str()) {
                            continue;
                        }
                        match field {
                            "vortexKind" => vortex.vortex_kind = value.as_str().map(str::to_string),
                            "position" => {
                                if let Some(position) = value_as_vec3(value) {
                                    vortex.position = position;
                                }
                            }
                            "direction" => vortex.direction = value_as_vec3(value),
                            "radius" => vortex.radius = value.as_f64(),
                            _ => {}
                        }
                    }
                }
            }
            "attraction" => {
                for attraction in fixture.attractions.iter_mut().filter(|attraction| id_set.contains(attraction.id.as_str())) {
                    match field {
                        "attracting" => {
                            if let Some(text) = value.as_str() {
                                attraction.attracting = text.into();
                            }
                        }
                        "attracted" => {
                            if let Some(text) = value.as_str() {
                                attraction.attracted = text.into();
                            }
                        }
                        "gap" => {
                            if let Some(v) = value.as_f64() {
                                attraction.gap = v;
                            }
                        }
                        "shift" => {
                            if let Some(v) = value.as_f64() {
                                attraction.shift = v;
                            }
                        }
                        "rise" => {
                            if let Some(v) = value.as_f64() {
                                attraction.rise = v;
                            }
                        }
                        "rotation" => {
                            if let Some(v) = value.as_f64() {
                                attraction.rotation = v;
                            }
                        }
                        "turn" => {
                            if let Some(v) = value.as_f64() {
                                attraction.turn = v;
                            }
                        }
                        "tilt" => {
                            if let Some(v) = value.as_f64() {
                                attraction.tilt = v;
                            }
                        }
                        _ => {}
                    }
                }
            }
            "reference" => {
                for reference in fixture.references.iter_mut().filter(|reference| id_set.contains(reference.id.as_str())) {
                    match field {
                        "sourceUrl" => {
                            if let Some(text) = value.as_str() {
                                reference.source.url = text.into();
                            }
                        }
                        "origin" => {
                            if let Some(origin) = value_as_vec3(value) {
                                reference.origin = origin;
                            }
                        }
                        "widthWorld" => {
                            if let Some(width) = value.as_f64() {
                                reference.width_world = width;
                            }
                        }
                        _ => {}
                    }
                }
            }
            "targetVolume" => {
                for volume in fixture.target_volumes.iter_mut().filter(|volume| id_set.contains(volume.id.as_str())) {
                    if field == "origin" {
                        if let Some(origin) = value_as_vec3(value) {
                            volume.origin = origin;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    //#endregion 🔖Document

    //#region 🔖AttractionResolve
    /// 📐 Attraction placement math — a quaternion-only port of compose's `compute_child_plane`
    /// (compose/client/lib/rs/lib.rs:1328) so it composes directly with `Puzzle3dObject.orientation`. Every attraction
    /// is directed (`attracting` → `attracted`); an attracted object's world pose is derived from the attracting
    /// vortex's world pose plus the 6 connection-style parameters (gap/shift/rise/rotation/turn/tilt, angles in
    /// degrees, same semantics as compose connections).
    const PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE: f64 = 0.01;

    fn vec3_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }

    fn vec3_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
    }

    fn vec3_scale(a: [f64; 3], s: f64) -> [f64; 3] {
        [a[0] * s, a[1] * s, a[2] * s]
    }

    fn vec3_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
    }

    fn vec3_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    fn vec3_len(a: [f64; 3]) -> f64 {
        vec3_dot(a, a).sqrt()
    }

    fn vec3_normalize(a: [f64; 3]) -> [f64; 3] {
        let len = vec3_len(a);
        if len < 1e-12 {
            a
        } else {
            vec3_scale(a, 1.0 / len)
        }
    }

    fn deg_to_rad(deg: f64) -> f64 {
        deg * std::f64::consts::PI / 180.0
    }

    fn rad_to_deg(rad: f64) -> f64 {
        rad * 180.0 / std::f64::consts::PI
    }

    fn quat_conjugate(q: [f64; 4]) -> [f64; 4] {
        [-q[0], -q[1], -q[2], q[3]]
    }

    fn quat_normalize(q: [f64; 4]) -> [f64; 4] {
        let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
        if len < 1e-12 {
            [0.0, 0.0, 0.0, 1.0]
        } else {
            [q[0] / len, q[1] / len, q[2] / len, q[3] / len]
        }
    }

    /// 🧭 Ports compose's `quaternion_from_unit_vectors` (compose/client/lib/rs/lib.rs:1276) — the quaternion rotating
    /// unit vector `from` onto unit vector `to`.
    fn puzzle3d_quaternion_from_unit_vectors(from: [f64; 3], to: [f64; 3]) -> [f64; 4] {
        let r = vec3_dot(from, to) + 1.0;
        let quat = if r < 0.000_001 {
            if from[0].abs() > from[2].abs() {
                [-from[1], from[0], 0.0, 0.0]
            } else {
                [0.0, -from[2], from[1], 0.0]
            }
        } else {
            let c = vec3_cross(from, to);
            [c[0], c[1], c[2], r]
        };
        quat_normalize(quat)
    }

    /// 🧲 Ports compose's align-quaternion special-case branch (compose/client/lib/rs/lib.rs:1345-1356) for when the
    /// attracted vortex is already (anti)parallel to the attracting vortex. Falls back to an alternate cross axis when
    /// the attracting direction is exactly ±Z — a double-degenerate corner compose's own branch doesn't otherwise guard.
    fn puzzle3d_attraction_align_quat(parent_dir: [f64; 3], child_dir: [f64; 3]) -> [f64; 4] {
        let reverse_child = vec3_scale(child_dir, -1.0);
        let cross_vec = vec3_cross(parent_dir, reverse_child);
        if vec3_len(cross_vec) < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
            if parent_dir[2].abs() < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
                puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
            } else {
                let mut axis = vec3_cross([0.0, 0.0, 1.0], parent_dir);
                if vec3_len(axis) < 1e-9 {
                    axis = vec3_cross([1.0, 0.0, 0.0], parent_dir);
                }
                let axis = vec3_normalize(axis);
                let half = std::f64::consts::FRAC_PI_2;
                quat_normalize([axis[0] * half.sin(), axis[1] * half.sin(), axis[2] * half.sin(), half.cos()])
            }
        } else {
            puzzle3d_quaternion_from_unit_vectors(reverse_child, parent_dir)
        }
    }

    /// 📌 Resolves an attraction endpoint (`objectId:vortexId`) to its owning object id and its vortex's LOCAL
    /// (object-frame) position/direction — the frame compose's connector math expects, before the object's own world
    /// transform is applied.
    fn puzzle3d_local_vortex_geom(fixture: &Puzzle3dFixture, full_id: &str) -> Option<(String, [f64; 3], [f64; 3])> {
        for object in &fixture.objects {
            for vortex in &object.vortices {
                if puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id {
                    return Some((object.id.clone(), vortex.position, vortex.direction.unwrap_or([0.0, 0.0, -1.0])));
                }
            }
        }
        None
    }

    /// 🔗 Resolves an attraction's `attracting`/`attracted` vortex full-ids to their owning object ids. Returns `None`
    /// for dangling references or same-object attractions (legal today but not a resolvable directed edge).
    fn puzzle3d_attraction_object_ids(fixture: &Puzzle3dFixture, attraction: &Puzzle3dAttraction) -> Option<(String, String)> {
        let attracting_object = puzzle3d_local_vortex_geom(fixture, &attraction.attracting)?.0;
        let attracted_object = puzzle3d_local_vortex_geom(fixture, &attraction.attracted)?.0;
        if attracting_object == attracted_object {
            return None;
        }
        Some((attracting_object, attracted_object))
    }

    /// 📐 Forward attraction placement — given the attracting object's world pose (`t_a`/`q_a`), both vortices' LOCAL
    /// position/direction, and the 6 connection-style parameters (angles in degrees), returns the attracted object's
    /// world pose. Exact quaternion port of compose's `compute_child_plane`.
    #[allow(clippy::too_many_arguments)]
    fn puzzle3d_attraction_child_pose(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], gap: f64, shift: f64, rise: f64, rotation_deg: f64, turn_deg: f64, tilt_deg: f64) -> ([f64; 3], [f64; 4]) {
        let parent_dir = vec3_normalize(d_a);
        let child_dir = vec3_normalize(d_b);
        let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);

        let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
        let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
        let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
        let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

        let rotate_q = quat_from_axis_angle(parent_dir[0], parent_dir[1], parent_dir[2], -deg_to_rad(rotation_deg));
        let turn_axis = quat_rotate_vector(rotate_q, raise_dir);
        let tilt_axis = quat_rotate_vector(rotate_q, shift_dir);
        let turn_q = quat_from_axis_angle(turn_axis[0], turn_axis[1], turn_axis[2], deg_to_rad(turn_deg));
        let tilt_q = quat_from_axis_angle(tilt_axis[0], tilt_axis[1], tilt_axis[2], deg_to_rad(tilt_deg));

        let mut orientation_local = quat_conjugate(align_q);
        orientation_local = quat_mul(orientation_local, quat_conjugate(rotate_q));
        orientation_local = quat_mul(orientation_local, quat_conjugate(turn_q));
        orientation_local = quat_mul(orientation_local, quat_conjugate(tilt_q));
        let orientation_local = quat_normalize(orientation_local);

        let offset = vec3_add(vec3_add(t_a, p_a), vec3_add(vec3_add(vec3_scale(gap_dir, gap), vec3_scale(shift_dir, shift)), vec3_scale(raise_dir, rise)));
        let t_b = vec3_sub(quat_rotate_vector(orientation_local, offset), p_b);
        let q_b = quat_normalize(quat_mul(orientation_local, q_a));
        (t_b, q_b)
    }

    /// 🔁 Inverse of `puzzle3d_attraction_child_pose` — given the attracted object's CURRENT world pose, derives the 6
    /// parameters that reproduce it exactly, so moving/rotating an attracted object never causes a resolve-triggered
    /// snap-back and creating an attraction never moves either endpoint.
    #[allow(clippy::too_many_arguments)]
    fn derive_attraction_params(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], t_b: [f64; 3], q_b: [f64; 4]) -> (f64, f64, f64, f64, f64, f64) {
        let parent_dir = vec3_normalize(d_a);
        let child_dir = vec3_normalize(d_b);
        let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);
        let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
        let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
        let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
        let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

        let orientation_local = quat_normalize(quat_mul(q_b, quat_conjugate(q_a)));

        let offset = quat_rotate_vector(quat_conjugate(orientation_local), vec3_add(t_b, p_b));
        let diff = vec3_sub(vec3_sub(offset, t_a), p_a);
        let gap = vec3_dot(diff, gap_dir);
        let shift = vec3_dot(diff, shift_dir);
        let rise = vec3_dot(diff, raise_dir);

        let residual = quat_mul(align_q, orientation_local);
        let m = quat_mul(quat_mul(quat_conjugate(pq), residual), pq);
        let col_x = quat_rotate_vector(m, [1.0, 0.0, 0.0]);
        let col_y = quat_rotate_vector(m, [0.0, 1.0, 0.0]);

        let clamp = |v: f64| v.clamp(-1.0, 1.0);
        let tilt_rad = -(clamp(col_y[2])).asin();
        let (rotation_rad, turn_rad) = if (col_y[2].abs() - 1.0).abs() < 1e-6 {
            (col_x[1].atan2(col_x[0]), 0.0)
        } else {
            let col_z = quat_rotate_vector(m, [0.0, 0.0, 1.0]);
            ((-col_x[2]).atan2(col_z[2]), col_y[0].atan2(col_y[1]))
        };

        (gap, shift, rise, rad_to_deg(rotation_rad), rad_to_deg(turn_rad), rad_to_deg(tilt_rad))
    }

    /// 🌲 Resolves every attracted object's world pose from its attracting root, over a directed BFS per
    /// weakly-connected component. Roots are in-degree-zero objects; a component that is a pure cycle (the "donut"
    /// case) picks the lexicographically smallest object id in that component as a deterministic root. Multiple
    /// incoming attractions to the same object are resolved first-visit-wins (mirrors compose's
    /// `flatten_design_positions` cycle handling). Idempotent: re-running against already-resolved poses reproduces
    /// them exactly. Returns, for every non-root object touched, the attraction index that positioned it — callers
    /// (e.g. `translateSelection`) use this to rederive params before a direct move so resolving never snaps it back.
    fn resolve_puzzle3d_attractions(fixture: &mut Puzzle3dFixture) -> HashMap<String, usize> {
        let mut edges: HashMap<String, Vec<(String, usize)>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut all_object_ids: Vec<String> = fixture.objects.iter().map(|object| object.id.clone()).collect();
        all_object_ids.sort();
        for id in &all_object_ids {
            in_degree.entry(id.clone()).or_insert(0);
        }
        for (index, attraction) in fixture.attractions.iter().enumerate() {
            if let Some((attracting_id, attracted_id)) = puzzle3d_attraction_object_ids(fixture, attraction) {
                edges.entry(attracting_id).or_default().push((attracted_id.clone(), index));
                *in_degree.entry(attracted_id).or_insert(0) += 1;
            }
        }

        fn find(parent_of: &mut HashMap<String, String>, id: &str) -> String {
            let mut current = id.to_string();
            while parent_of[&current] != current {
                let grandparent = parent_of[&parent_of[&current]].clone();
                parent_of.insert(current.clone(), grandparent.clone());
                current = grandparent;
            }
            current
        }
        fn union(parent_of: &mut HashMap<String, String>, a: &str, b: &str) {
            let root_a = find(parent_of, a);
            let root_b = find(parent_of, b);
            if root_a != root_b {
                parent_of.insert(root_a, root_b);
            }
        }
        let mut parent_of: HashMap<String, String> = all_object_ids.iter().map(|id| (id.clone(), id.clone())).collect();
        for (attracting_id, targets) in &edges {
            for (attracted_id, _) in targets {
                union(&mut parent_of, attracting_id, attracted_id);
            }
        }

        let mut components: HashMap<String, Vec<String>> = HashMap::new();
        for id in &all_object_ids {
            let root = find(&mut parent_of, id);
            components.entry(root).or_default().push(id.clone());
        }
        let mut component_keys: Vec<String> = components.keys().cloned().collect();
        component_keys.sort();

        let mut incoming: HashMap<String, usize> = HashMap::new();
        let mut visited: HashSet<String> = HashSet::new();

        for component_key in component_keys {
            let mut members = components.remove(&component_key).unwrap_or_default();
            members.sort();
            let roots: Vec<String> = members.iter().filter(|id| in_degree.get(id.as_str()).copied().unwrap_or(0) == 0).cloned().collect();
            let seed_roots: Vec<String> = if roots.is_empty() { vec![members[0].clone()] } else { roots };

            let mut queue: std::collections::VecDeque<String> = std::collections::VecDeque::new();
            for root in &seed_roots {
                if visited.insert(root.clone()) {
                    queue.push_back(root.clone());
                }
            }
            while let Some(current_id) = queue.pop_front() {
                let Some(targets) = edges.get(&current_id) else { continue };
                for (attracted_id, attraction_index) in targets.clone() {
                    if visited.contains(&attracted_id) {
                        continue;
                    }
                    let attraction = fixture.attractions[attraction_index].clone();
                    let (Some((_, p_a, d_a)), Some((_, p_b, d_b))) = (puzzle3d_local_vortex_geom(fixture, &attraction.attracting), puzzle3d_local_vortex_geom(fixture, &attraction.attracted)) else { continue };
                    let Some(attracting_object) = fixture.objects.iter().find(|object| object.id == current_id) else { continue };
                    let t_a = attracting_object.origin;
                    let q_a = attracting_object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                    let (t_b, q_b) = puzzle3d_attraction_child_pose(t_a, q_a, p_a, d_a, p_b, d_b, attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt);
                    if let Some(attracted_object) = fixture.objects.iter_mut().find(|object| object.id == attracted_id) {
                        attracted_object.origin = t_b;
                        attracted_object.orientation = Some(q_b);
                    }
                    incoming.insert(attracted_id.clone(), attraction_index);
                    visited.insert(attracted_id.clone());
                    queue.push_back(attracted_id);
                }
            }
        }
        incoming
    }

    /// 🧰 Rederives every attraction's 6 params from its endpoints' CURRENT poses. Used after merging externally
    /// computed poses (brush/fill placement via the collision-aware `puzzle_3d` engine, which knows nothing about
    /// gap/shift/rise/rotation/turn/tilt) so the follow-up resolve reproduces those poses exactly instead of
    /// re-deriving a bare port-to-port docking that could visibly jump the just-placed object.
    fn puzzle3d_rederive_all_attractions(fixture: &mut Puzzle3dFixture) {
        let ids: Vec<String> = fixture.attractions.iter().map(|attraction| attraction.id.clone()).collect();
        for id in ids {
            let Some(attraction) = fixture.attractions.iter().find(|attraction| attraction.id == id).cloned() else { continue };
            let (Some((attracting_id, p_a, d_a)), Some((attracted_id, p_b, d_b))) = (puzzle3d_local_vortex_geom(fixture, &attraction.attracting), puzzle3d_local_vortex_geom(fixture, &attraction.attracted)) else { continue };
            let pose = |object_id: &str| fixture.objects.iter().find(|object| object.id == object_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])));
            let (Some((t_a, q_a)), Some((t_b, q_b))) = (pose(&attracting_id), pose(&attracted_id)) else { continue };
            let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(t_a, q_a, p_a, d_a, p_b, d_b, t_b, q_b);
            if let Some(attraction) = fixture.attractions.iter_mut().find(|attraction| attraction.id == id) {
                attraction.gap = gap;
                attraction.shift = shift;
                attraction.rise = rise;
                attraction.rotation = rotation;
                attraction.turn = turn;
                attraction.tilt = tilt;
            }
        }
    }

    /// ✋ After a direct move/rotate on selected objects, rederives the 6 params of every moved object's incoming
    /// attraction (per the `incoming` map from a prior `resolve_puzzle3d_attractions` call) from its NEW pose, so the
    /// follow-up resolve reproduces that pose exactly instead of snapping the object back to its old one. Harmless for
    /// objects whose attracting object was moved by the same delta (relative pose is unchanged, so derived params come
    /// out unchanged too).
    fn puzzle3d_rederive_moved_attractions(fixture: &mut Puzzle3dFixture, moved_ids: &[String], incoming: &HashMap<String, usize>) {
        for object_id in moved_ids {
            let Some(&attraction_index) = incoming.get(object_id) else { continue };
            let Some(attraction) = fixture.attractions.get(attraction_index).cloned() else { continue };
            let (Some((attracting_id, p_a, d_a)), Some((_, p_b, d_b))) = (puzzle3d_local_vortex_geom(fixture, &attraction.attracting), puzzle3d_local_vortex_geom(fixture, &attraction.attracted)) else { continue };
            let Some(t_a_q_a) = fixture.objects.iter().find(|object| object.id == attracting_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]))) else { continue };
            let Some(t_b_q_b) = fixture.objects.iter().find(|object| &object.id == object_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]))) else { continue };
            let (t_a, q_a) = t_a_q_a;
            let (t_b, q_b) = t_b_q_b;
            let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(t_a, q_a, p_a, d_a, p_b, d_b, t_b, q_b);
            if let Some(attraction) = fixture.attractions.get_mut(attraction_index) {
                attraction.gap = gap;
                attraction.shift = shift;
                attraction.rise = rise;
                attraction.rotation = rotation;
                attraction.turn = turn;
                attraction.tilt = tilt;
            }
        }
    }
    //#endregion 🔖AttractionResolve

    //#region 🔖Terminology
    /// 🗣️ Complete UI label set for the 3d app; one field per label makes every terminology×locale combination compile-checked.
    struct Puzzle3dLabels {
        objects: &'static str,
        object: &'static str,
        vortices: &'static str,
        vortex: &'static str,
    }

    const PUZZLE3D_LABELS_NATIVE_EN: Puzzle3dLabels = Puzzle3dLabels { objects: "Objects", object: "Object", vortices: "Vortices", vortex: "Vortex" };
    const PUZZLE3D_LABELS_NATIVE_DE: Puzzle3dLabels = Puzzle3dLabels { objects: "Objekte", object: "Objekt", vortices: "Vortices", vortex: "Vortex" };
    const PUZZLE3D_LABELS_REUSE_EN: Puzzle3dLabels = Puzzle3dLabels { objects: "Building components", object: "Building component", vortices: "Connection points", vortex: "Connection point" };
    const PUZZLE3D_LABELS_REUSE_DE: Puzzle3dLabels = Puzzle3dLabels { objects: "Baukomponenten", object: "Baukomponente", vortices: "Verbindungspunkte", vortex: "Verbindungspunkt" };

    /// 🗣️ Resolves the active label set from the shell-provided locale/terminology; unknown terminology ids fall back to native.
    fn puzzle3d_labels(view_state: &ViewState) -> &'static Puzzle3dLabels {
        let terminology = view_state.terminology.as_deref().unwrap_or("native");
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        match (terminology, is_de) {
            ("reuse", true) => &PUZZLE3D_LABELS_REUSE_DE,
            ("reuse", false) => &PUZZLE3D_LABELS_REUSE_EN,
            (_, true) => &PUZZLE3D_LABELS_NATIVE_DE,
            (_, false) => &PUZZLE3D_LABELS_NATIVE_EN,
        }
    }
    //#endregion 🔖Terminology

    //#region 🔖Panels
    fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
        UiTreeItemNode {
            loading: None,
            id: id.into(),
            label: label.into(),
            description: None,
            icon_id: icon_id.map(str::to_string),
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

    fn puzzle3d_hide_lock_actions(hidden: bool, locked: bool, flag_args: impl Fn(&str) -> Value) -> Vec<UiTreeItemAction> {
        vec![
            UiTreeItemAction { icon_id: if hidden { "eye-off".into() } else { "eye".into() }, label: Some(if hidden { "Show".into() } else { "Hide".into() }), action: puzzle3d_action("setSelectionFlag", Some(flag_args("hidden"))), reveal_on_hover: Some(true) },
            UiTreeItemAction { icon_id: if locked { "lock".into() } else { "lock-open".into() }, label: Some(if locked { "Unlock".into() } else { "Lock".into() }), action: puzzle3d_action("setSelectionFlag", Some(flag_args("locked"))), reveal_on_hover: Some(true) },
        ]
    }

    fn build_document_tree(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> UiNode {
        let object_items: Vec<UiTreeItemNode> = envelope
            .fixture
            .objects
            .iter()
            .map(|object| {
                let vortex_items: Vec<UiTreeItemNode> = object
                    .vortices
                    .iter()
                    .map(|vortex| {
                        let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                        tree_item_with_action(
                            format!("puzzle3d-vortex:{full_id}"),
                            vortex.vortex_kind.clone().unwrap_or_else(|| vortex.id.clone()),
                            Some("circle-dot"),
                            puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [full_id], "attractionIds": [] } }))),
                        )
                    })
                    .collect();
                let flag_args = {
                    let id = object.id.clone();
                    move |flag: &str| json!({ "flag": flag, "value": true, "entity": "object", "ids": [id.clone()] })
                };
                UiTreeItemNode {
                    loading: None,
                    id: format!("puzzle3d-object:{}", object.id),
                    label: object.object_kind.clone().unwrap_or_else(|| object.id.clone()),
                    description: None,
                    icon_id: Some("box".into()),
                    selected: Some(envelope.runtime.selection.object_ids.contains(&object.id)),
                    default_open: Some(false),
                    action: Some(puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [object.id], "vortexIds": [], "attractionIds": [] } })))),
                    hover_action: Some(puzzle3d_action("setHover", Some(json!({ "objectId": object.id })))),
                    unhover_action: Some(puzzle3d_action("setHover", None)),
                    actions: Some(puzzle3d_hide_lock_actions(object.hidden, object.locked, flag_args)),
                    draggable: None,
                    drag_data: None,
                    items: if vortex_items.is_empty() { None } else { Some(vortex_items) },
                    control: None,
                    is_hidden: Some(object.hidden),
                }
            })
            .collect();
        let reference_items: Vec<UiTreeItemNode> = envelope
            .fixture
            .references
            .iter()
            .map(|reference| {
                let flag_args = {
                    let id = reference.id.clone();
                    move |flag: &str| json!({ "flag": flag, "value": true, "entity": "reference", "ids": [id.clone()] })
                };
                UiTreeItemNode {
                    loading: None,
                    id: format!("puzzle3d-reference:{}", reference.id),
                    label: reference.id.clone(),
                    description: Some(reference.source.url.clone()),
                    icon_id: Some("globe".into()),
                    selected: Some(envelope.runtime.selection.reference_ids.contains(&reference.id)),
                    default_open: None,
                    action: Some(puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [], "referenceIds": [reference.id] } })))),
                    hover_action: None,
                    unhover_action: None,
                    actions: Some(puzzle3d_hide_lock_actions(reference.hidden, reference.locked, flag_args)),
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: Some(reference.hidden),
                }
            })
            .collect();
        let target_volume_items: Vec<UiTreeItemNode> = envelope
            .fixture
            .target_volumes
            .iter()
            .map(|volume| {
                let flag_args = {
                    let id = volume.id.clone();
                    move |flag: &str| json!({ "flag": flag, "value": true, "entity": "targetVolume", "ids": [id.clone()] })
                };
                UiTreeItemNode {
                    loading: None,
                    id: format!("puzzle3d-target-volume:{}", volume.id),
                    label: volume.id.clone(),
                    description: None,
                    icon_id: Some("cylinder".into()),
                    selected: Some(envelope.runtime.selection.target_volume_ids.contains(&volume.id)),
                    default_open: None,
                    action: Some(puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [], "targetVolumeIds": [volume.id] } })))),
                    hover_action: None,
                    unhover_action: None,
                    actions: Some(puzzle3d_hide_lock_actions(volume.hidden, volume.locked, flag_args)),
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: Some(volume.hidden),
                }
            })
            .collect();
        let attraction_items: Vec<UiTreeItemNode> = envelope
            .fixture
            .attractions
            .iter()
            .map(|attraction| {
                tree_item_with_action(
                    format!("puzzle3d-attraction:{}", attraction.id),
                    format!("{} → {}", attraction.attracting, attraction.attracted),
                    Some("link"),
                    puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [attraction.id] } }))),
                )
            })
            .collect();
        UiNode::Tree(UiTreeNode {
            loading: None,
            sections: vec![
                UiTreeSectionNode { id: "puzzle3d-play-document.objects".into(), label: Some(labels.objects.into()), default_open: Some(true), loading: None, items: object_items },
                UiTreeSectionNode { id: "puzzle3d-play-document.references".into(), label: Some("References".into()), default_open: Some(false), loading: None, items: reference_items },
                UiTreeSectionNode { id: "puzzle3d-play-document.target-volumes".into(), label: Some("Target Volumes".into()), default_open: Some(false), loading: None, items: target_volume_items },
                UiTreeSectionNode { id: "puzzle3d-play-document.attractions".into(), label: Some("Attractions".into()), default_open: Some(false), loading: None, items: attraction_items },
            ],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        })
    }

    /// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx) reads to auto-wire catalogue drag sources.
    const PUZZLE3D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

    fn puzzle3d_catalog_entries<'a>(fixture: &'a Puzzle3dFixture, section: &str) -> &'a [Value] {
        fixture.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get(section)).and_then(|entries| entries.as_array()).map(Vec::as_slice).unwrap_or(&[])
    }

    fn puzzle3d_catalog_entry_label(entry: &Value) -> String {
        entry.get("label").and_then(|value| value.as_str()).or_else(|| entry.get("name").and_then(|value| value.as_str())).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
    }

    fn puzzle3d_object_kind_vortex_items(entry: &Value) -> Vec<UiTreeItemNode> {
        entry
            .get("vortices")
            .and_then(|value| value.as_array())
            .map(|templates| {
                templates
                    .iter()
                    .enumerate()
                    .map(|(index, template)| {
                        let vortex_kind = template.get("vortexKind").and_then(|value| value.as_str()).unwrap_or("vortex");
                        let position = template.get("position").cloned().unwrap_or(json!([0.0, 0.0, 0.0]));
                        UiTreeItemNode {
                            loading: None,
                            id: format!("puzzle3d-kind-vortex.{index}.{vortex_kind}"),
                            label: vortex_kind.into(),
                            description: Some(position.to_string()),
                            icon_id: Some("circle-dot".into()),
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
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn puzzle3d_object_kind_item(entry: &Value) -> UiTreeItemNode {
        let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
        let draggable = entry.get("meshUrl").and_then(|value| value.as_str()).map(|url| !url.is_empty()).unwrap_or(false);
        UiTreeItemNode {
            loading: None,
            id: format!("puzzle3d-kind:{kind_id}"),
            label: puzzle3d_catalog_entry_label(entry),
            description: Some(kind_id.clone()),
            icon_id: Some("box".into()),
            selected: None,
            default_open: Some(false),
            action: Some(puzzle3d_action("addObjectKind", Some(json!({ "objectKind": kind_id.clone() })))),
            hover_action: Some(puzzle3d_action("setKindHover", Some(json!({ "kindId": kind_id.clone() })))),
            unhover_action: Some(puzzle3d_action("setKindHover", Some(json!({ "kindId": Value::Null })))),
            actions: None,
            draggable: draggable.then_some(true),
            drag_data: draggable.then(|| HashMap::from([(PUZZLE3D_CATALOGUE_DRAG_MIME.to_string(), json!({ "objectKind": kind_id }).to_string())])),
            items: Some(puzzle3d_object_kind_vortex_items(entry)),
            control: None,
            is_hidden: None,
        }
    }

    fn puzzle3d_catalog_kind_item(entry: &Value, icon_id: &str) -> UiTreeItemNode {
        let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
        UiTreeItemNode {
            loading: None,
            id: format!("puzzle3d-kind-entry:{kind_id}"),
            label: puzzle3d_catalog_entry_label(entry),
            description: Some(kind_id),
            icon_id: Some(icon_id.into()),
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

    fn build_kinds_tree(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> UiNode {
        let object_entries = puzzle3d_catalog_entries(&envelope.fixture, "objects");
        let vortex_entries = puzzle3d_catalog_entries(&envelope.fixture, "vortices");
        let cable_entries = puzzle3d_catalog_entries(&envelope.fixture, "cables");
        let attraction_entries = puzzle3d_catalog_entries(&envelope.fixture, "attractions");
        UiNode::Tree(UiTreeNode {
            loading: None,
            sections: vec![
                UiTreeSectionNode { id: "puzzle3d-play-kinds.objects".into(), label: Some(labels.objects.into()), default_open: Some(true), loading: None, items: object_entries.iter().map(puzzle3d_object_kind_item).collect() },
                UiTreeSectionNode { id: "puzzle3d-play-kinds.vortices".into(), label: Some(labels.vortices.into()), default_open: Some(false), loading: None, items: vortex_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "circle-dot")).collect() },
                UiTreeSectionNode { id: "puzzle3d-play-kinds.cables".into(), label: Some("Cables".into()), default_open: Some(false), loading: None, items: cable_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "plug")).collect() },
                UiTreeSectionNode { id: "puzzle3d-play-kinds.attractions".into(), label: Some("Attractions".into()), default_open: Some(false), loading: None, items: attraction_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "link")).collect() },
            ],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        })
    }

    fn inspector_text_field(id: impl Into<String>, label: impl Into<String>, mixed_text: semio_framework_plugin::UiInspectorMixedText, action: ActionDescriptor) -> UiNode {
        let id = id.into();
        UiNode::Field(UiFieldNode {
            id: id.clone(),
            label: label.into(),
            child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                id: format!("{id}.input"),
                input_kind: "text".into(),
                value: mixed_text.value,
                placeholder: mixed_text.placeholder,
                commit: None,
                on_change: action,
                min: None,
                max: None,
                step: None,
                accept: None,
            })),
            description: None,
            required: None,
            error: None,
        })
    }

    fn inspector_number_field(id: impl Into<String>, label: impl Into<String>, mixed_number: semio_framework_plugin::UiInspectorMixedNumber, action: ActionDescriptor) -> UiNode {
        let id = id.into();
        UiNode::Field(UiFieldNode {
            id: id.clone(),
            label: label.into(),
            child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                id: format!("{id}.input"),
                input_kind: "number".into(),
                value: if mixed_number.uniform { mixed_number.value.to_string() } else { String::new() },
                placeholder: if mixed_number.uniform { None } else { Some(semio_framework_plugin::UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                commit: None,
                on_change: action,
                min: None,
                max: None,
                step: None,
                accept: None,
            })),
            description: None,
            required: None,
            error: None,
        })
    }

    fn inspector_vec3_field(id: impl Into<String>, label: impl Into<String>, mixed_vec3: semio_framework_plugin::UiInspectorMixedVec3, action: ActionDescriptor) -> UiNode {
        let id = id.into();
        UiNode::Field(UiFieldNode {
            id: id.clone(),
            label: label.into(),
            child: Box::new(UiNode::Vec3(semio_framework_plugin::UiVec3Node { id: format!("{id}.input"), value: mixed_vec3.value, on_change: action })),
            description: None,
            required: None,
            error: None,
        })
    }

    fn inspector_header_and_delete(count: usize, noun: &str) -> Vec<UiNode> {
        vec![
            ui_text(format!("{count} {noun} selected")),
            UiNode::Button(semio_framework_plugin::UiButtonNode { id: Some("puzzle3d-play-inspector.delete".into()), icon_id: "trash".into(), label: "Delete".into(), action: puzzle3d_action("deleteSelection", None), style: None, disabled: None, loading: None }),
        ]
    }

    fn build_inspector_tree(envelope: &Puzzle3dScene, term_labels: &Puzzle3dLabels) -> UiNode {
        let selection = &envelope.runtime.selection;
        if !selection.object_ids.is_empty() {
            let objects: Vec<&Puzzle3dObject> = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).collect();
            if !objects.is_empty() {
                let ids_json = json!(selection.object_ids);
                let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "object", "ids": ids_json, "field": field })));
                let mut fields = inspector_header_and_delete(objects.len(), term_labels.object);
                if let [object] = objects.as_slice() {
                    fields.push(ui_inspector_readonly_field("puzzle3d-play-inspector.object.id", "Id", &object.id));
                }
                let labels: Vec<String> = objects.iter().map(|object| object.label.clone().unwrap_or_default()).collect();
                let kinds: Vec<String> = objects.iter().map(|object| object.object_kind.clone().unwrap_or_default()).collect();
                let origins: Vec<[f64; 3]> = objects.iter().map(|object| object.origin).collect();
                fields.push(inspector_text_field("puzzle3d-play-inspector.object.label", "Label", semio_framework_plugin::ui_inspector_mixed_text(&labels), patch_cmd("label")));
                fields.push(ui_inspector_readonly_field("puzzle3d-play-inspector.object.kind", "Kind", &semio_framework_plugin::ui_inspector_mixed_text(&kinds).value));
                fields.push(inspector_vec3_field("puzzle3d-play-inspector.object.origin", "Origin", semio_framework_plugin::ui_inspector_mixed_vec3(&origins), patch_cmd("origin")));
                return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.object".into(), label: term_labels.object.into(), default_open: None, fields }]);
            }
        }
        if !selection.vortex_ids.is_empty() {
            let vortices: Vec<(&Puzzle3dObject, &Puzzle3dVortex)> = envelope
                .fixture
                .objects
                .iter()
                .flat_map(|object| object.vortices.iter().map(move |vortex| (object, vortex)))
                .filter(|(object, vortex)| selection.vortex_ids.contains(&puzzle3d_vortex_full_id(&object.id, &vortex.id)))
                .collect();
            if !vortices.is_empty() {
                let full_ids: Vec<String> = vortices.iter().map(|(object, vortex)| puzzle3d_vortex_full_id(&object.id, &vortex.id)).collect();
                let ids_json = json!(full_ids);
                let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "vortex", "ids": ids_json, "field": field })));
                let mut fields = inspector_header_and_delete(vortices.len(), term_labels.vortex);
                if let [(_, vortex)] = vortices.as_slice() {
                    fields.push(ui_inspector_readonly_field("puzzle3d-play-inspector.vortex.id", "Full Id", &full_ids[0]));
                    let _ = vortex;
                }
                let kinds: Vec<String> = vortices.iter().map(|(_, vortex)| vortex.vortex_kind.clone().unwrap_or_default()).collect();
                let positions: Vec<[f64; 3]> = vortices.iter().map(|(_, vortex)| vortex.position).collect();
                let radii: Vec<f64> = vortices.iter().map(|(_, vortex)| vortex.radius.unwrap_or(0.35)).collect();
                fields.push(inspector_text_field("puzzle3d-play-inspector.vortex.kind", "Vortex Kind", semio_framework_plugin::ui_inspector_mixed_text(&kinds), patch_cmd("vortexKind")));
                fields.push(inspector_vec3_field("puzzle3d-play-inspector.vortex.position", "Position", semio_framework_plugin::ui_inspector_mixed_vec3(&positions), patch_cmd("position")));
                fields.push(inspector_number_field("puzzle3d-play-inspector.vortex.radius", "Radius", semio_framework_plugin::ui_inspector_mixed_number(&radii), patch_cmd("radius")));
                return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.vortex".into(), label: term_labels.vortex.into(), default_open: None, fields }]);
            }
        }
        if !selection.attraction_ids.is_empty() {
            let attractions: Vec<&Puzzle3dAttraction> = envelope.fixture.attractions.iter().filter(|attraction| selection.attraction_ids.contains(&attraction.id)).collect();
            if !attractions.is_empty() {
                let ids_json = json!(selection.attraction_ids);
                let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "attraction", "ids": ids_json, "field": field })));
                let mut fields = inspector_header_and_delete(attractions.len(), "attraction");
                let attracting: Vec<String> = attractions.iter().map(|attraction| attraction.attracting.clone()).collect();
                let attracted: Vec<String> = attractions.iter().map(|attraction| attraction.attracted.clone()).collect();
                fields.push(inspector_text_field("puzzle3d-play-inspector.attraction.attracting", "Attracting", semio_framework_plugin::ui_inspector_mixed_text(&attracting), patch_cmd("attracting")));
                fields.push(inspector_text_field("puzzle3d-play-inspector.attraction.attracted", "Attracted", semio_framework_plugin::ui_inspector_mixed_text(&attracted), patch_cmd("attracted")));
                let gaps: Vec<f64> = attractions.iter().map(|attraction| attraction.gap).collect();
                let shifts: Vec<f64> = attractions.iter().map(|attraction| attraction.shift).collect();
                let rises: Vec<f64> = attractions.iter().map(|attraction| attraction.rise).collect();
                let rotations: Vec<f64> = attractions.iter().map(|attraction| attraction.rotation).collect();
                let turns: Vec<f64> = attractions.iter().map(|attraction| attraction.turn).collect();
                let tilts: Vec<f64> = attractions.iter().map(|attraction| attraction.tilt).collect();
                fields.push(inspector_number_field("puzzle3d-play-inspector.attraction.gap", "Gap", semio_framework_plugin::ui_inspector_mixed_number(&gaps), patch_cmd("gap")));
                fields.push(inspector_number_field("puzzle3d-play-inspector.attraction.shift", "Shift", semio_framework_plugin::ui_inspector_mixed_number(&shifts), patch_cmd("shift")));
                fields.push(inspector_number_field("puzzle3d-play-inspector.attraction.rise", "Rise", semio_framework_plugin::ui_inspector_mixed_number(&rises), patch_cmd("rise")));
                fields.push(inspector_number_field("puzzle3d-play-inspector.attraction.rotation", "Rotation (°)", semio_framework_plugin::ui_inspector_mixed_number(&rotations), patch_cmd("rotation")));
                fields.push(inspector_number_field("puzzle3d-play-inspector.attraction.turn", "Turn (°)", semio_framework_plugin::ui_inspector_mixed_number(&turns), patch_cmd("turn")));
                fields.push(inspector_number_field("puzzle3d-play-inspector.attraction.tilt", "Tilt (°)", semio_framework_plugin::ui_inspector_mixed_number(&tilts), patch_cmd("tilt")));
                return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.attraction".into(), label: "Attraction".into(), default_open: None, fields }]);
            }
        }
        if !selection.reference_ids.is_empty() {
            let references: Vec<&Puzzle3dReference> = envelope.fixture.references.iter().filter(|reference| selection.reference_ids.contains(&reference.id)).collect();
            if !references.is_empty() {
                let ids_json = json!(selection.reference_ids);
                let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "reference", "ids": ids_json, "field": field })));
                let mut fields = inspector_header_and_delete(references.len(), "reference");
                let urls: Vec<String> = references.iter().map(|reference| reference.source.url.clone()).collect();
                let origins: Vec<[f64; 3]> = references.iter().map(|reference| reference.origin).collect();
                let widths: Vec<f64> = references.iter().map(|reference| reference.width_world).collect();
                fields.push(inspector_text_field("puzzle3d-play-inspector.reference.url", "Source Url", semio_framework_plugin::ui_inspector_mixed_text(&urls), patch_cmd("sourceUrl")));
                fields.push(inspector_vec3_field("puzzle3d-play-inspector.reference.origin", "Position", semio_framework_plugin::ui_inspector_mixed_vec3(&origins), patch_cmd("origin")));
                fields.push(inspector_number_field("puzzle3d-play-inspector.reference.width", "Width", semio_framework_plugin::ui_inspector_mixed_number(&widths), patch_cmd("widthWorld")));
                return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.reference".into(), label: "Reference".into(), default_open: None, fields }]);
            }
        }
        if !selection.target_volume_ids.is_empty() {
            let volumes: Vec<&Puzzle3dTargetVolume> = envelope.fixture.target_volumes.iter().filter(|volume| selection.target_volume_ids.contains(&volume.id)).collect();
            if !volumes.is_empty() {
                let ids_json = json!(selection.target_volume_ids);
                let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "targetVolume", "ids": ids_json, "field": field })));
                let mut fields = inspector_header_and_delete(volumes.len(), "target volume");
                let origins: Vec<[f64; 3]> = volumes.iter().map(|volume| volume.origin).collect();
                fields.push(inspector_vec3_field("puzzle3d-play-inspector.target-volume.origin", "Origin", semio_framework_plugin::ui_inspector_mixed_vec3(&origins), patch_cmd("origin")));
                return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.target-volume".into(), label: "Target Volume".into(), default_open: None, fields }]);
            }
        }
        ui_stack_vertical(vec![ui_text(format!("Schema: {}", envelope.fixture.schema)), ui_text(format!("Domain: {}", envelope.fixture.domain)), ui_text(format!("Objects: {}", envelope.fixture.objects.len()))])
    }

    fn build_settings_body(envelope: &Puzzle3dScene) -> UiNode {
        let runtime = &envelope.runtime;
        let selection_mode_field = UiNode::Field(UiFieldNode {
            id: "puzzle3d-play-settings.selection-mode".into(),
            label: "Selection Mode".into(),
            child: Box::new(UiNode::Select(semio_framework_plugin::UiSelectNode {
                id: "puzzle3d-play-settings.selection-mode.input".into(),
                value: runtime.selection_mode_default.clone(),
                items: vec![
                    semio_framework_plugin::UiSelectItem { value: "default".into(), label: "Default".into() },
                    semio_framework_plugin::UiSelectItem { value: "additive".into(), label: "Additive".into() },
                    semio_framework_plugin::UiSelectItem { value: "subtractive".into(), label: "Subtractive".into() },
                    semio_framework_plugin::UiSelectItem { value: "invertive".into(), label: "Invertive".into() },
                ],
                placeholder: None,
                on_change: puzzle3d_action("setSelectionModeDefault", None),
            })),
            description: None,
            required: None,
            error: None,
        });
        ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
            id: "puzzle3d-play-settings".into(),
            label: "Settings".into(),
            default_open: Some(true),
            fields: vec![
                selection_mode_field,
                inspector_number_field(
                    "puzzle3d-play-settings.overlap-budget",
                    "Brush Overlap Budget (m³)",
                    semio_framework_plugin::UiInspectorMixedNumber { value: runtime.overlap_budget, uniform: true },
                    puzzle3d_action("setBrushPlacementOverlapBudget", None),
                ),
                inspector_number_field(
                    "puzzle3d-play-settings.proximity-radius",
                    "Proximity Radius",
                    semio_framework_plugin::UiInspectorMixedNumber { value: runtime.proximity_radius, uniform: true },
                    puzzle3d_action("setProximityRadius", None),
                ),
                inspector_number_field("puzzle3d-play-settings.chunk-size", "Chunk Size", semio_framework_plugin::UiInspectorMixedNumber { value: runtime.chunk_size, uniform: true }, puzzle3d_action("setChunkSize", None)),
                inspector_number_field("puzzle3d-play-settings.grid-factor", "Grid Factor", semio_framework_plugin::UiInspectorMixedNumber { value: runtime.grid_factor, uniform: true }, puzzle3d_action("setGridFactor", None)),
            ],
        }])
    }
    //#endregion 🔖Panels

    //#region 🔖Jack
    /// 🕸️ A row produced by [`puzzle3d_run_jack_query`] — `entity`/`id` let a click reselect the exact match in the scene.
    struct Puzzle3dJackRow {
        entity: &'static str,
        id: String,
        value: String,
    }

    /// 🕸️ Parses the one supported shape — `MATCH (n:Label) RETURN n.field` — this is a deliberately minimal, self-contained
    /// stand-in for premigration's full Jack graph-query language (ported here without a cross-technology dependency on
    /// `trinity-jack`/`trinity-ram`, which CLAUDE.md's "do not mix technologies" rule rules out for a puzzle-3d ticket).
    fn puzzle3d_parse_jack_query(query: &str) -> Option<(String, String)> {
        let query = query.trim();
        let match_marker = "MATCH (n:";
        let match_start = query.find(match_marker)? + match_marker.len();
        let match_end = match_start + query[match_start..].find(')')?;
        let label = query[match_start..match_end].trim().to_string();
        let return_marker = "RETURN n.";
        let return_start = query.find(return_marker)? + return_marker.len();
        let field = query[return_start..].trim().to_string();
        if label.is_empty() || field.is_empty() {
            return None;
        }
        Some((label, field))
    }

    fn puzzle3d_run_jack_query(fixture: &Puzzle3dFixture, query: &str) -> Result<Vec<Puzzle3dJackRow>, String> {
        let (label, field) = puzzle3d_parse_jack_query(query).ok_or_else(|| "expected \"MATCH (n:Label) RETURN n.field\"".to_string())?;
        match label.as_str() {
            "Object" => Ok(fixture
                .objects
                .iter()
                .map(|object| {
                    let value = match field.as_str() {
                        "id" => object.id.clone(),
                        "label" => object.label.clone().unwrap_or_default(),
                        "kind" => object.object_kind.clone().unwrap_or_default(),
                        _ => object.label.clone().or_else(|| object.object_kind.clone()).unwrap_or_else(|| object.id.clone()),
                    };
                    Puzzle3dJackRow { entity: "object", id: object.id.clone(), value }
                })
                .collect()),
            "Vortex" => Ok(fixture
                .objects
                .iter()
                .flat_map(|object| {
                    let field = field.as_str();
                    object.vortices.iter().map(move |vortex| {
                        let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                        let value = match field {
                            "id" => full_id.clone(),
                            "kind" => vortex.vortex_kind.clone().unwrap_or_default(),
                            _ => vortex.vortex_kind.clone().unwrap_or_else(|| full_id.clone()),
                        };
                        Puzzle3dJackRow { entity: "vortex", id: full_id, value }
                    })
                })
                .collect()),
            "Attraction" => Ok(fixture
                .attractions
                .iter()
                .map(|attraction| {
                    let value = match field.as_str() {
                        "id" => attraction.id.clone(),
                        _ => format!("{} → {}", attraction.attracting, attraction.attracted),
                    };
                    Puzzle3dJackRow { entity: "attraction", id: attraction.id.clone(), value }
                })
                .collect()),
            other => Err(format!("unknown label \"{other}\" — supported: Object, Vortex, Attraction")),
        }
    }

    fn jack_row_selection_args(row: &Puzzle3dJackRow) -> Value {
        match row.entity {
            "object" => json!({ "selection": { "objectIds": [row.id], "vortexIds": [], "attractionIds": [] } }),
            "vortex" => json!({ "selection": { "objectIds": [], "vortexIds": [row.id], "attractionIds": [] } }),
            "attraction" => json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [row.id] } }),
            _ => json!({ "selection": {} }),
        }
    }

    fn build_jack_body(envelope: &Puzzle3dScene) -> UiNode {
        let query_field = UiNode::Field(UiFieldNode {
            id: "puzzle3d-play-jack.query".into(),
            label: "Query".into(),
            child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                id: "puzzle3d-play-jack.query.input".into(),
                input_kind: "text".into(),
                value: envelope.runtime.jack_query.clone(),
                placeholder: Some("MATCH (n:Object) RETURN n.name".into()),
                commit: None,
                on_change: puzzle3d_action("setJackQuery", None),
                min: None,
                max: None,
                step: None,
                accept: None,
            })),
            description: None,
            required: None,
            error: None,
        });
        match puzzle3d_run_jack_query(&envelope.fixture, &envelope.runtime.jack_query) {
            Ok(rows) => {
                let items: Vec<UiTreeItemNode> = rows.iter().map(|row| tree_item_with_action(format!("puzzle3d-jack-row:{}:{}", row.entity, row.id), row.value.clone(), None, puzzle3d_action("setSelection", Some(jack_row_selection_args(row))))).collect();
                let results = UiNode::Tree(UiTreeNode {
                    loading: None,
                    sections: vec![UiTreeSectionNode { id: "puzzle3d-play-jack.results".into(), label: Some(format!("{} results", items.len())), default_open: Some(true), loading: None, items }],
                    selected_ids: None,
                    highlighted_ids: None,
                    selection_change: None,
                    drop_action: None,
                });
                ui_stack_vertical(vec![query_field, results])
            }
            Err(message) => ui_stack_vertical(vec![query_field, ui_text(format!("Error: {message}"))]),
        }
    }
    //#endregion 🔖Jack

    //#region 🔖Engagement
    fn parse_brush_candidates_free(raw: &str) -> Vec<Value> {
        let parsed: Value = serde_json::from_str(raw).unwrap_or(Value::Null);
        parsed.get("free").and_then(|value| value.as_array()).cloned().unwrap_or_default()
    }

    fn parse_brush_candidates_free_count(raw: &str) -> usize {
        parse_brush_candidates_free(raw).len()
    }

    fn puzzle3d_brush_target_vortex(envelope: &Puzzle3dScene) -> Option<String> {
        envelope
            .runtime
            .selection
            .vortex_ids
            .first()
            .cloned()
            .or_else(|| envelope.runtime.hovered_vortex_full_id.clone())
            .or_else(|| {
                let object_id = envelope.runtime.hovered_object_id.as_deref()?;
                let object = envelope.fixture.objects.iter().find(|entry| entry.id == object_id)?;
                let vortex = object.vortices.first()?;
                Some(puzzle3d_vortex_full_id(&object.id, &vortex.id))
            })
    }

    /// 🧰 The select/brush/fill switcher lives in the framework toolbar (declared via `.tool` +
    /// `.window_kind_tools`); the fill-count slider, voxel edit-mode picker, voxel-dimension steppers and
    /// brush placement picker now live as tagged [`WindowMeasure::Group`]s in [`puzzle3d_window_measures`]
    /// (surfaced by [`partition_window_measures`] in the dedicated "Tool Options" rail only while their
    /// tool is active), so the engagement HUD is a bare command input plus a status line.
    fn puzzle3d_engagement(envelope: &Puzzle3dScene) -> WindowEngagement {
        let object_count = envelope.fixture.objects.len();
        let attraction_count = envelope.fixture.attractions.len();
        let active_tool = envelope.active_tool.as_str();
        WindowEngagement {
            session_active: Some(active_tool != "select"),
            options: None,
            input: Some(WindowEngagementInput {
                id: Some("puzzle3d-engagement".into()),
                value: Some(envelope.runtime.engagement_input.clone()),
                placeholder: Some("select, brush, fill <n>, zoom, clear, rectangle, lasso".into()),
                disabled: None,
                on_change: Some(puzzle3d_action("engagementInput", None)),
                on_submit: Some(puzzle3d_action("engagementSubmit", None)),
                on_repeat_last: Some(puzzle3d_action("engagementRepeatLast", None)),
                on_abort: Some(puzzle3d_action("engagementAbort", None)),
            }),
            control: None,
            controls: None,
            status: Some(vec![semio_framework_plugin::WindowEngagementStatus { id: "puzzle3d-world-status".into(), text: format!("{object_count} objects · {attraction_count} attractions") }]),
            possible_engagements: None,
        }
    }

    fn puzzle3d_context_menu_json(envelope: &Puzzle3dScene) -> Option<String> {
        let selection = &envelope.runtime.selection;
        if !selection.object_ids.is_empty() {
            let all_hidden = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.hidden);
            let all_locked = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.locked);
            let items = vec![
                json!({
                    "id": "duplicate",
                    "label": "Duplicate",
                    "action": "duplicateSelection",
                }),
                json!({
                    "id": "select-same-kind",
                    "label": "Select all of same kind",
                    "action": "selectSameKindSelection",
                }),
                json!({
                    "id": "hide-show",
                    "label": if all_hidden { "Show" } else { "Hide" },
                    "action": "setSelectionFlag",
                    "args": { "flag": "hidden", "value": !all_hidden },
                }),
                json!({
                    "id": "lock-unlock",
                    "label": if all_locked { "Unlock" } else { "Lock" },
                    "action": "setSelectionFlag",
                    "args": { "flag": "locked", "value": !all_locked },
                }),
                json!({
                    "id": "zoom",
                    "label": "Zoom to selection",
                    "action": "zoomToSelection",
                }),
                json!({
                    "id": "delete",
                    "label": "Delete",
                    "action": "deleteSelection",
                }),
            ];
            return serde_json::to_string(&items).ok();
        }
        if !selection.vortex_ids.is_empty() {
            let mut items = Vec::new();
            if let [only] = selection.vortex_ids.as_slice() {
                items.push(json!({
                    "id": "suggest",
                    "label": "Suggest objects",
                    "action": "openVortexSuggestions",
                    "args": { "fullId": only },
                }));
            }
            return serde_json::to_string(&items).ok();
        }
        if let Some(id) = selection.attraction_ids.first() {
            let items = vec![json!({
                "id": "delete",
                "label": "Delete",
                "action": "deleteAttraction",
                "args": { "id": id },
            })];
            return serde_json::to_string(&items).ok();
        }
        if let Some(id) = selection.target_volume_ids.first() {
            let target_volume = envelope.fixture.target_volumes.iter().find(|volume| &volume.id == id);
            let hidden = target_volume.map(|volume| volume.hidden).unwrap_or(false);
            let locked = target_volume.map(|volume| volume.locked).unwrap_or(false);
            let items = vec![
                json!({
                    "id": "hide-show",
                    "label": if hidden { "Show" } else { "Hide" },
                    "action": "setTargetVolumeFlag",
                    "args": { "id": id, "flag": "hidden", "value": !hidden },
                }),
                json!({
                    "id": "lock-unlock",
                    "label": if locked { "Unlock" } else { "Lock" },
                    "action": "setTargetVolumeFlag",
                    "args": { "id": id, "flag": "locked", "value": !locked },
                }),
                json!({
                    "id": "delete",
                    "label": "Delete",
                    "action": "deleteTargetVolume",
                    "args": { "id": id },
                }),
            ];
            return serde_json::to_string(&items).ok();
        }
        None
    }
    //#endregion 🔖Engagement

    //#region 🔖Measures
    const PUZZLE3D_LOD_SLIDER_MIN: f64 = 0.0;
    const PUZZLE3D_LOD_SLIDER_MAX: f64 = 1000.0;

    fn puzzle3d_kind_ids(fixture: &Puzzle3dFixture, section: &str) -> Vec<String> {
        fixture
            .meta
            .kind_catalogs
            .as_ref()
            .and_then(|catalogs| catalogs.get(section))
            .and_then(|entries| entries.as_array())
            .map(|entries| entries.iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect())
            .unwrap_or_default()
    }

    fn puzzle3d_lod_measures_group(runtime: &Puzzle3dRuntime) -> WindowMeasure {
        WindowMeasure::Group {
            id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod"),
            label: "LOD".into(),
            default_open: Some(true),
            active_tool_id: None,
            children: vec![
                WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-auto"), icon_id: "zoom-in".into(), label: Some("Auto zoom".into()), pressed: runtime.lod_automatic, text: None, on_change: puzzle3d_action("setLodAutomatic", None) },
                WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-depth-variable"), icon_id: "layers".into(), label: Some("Depth-variable".into()), pressed: runtime.lod_depth_variable, text: None, on_change: puzzle3d_action("setLodDepthVariable", None) },
                WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-grid"), icon_id: "layout-grid".into(), label: Some("Grid".into()), pressed: runtime.lod_show_grid, text: None, on_change: puzzle3d_action("setLodShowGrid", None) },
                WindowMeasure::Slider { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-value"), label: Some(format!("LOD {:.0}", runtime.lod_manual)), value: runtime.lod_manual, min: PUZZLE3D_LOD_SLIDER_MIN, max: PUZZLE3D_LOD_SLIDER_MAX, step: Some(1.0), on_change: puzzle3d_action("setLodManual", None) },
            ],
        }
    }

    fn puzzle3d_select_measures_group(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
        WindowMeasure::Group {
            id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select"),
            label: "Select".into(),
            default_open: Some(true),
            active_tool_id: None,
            children: vec![
                WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-rectangle"), icon_id: "square".into(), label: Some("Rectangle".into()), pressed: runtime.selection_method == "rectangle", text: None, on_change: puzzle3d_action("setSelectionMethod", Some(json!({ "method": "rectangle" }))) },
                WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-lasso"), icon_id: "lasso".into(), label: Some("Lasso".into()), pressed: runtime.selection_method == "lasso", text: None, on_change: puzzle3d_action("setSelectionMethod", Some(json!({ "method": "lasso" }))) },
                WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-objects"), icon_id: "box".into(), label: Some(labels.objects.into()), pressed: runtime.selectable_kinds.objects, text: None, on_change: puzzle3d_action("setSelectableKind", Some(json!({ "kind": "objects" }))) },
                WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-vortices"), icon_id: "circle-dot".into(), label: Some(labels.vortices.into()), pressed: runtime.selectable_kinds.vortices, text: None, on_change: puzzle3d_action("setSelectableKind", Some(json!({ "kind": "vortices" }))) },
                WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-attractions"), icon_id: "link".into(), label: Some("Attractions".into()), pressed: runtime.selectable_kinds.attractions, text: None, on_change: puzzle3d_action("setSelectableKind", Some(json!({ "kind": "attractions" }))) },
            ],
        }
    }

    fn puzzle3d_kind_weight_measures(prefix: &str, kind_ids: &[String], weights: &HashMap<String, f64>, action: &str) -> Vec<WindowMeasure> {
        kind_ids
            .iter()
            .map(|kind_id| {
                let weight = weights.get(kind_id).copied().unwrap_or(1.0);
                WindowMeasure::Slider {
                    id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-{prefix}-{kind_id}"),
                    label: Some(format!("{kind_id} {:.0}%", weight * 100.0)),
                    value: weight,
                    min: 0.0,
                    max: 1.0,
                    step: Some(0.01),
                    on_change: puzzle3d_action(action, Some(json!({ "kindId": kind_id }))),
                }
            })
            .collect()
    }

    fn puzzle3d_brush_measures_group(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> WindowMeasure {
        let object_ids = puzzle3d_kind_ids(&envelope.fixture, "objects");
        let vortex_ids = puzzle3d_kind_ids(&envelope.fixture, "vortices");
        WindowMeasure::Group {
            id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush"),
            label: "Brush".into(),
            default_open: Some(false),
            active_tool_id: None,
            children: vec![
                WindowMeasure::Slider {
                    id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-overlap-budget"),
                    label: Some("Overlap budget (m³)".into()),
                    value: envelope.runtime.overlap_budget,
                    min: 0.0,
                    max: 1.0,
                    step: Some(0.01),
                    on_change: puzzle3d_action("setBrushPlacementOverlapBudget", None),
                },
                WindowMeasure::Group {
                    id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-distribution"),
                    label: "Distribution".into(),
                    default_open: Some(false),
                    active_tool_id: None,
                    children: vec![
                        WindowMeasure::Group {
                            id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-distribution-objects"),
                            label: labels.objects.into(),
                            default_open: Some(false),
                            active_tool_id: None,
                            children: puzzle3d_kind_weight_measures("object-kind", &object_ids, &envelope.runtime.object_kind_weights, "setObjectKindWeight"),
                        },
                        WindowMeasure::Group {
                            id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-distribution-vortices"),
                            label: labels.vortices.into(),
                            default_open: Some(false),
                            active_tool_id: None,
                            children: puzzle3d_kind_weight_measures("vortex-kind", &vortex_ids, &envelope.runtime.vortex_kind_weights, "setVortexKindWeight"),
                        },
                    ],
                },
            ],
        }
    }

    fn puzzle3d_view_measure(runtime: &Puzzle3dRuntime) -> WindowMeasure {
        WindowMeasure::Select {
            id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-view"),
            label: Some("View".into()),
            value: runtime.view_preset.clone(),
            items: vec![
                MeasureSelectItem { id: "perspective".into(), value: "perspective".into(), label: "Perspective".into() },
                MeasureSelectItem { id: "top".into(), value: "top".into(), label: "Top".into() },
                MeasureSelectItem { id: "front".into(), value: "front".into(), label: "Front".into() },
                MeasureSelectItem { id: "right".into(), value: "right".into(), label: "Right".into() },
            ],
            on_change: puzzle3d_action("setCameraViewPreset", None),
        }
    }

    /// 🪣 Fill-count slider measure — the fill-tool's core parameter, mirrors the retired
    /// `puzzle3d_fill_count_control` (`setFillCount` reads `count`-or-`value`, so a slider's `{value}` payload
    /// preserves the action semantics). Live build progress is folded into the label exactly as before.
    fn puzzle3d_fill_count_measure(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession) -> WindowMeasure {
        let progress: Value = serde_json::from_str(&precompute.fill_progress()).unwrap_or(Value::Null);
        let done = progress.get("done").and_then(Value::as_bool).unwrap_or(true);
        let built_count = progress.get("count").and_then(Value::as_u64).unwrap_or(0) as u32;
        let label = if done {
            format!("Fill {}", envelope.runtime.fill_count)
        } else {
            let max_count = progress.get("maxCount").and_then(Value::as_u64).unwrap_or(PUZZLE3D_FILL_COUNT_MAX as u64);
            format!("Fill {} (building {built_count}/{max_count})", envelope.runtime.fill_count)
        };
        let max = if done { PUZZLE3D_FILL_COUNT_MAX as f64 } else { (built_count.max(envelope.runtime.fill_count)) as f64 };
        WindowMeasure::Slider { id: "puzzle3d-fill-count".into(), label: Some(label), value: envelope.runtime.fill_count as f64, min: 0.0, max, step: Some(1.0), on_change: puzzle3d_action("setFillCount", None) }
    }

    /// 🧊 Fill/edit-volumes mode picker measure — replaces the retired `puzzle3d_voxel_mode_toggle`. A
    /// [`WindowMeasure::Select`] dispatches its item `value` under the `value` key, so `setFillEditTargetVolumes`
    /// now reads `id`-or-`value` (see its handler) to keep the `fill`/`edit-volumes` semantics identical.
    fn puzzle3d_voxel_mode_measure(runtime: &Puzzle3dRuntime) -> WindowMeasure {
        WindowMeasure::Select {
            id: "puzzle3d-voxel-edit-mode".into(),
            label: Some("Mode".into()),
            value: if runtime.fill_edit_target_volumes { "edit-volumes".into() } else { "fill".into() },
            items: vec![
                MeasureSelectItem { id: "fill".into(), value: "fill".into(), label: "Fill".into() },
                MeasureSelectItem { id: "edit-volumes".into(), value: "edit-volumes".into(), label: "Edit Volumes".into() },
            ],
            on_change: puzzle3d_action("setFillEditTargetVolumes", None),
        }
    }

    /// 🧊 Voxel width/depth/height measures — the retired `puzzle3d_voxel_dim_steppers` re-expressed as
    /// sliders (`WindowMeasure` has no stepper variant; both carry `value`/`min`/`max`/`step` + `on_change`).
    /// `setVoxelDims` reads `axis` (baked into `on_change.args`) plus the dispatched `value`, unchanged.
    fn puzzle3d_voxel_dim_measures(runtime: &Puzzle3dRuntime) -> Vec<WindowMeasure> {
        let [w, d, h] = runtime.voxel_dims;
        let axis_slider = |axis: &str, label: &str, value: u32| WindowMeasure::Slider {
            id: format!("puzzle3d-voxel-{axis}"),
            label: Some(format!("{label} {value}")),
            value: value as f64,
            min: 1.0,
            max: 64.0,
            step: Some(1.0),
            on_change: puzzle3d_action("setVoxelDims", Some(json!({ "axis": axis }))),
        };
        vec![axis_slider("w", "Width", w), axis_slider("d", "Depth", d), axis_slider("h", "Height", h)]
    }

    /// 🪣 Tool Options group for the Fill tool — the fill-count slider (hidden while editing target volumes,
    /// matching the retired engagement gate) plus the fill/edit-volumes mode picker. Tagged `Some("fill")` so
    /// [`partition_window_measures`] surfaces it in the Tool Options rail only while the Fill tool is active.
    fn puzzle3d_fill_tool_options(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession) -> WindowMeasure {
        let mut children = vec![puzzle3d_voxel_mode_measure(&envelope.runtime)];
        if !envelope.runtime.fill_edit_target_volumes {
            children.insert(0, puzzle3d_fill_count_measure(envelope, precompute));
        }
        WindowMeasure::Group { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-tool-options-fill"), label: "Fill".into(), default_open: Some(true), active_tool_id: Some("fill".into()), children }
    }

    /// 🧊 Tool Options group for the Fill tool's voxel-edit mode — the width/depth/height dimension sliders,
    /// tagged `Some("fill")` and emitted only while `fill_edit_target_volumes` is on (mirrors the old
    /// `voxel_edit_active` gate). `None` otherwise so the rail carries no stray dimension controls.
    fn puzzle3d_voxel_tool_options(runtime: &Puzzle3dRuntime) -> Option<WindowMeasure> {
        if !runtime.fill_edit_target_volumes {
            return None;
        }
        Some(WindowMeasure::Group { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-tool-options-voxel"), label: "Voxel".into(), default_open: Some(true), active_tool_id: Some("fill".into()), children: puzzle3d_voxel_dim_measures(runtime) })
    }

    /// 🖌️ Tool Options group for the Brush tool — the placement candidate picker (`ToggleGroup`→`Select`,
    /// both "pick one of several string values"; `engagementControlSelect` reads `id`-or-`value` so semantics
    /// hold). Tagged `Some("brush")`; `None` when there are no candidates to place, matching the old gate.
    fn puzzle3d_brush_tool_options(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession) -> Option<WindowMeasure> {
        let target = puzzle3d_brush_target_vortex(envelope)?;
        let raw = precompute.brush_candidates(&target);
        let candidates = parse_brush_candidates_free(&raw);
        if candidates.is_empty() {
            return None;
        }
        let items: Vec<MeasureSelectItem> = candidates
            .iter()
            .enumerate()
            .map(|(index, candidate)| {
                let label = candidate.get("objectKind").and_then(|value| value.as_str()).or_else(|| candidate.get("objectKindId").and_then(|value| value.as_str())).unwrap_or("kind");
                let id = format!("puzzle3d.brush.candidate.{index}");
                MeasureSelectItem { id: id.clone(), value: id, label: label.into() }
            })
            .collect();
        let selected_index = envelope.runtime.brush_candidate_index.min(items.len().saturating_sub(1));
        Some(WindowMeasure::Group {
            id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-tool-options-brush"),
            label: "Placement".into(),
            default_open: Some(true),
            active_tool_id: Some("brush".into()),
            children: vec![WindowMeasure::Select {
                id: "puzzle3d-brush-placement".into(),
                label: Some("Placement".into()),
                value: format!("puzzle3d.brush.candidate.{selected_index}"),
                items,
                on_change: puzzle3d_action("engagementControlSelect", None),
            }],
        })
    }

    fn puzzle3d_window_measures(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
        let mut measures = vec![
            puzzle3d_view_measure(&envelope.runtime),
            puzzle3d_lod_measures_group(&envelope.runtime),
            puzzle3d_select_measures_group(&envelope.runtime, labels),
            puzzle3d_brush_measures_group(envelope, labels),
            world3d_sun_measures("puzzle3d", &envelope.runtime.sun, puzzle3d_action),
            puzzle3d_fill_tool_options(envelope, precompute),
        ];
        if let Some(voxel) = puzzle3d_voxel_tool_options(&envelope.runtime) {
            measures.push(voxel);
        }
        if let Some(brush) = puzzle3d_brush_tool_options(envelope, precompute) {
            measures.push(brush);
        }
        measures
    }
    //#endregion 🔖Measures

    //#region 🔖Puzzle3dPlayApp
    /// 🧩 Puzzle-3d play app. Owns the precompute engine and ephemeral view `runtime`; the persisted
    /// document (bare `Puzzle3dFixture` json) lives in the wrapping `VcsDocumentApp`'s op store. Each
    /// action rehydrates the engine from the projection, mutates a transient {@link Puzzle3dScene},
    /// then emits the granular op delta. Undo/redo/checkpoints are handled by the wrapper — the former
    /// manual `undo_stack`/`redo_stack` machinery is gone.
    pub struct Puzzle3dPlayApp {
        precompute: Puzzle3dPrecomputeSession,
        runtime: Puzzle3dRuntime,
    }

    impl Default for Puzzle3dPlayApp {
        fn default() -> Self {
            Self { precompute: Puzzle3dPrecomputeSession::new(), runtime: Puzzle3dRuntime::default() }
        }
    }

    impl DocumentApp for Puzzle3dPlayApp {
        type Projection = Value;
        type Op = Puzzle3dOp;

        fn app_id(&self) -> &str {
            PUZZLE3D_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            PUZZLE3D_FIXTURE_SCHEMA
        }

        fn initial_projection(&self) -> Value {
            serde_json::to_value(default_fixture()).unwrap_or_else(|_| serde_json::to_value(empty_fixture()).unwrap_or(Value::Null))
        }

        fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> ActionEmit<Puzzle3dOp> {
            let before = doc.projection.clone();
            let active_tool_initial = view_state.active_tool_id.as_deref().unwrap_or(PUZZLE3D_DEFAULT_TOOL).to_string();
            let mut envelope = scene_from_projection(&before, self.runtime.clone(), &active_tool_initial);
            sync_precompute_session(&mut self.precompute, &envelope);
            match action {
                "setFixtureJson" => {
                    if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                        if let Ok(fixture) = serde_json::from_str::<Puzzle3dFixture>(json_text) {
                            envelope.fixture = fixture;
                            resolve_puzzle3d_attractions(&mut envelope.fixture);
                        }
                    }
                }
                "setActiveExample" => {
                    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                    let next = if example_id.is_empty() || example_id == "empty" {
                        Some(empty_fixture())
                    } else if example_id == PUZZLE3D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
                        Some(default_fixture())
                    } else if example_id == PUZZLE3D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
                        Some(nakagin_fixture())
                    } else {
                        None
                    };
                    if let Some(fixture) = next {
                        envelope.fixture = fixture;
                        envelope.runtime = Puzzle3dRuntime::default();
                    }
                    resolve_puzzle3d_attractions(&mut envelope.fixture);
                    drive_precompute(&mut self.precompute, &envelope);
                }
                "setSelection" => {
                    if let Some(selection) = args.and_then(|value| value.get("selection")) {
                        if let Ok(parsed) = serde_json::from_value(selection.clone()) {
                            envelope.runtime.selection = parsed;
                        }
                    }
                }
                SET_ACTIVE_TOOL_ACTION_ID => {
                    // 🧰 Host already applied `view_state.active_tool_id`; clear in-progress scratch and refresh
                    // the placement engine for the new tool. Emits no ops (View-kind) and no tool-switch effect.
                    envelope.runtime.hovered_object_id = None;
                    envelope.runtime.hovered_vortex_full_id = None;
                    envelope.runtime.suggestion_menu = None;
                    envelope.runtime.engagement_input = String::new();
                    envelope.runtime.brush_candidate_index = 0;
                    if envelope.active_tool == "brush" || envelope.active_tool == "fill" {
                        drive_precompute(&mut self.precompute, &envelope);
                    }
                }
                "addObjectKind" => {
                    let object_kind = args.and_then(|value| value.get("objectKind")).and_then(|value| value.as_str()).unwrap_or("Object");
                    let id = next_object_id();
                    let catalog_entry = envelope.fixture.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("objects")?.as_array()?.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(object_kind)).cloned());
                    let mesh_url = catalog_entry.as_ref().and_then(|entry| entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string));
                    let vortices = catalog_entry.as_ref().map(|entry| puzzle3d_vortices_from_kind_template(entry)).unwrap_or_default();
                    let origin = args
                        .and_then(|value| value.get("origin"))
                        .and_then(|value| value.as_array())
                        .map(|values| [values.first().and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0)])
                        .unwrap_or([0.0, 0.0, 0.0]);
                    envelope.fixture.objects.push(Puzzle3dObject {
                        id: id.clone(),
                        label: Some(object_kind.into()),
                        object_kind: Some(object_kind.into()),
                        origin,
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                        mesh_url,
                        vortices,
                        hidden: false,
                        locked: false,
                    });
                    envelope.runtime.selection.object_ids = vec![id];
                    resolve_puzzle3d_attractions(&mut envelope.fixture);
                }
                "deleteSelection" => {
                    let object_ids: Vec<String> = envelope.runtime.selection.object_ids.clone();
                    let vortex_ids: HashSet<String> = envelope.runtime.selection.vortex_ids.iter().cloned().collect();
                    let attraction_ids: Vec<String> = envelope.runtime.selection.attraction_ids.clone();
                    let target_volume_ids: Vec<String> = envelope.runtime.selection.target_volume_ids.clone();
                    envelope.fixture.objects.retain(|object| !object_ids.contains(&object.id));
                    if !vortex_ids.is_empty() {
                        for object in envelope.fixture.objects.iter_mut() {
                            object.vortices.retain(|vortex| !vortex_ids.contains(&puzzle3d_vortex_full_id(&object.id, &vortex.id)));
                        }
                    }
                    envelope.fixture.attractions.retain(|attraction| !attraction_ids.contains(&attraction.id) && !object_ids.iter().any(|id| attraction.attracting.starts_with(&format!("{id}:")) || attraction.attracted.starts_with(&format!("{id}:"))));
                    envelope.fixture.target_volumes.retain(|volume| !target_volume_ids.contains(&volume.id));
                    let reference_ids: Vec<String> = envelope.runtime.selection.reference_ids.clone();
                    envelope.fixture.references.retain(|reference| !reference_ids.contains(&reference.id));
                    envelope.runtime.selection = Puzzle3dSelection::default();
                }
                "duplicateSelection" => {
                    let ids = envelope.runtime.selection.object_ids.clone();
                    let clones: Vec<Puzzle3dObject> = envelope
                        .fixture
                        .objects
                        .iter()
                        .filter(|object| ids.contains(&object.id))
                        .map(|object| {
                            let mut clone = object.clone();
                            clone.id = next_object_id();
                            clone.origin[0] += 0.5;
                            clone.origin[1] += 0.5;
                            clone
                        })
                        .collect();
                    let new_ids: Vec<String> = clones.iter().map(|object| object.id.clone()).collect();
                    envelope.fixture.objects.extend(clones);
                    envelope.runtime.selection.object_ids = new_ids;
                    resolve_puzzle3d_attractions(&mut envelope.fixture);
                }
                "selectSameKindSelection" => {
                    let Some(first_id) = envelope.runtime.selection.object_ids.first() else {
                        self.runtime = envelope.runtime;
                        return ActionEmit::default();
                    };
                    let Some(kind) = envelope.fixture.objects.iter().find(|object| object.id == *first_id).and_then(|object| object.object_kind.clone()).filter(|kind| !kind.is_empty()) else {
                        self.runtime = envelope.runtime;
                        return ActionEmit::default();
                    };
                    envelope.runtime.selection.object_ids = envelope.fixture.objects.iter().filter(|object| object.object_kind.as_deref() == Some(kind.as_str())).map(|object| object.id.clone()).collect();
                }
                "setCamera" => {
                    if let Some(camera) = args.and_then(|value| value.get("camera")) {
                        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                            envelope.fixture.camera = parsed;
                        }
                    }
                }
                "setProjection" => {
                    if let Some(projection) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                        envelope.fixture.camera.projection = Some(projection.into());
                    }
                }
                "setCameraViewPreset" => {
                    if let Some(preset) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                        envelope.fixture.camera = puzzle3d_camera_view_preset(preset);
                        envelope.runtime.view_preset = preset.into();
                    }
                }
                "setJackQuery" => {
                    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                        envelope.runtime.jack_query = value.into();
                    }
                }
                "translateSelection" => {
                    let ids = mesh_selection_ids(args, &envelope.runtime.selection.object_ids);
                    let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let incoming = resolve_puzzle3d_attractions(&mut envelope.fixture);
                    for object in &mut envelope.fixture.objects {
                        if ids.contains(&object.id) {
                            object.origin[0] += dx;
                            object.origin[1] += dy;
                            object.origin[2] += dz;
                        }
                    }
                    puzzle3d_rederive_moved_attractions(&mut envelope.fixture, &ids, &incoming);
                    resolve_puzzle3d_attractions(&mut envelope.fixture);
                    if !ids.is_empty() {
                    }
                }
                "rotateSelection" => {
                    let ids = mesh_selection_ids(args, &envelope.runtime.selection.object_ids);
                    let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let delta = quat_from_axis_angle(ax, ay, az, angle);
                    let incoming = resolve_puzzle3d_attractions(&mut envelope.fixture);
                    for object in &mut envelope.fixture.objects {
                        if ids.contains(&object.id) {
                            let current = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                            object.orientation = Some(quat_mul(delta, current));
                        }
                    }
                    puzzle3d_rederive_moved_attractions(&mut envelope.fixture, &ids, &incoming);
                    resolve_puzzle3d_attractions(&mut envelope.fixture);
                    if !ids.is_empty() {
                    }
                }
                "scaleSelection" => {
                    let ids = mesh_selection_ids(args, &envelope.runtime.selection.object_ids);
                    let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    for object in &mut envelope.fixture.objects {
                        if ids.contains(&object.id) {
                            object.scale = Some(scale_value_mul(&object.scale, sx, sy, sz));
                        }
                    }
                    if !ids.is_empty() {
                    }
                }
                "worldSelect" => {
                    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                    envelope.runtime.selection.object_ids = merge_world_selection_ids(&envelope.runtime.selection.object_ids, &ids, merge);
                }
                "worldHover" => {
                    envelope.runtime.hovered_object_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                }
                "setHover" => {
                    if args.is_none() || args.and_then(|value| value.get("objectId")).is_none() {
                        envelope.runtime.hovered_object_id = None;
                    } else {
                        envelope.runtime.hovered_object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).map(str::to_string);
                    }
                }
                "worldPick" => {
                    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                    if args.and_then(|value| value.get("id")).map_or(true, |value| value.is_null()) {
                        if merge == "replace" {
                            envelope.runtime.selection.object_ids.clear();
                        }
                    }
                    let index = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                    if let Some(object) = envelope.fixture.objects.get(index).filter(|object| !object.locked && !object.hidden) {
                        let id = object.id.clone();
                        let merge_ids = if merge == "add" {
                            let mut merged = envelope.runtime.selection.object_ids.clone();
                            if !merged.contains(&id) {
                                merged.push(id.clone());
                            }
                            merged
                        } else if merge == "toggle" {
                            let mut merged = envelope.runtime.selection.object_ids.clone();
                            if let Some(pos) = merged.iter().position(|entry| entry == &id) {
                                merged.remove(pos);
                            } else {
                                merged.push(id);
                            }
                            merged
                        } else {
                            vec![id]
                        };
                        envelope.runtime.selection.object_ids = merge_ids;
                    }
                }
                "worldVortexHover" => {
                    envelope.runtime.hovered_vortex_full_id = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(str::to_string);
                    if envelope.active_tool == "brush" && envelope.runtime.hovered_vortex_full_id.is_some() {
                        drive_precompute(&mut self.precompute, &envelope);
                    }
                }
                "worldVortexSelect" => {
                    if let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) {
                        envelope.runtime.selection.vortex_ids = vec![full_id.to_string()];
                        envelope.runtime.selection.object_ids.clear();
                        drive_precompute(&mut self.precompute, &envelope);
                    }
                }
                "worldRelocate" => {
                    let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                    let position = args
                        .and_then(|value| value.get("position"))
                        .and_then(|value| value.as_array())
                        .map(|values| [values.first().and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0)]);
                    let proximity_radius = envelope.runtime.proximity_radius;
                    if let (Some(object), Some(position)) = (envelope.fixture.objects.iter_mut().find(|object| object.id == object_id && !object.locked && !object.hidden), position) {
                        object.origin = position;
                        let object_orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                        let mut source_vortex: Option<(String, [f64; 3], [f64; 3], [f64; 3])> = None;
                        for vortex in &object.vortices {
                            let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                            source_vortex = Some((full_id, world_vortex_position(object, vortex), vortex.position, vortex.direction.unwrap_or([0.0, 0.0, -1.0])));
                            break;
                        }
                        // 🌲 New attractions attach the MOVED object as `attracted`: the pre-existing, stationary
                        // structure it snapped onto stays the resolution root. Params are derived from the current
                        // (already-relocated) poses so nothing jumps when the resolver next runs.
                        if let Some((source_id, source_pos, source_local_pos, source_local_dir)) = source_vortex {
                            for other in &envelope.fixture.objects {
                                if other.id == object_id {
                                    continue;
                                }
                                for vortex in &other.vortices {
                                    let target_id = puzzle3d_vortex_full_id(&other.id, &vortex.id);
                                    if target_id == source_id {
                                        continue;
                                    }
                                    let target_pos = world_vortex_position(other, vortex);
                                    let dx = source_pos[0] - target_pos[0];
                                    let dy = source_pos[1] - target_pos[1];
                                    let dz = source_pos[2] - target_pos[2];
                                    let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                                    if distance <= proximity_radius {
                                        let already_connected = envelope.fixture.attractions.iter().any(|entry| entry.attracting == source_id && entry.attracted == target_id || entry.attracting == target_id && entry.attracted == source_id);
                                        if !already_connected {
                                            let attraction_id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                                            let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(other.origin, other.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), vortex.position, vortex.direction.unwrap_or([0.0, 0.0, -1.0]), source_local_pos, source_local_dir, position, object_orientation);
                                            envelope.fixture.attractions.push(Puzzle3dAttraction { id: attraction_id, attracting: target_id, attracted: source_id.clone(), gap, shift, rise, rotation, turn, tilt });
                                        }
                                    }
                                }
                            }
                        }
                        resolve_puzzle3d_attractions(&mut envelope.fixture);
                        sync_precompute_session(&mut self.precompute, &envelope);
                    }
                }
                "setSelectionMethod" => {
                    let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
                    envelope.runtime.selection_method = method.into();
                }
                "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                    apply_world3d_sun_action(&mut envelope.runtime.sun, action, args);
                }
                "setLodAutomatic" => {
                    envelope.runtime.lod_automatic = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.lod_automatic);
                }
                "setLodDepthVariable" => {
                    envelope.runtime.lod_depth_variable = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.lod_depth_variable);
                }
                "setLodShowGrid" => {
                    envelope.runtime.lod_show_grid = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.lod_show_grid);
                }
                "setLodManual" => {
                    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                        envelope.runtime.lod_manual = value.clamp(PUZZLE3D_LOD_SLIDER_MIN, PUZZLE3D_LOD_SLIDER_MAX);
                    }
                }
                "setGridSnapEnabled" => {
                    envelope.runtime.grid_snap_enabled = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.grid_snap_enabled);
                }
                "setGridFactor" => {
                    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                        envelope.runtime.grid_factor = value.max(0.1);
                    }
                }
                "setSelectionModeDefault" => {
                    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                        envelope.runtime.selection_mode_default = mode.into();
                    }
                }
                "setProximityRadius" => {
                    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                        envelope.runtime.proximity_radius = value.max(0.0);
                    }
                }
                "setChunkSize" => {
                    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                        envelope.runtime.chunk_size = value.max(1.0);
                    }
                }
                "setSelectableKind" => {
                    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("");
                    let pressed = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool());
                    match kind {
                        "objects" => envelope.runtime.selectable_kinds.objects = pressed.unwrap_or(!envelope.runtime.selectable_kinds.objects),
                        "vortices" => envelope.runtime.selectable_kinds.vortices = pressed.unwrap_or(!envelope.runtime.selectable_kinds.vortices),
                        "attractions" => envelope.runtime.selectable_kinds.attractions = pressed.unwrap_or(!envelope.runtime.selectable_kinds.attractions),
                        _ => {}
                    }
                }
                "setKindHover" => {
                    envelope.runtime.hovered_kind_id = args.and_then(|value| value.get("kindId")).and_then(|value| value.as_str()).map(str::to_string);
                }
                "setSelectionFlag" => {
                    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
                    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
                    let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str());
                    let explicit_ids: Option<Vec<String>> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok());
                    match (entity, explicit_ids) {
                        (Some(entity), Some(ids)) => apply_puzzle3d_selection_flag(&mut envelope.fixture, entity, &ids, flag, value),
                        _ => {
                            apply_puzzle3d_selection_flag(&mut envelope.fixture, "object", &envelope.runtime.selection.object_ids.clone(), flag, value);
                            apply_puzzle3d_selection_flag(&mut envelope.fixture, "vortex", &envelope.runtime.selection.vortex_ids.clone(), flag, value);
                            apply_puzzle3d_selection_flag(&mut envelope.fixture, "targetVolume", &envelope.runtime.selection.target_volume_ids.clone(), flag, value);
                        }
                    }
                }
                "patchInspector" => {
                    let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str()).unwrap_or("");
                    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                    let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                    apply_puzzle3d_inspector_patch(&mut envelope.fixture, entity, &ids, field, &value);
                    resolve_puzzle3d_attractions(&mut envelope.fixture);
                }
                "selectAll" => {
                    envelope.runtime.selection.object_ids = if envelope.runtime.selectable_kinds.objects {
                        envelope.fixture.objects.iter().filter(|object| !object.hidden && !object.locked).map(|object| object.id.clone()).collect()
                    } else {
                        Vec::new()
                    };
                    envelope.runtime.selection.vortex_ids.clear();
                    envelope.runtime.selection.attraction_ids.clear();
                    envelope.runtime.selection.target_volume_ids.clear();
                    envelope.runtime.selection.reference_ids.clear();
                }
                "clearSelection" => {
                    envelope.runtime.selection = Puzzle3dSelection::default();
                }
                "contextMenuAt" => {
                    // 🖱️ Right-click on an unselected entity selects it and opens its menu in one round trip,
                    // instead of requiring a separate pick action before the menu items become available.
                    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("");
                    let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
                    envelope.runtime.selection = Puzzle3dSelection::default();
                    match kind {
                        "object" => envelope.runtime.selection.object_ids = vec![id.to_string()],
                        "vortex" => envelope.runtime.selection.vortex_ids = vec![id.to_string()],
                        "attraction" => envelope.runtime.selection.attraction_ids = vec![id.to_string()],
                        "targetVolume" => envelope.runtime.selection.target_volume_ids = vec![id.to_string()],
                        "reference" => envelope.runtime.selection.reference_ids = vec![id.to_string()],
                        _ => {}
                    }
                }
                "focusSelection" => {
                    apply_puzzle3d_focus_selection(&mut envelope);
                }
                "engagementInput" => {
                    envelope.runtime.engagement_input = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                }
                "engagementSubmit" => {
                    let raw = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("").trim().to_string();
                    if let Some(rest) = strip_engagement_prefix(&raw, "fill") {
                        envelope.active_tool = "fill".into();
                        drive_precompute(&mut self.precompute, &envelope);
                        let count = rest.parse::<u32>().ok().unwrap_or(envelope.runtime.fill_count).min(PUZZLE3D_FILL_COUNT_MAX);
                        envelope = apply_puzzle3d_fill_count(&mut self.precompute, envelope, count);
                    } else {
                        match raw.to_lowercase().as_str() {
                            "select" => envelope.active_tool = "select".into(),
                            "brush" => {
                                envelope.active_tool = "brush".into();
                                drive_precompute(&mut self.precompute, &envelope);
                            }
                            "zoom" => apply_puzzle3d_focus_selection(&mut envelope),
                            "clear" => envelope.runtime.selection = Puzzle3dSelection::default(),
                            "rectangle" => envelope.runtime.selection_method = "rectangle".into(),
                            "lasso" => envelope.runtime.selection_method = "lasso".into(),
                            _ => {}
                        }
                    }
                    envelope.runtime.engagement_input = String::new();
                }
                "engagementRepeatLast" => {
                    if envelope.active_tool == "fill" {
                        let count = (envelope.runtime.fill_count + 1).min(PUZZLE3D_FILL_COUNT_MAX);
                        envelope = apply_puzzle3d_fill_count(&mut self.precompute, envelope, count);
                    }
                }
                "engagementAbort" => {
                    envelope.runtime.engagement_input = String::new();
                    envelope.runtime.brush_candidate_index = 0;
                    envelope.active_tool = "select".into();
                }
                "createAttraction" => {
                    let attracting = args.and_then(|value| value.get("attracting")).and_then(|value| value.as_str()).unwrap_or("");
                    let attracted = args.and_then(|value| value.get("attracted")).and_then(|value| value.as_str()).unwrap_or("");
                    if !attracting.is_empty() && !attracted.is_empty() && attracting != attracted {
                        let already_connected = envelope.fixture.attractions.iter().any(|attraction| (attraction.attracting == attracting && attraction.attracted == attracted) || (attraction.attracting == attracted && attraction.attracted == attracting));
                        let compatible = match (resolve_vortex_kind(&envelope.fixture, attracting), resolve_vortex_kind(&envelope.fixture, attracted)) {
                            (Some(source_kind), Some(target_kind)) => puzzle3d_kinds_compatible(&envelope.fixture, &source_kind, &target_kind),
                            _ => false,
                        };
                        if !already_connected && compatible {
                            let id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                            // 🌲 Keep the drag gesture's direction (source = attracting) but derive params from the
                            // CURRENT poses of both objects, so creating an attraction never moves either endpoint.
                            let (gap, shift, rise, rotation, turn, tilt) = match (puzzle3d_local_vortex_geom(&envelope.fixture, attracting), puzzle3d_local_vortex_geom(&envelope.fixture, attracted)) {
                                (Some((attracting_object_id, p_a, d_a)), Some((attracted_object_id, p_b, d_b))) => {
                                    let pose = |object_id: &str| envelope.fixture.objects.iter().find(|object| object.id == object_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])));
                                    match (pose(&attracting_object_id), pose(&attracted_object_id)) {
                                        (Some((t_a, q_a)), Some((t_b, q_b))) => derive_attraction_params(t_a, q_a, p_a, d_a, p_b, d_b, t_b, q_b),
                                        _ => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
                                    }
                                }
                                _ => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
                            };
                            envelope.fixture.attractions.push(Puzzle3dAttraction { id, attracting: attracting.into(), attracted: attracted.into(), gap, shift, rise, rotation, turn, tilt });
                            resolve_puzzle3d_attractions(&mut envelope.fixture);
                        }
                    }
                }
                "deleteAttraction" => {
                    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                        envelope.fixture.attractions.retain(|attraction| attraction.id != id);
                    }
                }
                "setFillEditTargetVolumes" => {
                    let id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str());
                    envelope.runtime.fill_edit_target_volumes = match id {
                        Some("edit-volumes") => true,
                        Some("fill") => false,
                        _ => !envelope.runtime.fill_edit_target_volumes,
                    };
                }
                "setVoxelDims" => {
                    let axis = args.and_then(|value| value.get("axis")).and_then(|value| value.as_str()).unwrap_or("");
                    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                        let dimension = value.max(1.0).round() as u32;
                        match axis {
                            "w" => envelope.runtime.voxel_dims[0] = dimension,
                            "d" => envelope.runtime.voxel_dims[1] = dimension,
                            "h" => envelope.runtime.voxel_dims[2] = dimension,
                            _ => {}
                        }
                    }
                }
                "addTargetVolume" => {
                    if let Some(origin) = args.and_then(|value| value.get("origin")).and_then(value_as_vec3) {
                        let grid_factor = envelope.runtime.grid_factor.max(0.1);
                        let snapped = [(origin[0] / grid_factor).round() * grid_factor, (origin[1] / grid_factor).round() * grid_factor, (origin[2] / grid_factor).round() * grid_factor];
                        let [w, d, h] = envelope.runtime.voxel_dims;
                        let scale = json!([w as f64 * grid_factor, d as f64 * grid_factor, h as f64 * grid_factor]);
                        let id = format!("target-volume-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                        envelope.fixture.target_volumes.push(Puzzle3dTargetVolume { id, origin: snapped, orientation: None, scale: Some(scale), hidden: false, locked: false });
                    }
                }
                "deleteTargetVolume" => {
                    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                        envelope.fixture.target_volumes.retain(|volume| volume.id != id);
                    }
                }
                "setTargetVolumeFlag" => {
                    let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
                    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
                    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(false);
                    if let Some(volume) = envelope.fixture.target_volumes.iter_mut().find(|volume| volume.id == id) {
                        match flag {
                            "hidden" => volume.hidden = value,
                            "locked" => volume.locked = value,
                            _ => {}
                        }
                    }
                }
                "engagementControlSelect" => {
                    let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
                    if let Some(index) = candidate_id.strip_prefix("puzzle3d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
                        envelope.runtime.brush_candidate_index = index;
                    }
                }
                "addBrushObject" => {
                    drive_precompute(&mut self.precompute, &envelope);
                    if let Some(payload_value) = args {
                        if let Ok(payload) = serde_json::from_value::<BrushPlacePayload>(payload_value.clone()) {
                            if let Ok(fixture_json) = self.precompute.apply_brush_placement_rust(&serde_json::to_string(&payload).unwrap_or_default()) {
                                if let Some(next) = fixture_from_engine_json(&envelope, &fixture_json) {
                                    envelope = next;
                                    puzzle3d_rederive_all_attractions(&mut envelope.fixture);
                                    resolve_puzzle3d_attractions(&mut envelope.fixture);
                                }
                            }
                        }
                    }
                }
                "setFillCount" => {
                    drive_precompute(&mut self.precompute, &envelope);
                    let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_f64()).map(|value| value.round().max(0.0) as u32).unwrap_or(0).min(PUZZLE3D_FILL_COUNT_MAX);
                    envelope = apply_puzzle3d_fill_count(&mut self.precompute, envelope, count);
                }
                "setBrushPlacementOverlapBudget" => {
                    if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()) {
                        envelope.runtime.overlap_budget = value.clamp(0.0, 1.0);
                        sync_precompute_session(&mut self.precompute, &envelope);
                    }
                }
                "setObjectKindWeight" | "setVortexKindWeight" => {
                    let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
                    let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0);
                    if action == "setObjectKindWeight" {
                        envelope.runtime.object_kind_weights.insert(kind_id.into(), value);
                    } else {
                        envelope.runtime.vortex_kind_weights.insert(kind_id.into(), value);
                    }
                    sync_precompute_session(&mut self.precompute, &envelope);
                }
                "cycleBrushCandidate" | "cycleBrushCandidateBack" => {
                    drive_precompute(&mut self.precompute, &envelope);
                    let default_delta = if action == "cycleBrushCandidateBack" { -1 } else { 1 };
                    let delta = args.and_then(|value| value.get("delta")).and_then(|value| value.as_i64()).unwrap_or(default_delta);
                    if let Some(vortex_id) = puzzle3d_brush_target_vortex(&envelope) {
                        let raw = self.precompute.brush_candidates(&vortex_id);
                        let free_count = parse_brush_candidates_free_count(&raw);
                        if free_count > 0 {
                            let current = envelope.runtime.brush_candidate_index as i64;
                            let next = (current + delta).rem_euclid(free_count as i64);
                            envelope.runtime.brush_candidate_index = next as usize;
                        }
                    } else {
                        envelope.runtime.brush_candidate_index = envelope.runtime.brush_candidate_index.saturating_add_signed(delta as isize);
                    }
                }
                "openVortexSuggestions" => {
                    if let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) {
                        envelope.runtime.selection.vortex_ids = vec![full_id.to_string()];
                        envelope.runtime.selection.object_ids.clear();
                        envelope.active_tool = "brush".into();
                        envelope.runtime.brush_candidate_index = 0;
                        let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                        let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                        envelope.runtime.suggestion_menu = Some([x, y]);
                        drive_precompute(&mut self.precompute, &envelope);
                    }
                }
                "closeVortexSuggestions" => {
                    envelope.runtime.suggestion_menu = None;
                }
                "hoverSuggestion" => {
                    if let Some(index) = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()) {
                        envelope.runtime.brush_candidate_index = index as usize;
                    }
                }
                "acceptSuggestion" => {
                    drive_precompute(&mut self.precompute, &envelope);
                    let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(envelope.runtime.brush_candidate_index as u64) as usize;
                    if let Some(vortex_id) = puzzle3d_brush_target_vortex(&envelope) {
                        if let Some(preview_json) = self.precompute.brush_preview_json(&vortex_id, index) {
                            if let Ok(fixture_json) = self.precompute.apply_brush_placement_rust(&preview_json) {
                                if let Some(next) = fixture_from_engine_json(&envelope, &fixture_json) {
                                    envelope = next;
                                    puzzle3d_rederive_all_attractions(&mut envelope.fixture);
                                    resolve_puzzle3d_attractions(&mut envelope.fixture);
                                    envelope.runtime.suggestion_menu = None;
                                }
                            }
                        }
                    }
                }
                "suggestionsTick" => {
                    drive_precompute(&mut self.precompute, &envelope);
                }
                "fillBuildTick" => {
                    drive_precompute(&mut self.precompute, &envelope);
                    let progress: Value = serde_json::from_str(&self.precompute.fill_progress()).unwrap_or(Value::Null);
                    let done = progress.get("done").and_then(Value::as_bool).unwrap_or(true);
                    if done && envelope.active_tool == "fill" && envelope.runtime.fill_count == 0 {
                        envelope = apply_puzzle3d_fill_count(&mut self.precompute, envelope, 1);
                    }
                }
                "registerBrushMesh" => {
                    if let (Some(url), Some(positions), Some(indices)) =
                        (args.and_then(|v| v.get("url")).and_then(|v| v.as_str()), args.and_then(|v| v.get("positions")).and_then(|v| v.as_array()), args.and_then(|v| v.get("indices")).and_then(|v| v.as_array()))
                    {
                        let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
                        let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect();
                        self.precompute.register_mesh(url, &positions, &indices);
                        if let Ok(mut registry) = PUZZLE3D_MESH_REGISTRY.lock() {
                            registry.insert(url.to_string(), (positions, indices));
                        }
                    }
                }
                "worldPointerDown" => {}
                _ => {}
            }
            let next_active_tool = envelope.active_tool.clone();
            self.runtime = envelope.runtime;
            let after = serde_json::to_value(&envelope.fixture).unwrap_or_else(|_| before.clone());
            let ops = puzzle3d_document_delta_ops(&before, &after);
            // 🌀 Coalesce each gumball drag tick into one undoable edit (compact per-object records, not full meshes).
            let coalesce_key = match action {
                "translateSelection" => Some("gumball-translate".to_string()),
                "rotateSelection" => Some("gumball-rotate".to_string()),
                "scaleSelection" => Some("gumball-scale".to_string()),
                _ => None,
            };
            // 🧰 Programmatic tool switches (engagement submit/abort, suggestions, fill) push the active tool
            // back into the host session; `setActiveTool` itself never re-emits (the host already applied it).
            let effects = if next_active_tool != active_tool_initial {
                vec![HostEffect::SetActiveTool { window_kind_id: PUZZLE3D_PLAY_WINDOW_MAIN.into(), tool_id: next_active_tool }]
            } else {
                Vec::new()
            };
            ActionEmit { ops, coalesce_key, effects, ..Default::default() }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> UiNode {
            let active_tool = view_state.active_tool_id.as_deref().unwrap_or(PUZZLE3D_DEFAULT_TOOL);
            let envelope = scene_from_projection(doc.projection, self.runtime.clone(), active_tool);
            let labels = puzzle3d_labels(view_state);
            match body_key {
                PUZZLE3D_PLAY_BODY_COMPOSITE => {
                    let brush_preview = world_brush_preview_json(&self.precompute, &envelope);
                    build_world_3d_scene(
                        PUZZLE3D_PLAY_SURFACE_VIEWPORT,
                        PUZZLE3D_PLAY_APP_ID,
                        world3d_scene_extended(
                            camera_json(&envelope.fixture.camera),
                            world_meshes_json(&envelope.fixture),
                            world_instances_json(&envelope.fixture, &envelope.runtime),
                            world_selection_json(&envelope),
                            Some(world_vortices_json(&envelope.fixture, &envelope.runtime)),
                            Some(world_attractions_json(&envelope.fixture)),
                            Some(world_target_volumes_json(&envelope.fixture)),
                            Some(world_references_json(&envelope.fixture)),
                            brush_preview,
                            Some(world_interaction_json(&envelope, &self.precompute)),
                            None,
                            Some(world3d_lod_json(&envelope.runtime)),
                            Some(world3d_chunking_json(envelope.runtime.chunk_size, 8000.0)),
                            puzzle3d_context_menu_json(&envelope),
                            Some(world3d_environment_json(&envelope.runtime.sun)),
                        ),
                    )
                }
                PUZZLE3D_PLAY_BODY_DOCUMENT => build_document_tree(&envelope, labels),
                PUZZLE3D_PLAY_BODY_KINDS => build_kinds_tree(&envelope, labels),
                PUZZLE3D_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope, labels),
                PUZZLE3D_PLAY_BODY_SETTINGS => build_settings_body(&envelope),
                PUZZLE3D_PLAY_BODY_JACK => build_jack_body(&envelope),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn window_engagements(&self, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
            let active_tool = view_state.active_tool_id.as_deref().unwrap_or(PUZZLE3D_DEFAULT_TOOL);
            let envelope = scene_from_projection(doc.projection, self.runtime.clone(), active_tool);
            HashMap::from([(PUZZLE3D_PLAY_WINDOW_MAIN.into(), puzzle3d_engagement(&envelope))])
        }

        fn window_measures(&self, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
            let active_tool = view_state.active_tool_id.as_deref().unwrap_or(PUZZLE3D_DEFAULT_TOOL);
            let envelope = scene_from_projection(doc.projection, self.runtime.clone(), active_tool);
            let labels = puzzle3d_labels(view_state);
            HashMap::from([(PUZZLE3D_PLAY_WINDOW_MAIN.into(), puzzle3d_window_measures(&envelope, &self.precompute, labels))])
        }
    }
    //#endregion 🔖Puzzle3dPlayApp

    //#region 🔖Manifest
    pub fn create_puzzle3d_app() -> App {
        let envelope = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_tool: PUZZLE3D_DEFAULT_TOOL.into() };
        App::from_builder(
            App::builder(PUZZLE3D_PLAY_APP_ID, "Puzzle 3D")
                .document(["semio", "puzzle", "3d"])
                .icon_id("puzzle")
                .terminology("reuse")
                .mode("edit", "Edit")
                .default_mode_id("edit")
                .window_kind_with_engagement(PUZZLE3D_PLAY_WINDOW_MAIN, "Puzzle 3D", PUZZLE3D_PLAY_BODY_COMPOSITE, SurfaceKind::World3d, puzzle3d_engagement(&envelope))
                .default_layout(create_default_layout(&[PUZZLE3D_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Puzzle 3D".into()])))
                .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PUZZLE3D_PLAY_BODY_DOCUMENT)
                .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PUZZLE3D_PLAY_BODY_KINDS)
                .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PUZZLE3D_PLAY_BODY_INSPECTOR)
                .panel_tab("puzzle3d.panel.settings", "Settings", PanelGroup::Settings, PUZZLE3D_PLAY_BODY_SETTINGS)
                .panel_tab("puzzle3d.panel.jack", "Jack", PanelGroup::Workbench, PUZZLE3D_PLAY_BODY_JACK)
                .keybinding("mod+a", "selectAll")
                .keybinding("escape", "engagementAbort")
                .keybinding("delete", "deleteSelection")
                .keybinding("backspace", "deleteSelection")
                .keybinding("mod+d", "duplicateSelection")
                .keybinding("tab", "cycleBrushCandidate")
                .keybinding("shift+tab", "cycleBrushCandidateBack")
                .keybinding("f", "focusSelection")
                // 🔧 Document-mutating operations (emit VCS ops through the before/after fixture delta).
                .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setFixtureJson", "Set Fixture Json", ActionKind::Operation) })
                .operation("setActiveExample", "Set Active Example")
                .operation("addObjectKind", "Add Object")
                .operation("deleteSelection", "Delete Selection")
                .operation("duplicateSelection", "Duplicate Selection")
                .operation("setCamera", "Set Camera")
                .operation("setProjection", "Set Projection")
                .operation("setCameraViewPreset", "Set Camera View Preset")
                .operation("translateSelection", "Translate Selection")
                .operation("rotateSelection", "Rotate Selection")
                .operation("scaleSelection", "Scale Selection")
                .operation("worldRelocate", "Relocate Object")
                .operation("setSelectionFlag", "Set Selection Flag")
                .operation("patchInspector", "Patch Inspector")
                .operation("focusSelection", "Focus Selection")
                .operation("engagementSubmit", "Engagement Submit")
                .operation("engagementRepeatLast", "Engagement Repeat Last")
                .operation("createAttraction", "Create Attraction")
                .operation("deleteAttraction", "Delete Attraction")
                .operation("addTargetVolume", "Add Target Volume")
                .operation("deleteTargetVolume", "Delete Target Volume")
                .operation("setTargetVolumeFlag", "Set Target Volume Flag")
                .operation("addBrushObject", "Add Brush Object")
                .operation("setFillCount", "Set Fill Count")
                .operation("acceptSuggestion", "Accept Suggestion")
                .operation("fillBuildTick", "Fill Build Tick")
                // 👁️ Ephemeral view state — selection, hover, camera scratch, tool-parameter runtime.
                .view_action("setSelection", "Set Selection")
                .view_action("selectSameKindSelection", "Select Same Kind")
                .view_action("setJackQuery", "Set Jack Query")
                .view_action("worldSelect", "World Select")
                .view_action("worldHover", "World Hover")
                .view_action("setHover", "Set Hover")
                .view_action("worldPick", "World Pick")
                .view_action("worldVortexHover", "World Vortex Hover")
                .view_action("worldVortexSelect", "World Vortex Select")
                .view_action("setSelectionMethod", "Set Selection Method")
                .view_action("toggleSun", "Toggle Sun")
                .view_action("setSunAzimuth", "Set Sun Azimuth")
                .view_action("setSunElevation", "Set Sun Elevation")
                .view_action("setSunIntensity", "Set Sun Intensity")
                .view_action("setLodAutomatic", "Set Lod Automatic")
                .view_action("setLodDepthVariable", "Set Lod Depth Variable")
                .view_action("setLodShowGrid", "Set Lod Show Grid")
                .view_action("setLodManual", "Set Lod Manual")
                .view_action("setGridSnapEnabled", "Set Grid Snap Enabled")
                .view_action("setGridFactor", "Set Grid Factor")
                .view_action("setSelectionModeDefault", "Set Selection Mode Default")
                .view_action("setProximityRadius", "Set Proximity Radius")
                .view_action("setChunkSize", "Set Chunk Size")
                .view_action("setSelectableKind", "Set Selectable Kind")
                .view_action("setKindHover", "Set Kind Hover")
                .view_action("selectAll", "Select All")
                .view_action("clearSelection", "Clear Selection")
                .view_action("contextMenuAt", "Context Menu At")
                .view_action("engagementInput", "Engagement Input")
                .view_action("engagementAbort", "Engagement Abort")
                .view_action("engagementControlSelect", "Engagement Control Select")
                .view_action("setFillEditTargetVolumes", "Set Fill Edit Target Volumes")
                .view_action("setVoxelDims", "Set Voxel Dims")
                .view_action("setBrushPlacementOverlapBudget", "Set Brush Placement Overlap Budget")
                .view_action("setObjectKindWeight", "Set Object Kind Weight")
                .view_action("setVortexKindWeight", "Set Vortex Kind Weight")
                .view_action("cycleBrushCandidate", "Cycle Brush Candidate")
                .view_action("cycleBrushCandidateBack", "Cycle Brush Candidate Back")
                .view_action("openVortexSuggestions", "Open Vortex Suggestions")
                .view_action("closeVortexSuggestions", "Close Vortex Suggestions")
                .view_action("hoverSuggestion", "Hover Suggestion")
                .view_action("suggestionsTick", "Suggestions Tick")
                .view_action("registerBrushMesh", "Register Brush Mesh")
                .view_action("worldPointerDown", "World Pointer Down")
                // 📝 Staged argument forms for the panel-visible create/query actions (P1).
                .action_args("addObjectKind", vec![
                    ActionArgDef::select("objectKind", "Kind", vec![ActionArgOption::new("Object", "Object")]).default_value("Object"),
                ])
                .action_args("setJackQuery", vec![
                    ActionArgDef::text("value", "Query").default_value(default_jack_query()),
                ])
                // 🧰 Flat per-window tool set (host-owned `view_state.active_tool_id`); `select` is the default.
                .tool(ToolDefinition { category: Some(ToolCategory::Selection), ..ToolDefinition::new("select", "Select", "cursor") })
                .tool(ToolDefinition { group: Some("transform".into()), ..ToolDefinition::new("move", "Move", "move") })
                .tool(ToolDefinition { group: Some("transform".into()), ..ToolDefinition::new("rotate", "Rotate", "rotate-cw") })
                .tool(ToolDefinition { group: Some("transform".into()), ..ToolDefinition::new("scale", "Scale", "maximize-2") })
                .tool(ToolDefinition::new("brush", "Brush", "brush"))
                .tool(ToolDefinition::new("fill", "Fill", "fill"))
                .tool(ToolDefinition::new("worldRelocate", "Relocate", "move-3d"))
                .window_kind_tools(PUZZLE3D_PLAY_WINDOW_MAIN, vec!["select".into(), "move".into(), "rotate".into(), "scale".into(), "brush".into(), "fill".into(), "worldRelocate".into()]),
        )
        .example("empty", "Empty", &serde_json::to_string(&empty_fixture()).unwrap())
        .example(PUZZLE3D_EXAMPLE_CONCRETE_FOREST, "Concrete Forest", CONCRETE_FOREST_EXAMPLE_JSON)
        .example(PUZZLE3D_EXAMPLE_NAKAGIN, "Nakagin Capsule Tower", NAKAGIN_EXAMPLE_JSON)
        .program("puzzle3d", "Puzzle 3D", "model")
    }

    /// 🗃️ Real GLB geometry the browser round-tripped via `registerBrushMesh` this session, keyed by mesh url; falls back to a box for anything not yet loaded. `fn` pointers can't capture state, so this backs the export handler's plain-function-pointer signature.
    static PUZZLE3D_MESH_REGISTRY: LazyLock<std::sync::Mutex<HashMap<String, (Vec<f32>, Vec<u32>)>>> = LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

    /// 🌀 Undoes glTF's Y-up convention to land in this world's Z-up frame — mirrors `GLB_MESH_FRAME_ROTATION_X` (a fixed +90° turn about X) from `@semio-tech/infinite-world-r3f`, which the viewer applies visually but which raw `registerBrushMesh` vertices never carry.
    fn glb_frame_correct(position: [f32; 3]) -> [f32; 3] {
        [position[0], -position[2], position[1]]
    }

    fn quat_rotate_point(point: [f32; 3], quat: [f64; 4]) -> [f32; 3] {
        let [qx, qy, qz, qw] = quat;
        let (x, y, z) = (point[0] as f64, point[1] as f64, point[2] as f64);
        let (cx, cy, cz) = (qy * z - qz * y, qz * x - qx * z, qx * y - qy * x);
        let (tx, ty, tz) = (2.0 * cx, 2.0 * cy, 2.0 * cz);
        let (ux, uy, uz) = (qy * tz - qz * ty, qz * tx - qx * tz, qx * ty - qy * tx);
        [(x + qw * tx + ux) as f32, (y + qw * ty + uy) as f32, (z + qw * tz + uz) as f32]
    }

    /// 💾 Bakes each object's world transform (GLB frame correction, then scale/orientation/origin) into a single merged mesh for OBJ/GLB export; objects whose GLB hasn't round-tripped through `registerBrushMesh` this session fall back to a box.
    fn puzzle3d_mesh_from_document(doc: &serde_json::Value) -> Result<semio_framework_plugin::MeshData, String> {
        let fixture: Puzzle3dFixture = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
        let registry = PUZZLE3D_MESH_REGISTRY.lock().map_err(|_| "puzzle3d mesh registry poisoned".to_string())?;
        let fallback = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
        let mut merged = semio_framework_plugin::MeshData::default();
        for object in fixture.objects.iter().filter(|object| !object.hidden) {
            let mesh_url = resolve_object_mesh_url(object, &fixture.meta);
            let (positions, indices): (&[f32], &[u32]) = match mesh_url.as_deref().and_then(|url| registry.get(url)) {
                Some((positions, indices)) => (positions, indices),
                None => (&fallback.positions, &fallback.indices),
            };
            let orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            let scale = object_scale_json(object);
            let index_offset = (merged.positions.len() / 3) as u32;
            for chunk in positions.chunks_exact(3) {
                let corrected = glb_frame_correct([chunk[0], chunk[1], chunk[2]]);
                let scaled = [corrected[0] * scale[0] as f32, corrected[1] * scale[1] as f32, corrected[2] * scale[2] as f32];
                let rotated = quat_rotate_point(scaled, orientation);
                merged.positions.push(rotated[0] + object.origin[0] as f32);
                merged.positions.push(rotated[1] + object.origin[1] as f32);
                merged.positions.push(rotated[2] + object.origin[2] as f32);
            }
            merged.indices.extend(indices.iter().map(|index| index + index_offset));
        }
        if merged.positions.is_empty() {
            return Ok(fallback);
        }
        merged.compute_normals();
        Ok(merged)
    }

    /// 📥 Tier C DWG mesh import — always returns the empty puzzle-3d fixture; never errors on a structurally valid mesh.
    fn puzzle3d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<serde_json::Value, String> {
        serde_json::to_value(empty_fixture()).map_err(|error| error.to_string())
    }

    pub fn register_puzzle3d_exports() {
        semio_framework_os::register_mesh_exporter("3d.puzzle", "puzzle", puzzle3d_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
        semio_framework_os::register_mesh_exporter("3d.puzzle", "puzzle", puzzle3d_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
        semio_framework_os::register_mesh_exporter("3d.puzzle", "puzzle", puzzle3d_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
        semio_framework_os::register_mesh_importer("3d.puzzle", puzzle3d_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
        semio_framework_os::register_mesh_importer("3d.puzzle", puzzle3d_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
        semio_framework_os::register_mesh_importer("3d.puzzle", puzzle3d_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
        semio_framework_os::register_mesh_dwg_export_handler("3d.puzzle", "puzzle", puzzle3d_mesh_from_document);
        semio_framework_os::register_mesh_dwg_import_handler("3d.puzzle", puzzle3d_document_from_mesh);
    }
    //#endregion 🔖Manifest

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{app::AppActionRegistry, ActionMeta, PluginApp, VcsDocumentApp};

        fn meta(actor: &str) -> ActionMeta {
            ActionMeta { actor: actor.into(), instance_id: 1 }
        }

        fn new_app() -> VcsDocumentApp<Puzzle3dPlayApp> {
            VcsDocumentApp::new(Puzzle3dPlayApp::default())
        }

        fn new_app_with_registry() -> VcsDocumentApp<Puzzle3dPlayApp> {
            let definition = create_puzzle3d_app().definition;
            VcsDocumentApp::with_registry(Puzzle3dPlayApp::default(), AppActionRegistry::from_definition(&definition))
        }

        fn object_count(app: &VcsDocumentApp<Puzzle3dPlayApp>) -> usize {
            app.projection().expect("projection").get("objects").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
        }

        #[test]
        fn renders_world_scene() {
            let mut app = new_app();
            let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("world-3d"));
        }

        #[test]
        fn initial_projection_is_the_concrete_forest_fixture() {
            let mut app = new_app();
            assert_eq!(app.projection().expect("projection").get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
            assert!(object_count(&app) > 0, "the concrete-forest default fixture ships with objects");
        }

        #[test]
        fn set_active_example_swaps_the_document_and_undo_restores_it() {
            let mut app = new_app();
            let loaded = object_count(&app);
            assert!(loaded > 0);
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": "empty" })), &ViewState::default(), &meta("local")).expect("empty");
            assert_eq!(object_count(&app), 0, "empty example clears the objects");
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            assert_eq!(object_count(&app), loaded, "undo restores the concrete-forest objects");
            app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
            assert_eq!(object_count(&app), 0);
        }

        #[test]
        fn nakagin_example_loads_via_ops() {
            let mut app = new_app();
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), &ViewState::default(), &meta("local")).expect("nakagin");
            let projection = app.projection().expect("projection");
            assert_eq!(projection.get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
            assert!(projection.get("objects").and_then(|value| value.as_array()).is_some_and(|objects| !objects.is_empty()));
        }

        #[test]
        fn document_and_inspector_panels_render() {
            let mut app = new_app();
            for body in [PUZZLE3D_PLAY_BODY_DOCUMENT, PUZZLE3D_PLAY_BODY_KINDS, PUZZLE3D_PLAY_BODY_INSPECTOR] {
                let node = app.render(body, None, &ViewState::default()).expect("render");
                assert!(!serde_json::to_string(&node).unwrap().is_empty());
            }
        }

        #[test]
        fn app_definition_has_the_main_world_window() {
            let app = create_puzzle3d_app();
            assert!(app.definition.window_kinds.iter().any(|window| window.id == PUZZLE3D_PLAY_WINDOW_MAIN));
        }

        //#region 🧭 Suggestions, select-then-open context menu, fill build progress (Round 2)
        fn first_vortex_full_id(app: &VcsDocumentApp<Puzzle3dPlayApp>) -> String {
            let projection = app.projection().expect("projection");
            let object = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.first()).expect("seed object");
            let object_id = object.get("id").and_then(Value::as_str).expect("object id");
            let vortex_id = object.get("vortices").and_then(Value::as_array).and_then(|vortices| vortices.first()).and_then(|vortex| vortex.get("id")).and_then(Value::as_str).expect("vortex id");
            puzzle3d_vortex_full_id(object_id, vortex_id)
        }

        fn render_composite(app: &mut VcsDocumentApp<Puzzle3dPlayApp>) -> Value {
            let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
            serde_json::to_value(&node).unwrap()
        }

        fn interaction_of(node: &Value) -> Value {
            node.pointer("/world3d/interactionJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
        }

        /// 🔍 Depth-first search for a [`WindowMeasure::Slider`]'s value by id, descending into groups (the
        /// fill-count slider now nests inside the fill Tool Options group rather than sitting on the engagement).
        fn find_measure_slider(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
            measures.iter().find_map(|measure| match measure {
                WindowMeasure::Slider { id, value, .. } if id == slider_id => Some(*value),
                WindowMeasure::Group { children, .. } => find_measure_slider(children, slider_id),
                _ => None,
            })
        }

        /// 🎯 Top-level tool tag of a [`WindowMeasure::Group`] by id, or `None` when the group is absent.
        fn measure_group_tag(measures: &[WindowMeasure], group_id: &str) -> Option<Option<String>> {
            measures.iter().find_map(|measure| match measure {
                WindowMeasure::Group { id, active_tool_id, .. } if id == group_id => Some(active_tool_id.clone()),
                _ => None,
            })
        }

        fn context_menu_of(node: &Value) -> Value {
            node.pointer("/world3d/contextMenuJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
        }

        fn brush_preview_of(node: &Value) -> Value {
            node.pointer("/world3d/brushPreviewJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
        }

        #[test]
        fn context_menu_at_selects_vortex_and_prepends_suggest_objects() {
            let mut app = new_app();
            let vortex = first_vortex_full_id(&app);
            app.handle_action("contextMenuAt", Some(&json!({ "kind": "vortex", "id": vortex })), &ViewState::default(), &meta("local")).expect("contextMenuAt");
            let menu = context_menu_of(&render_composite(&mut app));
            let menu_json = serde_json::to_string(&menu).unwrap();
            assert!(menu_json.contains("Suggest objects"), "menu should be {menu_json}");
            assert!(menu_json.contains("openVortexSuggestions"));
        }

        #[test]
        fn context_menu_at_selects_target_volume_and_set_target_volume_flag_toggles_hidden() {
            let mut app = new_app();
            app.handle_action("addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), &ViewState::default(), &meta("local")).expect("addTargetVolume");
            let projection = app.projection().expect("projection");
            let volume_id = projection.get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
            app.handle_action("contextMenuAt", Some(&json!({ "kind": "targetVolume", "id": volume_id })), &ViewState::default(), &meta("local")).expect("contextMenuAt");
            let menu_json = serde_json::to_string(&context_menu_of(&render_composite(&mut app))).unwrap();
            assert!(menu_json.contains("setTargetVolumeFlag"), "menu should be {menu_json}");
            app.handle_action("setTargetVolumeFlag", Some(&json!({ "id": volume_id, "flag": "hidden", "value": true })), &ViewState::default(), &meta("local")).expect("setTargetVolumeFlag");
            let projection = app.projection().expect("projection");
            let hidden = projection.get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("hidden")).and_then(Value::as_bool);
            assert_eq!(hidden, Some(true));
        }

        #[test]
        fn open_vortex_suggestions_opens_the_suggestion_popup() {
            let mut app = new_app();
            let vortex = first_vortex_full_id(&app);
            let result = app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 12.0, "y": 34.0 })), &ViewState::default(), &meta("local")).expect("openVortexSuggestions");
            assert!(
                matches!(result.requested_effects.as_slice(), [HostEffect::SetActiveTool { window_kind_id, tool_id }] if window_kind_id == PUZZLE3D_PLAY_WINDOW_MAIN && tool_id == "brush"),
                "opening suggestions switches the host-owned tool to brush",
            );
            let brush_view = ViewState { active_tool_id: Some("brush".into()), ..ViewState::default() };
            let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &brush_view).expect("render");
            let interaction = interaction_of(&serde_json::to_value(&node).unwrap());
            assert_eq!(interaction.get("activeTool").and_then(Value::as_str), Some("brush"));
            let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
            assert_eq!(menu.get("open").and_then(Value::as_bool), Some(true));
            assert_eq!(menu.get("x").and_then(Value::as_f64), Some(12.0));
            assert_eq!(menu.get("y").and_then(Value::as_f64), Some(34.0));
        }

        #[test]
        fn close_vortex_suggestions_clears_the_menu() {
            let mut app = new_app();
            let vortex = first_vortex_full_id(&app);
            app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), &ViewState::default(), &meta("local")).expect("openVortexSuggestions");
            app.handle_action("closeVortexSuggestions", None, &ViewState::default(), &meta("local")).expect("closeVortexSuggestions");
            let interaction = interaction_of(&render_composite(&mut app));
            assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
        }

        /// 🖱️ Hovering a row in the suggestion popup must live-update the 3D brush preview (rendered by
        /// `world_brush_preview_json`, which reads `runtime.brush_candidate_index`) to the hovered
        /// candidate, so the UI can highlight it in 3D before the user clicks to accept.
        #[test]
        fn hover_suggestion_updates_the_brush_candidate_index_and_live_preview() {
            // 🧰 `active_tool` (the brush-preview gate in `world_brush_preview_json`) mirrors the
            // host-owned `view_state.active_tool_id`, not `runtime.active_tool` — the render call must be
            // given a "brush" view state, exactly like `open_vortex_suggestions_opens_the_suggestion_popup`.
            let brush_view = ViewState { active_tool_id: Some("brush".into()), ..ViewState::default() };
            let mut app = new_app();
            let vortex = first_vortex_full_id(&app);
            app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), &ViewState::default(), &meta("local")).expect("openVortexSuggestions");
            let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &brush_view).expect("render");
            let composite = serde_json::to_value(&node).unwrap();
            let interaction = interaction_of(&composite);
            assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(0), "opening suggestions starts hover at the first candidate");
            let preview = brush_preview_of(&composite);
            assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the live preview must target the vortex the suggestion menu was opened on");
            assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "the live preview must resolve to a real candidate object kind");

            app.handle_action("hoverSuggestion", Some(&json!({ "index": 1 })), &ViewState::default(), &meta("local")).expect("hoverSuggestion");
            let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &brush_view).expect("render");
            let composite = serde_json::to_value(&node).unwrap();
            let interaction = interaction_of(&composite);
            assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(1), "hovering a different row must move the tracked candidate index");
            let preview = brush_preview_of(&composite);
            assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the preview must keep targeting the same vortex while only the hovered candidate changes");
        }

        #[test]
        fn accept_suggestion_appends_an_object_and_closes_the_menu() {
            let mut app = new_app();
            let object_count_before = object_count(&app);
            let vortex = first_vortex_full_id(&app);
            app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), &ViewState::default(), &meta("local")).expect("openVortexSuggestions");
            app.handle_action("acceptSuggestion", None, &ViewState::default(), &meta("local")).expect("acceptSuggestion");
            assert_eq!(object_count(&app), object_count_before + 1);
            let interaction = interaction_of(&render_composite(&mut app));
            assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
        }

        #[test]
        fn fill_build_tick_auto_starts_fill_when_active_and_done() {
            // 🐢 `drive_precompute` is now bounded to a small per-call budget (the fix for the UI-freeze
            // bug: a single action must never grind the whole precompute queue synchronously), so the
            // build converges over several ticks — exactly like the real 120ms `fillBuildTick` loop in
            // `world-3d-host.tsx` — rather than in one call.
            let mut app = new_app();
            let fill_view = ViewState { active_tool_id: Some("fill".into()), ..ViewState::default() };
            app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": "fill" })), &fill_view, &meta("local")).expect("select fill tool");
            for _ in 0..500 {
                app.handle_action("fillBuildTick", None, &fill_view, &meta("local")).expect("fillBuildTick");
            }
            let measures = app.window_measures(&fill_view);
            let window_measures = measures.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main window measures");
            match find_measure_slider(window_measures, "puzzle3d-fill-count") {
                Some(value) => assert_eq!(value, 1.0, "fillBuildTick should auto-start the fill count at 1 once the build is done"),
                None => panic!("expected a fill-count slider in the fill Tool Options"),
            }
        }

        #[test]
        fn fill_count_measure_shows_building_progress_while_precompute_incomplete() {
            let mut session = Puzzle3dPrecomputeSession::new();
            let scene = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_tool: "fill".into() };
            sync_precompute_session(&mut session, &scene);
            session.precompute_step(1);
            match puzzle3d_fill_count_measure(&scene, &session) {
                WindowMeasure::Slider { label: Some(label), .. } => assert!(label.contains("building"), "expected a building-progress label, got {label:?}"),
                other => panic!("expected a slider measure, got {other:?}"),
            }
        }

        /// 🎯 D-3 follow-up: the fill-count slider, fill/edit-volumes mode picker, voxel-dimension steppers and
        /// brush placement picker are tagged `WindowMeasure::Group`s in [`puzzle3d_window_measures`] (surfaced by
        /// `partition_window_measures` only for their active tool), never [`WindowEngagementControl`]s on the HUD.
        #[test]
        fn fill_and_brush_params_are_tagged_tool_options_not_engagement_controls() {
            let labels = puzzle3d_labels(&ViewState::default());
            let session = Puzzle3dPrecomputeSession::new();
            // 🪣 Fill tool: the fill Tool Options group is tagged for "fill"; the voxel-dimension group only
            // materialises once the edit-volumes mode is on (mirroring the retired `voxel_edit_active` gate).
            let fill_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_tool: "fill".into() };
            let fill_measures = puzzle3d_window_measures(&fill_scene, &session, labels);
            assert_eq!(measure_group_tag(&fill_measures, "puzzle3d-play-tool-options-fill"), Some(Some("fill".into())));
            assert_eq!(measure_group_tag(&fill_measures, "puzzle3d-play-tool-options-voxel"), None, "voxel-dimension group is absent until edit-volumes mode is on");
            assert!(find_measure_slider(&fill_measures, "puzzle3d-fill-count").is_some(), "fill-count slider lives in the fill Tool Options while not editing volumes");
            let mut edit_runtime = Puzzle3dRuntime::default();
            edit_runtime.fill_edit_target_volumes = true;
            let edit_scene = Puzzle3dScene { fixture: default_fixture(), runtime: edit_runtime, active_tool: "fill".into() };
            let edit_measures = puzzle3d_window_measures(&edit_scene, &session, labels);
            assert_eq!(measure_group_tag(&edit_measures, "puzzle3d-play-tool-options-voxel"), Some(Some("fill".into())), "voxel-dimension steppers surface (tagged for fill) in edit-volumes mode");
            assert!(find_measure_slider(&edit_measures, "puzzle3d-fill-count").is_none(), "the fill-count slider is hidden while editing target volumes, exactly like the old engagement gate");
            assert!(find_measure_slider(&edit_measures, "puzzle3d-voxel-w").is_some(), "voxel width slider is present in edit-volumes mode");
            // 🧰 The fill engagement HUD no longer carries any relocated control/controls.
            let fill_engagement = puzzle3d_engagement(&fill_scene);
            assert!(fill_engagement.control.is_none() && fill_engagement.controls.is_none(), "fill engagement HUD must no longer carry the relocated controls");
            // 🖌️ Brush tool: with no candidates to place the tagged group is absent (matching the old gate); the
            // engagement HUD is likewise bare. The positive candidate case is exercised through the app below.
            let brush_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_tool: "brush".into() };
            assert_eq!(measure_group_tag(&puzzle3d_window_measures(&brush_scene, &session, labels), "puzzle3d-play-tool-options-brush"), None);
            let brush_engagement = puzzle3d_engagement(&brush_scene);
            assert!(brush_engagement.control.is_none() && brush_engagement.controls.is_none(), "brush engagement HUD must no longer carry the relocated control");
            // 🖌️ Positive case: opening a vortex's suggestions selects it and drives the precompute so real
            // candidates exist — the brush Tool Options group must then surface, tagged for "brush".
            let brush_view = ViewState { active_tool_id: Some("brush".into()), ..ViewState::default() };
            let mut app = new_app();
            let vortex = first_vortex_full_id(&app);
            app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), &brush_view, &meta("local")).expect("openVortexSuggestions");
            let brush_app_measures = app.window_measures(&brush_view);
            let window_measures = brush_app_measures.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main window measures");
            assert_eq!(measure_group_tag(window_measures, "puzzle3d-play-tool-options-brush"), Some(Some("brush".into())), "the brush Tool Options group surfaces once there are candidates to place");
        }
        //#endregion 🧭 Suggestions, select-then-open context menu, fill build progress (Round 2)

        //#region 🧰 Window Actions & Tools contract
        #[test]
        fn add_object_kind_materializes_the_declared_kind_default() {
            // 📝 P1 arg form: firing addObjectKind with no args must materialize the declared `objectKind`
            // default and emit the object-add op under registry enforcement.
            let mut app = new_app_with_registry();
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": "empty" })), &ViewState::default(), &meta("local")).expect("empty");
            let before = object_count(&app);
            let result = app.handle_action("addObjectKind", None, &ViewState::default(), &meta("local")).expect("addObjectKind");
            assert!(!result.operations.is_empty(), "addObjectKind is an Operation that emits ops");
            assert_eq!(object_count(&app), before + 1, "the materialized default kind adds exactly one object");
            let projection = app.projection().expect("projection");
            let kind = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("objectKind")).and_then(Value::as_str);
            assert_eq!(kind, Some("Object"), "the declared objectKind default was materialized host-side");
        }

        #[test]
        fn set_active_tool_emits_no_ops_and_no_history_entry() {
            // 🧰 Switching tools is the framework-injected View action: no document ops, no undo entry, no
            // re-emitted tool-switch effect (the host already applied `view_state.active_tool_id`).
            let mut app = new_app_with_registry();
            let before = app.projection().expect("projection");
            let brush_view = ViewState { active_tool_id: Some("brush".into()), ..ViewState::default() };
            let result = app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": "brush" })), &brush_view, &meta("local")).expect("switch tool");
            assert!(result.operations.is_empty(), "tool switching never emits document ops");
            assert!(result.requested_effects.is_empty(), "a user tool switch does not re-emit SetActiveTool");
            assert_eq!(app.projection().expect("projection"), before, "tool switching does not mutate the document");
        }

        #[test]
        fn engagement_exposes_no_tool_switch_options() {
            // 🧰 select/brush/fill switching lives only on the framework toolbar (declared via `.tool` +
            // `.window_kind_tools`); the engagement HUD must not duplicate it as options.
            let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_tool: PUZZLE3D_DEFAULT_TOOL.into() };
            let engagement = puzzle3d_engagement(&scene);
            assert!(engagement.options.is_none(), "the puzzle3d engagement must not re-expose tool switching as options");
        }

        #[test]
        fn gumball_translate_drag_coalesces_into_one_edit() {
            // 🌀 Coalescing regression: three translate ticks with the same key are ONE undoable edit.
            let mut app = new_app();
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": "empty" })), &ViewState::default(), &meta("local")).expect("empty");
            app.handle_action("addObjectKind", Some(&json!({ "objectKind": "Object" })), &ViewState::default(), &meta("local")).expect("add object");
            let object_id = app.projection().expect("projection").get("objects").and_then(Value::as_array).and_then(|objects| objects.first()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("object id").to_string();
            let origin_before = |app: &VcsDocumentApp<Puzzle3dPlayApp>| -> Vec<f64> {
                app.projection().expect("projection").get("objects").and_then(Value::as_array).and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id.as_str()))).and_then(|object| object.get("origin")).and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()).unwrap_or_default()
            };
            let start = origin_before(&app);
            for dx in [1.0, 2.0, 3.0] {
                app.handle_action("translateSelection", Some(&json!({ "ids": [object_id], "dx": dx, "dy": 0.0, "dz": 0.0 })), &ViewState { active_tool_id: Some("move".into()), ..ViewState::default() }, &meta("local")).expect("drag tick");
            }
            let dragged = origin_before(&app);
            assert!((dragged[0] - start[0] - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            assert_eq!(origin_before(&app), start, "one undo restores the whole coalesced gumball drag");
        }
        //#endregion 🧰 Window Actions & Tools contract
    }
    //#endregion 🧪Tests
}
pub mod d5 {
    //! 👯 Puzzle 5D plugin — paired 2D board + 3D world puzzle play app bundled as a hot-swappable WASM component.

    use puzzle_5d::{puzzle5d_document_delta_ops, BrushPlacePayload, Puzzle5dOp, Puzzle5dPrecomputeSession};
    use semio_framework_os::{register_mesh_exporter, register_mesh_importer};
    use semio_framework_plugin::{
        apply_world3d_sun_action, build_puzzle2d_board_scene, build_world_3d_scene, create_default_layout,
        ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, DocumentApp, DocumentView, MeasureSelectItem, WindowEngagementStatus,
        merge_world_selection_ids, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text, world3d_chunking_json, world3d_environment_json, world3d_mesh_id_from_url, world3d_meshes_json_from_urls, world3d_scene_extended, world3d_selection_json, world3d_sun_measures, App,
        ActionDescriptor, PanelGroup, Puzzle2dBoardScene, SurfaceKind, ToolCategory, ToolDefinition, UiFieldNode, UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement,
        WindowEngagementInput, WindowMeasure, WorldSunConfig, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
        FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, SET_ACTIVE_TOOL_ACTION_ID,
    };
    use semio_framework_plugin::kernel::HostEffect;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{BTreeMap, HashMap, HashSet};
    use std::sync::atomic::{AtomicU32, Ordering};

    //#region 🔖Constants
    const PUZZLE5D_PLAY_APP_ID: &str = "puzzle5d-play";
    const PUZZLE5D_PLAY_CONTROLLER_ID: &str = "puzzle5d-play";
    const PUZZLE5D_PLAY_SURFACE_2D: &str = "puzzle.5d.play.2d";
    const PUZZLE5D_PLAY_SURFACE_3D: &str = "puzzle.5d.play.3d";
    const PUZZLE5D_PLAY_BODY_2D: &str = "puzzle.5d.play.2d";
    const PUZZLE5D_PLAY_BODY_3D: &str = "puzzle.5d.play.3d";
    const PUZZLE5D_PLAY_BODY_DOCUMENT: &str = "puzzle.5d.play.document";
    const PUZZLE5D_PLAY_BODY_KINDS: &str = "puzzle.5d.play.kinds";
    const PUZZLE5D_PLAY_BODY_INSPECTOR: &str = "puzzle.5d.play.inspector";
    const PUZZLE5D_PLAY_WINDOW_2D: &str = "puzzle5d-2d";
    const PUZZLE5D_PLAY_WINDOW_3D: &str = "puzzle5d-3d";
    const PUZZLE5D_PLAY_WINDOWS: [&str; 2] = [PUZZLE5D_PLAY_WINDOW_2D, PUZZLE5D_PLAY_WINDOW_3D];
    const PUZZLE5D_SCHEMA: &str = "puzzle.5d";
    const PUZZLE5D_BOARD_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
    const PUZZLE5D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
    const PUZZLE5D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";

    const PUZZLE5D_FALLBACK_MESH_KIND: &str = "box";
    /// 🧰 Host-owned active tool (`view_state.active_tool_id`) when the host hasn't set one yet — the first declared window tool.
    const PUZZLE5D_DEFAULT_TOOL: &str = "select";
    const PUZZLE5D_FILL_COUNT_MAX: u32 = 1000;
    const PUZZLE5D_LOD_MODE_AUTOMATIC: &str = "automatic";
    const PUZZLE5D_SUGGESTION_OFFSET_MIN: f64 = 0.0;
    const PUZZLE5D_SUGGESTION_OFFSET_MAX: f64 = 160.0;
    const PUZZLE5D_SUGGESTION_OFFSET_STEP: f64 = 4.0;
    const PUZZLE5D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;
    const PUZZLE5D_DEFAULT_PART_RADIUS: f64 = 20.0;
    const PUZZLE5D_BOARD_PLACEMENT_GAP: f64 = 16.0;
    const PUZZLE5D_PROXIMITY_RADIUS: f64 = 0.75;

    const CONCRETE_FOREST_EXAMPLE_JSON: &str = include_str!("../../5d/example/concrete-forest.5d.json");
    const NAKAGIN_EXAMPLE_JSON: &str = include_str!("../../5d/example/nakagin-capsule-tower.5d.json");

    static PUZZLE5D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
    //#endregion 🔖Constants

    //#region 🔖Terminology
    /// 🗣️ Complete UI label set for the 5D app; one field per label makes every locale combination compile-checked.
    struct Puzzle5dLabels {
        parts: &'static str,
        fasteners: &'static str,
        grips: &'static str,
        ropes: &'static str,
        part: &'static str,
        grip: &'static str,
        select: &'static str,
        brush: &'static str,
        fill: &'static str,
        placement: &'static str,
        duplicate: &'static str,
        select_same_kind: &'static str,
        zoom_to_selection: &'static str,
        delete: &'static str,
        lod: &'static str,
        automatic: &'static str,
        suggestion: &'static str,
        offset: &'static str,
        part_weights: &'static str,
        grip_weights: &'static str,
        overlap: &'static str,
        window_2d: &'static str,
        window_3d: &'static str,
    }

    const PUZZLE5D_LABELS_NATIVE_EN: Puzzle5dLabels = Puzzle5dLabels {
        parts: "Parts",
        fasteners: "Fasteners",
        grips: "Grips",
        ropes: "Ropes",
        part: "Part",
        grip: "Grip",
        select: "Select",
        brush: "Brush",
        fill: "Fill",
        placement: "Placement",
        duplicate: "Duplicate",
        select_same_kind: "Select all of same kind",
        zoom_to_selection: "Zoom to selection",
        delete: "Delete",
        lod: "LOD",
        automatic: "Automatic",
        suggestion: "Suggestion",
        offset: "Offset",
        part_weights: "Part Weights",
        grip_weights: "Grip Weights",
        overlap: "Overlap",
        window_2d: "Puzzle 2D",
        window_3d: "Puzzle 3D",
    };

    const PUZZLE5D_LABELS_NATIVE_DE: Puzzle5dLabels = Puzzle5dLabels {
        parts: "Teile",
        fasteners: "Verbinder",
        grips: "Griffe",
        ropes: "Seile",
        part: "Teil",
        grip: "Griff",
        select: "Auswählen",
        brush: "Pinsel",
        fill: "Füllen",
        placement: "Platzierung",
        duplicate: "Duplizieren",
        select_same_kind: "Alle gleicher Art auswählen",
        zoom_to_selection: "Auf Auswahl zoomen",
        delete: "Löschen",
        lod: "LOD",
        automatic: "Automatisch",
        suggestion: "Vorschlag",
        offset: "Versatz",
        part_weights: "Teilgewichte",
        grip_weights: "Griffgewichte",
        overlap: "Überlappung",
        window_2d: "Puzzle 2D",
        window_3d: "Puzzle 3D",
    };

    /// 🗣️ Resolves the active label set from the shell-provided locale; puzzle5d has no alternate terminology, only native language switching.
    fn puzzle5d_labels(view_state: &ViewState) -> &'static Puzzle5dLabels {
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        if is_de { &PUZZLE5D_LABELS_NATIVE_DE } else { &PUZZLE5D_LABELS_NATIVE_EN }
    }
    //#endregion 🔖Terminology

    //#region 🔖Document
    fn one_f64() -> f64 {
        1.0
    }

    fn default_selection_method() -> String {
        "rectangle".into()
    }

    fn default_overlap_budget() -> f64 {
        0.02
    }

    fn default_lod_mode() -> String {
        PUZZLE5D_LOD_MODE_AUTOMATIC.into()
    }

    fn default_suggestion_offset() -> f64 {
        PUZZLE5D_DEFAULT_SUGGESTION_OFFSET
    }

    fn default_true() -> bool {
        true
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dCamera2d {
        #[serde(default)]
        x: f64,
        #[serde(default)]
        y: f64,
        #[serde(default = "one_f64")]
        zoom: f64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dCamera3d {
        #[serde(default)]
        position: [f64; 3],
        #[serde(default)]
        target: [f64; 3],
        #[serde(default = "one_f64")]
        zoom: f64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dGrip2d {
        #[serde(default)]
        angle: f64,
        #[serde(default, rename = "gripKind")]
        grip_kind: String,
        #[serde(default)]
        radius: f64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dGrip3d {
        #[serde(default)]
        position: [f64; 3],
        #[serde(default)]
        direction: Option<[f64; 3]>,
        #[serde(default)]
        radius: f64,
        #[serde(default)]
        label: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dGrip {
        id: String,
        #[serde(default, rename = "gripKind")]
        grip_kind: String,
        #[serde(default, rename = "2d")]
        grip_2d: Puzzle5dGrip2d,
        #[serde(default, rename = "3d")]
        grip_3d: Puzzle5dGrip3d,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dFastener {
        id: String,
        source: String,
        target: String,
        #[serde(default, rename = "fastenerKind", skip_serializing_if = "Option::is_none")]
        fastener_kind: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dPart2d {
        #[serde(default)]
        x: f64,
        #[serde(default)]
        y: f64,
        #[serde(default)]
        shape: String,
        #[serde(default)]
        radius: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        width: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        height: Option<f64>,
        #[serde(default)]
        text: String,
        #[serde(default, rename = "iconKind", skip_serializing_if = "Option::is_none")]
        icon_kind: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hidden: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        locked: Option<bool>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dPart3d {
        #[serde(default)]
        origin: [f64; 3],
        #[serde(default, rename = "meshUrl")]
        mesh_url: Option<String>,
        #[serde(default)]
        orientation: Option<[f64; 4]>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scale: Option<Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dPart {
        id: String,
        #[serde(rename = "partKind")]
        part_kind: String,
        #[serde(default, rename = "2d")]
        part_2d: Puzzle5dPart2d,
        #[serde(default, rename = "3d")]
        part_3d: Puzzle5dPart3d,
        #[serde(default)]
        grips: Vec<Puzzle5dGrip>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dDocument {
        schema: String,
        #[serde(default)]
        domain: String,
        #[serde(default)]
        camera2d: Puzzle5dCamera2d,
        #[serde(default)]
        camera3d: Puzzle5dCamera3d,
        #[serde(default)]
        parts: Vec<Puzzle5dPart>,
        #[serde(default)]
        fasteners: Vec<Puzzle5dFastener>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        meta: Option<Value>,
        #[serde(default, rename = "kindCatalogs")]
        kind_catalogs: Option<Value>,
        #[serde(default, rename = "kindCompatibility")]
        kind_compatibility: Option<Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dSelection {
        #[serde(default)]
        part_ids: Vec<String>,
        #[serde(default)]
        grip_ids: Vec<String>,
        #[serde(default)]
        fastener_ids: Vec<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Puzzle5dRuntime {
        #[serde(default)]
        selection: Puzzle5dSelection,
        #[serde(default = "default_selection_method")]
        selection_method: String,
        #[serde(default)]
        hovered_part_id: Option<String>,
        #[serde(default)]
        fill_count: u32,
        #[serde(default)]
        brush_candidate_index: usize,
        #[serde(default = "default_overlap_budget")]
        overlap_budget: f64,
        #[serde(default = "default_lod_mode")]
        lod_mode: String,
        #[serde(default = "default_suggestion_offset")]
        suggestion_offset: f64,
        #[serde(default = "default_true")]
        grid_snap_enabled: bool,
        #[serde(default = "one_f64")]
        grid_factor: f64,
        #[serde(default)]
        engagement_input_by_window: BTreeMap<String, String>,
        #[serde(default)]
        object_kind_weights: HashMap<String, f64>,
        #[serde(default)]
        vortex_kind_weights: HashMap<String, f64>,
        #[serde(default)]
        sun: WorldSunConfig,
    }

    /// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
    impl Default for Puzzle5dRuntime {
        fn default() -> Self {
            Self {
                selection: Puzzle5dSelection::default(),
                selection_method: default_selection_method(),
                hovered_part_id: None,
                fill_count: 0,
                brush_candidate_index: 0,
                overlap_budget: default_overlap_budget(),
                lod_mode: default_lod_mode(),
                suggestion_offset: default_suggestion_offset(),
                grid_snap_enabled: true,
                grid_factor: 1.0,
                engagement_input_by_window: BTreeMap::new(),
                object_kind_weights: HashMap::new(),
                vortex_kind_weights: HashMap::new(),
                sun: WorldSunConfig::default(),
            }
        }
    }

    /// 🧾 Transient render/mutation bundle pairing the persisted projection (the bare `Puzzle5dDocument`
    /// json) with the app's ephemeral view state. Never persisted — the {@link VcsDocumentApp} store owns
    /// the document and {@link Puzzle5dPlayApp} owns the runtime — but rebuilt per call so the existing
    /// board/world/engagement helpers keep their `&scene` signatures.
    #[derive(Clone)]
    struct Puzzle5dScene {
        document: Puzzle5dDocument,
        runtime: Puzzle5dRuntime,
        /// 🧰 Host-owned active tool mirrored from `view_state.active_tool_id` — transient, never persisted.
        active_tool: String,
    }

    /// 🧭 The select/brush/fill interaction mode the world engine reads, derived from the flat active tool
    /// (the transform gumball tools `move`/`rotate`/`scale` and `worldRelocate` all present as `select`).
    fn puzzle5d_scene_mode(active_tool: &str) -> &str {
        match active_tool {
            "brush" => "brush",
            "fill" => "fill",
            _ => "select",
        }
    }

    /// 🎚️ The gumball handle the world engine draws, derived from the flat active tool (non-transform tools fall back to `move`).
    fn puzzle5d_transform_handle(active_tool: &str) -> &str {
        match active_tool {
            "rotate" => "rotate",
            "scale" => "scale",
            _ => "move",
        }
    }

    fn empty_document() -> Puzzle5dDocument {
        Puzzle5dDocument {
            schema: PUZZLE5D_SCHEMA.into(),
            domain: "architecture".into(),
            camera2d: Puzzle5dCamera2d { x: 0.0, y: 0.0, zoom: 1.0 },
            camera3d: Puzzle5dCamera3d { position: [8.0, -8.0, 8.0], target: [0.0, 0.0, 0.0], zoom: 1.0 },
            parts: Vec::new(),
            fasteners: Vec::new(),
            meta: None,
            kind_catalogs: None,
            kind_compatibility: None,
            label: None,
        }
    }

    fn document_from_json(json_text: &str) -> Puzzle5dDocument {
        serde_json::from_str::<Puzzle5dDocument>(json_text).unwrap_or_else(|_| empty_document())
    }

    fn default_document() -> Puzzle5dDocument {
        document_from_json(CONCRETE_FOREST_EXAMPLE_JSON)
    }

    /// 🧾 Materializes the transient scene from the persisted projection (bare document json) and the
    /// app's current view state; an unparseable projection degrades to an empty document.
    fn scene_from_projection(projection: &Value, runtime: Puzzle5dRuntime, active_tool: &str) -> Puzzle5dScene {
        let document = serde_json::from_value::<Puzzle5dDocument>(projection.clone()).unwrap_or_else(|_| empty_document());
        Puzzle5dScene { document, runtime, active_tool: active_tool.to_string() }
    }

    fn puzzle5d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor { controller_id: PUZZLE5D_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
    }

    fn puzzle5d_grip_full_id(part_id: &str, grip_id: &str) -> String {
        if grip_id.contains(':') {
            grip_id.to_string()
        } else {
            format!("{part_id}:{grip_id}")
        }
    }

    fn next_part_id() -> String {
        let next = PUZZLE5D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        format!("part-{next}")
    }

    fn next_fastener_id() -> String {
        let next = PUZZLE5D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        format!("fastener-{next}")
    }

    fn parse_vec3(text: &str) -> Option<[f64; 3]> {
        let values: Vec<f64> = text.split(',').filter_map(|part| part.trim().parse::<f64>().ok()).collect();
        if values.len() < 3 {
            return None;
        }
        Some([values[0], values[1], values[2]])
    }

    fn resolve_part_mesh_url(part: &Puzzle5dPart, kind_catalogs: Option<&Value>) -> Option<String> {
        if let Some(url) = part.part_3d.mesh_url.as_ref().filter(|url| !url.is_empty()) {
            return Some(url.clone());
        }
        resolve_part_kind_mesh_url(&part.part_kind, kind_catalogs)
    }

    fn resolve_part_kind_mesh_url(part_kind: &str, kind_catalogs: Option<&Value>) -> Option<String> {
        let parts = kind_catalogs?.get("parts")?.as_array()?;
        parts.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(part_kind)).and_then(|entry| entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string))
    }

    fn collect_mesh_urls(document: &Puzzle5dDocument) -> Vec<String> {
        let mut urls = HashSet::new();
        for part in &document.parts {
            if let Some(url) = resolve_part_mesh_url(part, document.kind_catalogs.as_ref()) {
                urls.insert(url);
            }
        }
        if let Some(parts) = document.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("parts")).and_then(|v| v.as_array()) {
            for entry in parts {
                if let Some(url) = entry.get("meshUrl").and_then(|v| v.as_str()) {
                    urls.insert(url.to_string());
                }
            }
        }
        urls.into_iter().collect()
    }

    fn part_kind_grip_templates(document: &Puzzle5dDocument, part_kind: &str) -> Vec<Value> {
        document
            .kind_catalogs
            .as_ref()
            .and_then(|catalogs| catalogs.get("parts"))
            .and_then(|parts| parts.as_array())
            .and_then(|parts| parts.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(part_kind)))
            .and_then(|entry| entry.get("grips"))
            .and_then(|grips| grips.as_array())
            .cloned()
            .unwrap_or_default()
    }

    fn grips_from_templates(document: &Puzzle5dDocument, part_kind: &str) -> Vec<Puzzle5dGrip> {
        part_kind_grip_templates(document, part_kind)
            .iter()
            .enumerate()
            .map(|(index, template)| {
                let grip_kind = template.get("gripKind").and_then(|v| v.as_str()).unwrap_or("grip").to_string();
                let grip_2d: Puzzle5dGrip2d = template.get("2d").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                let grip_3d: Puzzle5dGrip3d = template.get("3d").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                Puzzle5dGrip { id: format!("v{index}"), grip_kind, grip_2d, grip_3d }
            })
            .collect()
    }

    fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
        [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
    }

    fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
        let len = (ax * ax + ay * ay + az * az).sqrt();
        if len < 1e-8 {
            return [0.0, 0.0, 0.0, 1.0];
        }
        let half = angle * 0.5;
        let s = half.sin();
        [ax / len * s, ay / len * s, az / len * s, half.cos()]
    }

    fn quat_rotate_vector(quat: [f64; 4], vector: [f64; 3]) -> [f64; 3] {
        let [x, y, z, w] = quat;
        let vx = vector[0];
        let vy = vector[1];
        let vz = vector[2];
        let ix = w * vx + y * vz - z * vy;
        let iy = w * vy + z * vx - x * vz;
        let iz = w * vz + x * vy - y * vx;
        let iw = -x * vx - y * vy - z * vz;
        [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x]
    }

    fn world_grip_position(part: &Puzzle5dPart, grip: &Puzzle5dGrip) -> [f64; 3] {
        let orientation = part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let rotated = quat_rotate_vector(orientation, grip.grip_3d.position);
        [part.part_3d.origin[0] + rotated[0], part.part_3d.origin[1] + rotated[1], part.part_3d.origin[2] + rotated[2]]
    }

    fn world_grip_direction(part: &Puzzle5dPart, grip: &Puzzle5dGrip) -> [f64; 3] {
        let direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
        quat_rotate_vector(part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), direction)
    }

    fn resolve_grip_world_position(document: &Puzzle5dDocument, full_id: &str) -> Option<[f64; 3]> {
        for part in &document.parts {
            for grip in &part.grips {
                if puzzle5d_grip_full_id(&part.id, &grip.id) == full_id {
                    return Some(world_grip_position(part, grip));
                }
            }
        }
        None
    }

    fn find_part_by_grip_full_id<'a>(document: &'a Puzzle5dDocument, full_id: &str) -> Option<(&'a Puzzle5dPart, &'a Puzzle5dGrip)> {
        for part in &document.parts {
            for grip in &part.grips {
                if puzzle5d_grip_full_id(&part.id, &grip.id) == full_id {
                    return Some((part, grip));
                }
            }
        }
        None
    }

    fn strip_tree_prefix(id: &str) -> &str {
        for prefix in ["puzzle5d-play-document.part.", "puzzle5d-play-document.grip.", "puzzle5d-play-document.fastener."] {
            if let Some(rest) = id.strip_prefix(prefix) {
                return rest;
            }
        }
        id
    }

    fn classify_selection(document: &Puzzle5dDocument, ids: &[String]) -> Puzzle5dSelection {
        let part_ids: HashSet<&str> = document.parts.iter().map(|part| part.id.as_str()).collect();
        let fastener_ids: HashSet<&str> = document.fasteners.iter().map(|fastener| fastener.id.as_str()).collect();
        let grip_ids: HashSet<String> = document.parts.iter().flat_map(|part| part.grips.iter().map(|grip| puzzle5d_grip_full_id(&part.id, &grip.id))).collect();
        let mut selection = Puzzle5dSelection::default();
        for raw in ids {
            let id = strip_tree_prefix(raw);
            if part_ids.contains(id) {
                selection.part_ids.push(id.to_string());
            } else if fastener_ids.contains(id) {
                selection.fastener_ids.push(id.to_string());
            } else if grip_ids.contains(id) {
                selection.grip_ids.push(id.to_string());
            }
        }
        selection
    }

    fn selection_flat_ids(selection: &Puzzle5dSelection) -> Vec<String> {
        selection.part_ids.iter().chain(selection.grip_ids.iter()).chain(selection.fastener_ids.iter()).cloned().collect()
    }

    fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
        args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()).filter(|ids| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
    }

    fn remove_parts(document: &mut Puzzle5dDocument, part_ids: &[String]) {
        let removed_grips: Vec<String> = document.parts.iter().filter(|part| part_ids.contains(&part.id)).flat_map(|part| part.grips.iter().map(|grip| puzzle5d_grip_full_id(&part.id, &grip.id))).collect();
        document.parts.retain(|part| !part_ids.contains(&part.id));
        document.fasteners.retain(|fastener| !removed_grips.contains(&fastener.source) && !removed_grips.contains(&fastener.target));
    }

    fn remove_grips(document: &mut Puzzle5dDocument, grip_full_ids: &[String]) {
        if grip_full_ids.is_empty() {
            return;
        }
        for part in &mut document.parts {
            let part_id = part.id.clone();
            part.grips.retain(|grip| !grip_full_ids.contains(&puzzle5d_grip_full_id(&part_id, &grip.id)));
        }
        document.fasteners.retain(|fastener| !grip_full_ids.contains(&fastener.source) && !grip_full_ids.contains(&fastener.target));
    }
    //#endregion 🔖Document

    //#region 🔖Board
    fn board_camera_value(camera: &Puzzle5dCamera2d) -> Value {
        json!({ "x": camera.x, "y": camera.y, "zoom": camera.zoom })
    }

    fn board_node_value(part: &Puzzle5dPart) -> Value {
        let shape = if part.part_2d.shape.is_empty() { "circle" } else { part.part_2d.shape.as_str() };
        let handles: Vec<Value> = part
            .grips
            .iter()
            .map(|grip| {
                json!({
                    "id": puzzle5d_grip_full_id(&part.id, &grip.id),
                    "handleKind": if grip.grip_kind.is_empty() { grip.grip_2d.grip_kind.clone() } else { grip.grip_kind.clone() },
                    "angle": grip.grip_2d.angle,
                    "radius": if grip.grip_2d.radius > 0.0 { grip.grip_2d.radius } else { 3.0 },
                })
            })
            .collect();
        let mut node = json!({
            "id": part.id,
            "nodeKind": part.part_kind,
            "shape": shape,
            "x": part.part_2d.x,
            "y": part.part_2d.y,
            "text": part.part_2d.text,
            "handles": handles,
        });
        if shape == "rectangle" {
            node["width"] = json!(part.part_2d.width.unwrap_or(48.0));
            node["height"] = json!(part.part_2d.height.unwrap_or(48.0));
        } else {
            node["radius"] = json!(if part.part_2d.radius > 0.0 { part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS });
        }
        if let Some(icon) = part.part_2d.icon_kind.as_ref() {
            node["iconKind"] = json!(icon);
        }
        if let Some(hidden) = part.part_2d.hidden {
            node["hidden"] = json!(hidden);
        }
        if let Some(locked) = part.part_2d.locked {
            node["locked"] = json!(locked);
        }
        node
    }

    /// 🗂️ Projects the unified 5d kind bundle (`parts/grips/fasteners/ropes`) to the board's `nodes/handles/edges/wires` naming.
    fn board_kind_catalogs_value(document: &Puzzle5dDocument) -> Value {
        let catalogs = document.kind_catalogs.clone().unwrap_or(json!({}));
        json!({
            "nodes": catalogs.get("parts").cloned().unwrap_or(json!([])),
            "handles": catalogs.get("grips").cloned().unwrap_or(json!([])),
            "edges": catalogs.get("fasteners").cloned().unwrap_or(json!([])),
            "wires": catalogs.get("ropes").cloned().unwrap_or(json!([])),
        })
    }

    fn board_fixture_value(document: &Puzzle5dDocument) -> Value {
        let nodes: Vec<Value> = document.parts.iter().map(board_node_value).collect();
        let edges: Vec<Value> = document
            .fasteners
            .iter()
            .map(|fastener| {
                json!({
                    "id": fastener.id,
                    "edgeKind": fastener.fastener_kind.clone().unwrap_or_else(|| "link".into()),
                    "source": fastener.source,
                    "target": fastener.target,
                })
            })
            .collect();
        json!({
            "schema": PUZZLE5D_BOARD_FIXTURE_SCHEMA,
            "camera": board_camera_value(&document.camera2d),
            "nodes": nodes,
            "edges": edges,
            "wires": [],
            "meta": {
                "kindCatalogs": board_kind_catalogs_value(document),
                "kindCompatibility": document.kind_compatibility.clone().unwrap_or(json!([])),
            },
        })
    }

    fn board_brush_kind_weights_json(runtime: &Puzzle5dRuntime) -> String {
        json!({ "nodeWeights": runtime.object_kind_weights, "handleWeights": runtime.vortex_kind_weights }).to_string()
    }

    fn puzzle5d_board_scene(envelope: &Puzzle5dScene) -> Puzzle2dBoardScene {
        Puzzle2dBoardScene {
            fixture_json: board_fixture_value(&envelope.document).to_string(),
            camera_json: board_camera_value(&envelope.document.camera2d).to_string(),
            kind_catalogs_json: board_kind_catalogs_value(&envelope.document).to_string(),
            selection_json: serde_json::to_string(&selection_flat_ids(&envelope.runtime.selection)).unwrap_or_else(|_| "[]".into()),
            interactive: true,
            hovered_id: envelope.runtime.hovered_part_id.clone(),
            active_tool: Some(puzzle5d_scene_mode(&envelope.active_tool).to_string()),
            selection_method: envelope.runtime.selection_method.clone(),
            grid_snap_enabled: envelope.runtime.grid_snap_enabled,
            grid_factor: envelope.runtime.grid_factor,
            suggestion_offset: envelope.runtime.suggestion_offset,
            brush_kind_weights_json: board_brush_kind_weights_json(&envelope.runtime),
            kind_compatibility_json: envelope.document.kind_compatibility.clone().unwrap_or(json!([])).to_string(),
            lod_mode: envelope.runtime.lod_mode.clone(),
        }
    }

    fn set_part_2d_position(document: &mut Puzzle5dDocument, part_id: &str, x: Option<f64>, y: Option<f64>) {
        if let Some(part) = document.parts.iter_mut().find(|part| part.id == part_id) {
            if let Some(x) = x {
                part.part_2d.x = x;
            }
            if let Some(y) = y {
                part.part_2d.y = y;
            }
        }
    }

    /// 🎨 Palette drop: creates a free paired part at the flat drop point, deriving the volume origin from the nearest peer part's offset.
    fn add_palette_part(envelope: &mut Puzzle5dScene, part_kind: &str, x: f64, y: f64) {
        let flat_to_world = 1.0 / 48.0;
        let origin = envelope
            .document
            .parts
            .first()
            .map(|peer| [peer.part_3d.origin[0] + (x - peer.part_2d.x) * flat_to_world, peer.part_3d.origin[1] - (y - peer.part_2d.y) * flat_to_world, peer.part_3d.origin[2]])
            .unwrap_or([x * flat_to_world, -y * flat_to_world, 0.0]);
        let id = next_part_id();
        let mesh_url = resolve_part_kind_mesh_url(part_kind, envelope.document.kind_catalogs.as_ref());
        let grips = grips_from_templates(&envelope.document, part_kind);
        envelope.document.parts.push(Puzzle5dPart {
            id: id.clone(),
            part_kind: part_kind.into(),
            part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind.into(), icon_kind: None, hidden: None, locked: None },
            part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
            grips,
        });
        envelope.runtime.selection = Puzzle5dSelection { part_ids: vec![id], grip_ids: Vec::new(), fastener_ids: Vec::new() };
    }
    //#endregion 🔖Board

    //#region 🔖Engine
    /// 🧠 Maps the unified 5d kind bundle to the puzzle 3d engine naming (`objects` with `vortices` templates, `vortices`, `cables`).
    fn engine_kind_catalogs_value(document: &Puzzle5dDocument) -> Option<Value> {
        let catalogs = document.kind_catalogs.as_ref()?;
        let objects: Vec<Value> = catalogs
            .get("parts")
            .and_then(|parts| parts.as_array())
            .into_iter()
            .flatten()
            .map(|entry| {
                let mut object = entry.clone();
                let vortices: Vec<Value> = entry
                    .get("grips")
                    .and_then(|grips| grips.as_array())
                    .into_iter()
                    .flatten()
                    .map(|template| {
                        let volume = template.get("3d").cloned().unwrap_or(json!({}));
                        json!({
                            "vortexKind": template.get("gripKind").cloned().unwrap_or(json!("grip")),
                            "position": volume.get("position").cloned().unwrap_or(json!([0.0, 0.0, 0.0])),
                            "direction": volume.get("direction").cloned().unwrap_or(json!([0.0, 0.0, -1.0])),
                            "radius": volume.get("radius").cloned().unwrap_or(json!(0.36)),
                        })
                    })
                    .collect();
                if let Some(object) = object.as_object_mut() {
                    object.remove("grips");
                    object.insert("vortices".into(), json!(vortices));
                }
                object
            })
            .collect();
        Some(json!({
            "objects": objects,
            "vortices": catalogs.get("grips").cloned().unwrap_or(json!([])),
            "cables": catalogs.get("ropes").cloned().unwrap_or(json!([])),
        }))
    }

    fn scene_config_json(envelope: &Puzzle5dScene) -> String {
        let objects: Vec<Value> = envelope
            .document
            .parts
            .iter()
            .map(|part| {
                json!({
                    "id": part.id,
                    "objectKind": part.part_kind,
                    "meshUrl": resolve_part_mesh_url(part, envelope.document.kind_catalogs.as_ref()),
                    "origin": part.part_3d.origin,
                    "orientation": part.part_3d.orientation,
                    "scale": part.part_3d.scale,
                    "vortices": part.grips.iter().map(|grip| json!({
                        "id": grip.id,
                        "vortexKind": if grip.grip_kind.is_empty() { grip.grip_2d.grip_kind.clone() } else { grip.grip_kind.clone() },
                        "position": grip.grip_3d.position,
                        "direction": grip.grip_3d.direction,
                    })).collect::<Vec<_>>(),
                })
            })
            .collect();
        let attractions: Vec<Value> = envelope.document.fasteners.iter().map(|fastener| json!({ "id": fastener.id, "attracting": fastener.source, "attracted": fastener.target })).collect();
        json!({
            "fixture": {
                "objects": objects,
                "attractions": attractions,
                "targetVolumes": [],
            },
            "kindCatalogs": engine_kind_catalogs_value(&envelope.document),
            "kindCompatibility": envelope.document.kind_compatibility.clone().unwrap_or(json!([])),
            "overlapBudget": envelope.runtime.overlap_budget,
            "seed": 1,
            "hostRules": {},
            "weights": {
                "objectWeights": envelope.runtime.object_kind_weights,
                "vortexWeights": envelope.runtime.vortex_kind_weights,
            },
        })
        .to_string()
    }

    /// 🔄 Adopts an engine fixture while preserving flat aspects: existing parts keep `2d`, new parts get a synthesized flat aspect.
    fn merge_engine_fixture(envelope: &Puzzle5dScene, fixture_json: &str) -> Option<Puzzle5dScene> {
        let parsed: Value = serde_json::from_str(fixture_json).ok()?;
        let objects = parsed.get("objects")?.as_array()?;
        let mut next = envelope.clone();
        let existing: HashMap<String, Puzzle5dPart> = envelope.document.parts.iter().map(|part| (part.id.clone(), part.clone())).collect();
        let mut new_ids: Vec<String> = Vec::new();
        next.document.parts = objects
            .iter()
            .filter_map(|object| {
                let id = object.get("id")?.as_str()?.to_string();
                let part_kind = object.get("objectKind").and_then(|value| value.as_str()).unwrap_or("Part").to_string();
                let origin: [f64; 3] = object.get("origin").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]);
                let orientation: Option<[f64; 4]> = object.get("orientation").and_then(|value| serde_json::from_value(value.clone()).ok());
                let mesh_url = object.get("meshUrl").and_then(|value| value.as_str()).map(str::to_string);
                let scale = object.get("scale").cloned().filter(|value| !value.is_null());
                if let Some(previous) = existing.get(&id) {
                    let mut part = previous.clone();
                    part.part_kind = part_kind;
                    part.part_3d.origin = origin;
                    part.part_3d.orientation = orientation.or(part.part_3d.orientation);
                    part.part_3d.mesh_url = mesh_url.or(part.part_3d.mesh_url.clone());
                    if scale.is_some() {
                        part.part_3d.scale = scale;
                    }
                    return Some(part);
                }
                let templates = grips_from_templates(&envelope.document, &part_kind);
                let grips: Vec<Puzzle5dGrip> = object
                    .get("vortices")
                    .and_then(|value| value.as_array())
                    .into_iter()
                    .flatten()
                    .enumerate()
                    .map(|(index, vortex)| {
                        let template = templates.get(index);
                        Puzzle5dGrip {
                            id: vortex.get("id").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(|| format!("v{index}")),
                            grip_kind: vortex.get("vortexKind").and_then(|value| value.as_str()).map(str::to_string).or_else(|| template.map(|t| t.grip_kind.clone())).unwrap_or_else(|| "grip".into()),
                            grip_2d: template.map(|t| t.grip_2d.clone()).unwrap_or_default(),
                            grip_3d: Puzzle5dGrip3d {
                                position: vortex.get("position").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]),
                                direction: vortex.get("direction").and_then(|value| serde_json::from_value(value.clone()).ok()),
                                radius: vortex.get("radius").and_then(|value| value.as_f64()).unwrap_or(0.36),
                                label: vortex.get("label").and_then(|value| value.as_str()).map(str::to_string),
                            },
                        }
                    })
                    .collect();
                let grips = if grips.is_empty() { templates } else { grips };
                new_ids.push(id.clone());
                Some(Puzzle5dPart {
                    id,
                    part_kind: part_kind.clone(),
                    part_2d: Puzzle5dPart2d { x: 0.0, y: 0.0, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind, icon_kind: None, hidden: None, locked: None },
                    part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: orientation.or(Some([0.0, 0.0, 0.0, 1.0])), scale, label: None },
                    grips,
                })
            })
            .collect();
        let existing_kinds: HashMap<String, Option<String>> = envelope.document.fasteners.iter().map(|fastener| (fastener.id.clone(), fastener.fastener_kind.clone())).collect();
        next.document.fasteners = parsed
            .get("attractions")
            .and_then(|value| value.as_array())
            .into_iter()
            .flatten()
            .filter_map(|attraction| {
                let id = attraction.get("id").and_then(|value| value.as_str()).unwrap_or("fastener").to_string();
                Some(Puzzle5dFastener {
                    fastener_kind: existing_kinds.get(&id).cloned().flatten().or_else(|| attraction.get("attractionKind").and_then(|value| value.as_str()).map(str::to_string)),
                    source: attraction.get("attracting")?.as_str()?.to_string(),
                    target: attraction.get("attracted")?.as_str()?.to_string(),
                    id,
                })
            })
            .collect();
        synthesize_flat_for_new_parts(&mut next.document, &new_ids);
        Some(next)
    }

    /// 🌤️ Places flat centers for freshly-adopted parts next to their fastened neighbor, walking chains until every new part is placed.
    fn synthesize_flat_for_new_parts(document: &mut Puzzle5dDocument, new_ids: &[String]) {
        let mut pending: HashSet<String> = new_ids.iter().cloned().collect();
        for _ in 0..=new_ids.len() {
            if pending.is_empty() {
                break;
            }
            let mut placed: Vec<(String, f64, f64)> = Vec::new();
            for fastener in &document.fasteners {
                for (own, other) in [(&fastener.source, &fastener.target), (&fastener.target, &fastener.source)] {
                    let Some((own_part, _)) = find_part_by_grip_full_id(document, own) else {
                        continue;
                    };
                    if !pending.contains(&own_part.id) {
                        continue;
                    }
                    let Some((other_part, other_grip)) = find_part_by_grip_full_id(document, other) else {
                        continue;
                    };
                    if pending.contains(&other_part.id) {
                        continue;
                    }
                    let angle = other_grip.grip_2d.angle;
                    let own_radius = if own_part.part_2d.radius > 0.0 { own_part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS };
                    let other_radius = if other_part.part_2d.radius > 0.0 { other_part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS };
                    let distance = own_radius + other_radius + PUZZLE5D_BOARD_PLACEMENT_GAP;
                    placed.push((own_part.id.clone(), other_part.part_2d.x + angle.cos() * distance, other_part.part_2d.y + angle.sin() * distance));
                }
            }
            if placed.is_empty() {
                break;
            }
            for (id, x, y) in placed {
                set_part_2d_position(document, &id, Some(x), Some(y));
                pending.remove(&id);
            }
        }
        let mut column = 0usize;
        for id in pending {
            set_part_2d_position(document, &id, Some(120.0 + column as f64 * 56.0), Some(120.0));
            column += 1;
        }
    }
    //#endregion 🔖Engine

    //#region 🔖World
    fn world_instances_json(document: &Puzzle5dDocument, runtime: &Puzzle5dRuntime) -> String {
        let instances: Vec<Value> = document
            .parts
            .iter()
            .map(|part| {
                let selected = runtime.selection.part_ids.contains(&part.id);
                let hovered = runtime.hovered_part_id.as_deref() == Some(part.id.as_str());
                let mesh_id = resolve_part_mesh_url(part, document.kind_catalogs.as_ref()).map(|url| world3d_mesh_id_from_url(&url)).unwrap_or_else(|| PUZZLE5D_FALLBACK_MESH_KIND.into());
                json!({
                    "id": part.id,
                    "meshId": mesh_id,
                    "position": part.part_3d.origin,
                    "rotation": part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                    "scale": part_scale_json(part),
                    "label": part.part_3d.label.clone().unwrap_or_else(|| part.part_kind.clone()),
                    "selected": selected,
                    "hovered": hovered,
                })
            })
            .collect();
        serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
    }

    fn part_scale_json(part: &Puzzle5dPart) -> [f64; 3] {
        match &part.part_3d.scale {
            Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
            Some(Value::Number(value)) => {
                let factor = value.as_f64().unwrap_or(1.0);
                [factor, factor, factor]
            }
            _ => [1.0, 1.0, 1.0],
        }
    }

    fn world_meshes_json(document: &Puzzle5dDocument) -> String {
        world3d_meshes_json_from_urls(&collect_mesh_urls(document))
    }

    fn grip_color(kind_catalogs: Option<&Value>, grip_kind: &str) -> String {
        kind_catalogs
            .and_then(|catalogs| catalogs.get("grips"))
            .and_then(|value| value.as_array())
            .and_then(|entries| entries.iter().find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(grip_kind)))
            .and_then(|entry| entry.get("color").and_then(|value| value.as_str()).map(str::to_string))
            .unwrap_or_else(|| "#38bdf8".into())
    }

    fn world_grips_json(document: &Puzzle5dDocument) -> String {
        let mut records = Vec::new();
        for part in &document.parts {
            for grip in &part.grips {
                records.push(json!({
                    "fullId": puzzle5d_grip_full_id(&part.id, &grip.id),
                    "objectId": part.id,
                    "vortexKind": grip.grip_kind,
                    "position": world_grip_position(part, grip),
                    "direction": world_grip_direction(part, grip),
                    "radius": grip.grip_3d.radius.max(0.36),
                    "color": grip_color(document.kind_catalogs.as_ref(), &grip.grip_kind),
                }));
            }
        }
        serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
    }

    fn world_fasteners_json(document: &Puzzle5dDocument) -> String {
        let records: Vec<Value> = document
            .fasteners
            .iter()
            .filter_map(|fastener| {
                let from = resolve_grip_world_position(document, &fastener.source)?;
                let to = resolve_grip_world_position(document, &fastener.target)?;
                Some(json!({ "id": fastener.id, "from": from, "to": to, "color": "#60a5fa" }))
            })
            .collect();
        serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
    }

    fn gumball_target_world(envelope: &Puzzle5dScene) -> Option<[f64; 3]> {
        let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| envelope.runtime.selection.part_ids.contains(&part.id)).collect();
        if selected.is_empty() {
            return None;
        }
        let mut sum = [0.0, 0.0, 0.0];
        for part in &selected {
            sum[0] += part.part_3d.origin[0];
            sum[1] += part.part_3d.origin[1];
            sum[2] += part.part_3d.origin[2];
        }
        let count = selected.len() as f64;
        Some([sum[0] / count, sum[1] / count, sum[2] / count])
    }

    /// 🎯 Base selection JSON augmented with the mesh granularity, transform tool, and gumball fields the world-3d host reads.
    fn world_selection_json_ex(envelope: &Puzzle5dScene) -> String {
        let runtime = &envelope.runtime;
        let mut value: Value = serde_json::from_str(&world3d_selection_json(&runtime.selection_method, &runtime.selection.part_ids, runtime.hovered_part_id.as_deref())).unwrap_or_else(|_| json!({}));
        if let Some(object) = value.as_object_mut() {
            object.insert("granularity".into(), json!("mesh"));
            object.insert("selectionMode".into(), json!("mesh"));
            object.insert("targets".into(), json!({ "mesh": true, "vertex": false, "edge": false, "face": false }));
            object.insert("transformTool".into(), json!(puzzle5d_transform_handle(&envelope.active_tool)));
            if let Some(active_id) = runtime.selection.part_ids.first() {
                object.insert("activeObjectId".into(), json!(active_id));
            }
            let gumball_active = !runtime.selection.part_ids.is_empty();
            object.insert("gumballActive".into(), json!(gumball_active));
            if gumball_active {
                if let Some(target) = gumball_target_world(envelope) {
                    object.insert("gumballTarget".into(), json!(target));
                }
            }
        }
        value.to_string()
    }

    fn world_interaction_json(runtime: &Puzzle5dRuntime, active_tool: &str) -> String {
        json!({
            "activeTool": puzzle5d_scene_mode(active_tool),
            "brushCandidateIndex": runtime.brush_candidate_index,
            "fillCount": runtime.fill_count,
            "hoveredVortexFullId": runtime.selection.grip_ids.first().cloned(),
        })
        .to_string()
    }

    fn puzzle5d_context_menu_json(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> Option<String> {
        if envelope.runtime.selection.part_ids.is_empty() {
            return None;
        }
        let items = vec![
            json!({ "id": "duplicate", "label": labels.duplicate, "action": "duplicateSelection" }),
            json!({ "id": "select-same-kind", "label": labels.select_same_kind, "action": "selectSameKindSelection" }),
            json!({ "id": "zoom", "label": labels.zoom_to_selection, "action": "zoomToSelection" }),
            json!({ "id": "delete", "label": labels.delete, "action": "deleteSelection" }),
        ];
        serde_json::to_string(&items).ok()
    }

    fn camera3d_json(camera: &Puzzle5dCamera3d) -> String {
        json!({ "position": camera.position, "target": camera.target, "zoom": camera.zoom, "fov": 45.0 }).to_string()
    }
    //#endregion 🔖World

    //#region 🔖Brush
    fn puzzle5d_brush_target_grip(envelope: &Puzzle5dScene) -> Option<String> {
        envelope.runtime.selection.grip_ids.first().cloned().or_else(|| {
            let part_id = envelope.runtime.hovered_part_id.as_deref().or_else(|| envelope.runtime.selection.part_ids.first().map(String::as_str))?;
            let part = envelope.document.parts.iter().find(|part| part.id == part_id)?;
            let grip = part.grips.first()?;
            Some(puzzle5d_grip_full_id(&part.id, &grip.id))
        })
    }

    fn parse_brush_candidates_free(raw: &str) -> Vec<Value> {
        let parsed: Value = serde_json::from_str(raw).unwrap_or(Value::Null);
        parsed.get("free").and_then(|value| value.as_array()).cloned().unwrap_or_default()
    }

    fn world_brush_preview_json(session: &Puzzle5dPrecomputeSession, envelope: &Puzzle5dScene) -> Option<String> {
        if envelope.active_tool != "brush" {
            return None;
        }
        let full_id = puzzle5d_brush_target_grip(envelope)?;
        session.brush_preview_json(&full_id, envelope.runtime.brush_candidate_index)
    }
    //#endregion 🔖Brush

    //#region 🔖Engagement
    /// 🧰 The select/brush/fill switcher lives in the framework toolbar (declared via `.tool` +
    /// `.window_kind_tools`); the fill-count slider and brush placement picker now live as tagged
    /// [`WindowMeasure::Group`]s in [`puzzle5d_window_measures`] (surfaced by [`partition_window_measures`]
    /// in the dedicated "Tool Options" rail only while their tool is active), so the engagement HUD is a
    /// bare command input plus a status line.
    fn puzzle5d_engagement(envelope: &Puzzle5dScene, window: &str) -> WindowEngagement {
        let part_count = envelope.document.parts.len();
        let fastener_count = envelope.document.fasteners.len();
        let active_tool = envelope.active_tool.as_str();
        let input_value = envelope.runtime.engagement_input_by_window.get(window).cloned().unwrap_or_default();
        let placeholder = match active_tool {
            "fill" => "Fill",
            "brush" => "Brush",
            _ => "select, brush, fill, clear",
        };
        WindowEngagement {
            session_active: Some(active_tool != "select"),
            input: Some(WindowEngagementInput {
                id: Some(format!("puzzle5d-engagement-{window}")),
                value: Some(input_value),
                placeholder: Some(placeholder.into()),
                disabled: None,
                on_change: Some(puzzle5d_action("engagementInput", Some(json!({ "window": window })))),
                on_submit: Some(puzzle5d_action("engagementSubmit", Some(json!({ "window": window })))),
                on_repeat_last: None,
                on_abort: Some(puzzle5d_action("engagementAbort", Some(json!({ "window": window })))),
            }),
            control: None,
            controls: None,
            status: Some(vec![WindowEngagementStatus { id: format!("puzzle5d-status-{window}"), text: format!("{part_count} parts · {fastener_count} fasteners · tool {}", active_tool) }]),
            options: None,
            possible_engagements: None,
        }
    }
    //#endregion 🔖Engagement

    //#region 🔖Measures
    fn puzzle5d_lod_tier_ids() -> Vec<String> {
        serde_json::from_str::<Vec<Value>>(&puzzle_2d::puzzle_2d_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
    }

    fn puzzle5d_kind_ids(document: &Puzzle5dDocument, slice: &str) -> Vec<String> {
        let mut ids: Vec<String> = document
            .kind_catalogs
            .as_ref()
            .and_then(|catalogs| catalogs.get(slice))
            .and_then(|value| value.as_array())
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string))
            .collect();
        if ids.is_empty() {
            let mut inferred: Vec<String> = match slice {
                "parts" => document.parts.iter().map(|part| part.part_kind.clone()).collect(),
                "grips" => document.parts.iter().flat_map(|part| part.grips.iter().map(|grip| grip.grip_kind.clone())).collect(),
                _ => Vec::new(),
            };
            inferred.sort();
            inferred.dedup();
            ids = inferred;
        }
        ids
    }

    fn puzzle5d_lod_measure(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
        let mut items = vec![MeasureSelectItem { id: PUZZLE5D_LOD_MODE_AUTOMATIC.into(), value: PUZZLE5D_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() }];
        items.extend(puzzle5d_lod_tier_ids().into_iter().map(|tier| MeasureSelectItem { id: tier.clone(), value: tier.clone(), label: tier }));
        WindowMeasure::Select { id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-lod"), label: Some(labels.lod.into()), value: runtime.lod_mode.clone(), items, on_change: puzzle5d_action("setLodMode", None) }
    }

    fn puzzle5d_kind_weight_measures(prefix: &str, action: &str, ids: &[String], weights: &HashMap<String, f64>) -> Vec<WindowMeasure> {
        ids.iter()
            .map(|kind_id| {
                let weight = weights.get(kind_id).copied().unwrap_or(0.0);
                WindowMeasure::Slider {
                    id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-{prefix}-{kind_id}"),
                    label: Some(format!("{kind_id} {:.0}%", weight * 100.0)),
                    value: weight,
                    min: 0.0,
                    max: 1.0,
                    step: Some(0.01),
                    on_change: puzzle5d_action(action, Some(json!({ "kindId": kind_id }))),
                }
            })
            .collect()
    }

    fn puzzle5d_suggestion_measures_group(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowMeasure {
        let part_ids = puzzle5d_kind_ids(&envelope.document, "parts");
        let grip_ids = puzzle5d_kind_ids(&envelope.document, "grips");
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion"),
            label: labels.suggestion.into(),
            default_open: Some(false),
            active_tool_id: None,
            children: vec![
                WindowMeasure::Slider {
                    id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-offset"),
                    label: Some(labels.offset.into()),
                    value: envelope.runtime.suggestion_offset,
                    min: PUZZLE5D_SUGGESTION_OFFSET_MIN,
                    max: PUZZLE5D_SUGGESTION_OFFSET_MAX,
                    step: Some(PUZZLE5D_SUGGESTION_OFFSET_STEP),
                    on_change: puzzle5d_action("setSuggestionOffset", None),
                },
                WindowMeasure::Group {
                    id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-parts"),
                    label: labels.part_weights.into(),
                    default_open: Some(false),
                    active_tool_id: None,
                    children: puzzle5d_kind_weight_measures("part-kind", "setObjectKindWeight", &part_ids, &envelope.runtime.object_kind_weights),
                },
                WindowMeasure::Group {
                    id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-grips"),
                    label: labels.grip_weights.into(),
                    default_open: Some(false),
                    active_tool_id: None,
                    children: puzzle5d_kind_weight_measures("grip-kind", "setVortexKindWeight", &grip_ids, &envelope.runtime.vortex_kind_weights),
                },
            ],
        }
    }

    fn puzzle5d_brush_measures_group(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowMeasure {
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-brush"),
            label: labels.brush.into(),
            default_open: Some(false),
            active_tool_id: None,
            children: vec![WindowMeasure::Slider {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-brush-overlap"),
                label: Some(labels.overlap.into()),
                value: envelope.runtime.overlap_budget,
                min: 0.0,
                max: 0.2,
                step: Some(0.005),
                on_change: puzzle5d_action("setBrushPlacementOverlapBudget", None),
            }],
        }
    }

    /// 🪣 Fill-count slider measure — the fill-tool's core parameter, mirrors the retired
    /// `puzzle5d_fill_count_control` (`setFillCount` reads `count`-or-`value`, so the slider's `{value}`
    /// payload preserves the action semantics).
    fn puzzle5d_fill_count_measure(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowMeasure {
        WindowMeasure::Slider {
            id: "puzzle5d-fill-count".into(),
            label: Some(format!("{} {}", labels.fill, envelope.runtime.fill_count)),
            value: envelope.runtime.fill_count as f64,
            min: 0.0,
            max: PUZZLE5D_FILL_COUNT_MAX as f64,
            step: Some(1.0),
            on_change: puzzle5d_action("setFillCount", None),
        }
    }

    /// 🪣 Tool Options group for the Fill tool — the fill-count slider, tagged `Some("fill")` so
    /// [`partition_window_measures`] surfaces it in the Tool Options rail only while the Fill tool is active.
    fn puzzle5d_fill_tool_options(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowMeasure {
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-tool-options-fill"),
            label: labels.fill.into(),
            default_open: Some(true),
            active_tool_id: Some("fill".into()),
            children: vec![puzzle5d_fill_count_measure(envelope, labels)],
        }
    }

    /// 🖌️ Tool Options group for the Brush tool — the placement candidate picker (`ToggleGroup`→`Select`,
    /// both "pick one of several string values"; `engagementControlSelect` reads `id`-or-`value` so semantics
    /// hold). Tagged `Some("brush")`; `None` when there are no candidates to place, matching the old gate.
    fn puzzle5d_brush_tool_options(envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> Option<WindowMeasure> {
        let target = puzzle5d_brush_target_grip(envelope)?;
        let candidates = parse_brush_candidates_free(&precompute.brush_candidates(&target));
        if candidates.is_empty() {
            return None;
        }
        let items: Vec<MeasureSelectItem> = candidates
            .iter()
            .enumerate()
            .map(|(index, candidate)| {
                let label = candidate.get("objectKind").and_then(|value| value.as_str()).or_else(|| candidate.get("objectKindId").and_then(|value| value.as_str())).unwrap_or("kind");
                let id = format!("puzzle5d.brush.candidate.{index}");
                MeasureSelectItem { id: id.clone(), value: id, label: label.into() }
            })
            .collect();
        let selected_index = envelope.runtime.brush_candidate_index.min(items.len().saturating_sub(1));
        Some(WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-tool-options-brush"),
            label: labels.placement.into(),
            default_open: Some(true),
            active_tool_id: Some("brush".into()),
            children: vec![WindowMeasure::Select {
                id: "puzzle5d-brush-placement".into(),
                label: Some(labels.placement.into()),
                value: format!("puzzle5d.brush.candidate.{selected_index}"),
                items,
                on_change: puzzle5d_action("engagementControlSelect", None),
            }],
        })
    }

    fn puzzle5d_window_measures(window: &str, envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
        let mut measures = if window == PUZZLE5D_PLAY_WINDOW_2D {
            vec![puzzle5d_lod_measure(&envelope.runtime, labels), puzzle5d_suggestion_measures_group(envelope, labels), puzzle5d_brush_measures_group(envelope, labels)]
        } else {
            vec![
                puzzle5d_suggestion_measures_group(envelope, labels),
                puzzle5d_brush_measures_group(envelope, labels),
                world3d_sun_measures("puzzle5d", &envelope.runtime.sun, puzzle5d_action),
            ]
        };
        measures.push(puzzle5d_fill_tool_options(envelope, labels));
        if let Some(brush) = puzzle5d_brush_tool_options(envelope, precompute, labels) {
            measures.push(brush);
        }
        measures
    }
    //#endregion 🔖Measures

    //#region 🔖Panels
    fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
        let mut item = UiTreeItemNode::base(id, label);
        item.icon_id = icon_id.map(str::to_string);
        item.action = Some(action);
        item
    }

    fn tree_info_item(id: impl Into<String>, label: impl Into<String>, description: Option<String>) -> UiTreeItemNode {
        let mut item = UiTreeItemNode::base(id, label);
        item.description = description;
        item
    }

    fn part_label(part: &Puzzle5dPart) -> String {
        if !part.part_2d.text.is_empty() {
            return part.part_2d.text.clone();
        }
        part.part_3d.label.clone().unwrap_or_else(|| part.part_kind.clone())
    }

    fn fastener_label(document: &Puzzle5dDocument, fastener: &Puzzle5dFastener) -> String {
        let side = |full_id: &str| find_part_by_grip_full_id(document, full_id).map(|(part, _)| part_label(part)).unwrap_or_else(|| full_id.to_string());
        format!("{} → {}", side(&fastener.source), side(&fastener.target))
    }

    fn document_tree_selected_ids(envelope: &Puzzle5dScene) -> Vec<String> {
        let selection = &envelope.runtime.selection;
        selection
            .part_ids
            .iter()
            .map(|id| format!("puzzle5d-play-document.part.{id}"))
            .chain(selection.grip_ids.iter().map(|id| format!("puzzle5d-play-document.grip.{id}")))
            .chain(selection.fastener_ids.iter().map(|id| format!("puzzle5d-play-document.fastener.{id}")))
            .collect()
    }

    fn build_document_tree(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiNode {
        let part_items: Vec<UiTreeItemNode> = envelope
            .document
            .parts
            .iter()
            .map(|part| {
                let grip_items: Vec<UiTreeItemNode> = part
                    .grips
                    .iter()
                    .map(|grip| {
                        let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
                        tree_item_with_action(format!("puzzle5d-play-document.grip.{full_id}"), format!("{} ({})", grip.id, grip.grip_kind), Some("circle-dot"), puzzle5d_action("setSelection", Some(json!({ "gripIds": [full_id] }))))
                    })
                    .collect();
                let mut item = tree_item_with_action(format!("puzzle5d-play-document.part.{}", part.id), part_label(part), Some("box"), puzzle5d_action("setSelection", Some(json!({ "partIds": [part.id] }))));
                item.description = Some(part.part_kind.clone());
                if !grip_items.is_empty() {
                    item.items = Some(grip_items);
                }
                item
            })
            .collect();
        let fastener_items: Vec<UiTreeItemNode> = envelope
            .document
            .fasteners
            .iter()
            .map(|fastener| tree_item_with_action(format!("puzzle5d-play-document.fastener.{}", fastener.id), fastener_label(&envelope.document, fastener), Some("link"), puzzle5d_action("setSelection", Some(json!({ "fastenerIds": [fastener.id] })))))
            .collect();
        UiNode::Tree(UiTreeNode {
            loading: None,
            sections: vec![
                UiTreeSectionNode {
                    loading: None,
                    id: "puzzle5d-play-document.parts".into(),
                    label: Some(labels.parts.into()),
                    default_open: Some(true),
                    items: if part_items.is_empty() { vec![tree_info_item("puzzle5d-play-document.parts.empty", "(none)", None)] } else { part_items },
                },
                UiTreeSectionNode {
                    loading: None,
                    id: "puzzle5d-play-document.fasteners".into(),
                    label: Some(labels.fasteners.into()),
                    default_open: Some(false),
                    items: if fastener_items.is_empty() { vec![tree_info_item("puzzle5d-play-document.fasteners.empty", "(none)", None)] } else { fastener_items },
                },
            ],
            selected_ids: Some(document_tree_selected_ids(envelope)),
            highlighted_ids: None,
            selection_change: Some(puzzle5d_action("setSelection", None)),
            drop_action: None,
        })
    }

    fn catalog_kind_label(entry: &Value) -> String {
        entry
            .get("label")
            .and_then(|value| value.as_str())
            .filter(|value| !value.is_empty())
            .or_else(|| entry.get("name").and_then(|value| value.as_str()).filter(|value| !value.is_empty()))
            .or_else(|| entry.get("id").and_then(|value| value.as_str()))
            .unwrap_or("kind")
            .into()
    }

    /// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx) reads to auto-wire catalogue drag sources.
    const PUZZLE5D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

    fn puzzle5d_catalog_item_drag_data(kind_id: &str, entry: &Value) -> HashMap<String, String> {
        let mut payload = json!({ "kindId": kind_id, "catalogSlice": "nodes" });
        if let Some(object) = payload.as_object_mut() {
            for key in ["shape", "radius", "width", "height", "iconKind"] {
                if let Some(value) = entry.get(key) {
                    object.insert(key.into(), value.clone());
                }
            }
        }
        HashMap::from([(PUZZLE5D_CATALOGUE_DRAG_MIME.to_string(), payload.to_string())])
    }

    fn kind_catalog_section(section_id: &str, label: &str, entries: &[Value], add_action: Option<&str>) -> UiTreeSectionNode {
        let items: Vec<UiTreeItemNode> = entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
                match add_action {
                    Some(action) => {
                        let mut item = tree_item_with_action(format!("{section_id}.{index}.{kind_id}"), catalog_kind_label(entry), Some("box"), puzzle5d_action(action, Some(json!({ "partKind": kind_id }))));
                        item.description = Some(kind_id.into());
                        item.draggable = Some(true);
                        item.drag_data = Some(puzzle5d_catalog_item_drag_data(kind_id, entry));
                        item
                    }
                    None => tree_info_item(format!("{section_id}.{index}.{kind_id}"), catalog_kind_label(entry), Some(kind_id.into())),
                }
            })
            .collect();
        UiTreeSectionNode {
            loading: None,
            id: section_id.into(),
            label: Some(label.into()),
            default_open: Some(!items.is_empty()),
            items: if items.is_empty() { vec![tree_info_item(format!("{section_id}.empty"), "(none)", None)] } else { items },
        }
    }

    fn build_kinds_tree(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiNode {
        let catalogs = envelope.document.kind_catalogs.clone().unwrap_or(json!({}));
        let slice = |key: &str| catalogs.get(key).and_then(|value| value.as_array()).cloned().unwrap_or_default();
        let mut part_entries = slice("parts");
        if part_entries.is_empty() {
            let mut ids: Vec<String> = envelope.document.parts.iter().map(|part| part.part_kind.clone()).collect();
            ids.sort();
            ids.dedup();
            part_entries = ids.into_iter().map(|id| json!({ "id": id, "name": id })).collect();
        }
        UiNode::Tree(UiTreeNode {
            loading: None,
            sections: vec![
                kind_catalog_section("puzzle5d-play-kinds.parts", labels.parts, &part_entries, Some("addPartKind")),
                kind_catalog_section("puzzle5d-play-kinds.grips", labels.grips, &slice("grips"), None),
                kind_catalog_section("puzzle5d-play-kinds.fasteners", labels.fasteners, &slice("fasteners"), None),
                kind_catalog_section("puzzle5d-play-kinds.ropes", labels.ropes, &slice("ropes"), None),
            ],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        })
    }

    fn inspector_text_field(id: &str, label: &str, value: String, action: ActionDescriptor) -> UiNode {
        UiNode::Field(UiFieldNode {
            id: id.into(),
            label: label.into(),
            description: None,
            required: None,
            error: None,
            child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                id: format!("{id}.input"),
                input_kind: "text".into(),
                value,
                placeholder: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                on_change: action,
            })),
        })
    }

    fn build_part_inspector(part: &Puzzle5dPart, labels: &Puzzle5dLabels) -> UiNode {
        let origin = part.part_3d.origin;
        ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
            id: "puzzle5d-play-inspector.part".into(),
            label: labels.part.into(),
            default_open: None,
            fields: vec![
                ui_inspector_readonly_field("puzzle5d-play-inspector.part.id", "Id", &part.id),
                inspector_text_field("puzzle5d-play-inspector.part.kind", "Kind", part.part_kind.clone(), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "partKind" })))),
                inspector_text_field("puzzle5d-play-inspector.part.label", "Label", part.part_3d.label.clone().unwrap_or_default(), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "label" })))),
                inspector_text_field("puzzle5d-play-inspector.part.text", "Flat text", part.part_2d.text.clone(), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "text" })))),
                inspector_text_field("puzzle5d-play-inspector.part.x", "Flat x", format!("{}", part.part_2d.x), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "x" })))),
                inspector_text_field("puzzle5d-play-inspector.part.y", "Flat y", format!("{}", part.part_2d.y), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "y" })))),
                inspector_text_field("puzzle5d-play-inspector.part.origin", "Volume origin", format!("{:.3}, {:.3}, {:.3}", origin[0], origin[1], origin[2]), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "origin" })))),
            ],
        }])
    }

    fn build_grip_inspector(part: &Puzzle5dPart, grip: &Puzzle5dGrip, labels: &Puzzle5dLabels) -> UiNode {
        let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
        let position = grip.grip_3d.position;
        let direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
        ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
            id: "puzzle5d-play-inspector.grip".into(),
            label: labels.grip.into(),
            default_open: None,
            fields: vec![
                ui_inspector_readonly_field("puzzle5d-play-inspector.grip.id", "Id", &full_id),
                inspector_text_field("puzzle5d-play-inspector.grip.kind", "Kind", grip.grip_kind.clone(), puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": "gripKind" })))),
                inspector_text_field("puzzle5d-play-inspector.grip.angle", "Flat angle", format!("{}", grip.grip_2d.angle), puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": "angle" })))),
                inspector_text_field("puzzle5d-play-inspector.grip.radius", "Radius", format!("{}", grip.grip_3d.radius), puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": "radius" })))),
                inspector_text_field("puzzle5d-play-inspector.grip.position", "Position", format!("{:.3}, {:.3}, {:.3}", position[0], position[1], position[2]), puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": "position" })))),
                inspector_text_field("puzzle5d-play-inspector.grip.direction", "Direction", format!("{:.3}, {:.3}, {:.3}", direction[0], direction[1], direction[2]), puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": "direction" })))),
            ],
        }])
    }

    fn build_inspector_tree(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiNode {
        if let Some(grip_full_id) = envelope.runtime.selection.grip_ids.first() {
            if let Some((part, grip)) = find_part_by_grip_full_id(&envelope.document, grip_full_id) {
                return build_grip_inspector(part, grip, labels);
            }
        }
        if let Some(part_id) = envelope.runtime.selection.part_ids.first() {
            if let Some(part) = envelope.document.parts.iter().find(|entry| &entry.id == part_id) {
                return build_part_inspector(part, labels);
            }
        }
        if let Some(fastener_id) = envelope.runtime.selection.fastener_ids.first() {
            if let Some(fastener) = envelope.document.fasteners.iter().find(|entry| &entry.id == fastener_id) {
                return ui_stack_vertical(vec![
                    ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.id", "Id", &fastener.id),
                    ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.source", "Source", &fastener.source),
                    ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.target", "Target", &fastener.target),
                    ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.kind", "Kind", fastener.fastener_kind.as_deref().unwrap_or("link")),
                ]);
            }
        }
        ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", envelope.document.schema)),
            ui_text(format!("Parts: {}", envelope.document.parts.len())),
            ui_text(format!("Fasteners: {}", envelope.document.fasteners.len())),
            ui_text(format!("Tool: {}", envelope.active_tool)),
        ])
    }
    //#endregion 🔖Panels

    //#region 🔖Puzzle5dPlayApp
    /// 🧩 Puzzle-5d play app. Owns the precompute engine, the registered-mesh cache, and the ephemeral
    /// view `runtime`; the persisted document (bare `Puzzle5dDocument` json) lives in the wrapping
    /// `VcsDocumentApp`. Each action mutates a transient {@link Puzzle5dScene}, then emits the granular
    /// op delta. Undo/redo/checkpoints are handled by the wrapper.
    pub struct Puzzle5dPlayApp {
        precompute: Puzzle5dPrecomputeSession,
        registered_mesh_urls: HashSet<String>,
        runtime: Puzzle5dRuntime,
    }

    impl Default for Puzzle5dPlayApp {
        fn default() -> Self {
            Self { precompute: Puzzle5dPrecomputeSession::new(), registered_mesh_urls: HashSet::new(), runtime: Puzzle5dRuntime::default() }
        }
    }

    impl Puzzle5dPlayApp {
        fn drive_precompute(&mut self, envelope: &Puzzle5dScene) {
            let _ = self.precompute.set_scene(&scene_config_json(envelope));
            // 🧊 Guarded by `has_mesh` (mirrors the puzzle3d path): `register_mesh` now invalidates the
            // precompute cache, so re-registering the same fallback body on every drive would wipe
            // suggestion/fill progress every call and defeat `set_scene`'s idempotence above.
            if !self.precompute.has_mesh(PUZZLE5D_FALLBACK_MESH_KIND) {
                let fallback = semio_framework_plugin::mesh_from_kind(PUZZLE5D_FALLBACK_MESH_KIND);
                self.precompute.register_mesh(PUZZLE5D_FALLBACK_MESH_KIND, &fallback.positions, &fallback.indices);
            }
            for url in collect_mesh_urls(&envelope.document) {
                if !self.registered_mesh_urls.contains(&url) && !self.precompute.has_mesh(&url) {
                    let fallback = semio_framework_plugin::mesh_from_kind(PUZZLE5D_FALLBACK_MESH_KIND);
                    self.precompute.register_mesh(&url, &fallback.positions, &fallback.indices);
                }
            }
            let _ = self.precompute.precompute_step(8);
        }

        fn apply_engine_brush_placement(&mut self, envelope: &Puzzle5dScene, payload: &Value) -> Option<Puzzle5dScene> {
            let brush_payload = serde_json::from_value::<BrushPlacePayload>(payload.clone()).ok()?;
            let fixture_json = self.precompute.apply_brush_placement_rust(&serde_json::to_string(&brush_payload).ok()?).ok()?;
            merge_engine_fixture(envelope, &fixture_json)
        }

        /// 🖌️ Paired placement for a board `brushPlace` event: the engine picks the volume pose for the flat payload's kind, both aspects land in one part.
        fn apply_board_brush_place(&mut self, envelope: &mut Puzzle5dScene, payload: &Value) {
            self.drive_precompute(envelope);
            let node_kind = payload.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("Part").to_string();
            let source_grip = payload.get("sourceHandleId").and_then(|value| value.as_str()).map(str::to_string).or_else(|| puzzle5d_brush_target_grip(envelope));
            if let Some(source_grip) = source_grip.as_ref() {
                let candidates = parse_brush_candidates_free(&self.precompute.brush_candidates(source_grip));
                let candidate_index = candidates
                    .iter()
                    .position(|candidate| candidate.get("objectKindId").or_else(|| candidate.get("objectKind")).and_then(|value| value.as_str()) == Some(node_kind.as_str()))
                    .unwrap_or(envelope.runtime.brush_candidate_index);
                let engine_payload = json!({ "objectKindId": node_kind, "targetVortexFullId": source_grip, "candidateIndex": candidate_index });
                if let Some(mut next) = self.apply_engine_brush_placement(envelope, &engine_payload) {
                    let previous_ids: HashSet<String> = envelope.document.parts.iter().map(|part| part.id.clone()).collect();
                    let new_id = next.document.parts.iter().map(|part| part.id.clone()).find(|id| !previous_ids.contains(id));
                    if let Some(new_id) = new_id {
                        let x = payload.get("x").and_then(|value| value.as_f64());
                        let y = payload.get("y").and_then(|value| value.as_f64());
                        set_part_2d_position(&mut next.document, &new_id, x, y);
                        next.runtime.selection = Puzzle5dSelection { part_ids: vec![new_id], grip_ids: Vec::new(), fastener_ids: Vec::new() };
                    }
                    *envelope = next;
                    return;
                }
            }
            let id = payload.get("nodeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(next_part_id);
            let x = payload.get("x").and_then(|value| value.as_f64()).unwrap_or(120.0);
            let y = payload.get("y").and_then(|value| value.as_f64()).unwrap_or(120.0);
            let mesh_url = resolve_part_kind_mesh_url(&node_kind, envelope.document.kind_catalogs.as_ref());
            let grips = grips_from_templates(&envelope.document, &node_kind);
            let source_world = source_grip.as_ref().and_then(|full_id| find_part_by_grip_full_id(&envelope.document, full_id).map(|(part, grip)| (world_grip_position(part, grip), world_grip_direction(part, grip))));
            let origin = source_world.map(|(position, direction)| [position[0] + direction[0], position[1] + direction[1], position[2] + direction[2]]).unwrap_or([0.0, 0.0, 0.0]);
            envelope.document.parts.push(Puzzle5dPart {
                id: id.clone(),
                part_kind: node_kind.clone(),
                part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: node_kind, icon_kind: None, hidden: None, locked: None },
                part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
                grips,
            });
            if let (Some(source), Some(part)) = (source_grip, envelope.document.parts.last()) {
                if let Some(grip) = part.grips.first() {
                    let target = puzzle5d_grip_full_id(&part.id, &grip.id);
                    envelope.document.fasteners.push(Puzzle5dFastener { id: payload.get("edgeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(next_fastener_id), source, target, fastener_kind: None });
                }
            }
            envelope.runtime.selection = Puzzle5dSelection { part_ids: vec![id], grip_ids: Vec::new(), fastener_ids: Vec::new() };
        }

        fn apply_board_events_from_json(&mut self, events_json: &str, envelope: &mut Puzzle5dScene) {
            let Ok(events) = serde_json::from_str::<Vec<Value>>(events_json) else {
                return;
            };
            for event in events {
                let Some(name) = event.get("name").and_then(|value| value.as_str()) else {
                    continue;
                };
                let payload = event.get("payload").cloned().unwrap_or(Value::Null);
                match name {
                    "camera" => {
                        if let Ok(camera) = serde_json::from_value::<Puzzle5dCamera2d>(payload) {
                            envelope.document.camera2d = camera;
                        }
                    }
                    "select" => {
                        if let Some(ids) = payload.get("ids").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                            envelope.runtime.selection = classify_selection(&envelope.document, &ids);
                        }
                    }
                    "nodeDragEnd" => {
                        for entry in payload.get("moves").and_then(|value| value.as_array()).into_iter().flatten() {
                            if let Some(id) = entry.get("id").and_then(|value| value.as_str()) {
                                set_part_2d_position(&mut envelope.document, id, entry.get("x").and_then(|value| value.as_f64()), entry.get("y").and_then(|value| value.as_f64()));
                            }
                        }
                    }
                    "nodeMove" => {
                        if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                            set_part_2d_position(&mut envelope.document, id, payload.get("x").and_then(|value| value.as_f64()), payload.get("y").and_then(|value| value.as_f64()));
                        }
                    }
                    "brushPlace" => {
                        self.apply_board_brush_place(envelope, &payload);
                    }
                    "edgeCreate" => {
                        let source = payload.get("source").and_then(|value| value.as_str()).unwrap_or("").to_string();
                        let target = payload.get("target").and_then(|value| value.as_str()).unwrap_or("").to_string();
                        if !source.is_empty() && !target.is_empty() && !envelope.document.fasteners.iter().any(|entry| entry.source == source && entry.target == target || entry.source == target && entry.target == source) {
                            envelope.document.fasteners.push(Puzzle5dFastener {
                                id: payload.get("id").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(next_fastener_id),
                                source,
                                target,
                                fastener_kind: payload.get("edgeKind").and_then(|value| value.as_str()).map(str::to_string),
                            });
                        }
                    }
                    "nodeDelete" => {
                        if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                            remove_parts(&mut envelope.document, &[id.to_string()]);
                            envelope.runtime.selection = Puzzle5dSelection::default();
                        }
                    }
                    "edgeDelete" => {
                        if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                            envelope.document.fasteners.retain(|fastener| fastener.id != id);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    impl DocumentApp for Puzzle5dPlayApp {
        type Projection = Value;
        type Op = Puzzle5dOp;

        fn app_id(&self) -> &str {
            PUZZLE5D_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            PUZZLE5D_SCHEMA
        }

        fn initial_projection(&self) -> Value {
            serde_json::to_value(default_document()).unwrap_or(Value::Null)
        }

        fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> ActionEmit<Puzzle5dOp> {
            let before = doc.projection.clone();
            let active_tool_initial = view_state.active_tool_id.as_deref().unwrap_or(PUZZLE5D_DEFAULT_TOOL).to_string();
            let mut envelope = scene_from_projection(&before, self.runtime.clone(), &active_tool_initial);
            match action {
                "setFixtureJson" => {
                    if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                        if let Ok(document) = serde_json::from_str::<Puzzle5dDocument>(json_text) {
                            envelope.document = document;
                        }
                    }
                }
                "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                    apply_world3d_sun_action(&mut envelope.runtime.sun, action, args);
                }
                "setActiveExample" => {
                    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                    let next = if example_id.is_empty() || example_id == "empty" {
                        Some(empty_document())
                    } else if example_id == PUZZLE5D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
                        Some(default_document())
                    } else if example_id == PUZZLE5D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
                        Some(document_from_json(NAKAGIN_EXAMPLE_JSON))
                    } else {
                        None
                    };
                    if let Some(document) = next {
                        envelope.document = document;
                        envelope.runtime = Puzzle5dRuntime::default();
                    }
                    self.drive_precompute(&envelope);
                }
                "setSelection" | "documentSelect" => {
                    if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                        envelope.runtime.selection = classify_selection(&envelope.document, &ids);
                    } else {
                        let read = |key: &str| args.and_then(|value| value.get(key)).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok());
                        envelope.runtime.selection = Puzzle5dSelection { part_ids: read("partIds").unwrap_or_default(), grip_ids: read("gripIds").unwrap_or_default(), fastener_ids: read("fastenerIds").unwrap_or_default() };
                    }
                }
                "clearSelection" => {
                    envelope.runtime.selection = Puzzle5dSelection::default();
                }
                "selectAll" => {
                    envelope.runtime.selection = Puzzle5dSelection { part_ids: envelope.document.parts.iter().map(|part| part.id.clone()).collect(), grip_ids: Vec::new(), fastener_ids: Vec::new() };
                }
                "deleteSelection" => {
                    let selection = envelope.runtime.selection.clone();
                    remove_parts(&mut envelope.document, &selection.part_ids);
                    remove_grips(&mut envelope.document, &selection.grip_ids);
                    envelope.document.fasteners.retain(|fastener| !selection.fastener_ids.contains(&fastener.id));
                    envelope.runtime.selection = Puzzle5dSelection::default();
                }
                "duplicateSelection" => {
                    let ids = envelope.runtime.selection.part_ids.clone();
                    let clones: Vec<Puzzle5dPart> = envelope
                        .document
                        .parts
                        .iter()
                        .filter(|part| ids.contains(&part.id))
                        .map(|part| {
                            let mut clone = part.clone();
                            clone.id = next_part_id();
                            clone.part_3d.origin[0] += 0.5;
                            clone.part_3d.origin[1] += 0.5;
                            clone.part_2d.x += 48.0;
                            clone.part_2d.y += 24.0;
                            clone
                        })
                        .collect();
                    if clones.is_empty() {
                        return ActionEmit::default();
                    }
                    let new_ids: Vec<String> = clones.iter().map(|part| part.id.clone()).collect();
                    envelope.document.parts.extend(clones);
                    envelope.runtime.selection = Puzzle5dSelection { part_ids: new_ids, grip_ids: Vec::new(), fastener_ids: Vec::new() };
                }
                "selectSameKindSelection" | "selectSameKind" => {
                    let Some(kind) = envelope.runtime.selection.part_ids.first().and_then(|id| envelope.document.parts.iter().find(|part| &part.id == id)).map(|part| part.part_kind.clone()) else {
                        return ActionEmit::default();
                    };
                    envelope.runtime.selection.part_ids = envelope.document.parts.iter().filter(|part| part.part_kind == kind).map(|part| part.id.clone()).collect();
                }
                "addNode" => {
                    let part_kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
                    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                    add_palette_part(&mut envelope, &part_kind, x, y);
                }
                "setSelectionFlag" => {
                    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
                    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(false);
                    let part_ids = envelope.runtime.selection.part_ids.clone();
                    for part in &mut envelope.document.parts {
                        if !part_ids.contains(&part.id) {
                            continue;
                        }
                        match flag {
                            "hidden" => part.part_2d.hidden = Some(value),
                            "locked" => part.part_2d.locked = Some(value),
                            _ => {}
                        }
                    }
                    if !part_ids.is_empty() && (flag == "hidden" || flag == "locked") {
                    }
                }
                "zoomToSelection" | "focusSelection" => {
                    let Some(target) = gumball_target_world(&envelope) else {
                        return ActionEmit::default();
                    };
                    let camera = &mut envelope.document.camera3d;
                    let offset = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
                    camera.target = target;
                    camera.position = [target[0] + offset[0], target[1] + offset[1], target[2] + offset[2]];
                    let selected_2d: Vec<(f64, f64)> = envelope.document.parts.iter().filter(|part| envelope.runtime.selection.part_ids.contains(&part.id)).map(|part| (part.part_2d.x, part.part_2d.y)).collect();
                    if !selected_2d.is_empty() {
                        envelope.document.camera2d.x = selected_2d.iter().map(|(x, _)| x).sum::<f64>() / selected_2d.len() as f64;
                        envelope.document.camera2d.y = selected_2d.iter().map(|(_, y)| y).sum::<f64>() / selected_2d.len() as f64;
                    }
                }
                SET_ACTIVE_TOOL_ACTION_ID => {
                    // 🧰 Host already applied `view_state.active_tool_id`; clear per-window engagement scratch and
                    // refresh the placement engine for the new tool. Emits no ops and no tool-switch effect.
                    for window in PUZZLE5D_PLAY_WINDOWS {
                        envelope.runtime.engagement_input_by_window.insert(window.to_string(), String::new());
                    }
                    envelope.runtime.brush_candidate_index = 0;
                    if envelope.active_tool == "brush" || envelope.active_tool == "fill" {
                        self.drive_precompute(&envelope);
                    }
                }
                "engagementInput" => {
                    let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(PUZZLE5D_PLAY_WINDOW_2D);
                    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                    if PUZZLE5D_PLAY_WINDOWS.contains(&window) {
                        envelope.runtime.engagement_input_by_window.insert(window.to_string(), value.to_string());
                    }
                }
                "engagementSubmit" => {
                    let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(PUZZLE5D_PLAY_WINDOW_2D).to_string();
                    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map(str::trim).unwrap_or("").to_lowercase();
                    match value.as_str() {
                        "select" | "brush" | "fill" => {
                            envelope.active_tool = value;
                            if envelope.active_tool != "select" {
                                self.drive_precompute(&envelope);
                            }
                        }
                        "clear" => envelope.runtime.selection = Puzzle5dSelection::default(),
                        "rectangle" | "lasso" => envelope.runtime.selection_method = value,
                        _ => {}
                    }
                    if PUZZLE5D_PLAY_WINDOWS.contains(&window.as_str()) {
                        envelope.runtime.engagement_input_by_window.insert(window, String::new());
                    }
                }
                "engagementAbort" => {
                    if let Some(window) = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()) {
                        if PUZZLE5D_PLAY_WINDOWS.contains(&window) {
                            envelope.runtime.engagement_input_by_window.insert(window.to_string(), String::new());
                        }
                    }
                    envelope.active_tool = "select".into();
                }
                "engagementControlSelect" => {
                    let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
                    if let Some(index) = candidate_id.strip_prefix("puzzle5d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
                        envelope.runtime.brush_candidate_index = index;
                    }
                }
                "addBrushPart" | "addBrushObject" => {
                    self.drive_precompute(&envelope);
                    if let Some(payload_value) = args {
                        let mut payload = payload_value.clone();
                        if let Some(object) = payload.as_object_mut() {
                            if let Some(part_kind) = object.remove("partKind") {
                                object.insert("objectKindId".to_string(), part_kind);
                            }
                            if object.get("targetVortexFullId").is_none() {
                                if let Some(grip_id) = puzzle5d_brush_target_grip(&envelope) {
                                    object.insert("targetVortexFullId".to_string(), json!(grip_id));
                                }
                            }
                        }
                        if let Some(next) = self.apply_engine_brush_placement(&envelope, &payload) {
                            envelope = next;
                        }
                    }
                    let part_kind = args.and_then(|value| value.get("partKind").or_else(|| value.get("objectKindId"))).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
                    let payload = json!({ "nodeKind": part_kind, "x": args.and_then(|value| value.get("x")).cloned().unwrap_or(json!(120.0)), "y": args.and_then(|value| value.get("y")).cloned().unwrap_or(json!(120.0)) });
                    self.apply_board_brush_place(&mut envelope, &payload);
                }
                "setFillCount" => {
                    self.drive_precompute(&envelope);
                    let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_f64()).map(|value| value.round().max(0.0) as u32).unwrap_or(0).min(PUZZLE5D_FILL_COUNT_MAX);
                    envelope.runtime.fill_count = count;
                    if count > 0 {
                        envelope.active_tool = "fill".into();
                        if let Ok(fixture_json) = self.precompute.apply_fill_count_rust(count) {
                            if let Some(next) = merge_engine_fixture(&envelope, &fixture_json) {
                                envelope = next;
                            }
                        }
                    }
                }
                "cycleBrushCandidate" => {
                    self.drive_precompute(&envelope);
                    if let Some(grip_full_id) = puzzle5d_brush_target_grip(&envelope) {
                        let free = parse_brush_candidates_free(&self.precompute.brush_candidates(&grip_full_id)).len();
                        if free > 0 {
                            envelope.runtime.brush_candidate_index = (envelope.runtime.brush_candidate_index + 1) % free;
                        }
                    } else {
                        envelope.runtime.brush_candidate_index = envelope.runtime.brush_candidate_index.saturating_add(1);
                    }
                }
                "registerBrushMesh" => {
                    if let (Some(url), Some(positions), Some(indices)) =
                        (args.and_then(|v| v.get("url")).and_then(|v| v.as_str()), args.and_then(|v| v.get("positions")).and_then(|v| v.as_array()), args.and_then(|v| v.get("indices")).and_then(|v| v.as_array()))
                    {
                        let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
                        let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect();
                        self.precompute.register_mesh(url, &positions, &indices);
                        self.registered_mesh_urls.insert(url.to_string());
                    }
                    return ActionEmit::default();
                }
                "setBrushPlacementOverlapBudget" => {
                    if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()) {
                        envelope.runtime.overlap_budget = value.clamp(0.0, 1.0);
                        self.drive_precompute(&envelope);
                    }
                }
                "setObjectKindWeight" | "setVortexKindWeight" => {
                    let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
                    let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0);
                    if action == "setObjectKindWeight" {
                        envelope.runtime.object_kind_weights.insert(kind_id.into(), value);
                    } else {
                        envelope.runtime.vortex_kind_weights.insert(kind_id.into(), value);
                    }
                    self.drive_precompute(&envelope);
                }
                "addPartKind" => {
                    let part_kind = args.and_then(|value| value.get("partKind")).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
                    let payload = json!({ "nodeKind": part_kind, "x": 120.0, "y": 120.0 });
                    self.apply_board_brush_place(&mut envelope, &payload);
                }
                "patchPart" => {
                    let part_id = args.and_then(|value| value.get("partId")).and_then(|value| value.as_str()).unwrap_or("");
                    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                    let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                    let text = value.as_str().map(str::to_string).unwrap_or_else(|| value.to_string());
                    for part in &mut envelope.document.parts {
                        if part.id != part_id {
                            continue;
                        }
                        match field {
                            "partKind" => part.part_kind = text.clone(),
                            "text" => part.part_2d.text = text.clone(),
                            "label" => part.part_3d.label = if text.is_empty() { None } else { Some(text.clone()) },
                            "x" => {
                                if let Ok(parsed) = text.trim().parse::<f64>() {
                                    part.part_2d.x = parsed;
                                }
                            }
                            "y" => {
                                if let Ok(parsed) = text.trim().parse::<f64>() {
                                    part.part_2d.y = parsed;
                                }
                            }
                            "origin" => {
                                if let Some(origin) = parse_vec3(&text) {
                                    part.part_3d.origin = origin;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                "patchGrip" => {
                    let grip_full_id = args.and_then(|value| value.get("gripFullId")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                    let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                    let text = value.as_str().map(str::to_string).unwrap_or_else(|| value.to_string());
                    for part in &mut envelope.document.parts {
                        let part_id = part.id.clone();
                        for grip in &mut part.grips {
                            if puzzle5d_grip_full_id(&part_id, &grip.id) != grip_full_id {
                                continue;
                            }
                            match field {
                                "gripKind" => {
                                    grip.grip_kind = text.clone();
                                    grip.grip_2d.grip_kind = text.clone();
                                }
                                "angle" => {
                                    if let Ok(parsed) = text.trim().parse::<f64>() {
                                        grip.grip_2d.angle = parsed;
                                    }
                                }
                                "radius" => {
                                    if let Ok(parsed) = text.trim().parse::<f64>() {
                                        grip.grip_2d.radius = parsed;
                                        grip.grip_3d.radius = parsed;
                                    }
                                }
                                "position" => {
                                    if let Some(position) = parse_vec3(&text) {
                                        grip.grip_3d.position = position;
                                    }
                                }
                                "direction" => {
                                    if let Some(direction) = parse_vec3(&text) {
                                        grip.grip_3d.direction = Some(direction);
                                    }
                                }
                                "label" => grip.grip_3d.label = if text.is_empty() { None } else { Some(text.clone()) },
                                _ => {}
                            }
                        }
                    }
                }
                "setCamera" => {
                    if let Some(camera) = args.and_then(|value| value.get("camera")) {
                        let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                        if surface_id == PUZZLE5D_PLAY_SURFACE_2D || camera.get("position").is_none() {
                            if let Ok(parsed) = serde_json::from_value::<Puzzle5dCamera2d>(camera.clone()) {
                                envelope.document.camera2d = parsed;
                            }
                        } else if let Ok(parsed) = serde_json::from_value::<Puzzle5dCamera3d>(camera.clone()) {
                            envelope.document.camera3d = parsed;
                        }
                    }
                }
                "setCamera2d" => {
                    if let Some(camera) = args.and_then(|value| value.get("camera")) {
                        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                            envelope.document.camera2d = parsed;
                        }
                    }
                }
                "setCamera3d" => {
                    if let Some(camera) = args.and_then(|value| value.get("camera")) {
                        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                            envelope.document.camera3d = parsed;
                        }
                    }
                }
                "translateSelection" => {
                    let ids = mesh_selection_ids(args, &envelope.runtime.selection.part_ids);
                    let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    for part in &mut envelope.document.parts {
                        if ids.contains(&part.id) {
                            part.part_3d.origin[0] += dx;
                            part.part_3d.origin[1] += dy;
                            part.part_3d.origin[2] += dz;
                        }
                    }
                    if !ids.is_empty() {
                    }
                }
                "rotateSelection" => {
                    let ids = mesh_selection_ids(args, &envelope.runtime.selection.part_ids);
                    let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let delta = quat_from_axis_angle(ax, ay, az, angle);
                    for part in &mut envelope.document.parts {
                        if ids.contains(&part.id) {
                            let current = part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                            part.part_3d.orientation = Some(quat_mul(delta, current));
                        }
                    }
                    if !ids.is_empty() {
                    }
                }
                "scaleSelection" => {
                    let ids = mesh_selection_ids(args, &envelope.runtime.selection.part_ids);
                    let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    for part in &mut envelope.document.parts {
                        if ids.contains(&part.id) {
                            let current = part_scale_json(part);
                            part.part_3d.scale = Some(json!([current[0] * sx, current[1] * sy, current[2] * sz]));
                        }
                    }
                    if !ids.is_empty() {
                    }
                }
                "worldSelect" => {
                    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                    envelope.runtime.selection.part_ids = merge_world_selection_ids(&envelope.runtime.selection.part_ids, &ids, merge);
                }
                "worldPick" => {
                    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                    if args.and_then(|value| value.get("id")).is_none_or(|value| value.is_null()) {
                        if merge == "replace" {
                            envelope.runtime.selection.part_ids.clear();
                        }
                    }
                    let index = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                    if let Some(part) = envelope.document.parts.get(index) {
                        let id = part.id.clone();
                        envelope.runtime.selection.part_ids = match merge {
                            "add" => {
                                let mut merged = envelope.runtime.selection.part_ids.clone();
                                if !merged.contains(&id) {
                                    merged.push(id);
                                }
                                merged
                            }
                            "toggle" => {
                                let mut merged = envelope.runtime.selection.part_ids.clone();
                                if let Some(position) = merged.iter().position(|entry| entry == &id) {
                                    merged.remove(position);
                                } else {
                                    merged.push(id);
                                }
                                merged
                            }
                            _ => vec![id],
                        };
                    }
                }
                "worldHover" => {
                    envelope.runtime.hovered_part_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                }
                "setHover" => {
                    envelope.runtime.hovered_part_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).map(str::to_string);
                }
                "worldVortexHover" => {
                    envelope.runtime.selection.grip_ids = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(|full_id| vec![full_id.to_string()]).unwrap_or_default();
                    if envelope.active_tool == "brush" && !envelope.runtime.selection.grip_ids.is_empty() {
                        self.drive_precompute(&envelope);
                    }
                }
                "worldVortexSelect" => {
                    if let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) {
                        envelope.runtime.selection.grip_ids = vec![full_id.to_string()];
                        envelope.runtime.selection.part_ids.clear();
                        self.drive_precompute(&envelope);
                    }
                }
                "worldRelocate" => {
                    let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                    let position = args.and_then(|value| value.get("position")).and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok());
                    if let (Some(part), Some(position)) = (envelope.document.parts.iter_mut().find(|part| part.id == object_id), position) {
                        part.part_3d.origin = position;
                        let source_grip = part.grips.first().map(|grip| (puzzle5d_grip_full_id(&part.id, &grip.id), world_grip_position(part, grip)));
                        if let Some((source_id, source_position)) = source_grip {
                            for other in &envelope.document.parts {
                                if other.id == object_id {
                                    continue;
                                }
                                for grip in &other.grips {
                                    let target_id = puzzle5d_grip_full_id(&other.id, &grip.id);
                                    if target_id == source_id {
                                        continue;
                                    }
                                    let target_position = world_grip_position(other, grip);
                                    let dx = source_position[0] - target_position[0];
                                    let dy = source_position[1] - target_position[1];
                                    let dz = source_position[2] - target_position[2];
                                    if (dx * dx + dy * dy + dz * dz).sqrt() <= PUZZLE5D_PROXIMITY_RADIUS
                                        && !envelope.document.fasteners.iter().any(|entry| entry.source == source_id && entry.target == target_id || entry.source == target_id && entry.target == source_id)
                                    {
                                        envelope.document.fasteners.push(Puzzle5dFastener { id: next_fastener_id(), source: source_id.clone(), target: target_id, fastener_kind: None });
                                    }
                                }
                            }
                        }
                        self.drive_precompute(&envelope);
                    }
                }
                "setSelectionMethod" => {
                    let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
                    envelope.runtime.selection_method = method.into();
                }
                "setLodMode" => {
                    if let Some(mode) = args.and_then(|value| value.get("value").or_else(|| value.get("mode"))).and_then(|value| value.as_str()) {
                        envelope.runtime.lod_mode = mode.into();
                    }
                }
                "setSuggestionOffset" => {
                    if let Some(distance) = args.and_then(|value| value.get("distance").or_else(|| value.get("value"))).and_then(|value| value.as_f64()) {
                        envelope.runtime.suggestion_offset = distance.clamp(PUZZLE5D_SUGGESTION_OFFSET_MIN, PUZZLE5D_SUGGESTION_OFFSET_MAX);
                    }
                }
                "setGridSnapEnabled" => {
                    envelope.runtime.grid_snap_enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(false);
                }
                "setGridFactor" => {
                    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                        envelope.runtime.grid_factor = value;
                    }
                }
                "applyBoardEvents" => {
                    if let Some(events_json) = args.and_then(|value| value.get("eventsJson")).and_then(|value| value.as_str()) {
                        self.apply_board_events_from_json(events_json, &mut envelope);
                    }
                }
                "worldPointerDown" | "canvasPointerDown" => return ActionEmit::default(),
                _ => {}
            }
            let next_active_tool = envelope.active_tool.clone();
            self.runtime = envelope.runtime;
            let after = serde_json::to_value(&envelope.document).unwrap_or_else(|_| before.clone());
            let ops = puzzle5d_document_delta_ops(&before, &after);
            // 🌀 Coalesce each gumball drag tick into one undoable edit (compact per-part records, not full meshes).
            let coalesce_key = match action {
                "translateSelection" => Some("gumball-translate".to_string()),
                "rotateSelection" => Some("gumball-rotate".to_string()),
                "scaleSelection" => Some("gumball-scale".to_string()),
                _ => None,
            };
            // 🧰 Programmatic tool switches (engagement submit/abort, fill) push the active tool back into the
            // host session for both windows; `setActiveTool` itself never re-emits (the host already applied it).
            let effects = if next_active_tool != active_tool_initial {
                PUZZLE5D_PLAY_WINDOWS.iter().map(|window| HostEffect::SetActiveTool { window_kind_id: (*window).into(), tool_id: next_active_tool.clone() }).collect()
            } else {
                Vec::new()
            };
            ActionEmit { ops, coalesce_key, effects, ..Default::default() }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> UiNode {
            let active_tool = view_state.active_tool_id.as_deref().unwrap_or(PUZZLE5D_DEFAULT_TOOL);
            let envelope = scene_from_projection(doc.projection, self.runtime.clone(), active_tool);
            let labels = puzzle5d_labels(view_state);
            match body_key {
                PUZZLE5D_PLAY_BODY_2D => build_puzzle2d_board_scene(PUZZLE5D_PLAY_SURFACE_2D, PUZZLE5D_PLAY_CONTROLLER_ID, puzzle5d_board_scene(&envelope)),
                PUZZLE5D_PLAY_BODY_3D => {
                    let brush_preview = world_brush_preview_json(&self.precompute, &envelope);
                    build_world_3d_scene(
                        PUZZLE5D_PLAY_SURFACE_3D,
                        PUZZLE5D_PLAY_CONTROLLER_ID,
                        world3d_scene_extended(
                            camera3d_json(&envelope.document.camera3d),
                            world_meshes_json(&envelope.document),
                            world_instances_json(&envelope.document, &envelope.runtime),
                            world_selection_json_ex(&envelope),
                            Some(world_grips_json(&envelope.document)),
                            Some(world_fasteners_json(&envelope.document)),
                            None,
                            None,
                            brush_preview,
                            Some(world_interaction_json(&envelope.runtime, &envelope.active_tool)),
                            None,
                            None,
                            Some(world3d_chunking_json(256.0, 8000.0)),
                            puzzle5d_context_menu_json(&envelope, labels),
                            Some(world3d_environment_json(&envelope.runtime.sun)),
                        ),
                    )
                }
                PUZZLE5D_PLAY_BODY_DOCUMENT => build_document_tree(&envelope, labels),
                PUZZLE5D_PLAY_BODY_KINDS => build_kinds_tree(&envelope, labels),
                PUZZLE5D_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope, labels),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn window_engagements(&self, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
            let active_tool = view_state.active_tool_id.as_deref().unwrap_or(PUZZLE5D_DEFAULT_TOOL);
            let envelope = scene_from_projection(doc.projection, self.runtime.clone(), active_tool);
            PUZZLE5D_PLAY_WINDOWS.iter().map(|window| (window.to_string(), puzzle5d_engagement(&envelope, window))).collect()
        }

        fn window_measures(&self, doc: &DocumentView<'_, Value>, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
            let active_tool = view_state.active_tool_id.as_deref().unwrap_or(PUZZLE5D_DEFAULT_TOOL);
            let envelope = scene_from_projection(doc.projection, self.runtime.clone(), active_tool);
            let labels = puzzle5d_labels(view_state);
            PUZZLE5D_PLAY_WINDOWS.iter().map(|window| (window.to_string(), puzzle5d_window_measures(window, &envelope, &self.precompute, labels))).collect()
        }

        fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
            let labels = puzzle5d_labels(view_state);
            semio_framework_plugin::AppLabelsOverlay {
                app_label: None,
                window_kind_labels: std::collections::HashMap::from([
                    (PUZZLE5D_PLAY_WINDOW_2D.to_string(), labels.window_2d.to_string()),
                    (PUZZLE5D_PLAY_WINDOW_3D.to_string(), labels.window_3d.to_string()),
                ]),
                panel_tab_labels: std::collections::HashMap::new(),
                mode_labels: std::collections::HashMap::new(),
            }
        }
    }
    //#endregion 🔖Puzzle5dPlayApp

    //#region 🔖Manifest
    pub fn create_puzzle5d_app() -> App {
        let envelope = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_tool: PUZZLE5D_DEFAULT_TOOL.into() };
        let precompute = Puzzle5dPrecomputeSession::new();
        let manifest_labels = puzzle5d_labels(&ViewState::default());
        let mut app = App::from_builder(
            App::builder(PUZZLE5D_PLAY_APP_ID, "Puzzle 5D")
                .document(["semio", "puzzle", "5d"])
                .icon_id("puzzle")
                .mode("edit", "Edit")
                .default_mode_id("edit")
                .window_kind_with_engagement(PUZZLE5D_PLAY_WINDOW_2D, "Puzzle 2D", PUZZLE5D_PLAY_BODY_2D, SurfaceKind::Puzzle2dBoard, puzzle5d_engagement(&envelope, PUZZLE5D_PLAY_WINDOW_2D))
                .window_kind_with_engagement(PUZZLE5D_PLAY_WINDOW_3D, "Puzzle 3D", PUZZLE5D_PLAY_BODY_3D, SurfaceKind::World3d, puzzle5d_engagement(&envelope, PUZZLE5D_PLAY_WINDOW_3D))
                .default_layout(create_default_layout(&[PUZZLE5D_PLAY_WINDOW_2D.into(), PUZZLE5D_PLAY_WINDOW_3D.into()], "row", Some(&[50.0, 50.0]), Some(&["Puzzle 2D".into(), "Puzzle 3D".into()])))
                .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PUZZLE5D_PLAY_BODY_DOCUMENT)
                .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PUZZLE5D_PLAY_BODY_KINDS)
                .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PUZZLE5D_PLAY_BODY_INSPECTOR)
                // 🔧 Document-mutating operations (emit VCS ops through the before/after document delta).
                .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setFixtureJson", "Set Fixture Json", ActionKind::Operation) })
                .operation("setActiveExample", "Set Active Example")
                .operation("addNode", "Add Node")
                .operation("addPartKind", "Add Part")
                .operation("addBrushPart", "Add Brush Part")
                .operation("addBrushObject", "Add Brush Object")
                .operation("deleteSelection", "Delete Selection")
                .operation("duplicateSelection", "Duplicate Selection")
                .operation("setSelectionFlag", "Set Selection Flag")
                .operation("zoomToSelection", "Zoom To Selection")
                .operation("focusSelection", "Focus Selection")
                .operation("engagementSubmit", "Engagement Submit")
                .operation("setFillCount", "Set Fill Count")
                .operation("patchPart", "Patch Part")
                .operation("patchGrip", "Patch Grip")
                .operation("setCamera", "Set Camera")
                .operation("setCamera2d", "Set Camera 2D")
                .operation("setCamera3d", "Set Camera 3D")
                .operation("translateSelection", "Translate Selection")
                .operation("rotateSelection", "Rotate Selection")
                .operation("scaleSelection", "Scale Selection")
                .operation("worldRelocate", "Relocate Part")
                .operation("applyBoardEvents", "Apply Board Events")
                // 👁️ Ephemeral view state — selection, hover, tool parameters, brush cycling.
                .view_action("setSelection", "Set Selection")
                .view_action("documentSelect", "Document Select")
                .view_action("clearSelection", "Clear Selection")
                .view_action("selectAll", "Select All")
                .view_action("selectSameKindSelection", "Select Same Kind")
                .view_action("selectSameKind", "Select Same Kind (alias)")
                .view_action("toggleSun", "Toggle Sun")
                .view_action("setSunAzimuth", "Set Sun Azimuth")
                .view_action("setSunElevation", "Set Sun Elevation")
                .view_action("setSunIntensity", "Set Sun Intensity")
                .view_action("engagementInput", "Engagement Input")
                .view_action("engagementAbort", "Engagement Abort")
                .view_action("engagementControlSelect", "Engagement Control Select")
                .view_action("cycleBrushCandidate", "Cycle Brush Candidate")
                .view_action("registerBrushMesh", "Register Brush Mesh")
                .view_action("setBrushPlacementOverlapBudget", "Set Brush Placement Overlap Budget")
                .view_action("setObjectKindWeight", "Set Object Kind Weight")
                .view_action("setVortexKindWeight", "Set Vortex Kind Weight")
                .view_action("worldSelect", "World Select")
                .view_action("worldPick", "World Pick")
                .view_action("worldHover", "World Hover")
                .view_action("setHover", "Set Hover")
                .view_action("worldVortexHover", "World Vortex Hover")
                .view_action("worldVortexSelect", "World Vortex Select")
                .view_action("setSelectionMethod", "Set Selection Method")
                .view_action("setLodMode", "Set Lod Mode")
                .view_action("setSuggestionOffset", "Set Suggestion Offset")
                .view_action("setGridSnapEnabled", "Set Grid Snap Enabled")
                .view_action("setGridFactor", "Set Grid Factor")
                .view_action("worldPointerDown", "World Pointer Down")
                .view_action("canvasPointerDown", "Canvas Pointer Down")
                // 📝 Staged argument forms for the brush create actions (P1).
                .action_args("addPartKind", vec![
                    ActionArgDef::select("partKind", "Kind", vec![ActionArgOption::new("Part", "Part")]).default_value("Part"),
                ])
                .action_args("addBrushPart", vec![
                    ActionArgDef::select("partKind", "Kind", vec![ActionArgOption::new("Part", "Part")]).default_value("Part"),
                ])
                .action_args("addBrushObject", vec![
                    ActionArgDef::select("partKind", "Kind", vec![ActionArgOption::new("Part", "Part")]).default_value("Part"),
                ])
                // 🧰 Flat per-window tool set (host-owned `view_state.active_tool_id`); `select` is the default.
                .tool(ToolDefinition { category: Some(ToolCategory::Selection), ..ToolDefinition::new("select", "Select", "cursor") })
                .tool(ToolDefinition { group: Some("transform".into()), ..ToolDefinition::new("move", "Move", "move") })
                .tool(ToolDefinition { group: Some("transform".into()), ..ToolDefinition::new("rotate", "Rotate", "rotate-cw") })
                .tool(ToolDefinition { group: Some("transform".into()), ..ToolDefinition::new("scale", "Scale", "maximize-2") })
                .tool(ToolDefinition::new("brush", "Brush", "brush"))
                .tool(ToolDefinition::new("fill", "Fill", "fill"))
                .tool(ToolDefinition::new("worldRelocate", "Relocate", "move-3d"))
                .window_kind_tools(PUZZLE5D_PLAY_WINDOW_2D, vec!["select".into(), "brush".into(), "fill".into()])
                .window_kind_tools(PUZZLE5D_PLAY_WINDOW_3D, vec!["select".into(), "move".into(), "rotate".into(), "scale".into(), "brush".into(), "fill".into(), "worldRelocate".into()])
                // 📇 Per-window action scoping — the 3D window (World3d) owns the transform-gumball ops
                // (move/rotate/scale/relocate tools are 3D-only) plus its own camera; the 2D window
                // (Puzzle2dBoard) owns board-event dispatch and its own camera. Select/brush/fill create
                // ops, deletion, engagement, and global example/json actions apply to both surfaces and
                // stay unscoped orphans, appearing on both windows.
                .window_kind_actions(PUZZLE5D_PLAY_WINDOW_3D, vec![
                    "translateSelection".into(), "rotateSelection".into(), "scaleSelection".into(),
                    "worldRelocate".into(), "setCamera3d".into(),
                ])
                .window_kind_actions(PUZZLE5D_PLAY_WINDOW_2D, vec![
                    "applyBoardEvents".into(), "setCamera2d".into(),
                ]),
        );
        for window in PUZZLE5D_PLAY_WINDOWS {
            if let Some(window_kind) = app.definition.window_kinds.iter_mut().find(|window_kind| window_kind.id == window) {
                window_kind.options.measures = puzzle5d_window_measures(window, &envelope, &precompute, manifest_labels);
            }
        }
        app.example("empty", "Empty", serde_json::to_string(&empty_document()).unwrap())
            .example(PUZZLE5D_EXAMPLE_CONCRETE_FOREST, "Concrete Forest", CONCRETE_FOREST_EXAMPLE_JSON)
            .example(PUZZLE5D_EXAMPLE_NAKAGIN, "Nakagin Capsule Tower", NAKAGIN_EXAMPLE_JSON)
            .program("puzzle5d", "Puzzle 5D", "model")
    }

    /// 📥 Tier C DWG mesh import — always returns the empty puzzle-5d document; never errors on a structurally valid mesh.
    fn puzzle5d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
        serde_json::to_value(empty_document()).map_err(|error| error.to_string())
    }

    pub fn register_puzzle5d_exports() {
        register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::ObjExporter));
        register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::GlbExporter));
        register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::StlExporter));
        register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
        register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
        register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
        semio_framework_os::register_mesh_dwg_export_handler("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")));
        semio_framework_os::register_mesh_dwg_import_handler("5d.puzzle", puzzle5d_document_from_mesh);
    }
    //#endregion 🔖Manifest

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{app::AppActionRegistry, ActionMeta, PluginApp, VcsDocumentApp};

        fn meta(actor: &str) -> ActionMeta {
            ActionMeta { actor: actor.into(), instance_id: 1 }
        }

        fn new_app() -> VcsDocumentApp<Puzzle5dPlayApp> {
            VcsDocumentApp::new(Puzzle5dPlayApp::default())
        }

        fn new_app_with_registry() -> VcsDocumentApp<Puzzle5dPlayApp> {
            let definition = create_puzzle5d_app().definition;
            VcsDocumentApp::with_registry(Puzzle5dPlayApp::default(), AppActionRegistry::from_definition(&definition))
        }

        fn part_count(app: &VcsDocumentApp<Puzzle5dPlayApp>) -> usize {
            app.projection().expect("projection").get("parts").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
        }

        #[test]
        fn renders_paired_board_and_world_scenes() {
            let mut app = new_app();
            let board = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_2D, None, &ViewState::default()).expect("render 2d")).unwrap();
            assert!(board.contains("puzzle2d-board"));
            let world = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_3D, None, &ViewState::default()).expect("render 3d")).unwrap();
            assert!(world.contains("world-3d"));
        }

        #[test]
        fn initial_projection_is_the_concrete_forest_document() {
            let mut app = new_app();
            assert_eq!(app.projection().expect("projection").get("schema").and_then(|value| value.as_str()), Some(PUZZLE5D_SCHEMA));
            assert!(part_count(&app) > 0, "the concrete-forest default document ships with parts");
        }

        #[test]
        fn set_active_example_swaps_the_document_and_undo_restores_it() {
            let mut app = new_app();
            let loaded = part_count(&app);
            assert!(loaded > 0);
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": "empty" })), &ViewState::default(), &meta("local")).expect("empty");
            assert_eq!(part_count(&app), 0, "empty example clears the parts");
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            assert_eq!(part_count(&app), loaded, "undo restores the concrete-forest parts");
            app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
            assert_eq!(part_count(&app), 0);
        }

        #[test]
        fn document_panel_renders() {
            let mut app = new_app();
            let node = app.render(PUZZLE5D_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
            assert!(!serde_json::to_string(&node).unwrap().is_empty());
        }

        #[test]
        fn app_definition_has_the_paired_windows() {
            let app = create_puzzle5d_app();
            let ids: Vec<&str> = app.definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
            assert!(ids.contains(&PUZZLE5D_PLAY_WINDOW_2D) && ids.contains(&PUZZLE5D_PLAY_WINDOW_3D));
        }

        #[test]
        fn window_kind_actions_scope_transform_to_3d_only() {
            let definition = create_puzzle5d_app().definition;
            let resolve = |window_id: &str| -> Vec<String> {
                let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
                semio_framework_plugin::resolve_window_actions(&definition, window)
                    .into_iter()
                    .map(|action| action.id.clone())
                    .collect()
            };
            let board = resolve(PUZZLE5D_PLAY_WINDOW_2D);
            let world = resolve(PUZZLE5D_PLAY_WINDOW_3D);
            for transform_op in ["translateSelection", "rotateSelection", "scaleSelection", "worldRelocate", "setCamera3d"] {
                assert!(world.contains(&transform_op.to_string()), "3D must expose {transform_op}");
                assert!(!board.contains(&transform_op.to_string()), "2D must NOT expose {transform_op}");
            }
            assert!(board.contains(&"applyBoardEvents".to_string()), "2D must expose applyBoardEvents");
            assert!(!world.contains(&"applyBoardEvents".to_string()), "3D must NOT expose applyBoardEvents");
            for shared in ["addBrushPart", "deleteSelection"] {
                assert!(board.contains(&shared.to_string()) && world.contains(&shared.to_string()), "{shared} stays on both windows");
            }
        }

        #[test]
        fn window_engagements_cover_both_windows() {
            let mut app = new_app();
            let engagements = app.window_engagements(&ViewState::default());
            assert!(engagements.contains_key(PUZZLE5D_PLAY_WINDOW_2D));
            assert!(engagements.contains_key(PUZZLE5D_PLAY_WINDOW_3D));
        }

        //#region 🧰 Window Actions & Tools contract
        #[test]
        fn add_part_kind_materializes_the_declared_kind_default() {
            // 📝 P1 arg form: addPartKind with no args materializes the declared `partKind` default and adds a part.
            let mut app = new_app_with_registry();
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": "empty" })), &ViewState::default(), &meta("local")).expect("empty");
            let before = part_count(&app);
            let result = app.handle_action("addPartKind", None, &ViewState::default(), &meta("local")).expect("addPartKind");
            assert!(!result.operations.is_empty(), "addPartKind is an Operation that emits ops");
            assert_eq!(part_count(&app), before + 1, "the materialized default kind adds exactly one part");
            let projection = app.projection().expect("projection");
            let kind = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.last()).and_then(|part| part.get("partKind")).and_then(Value::as_str);
            assert_eq!(kind, Some("Part"), "the declared partKind default was materialized host-side");
        }

        #[test]
        fn set_active_tool_emits_no_ops_and_no_history_entry() {
            // 🧰 Switching tools is the framework View action: no document ops, no undo entry, no re-emitted effect.
            let mut app = new_app_with_registry();
            let before = app.projection().expect("projection");
            let brush_view = ViewState { active_tool_id: Some("brush".into()), ..ViewState::default() };
            let result = app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": "brush" })), &brush_view, &meta("local")).expect("switch tool");
            assert!(result.operations.is_empty(), "tool switching never emits document ops");
            assert!(result.requested_effects.is_empty(), "a user tool switch does not re-emit SetActiveTool");
            assert_eq!(app.projection().expect("projection"), before, "tool switching does not mutate the document");
        }

        #[test]
        fn engagements_expose_no_tool_switch_options_for_either_window() {
            // 🧰 select/brush/fill switching lives only on the framework toolbar; neither the 2D nor the 3D
            // engagement HUD may duplicate it as options.
            let mut app = new_app();
            let engagements = app.window_engagements(&ViewState::default());
            for window in [PUZZLE5D_PLAY_WINDOW_2D, PUZZLE5D_PLAY_WINDOW_3D] {
                assert!(engagements.get(window).expect("engagement").options.is_none(), "the {window} engagement must not re-expose tool switching as options");
            }
        }

        /// 🎯 D-3 follow-up: the fill-count slider and brush placement picker are tagged `WindowMeasure::Group`s
        /// in [`puzzle5d_window_measures`] (surfaced by `partition_window_measures` only for their active tool),
        /// never `WindowEngagementControl`s on the HUD — for both the 2D and 3D windows.
        #[test]
        fn fill_and_brush_params_are_tagged_tool_options_not_engagement_controls() {
            let labels = puzzle5d_labels(&ViewState::default());
            let session = Puzzle5dPrecomputeSession::new();
            let group_tag = |measures: &[WindowMeasure], id: &str| {
                measures.iter().find_map(|measure| match measure {
                    WindowMeasure::Group { id: gid, active_tool_id, .. } if gid == id => Some(active_tool_id.clone()),
                    _ => None,
                })
            };
            let fill_slider_in_group = |measures: &[WindowMeasure], group_id: &str| {
                measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, children, .. } if id == group_id && children.iter().any(|child| matches!(child, WindowMeasure::Slider { id, .. } if id == "puzzle5d-fill-count"))))
            };
            // 🪣 Fill tool: the fill-count slider now lives in a "fill"-tagged Tool Options group (per window),
            // NOT the engagement HUD.
            let mut fill_runtime = Puzzle5dRuntime::default();
            fill_runtime.fill_count = 3;
            let fill_scene = Puzzle5dScene { document: default_document(), runtime: fill_runtime, active_tool: "fill".into() };
            for window in [PUZZLE5D_PLAY_WINDOW_2D, PUZZLE5D_PLAY_WINDOW_3D] {
                let measures = puzzle5d_window_measures(window, &fill_scene, &session, labels);
                assert_eq!(group_tag(&measures, "puzzle5d-play-tool-options-fill"), Some(Some("fill".into())), "{window} fill Tool Options must be tagged for the fill tool");
                assert!(fill_slider_in_group(&measures, "puzzle5d-play-tool-options-fill"), "{window} fill Tool Options must carry the fill-count slider");
                let fill_hud = puzzle5d_engagement(&fill_scene, window);
                assert!(fill_hud.control.is_none() && fill_hud.controls.is_none(), "{window} fill engagement HUD must no longer carry the relocated control");
            }
            // 🖌️ Brush tool: with no candidates to place, the "brush"-tagged group is absent (matching the old
            // gate), and the engagement HUD is likewise bare.
            let brush_scene = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_tool: "brush".into() };
            for window in [PUZZLE5D_PLAY_WINDOW_2D, PUZZLE5D_PLAY_WINDOW_3D] {
                assert_eq!(group_tag(&puzzle5d_window_measures(window, &brush_scene, &session, labels), "puzzle5d-play-tool-options-brush"), None, "{window} brush Tool Options is absent without candidates");
                let brush_hud = puzzle5d_engagement(&brush_scene, window);
                assert!(brush_hud.control.is_none() && brush_hud.controls.is_none(), "{window} brush engagement HUD must no longer carry the relocated control");
            }
            // 🖌️ The positive brush-candidate surfacing (group present ⇒ tagged "brush") is proven by
            // construction: `puzzle5d_brush_tool_options` returns the same tagged `Select` group shape as the
            // d3 helper, whose end-to-end positive path is covered by the sibling d3 test.
        }

        #[test]
        fn engagement_submit_switches_tool_via_host_effect_for_both_windows() {
            // 🧰 Reconciled dual entry point: the engagement token drives the same host-owned tool switch, once per window.
            let mut app = new_app();
            let result = app.handle_action("engagementSubmit", Some(&json!({ "window": PUZZLE5D_PLAY_WINDOW_3D, "value": "brush" })), &ViewState::default(), &meta("local")).expect("submit");
            let windows: Vec<&str> = result.requested_effects.iter().filter_map(|effect| match effect { HostEffect::SetActiveTool { window_kind_id, tool_id } if tool_id == "brush" => Some(window_kind_id.as_str()), _ => None }).collect();
            assert!(windows.contains(&PUZZLE5D_PLAY_WINDOW_2D) && windows.contains(&PUZZLE5D_PLAY_WINDOW_3D), "brush switch is pushed to both windows, got {windows:?}");
        }

        #[test]
        fn gumball_translate_drag_coalesces_into_one_edit() {
            // 🌀 Coalescing regression: three translate ticks with the same key are ONE undoable edit.
            let mut app = new_app();
            let part_id = app.projection().expect("projection").get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).and_then(|part| part.get("id")).and_then(Value::as_str).expect("part id").to_string();
            let origin_x = |app: &VcsDocumentApp<Puzzle5dPlayApp>| -> f64 {
                app.projection().expect("projection").get("parts").and_then(Value::as_array).and_then(|parts| parts.iter().find(|part| part.get("id").and_then(Value::as_str) == Some(part_id.as_str()))).and_then(|part| part.pointer("/3d/origin/0")).and_then(Value::as_f64).unwrap_or(0.0)
            };
            let start = origin_x(&app);
            let move_view = ViewState { active_tool_id: Some("move".into()), ..ViewState::default() };
            for dx in [1.0, 2.0, 3.0] {
                app.handle_action("translateSelection", Some(&json!({ "ids": [part_id], "dx": dx, "dy": 0.0, "dz": 0.0 })), &move_view, &meta("local")).expect("drag tick");
            }
            assert!((origin_x(&app) - start - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            assert!((origin_x(&app) - start).abs() < 1e-9, "one undo restores the whole coalesced gumball drag");
        }
        //#endregion 🧰 Window Actions & Tools contract
    }
    //#endregion 🧪Tests
}

use std::sync::LazyLock;

use semio_framework_plugin::{install_plugin_bundle, PluginBundle};

//#region 🔖Bundle
fn register_puzzle_exports() {
    d2::register_puzzle2d_exports();
    d3::register_puzzle3d_exports();
    d5::register_puzzle5d_exports();
}

semio_framework_plugin::semio_plugin! {
    id: "puzzle",
    label: "Puzzle",
    version: "0.1.0",
    setup: register_puzzle_exports,
    apps: [
        d2::create_puzzle2d_app => d2::Puzzle2dPlayApp,
        d3::create_puzzle3d_app => d3::Puzzle3dPlayApp,
        d5::create_puzzle5d_app => d5::Puzzle5dPlayApp,
    ]
}
//#endregion 🔖Bundle
