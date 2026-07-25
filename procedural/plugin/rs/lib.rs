//! 🔧 Procedural plugin — 2D and 3D flow apps in one hot-swappable WASM component.

pub mod app_2d {
    //! 🎲 Procedural 2D plugin — procedural flow play app bundled as a hot-swappable WASM component.

    use flow_core::{dag::DagFixture, flow_backed_node_graph_extras, flow_neuron_kind_infos_json, forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec}, CameraJson, FlowEvalDriver, FlowFixture, FlowHost, Widget};
    use flow_module_draw::render_scene_json;
    use procedural_2d::{procedural2d_fixture_operations, Procedural2dDocument, Procedural2dOperation, PROCEDURAL_2D_SCHEMA};
    use protocol::{
        apply_generation_operation, generation_operations, render_generation_form_body, render_generation_preview_text,
        render_generations_tree, select_generation, selected_generation, GenerationPlayState,
    };
    use semio_framework_plugin::{SurfaceKind, PanelGroup,
        build_canvas_2d_scene, build_node_graph_scene, create_default_layout, create_named_layout,
        ui_inspector_groups_to_tree, ui_inspector_readonly_field,
        ui_stack_vertical, ui_text, tree_item_with_action, ActionArgDef, ActionArgOption, ActionEmit, App,
        AppLabelsOverlayExt, Canvas2dScene, ActionDescriptor, DocumentApp, DocumentView,
        NodeGraphScene, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelTreeBuilder, ResourceKindSpec, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode,
        ViewState,
        FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
        FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    };
    use serde::Serialize;
    use serde_json::{json, Value};

    //#region 🔖Constants
    const PROCEDURAL2D_PLAY_APP_ID: &str = "procedural2d-play";
    const PROCEDURAL2D_PLAY_SURFACE_MAIN: &str = "procedural2d.play.main";
    const PROCEDURAL2D_PLAY_SURFACE_PREVIEW: &str = "procedural2d.play.preview";
    const PROCEDURAL2D_PLAY_BODY_MAIN: &str = "procedural2d.play.main";
    const PROCEDURAL2D_PLAY_BODY_PREVIEW: &str = "procedural2d.play.preview";
    const PROCEDURAL2D_PLAY_BODY_DOCUMENT: &str = "procedural2d.play.document";
    const PROCEDURAL2D_PLAY_BODY_CATALOGUE: &str = "procedural2d.play.catalogue";
    const PROCEDURAL2D_PLAY_BODY_INSPECTION: &str = "procedural2d.play.inspection";
    const PROCEDURAL2D_PLAY_WINDOW_MAIN: &str = "procedural2d-main";
    const PROCEDURAL2D_PLAY_WINDOW_PREVIEW: &str = "procedural2d-preview";
    const PROCEDURAL2D_PLAY_WINDOW_GENERATIONS: &str = "procedural2d-generations";
    const PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM: &str = "procedural2d-generate-form";
    const PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW: &str = "procedural2d-generate-preview";
    const PROCEDURAL2D_PLAY_BODY_GENERATIONS: &str = "procedural2d.play.generations";
    const PROCEDURAL2D_PLAY_BODY_GENERATE_FORM: &str = "procedural2d.play.generate-form";
    const PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW: &str = "procedural2d.play.generate-preview";
    const PROCEDURAL2D_PLAY_SURFACE_GENERATIONS: &str = "procedural2d.play.generations";
    const PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW: &str = "procedural2d.play.generate-preview";
    const DEFAULT_PROCEDURAL2D_FIXTURE_JSON: &str = include_str!("../../2d/example/default.procedural2d.json");
    //#endregion 🔖Constants

    //#region 🔖Types
    /// 👁️ Ephemeral per-session view state — never part of the persisted document. Selection, the
    /// graph camera, the active show mode, the off-main-thread eval driver, and the derived generation
    /// preview all live here on the app struct, out of the VCS document.
    #[derive(Clone, Debug)]
    struct Procedural2dPlayRuntime {
        selected_ids: Vec<String>,
        camera: CameraJson,
        show_mode: String,
        eval_driver: FlowEvalDriver,
        selected_generation_id: Option<String>,
        generation_preview_text: Option<String>,
    }

    impl Default for Procedural2dPlayRuntime {
        fn default() -> Self {
            Self {
                selected_ids: Vec::new(),
                camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
                show_mode: default_show_mode(),
                eval_driver: FlowEvalDriver::default(),
                selected_generation_id: None,
                generation_preview_text: None,
            }
        }
    }

    fn default_show_mode() -> String {
        "preview".into()
    }

    /// 🧾 Transient render/action bundle — the persisted projection (fixture + generations) with the
    /// ephemeral runtime's selection and derived preview overlaid, so the pure panel/render helpers
    /// keep reading a single value. Assembled per call; never serialized as the document.
    struct Procedural2dPlayView {
        fixture: FlowFixture,
        runtime: Procedural2dPlayRuntime,
        generation: GenerationPlayState,
    }

    /// 🧾 Overlays the ephemeral runtime's selection and derived preview onto the persisted
    /// generation state to build a {@link Procedural2dPlayView} for rendering.
    fn play_view(projection: &Procedural2dDocument, runtime: &Procedural2dPlayRuntime) -> Procedural2dPlayView {
        let mut generation = projection.generation.clone();
        generation.selected_generation_id = runtime.selected_generation_id.clone();
        generation.preview_text = runtime.generation_preview_text.clone();
        Procedural2dPlayView { fixture: projection.fixture.clone(), runtime: runtime.clone(), generation }
    }
    //#endregion 🔖Types

    //#region 🔖DocumentHelpers
    fn default_fixture() -> FlowFixture {
        serde_json::from_str(DEFAULT_PROCEDURAL2D_FIXTURE_JSON).unwrap_or_default()
    }

    fn default_projection() -> Procedural2dDocument {
        Procedural2dDocument { fixture: default_fixture(), generation: GenerationPlayState::default() }
    }

    fn procedural2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor {
            controller_id: PROCEDURAL2D_PLAY_APP_ID.into(),
            action: action.into(),
            args,
        }
    }

    /// 🧠 Process-wide [`flow_core::neural::NeuralCache`] shared across `FlowHost` reconstructions —
    /// lets a `flowEvalTick` chain's per-tick host rebuild pick up earlier ticks' cached node outputs
    /// instead of recomputing the whole graph from scratch every tick.
    static PROCEDURAL2D_NEURAL_CACHE: std::sync::OnceLock<std::sync::Arc<flow_core::neural::NeuralCache>> = std::sync::OnceLock::new();

    fn procedural2d_neural_cache() -> std::sync::Arc<flow_core::neural::NeuralCache> {
        PROCEDURAL2D_NEURAL_CACHE.get_or_init(|| std::sync::Arc::new(flow_core::neural::NeuralCache::new())).clone()
    }

    fn host_from_fixture(fixture: &FlowFixture) -> FlowHost {
        let mut host = FlowHost::from_fixture_with_cache(fixture.clone(), procedural2d_neural_cache());
        host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
        host
    }

    /// 🎯 `semio_framework_plugin::selection_ids`'s "ids" array plus a singular "nodeIds" fallback —
    /// this app's actions accept either shape depending on the caller.
    fn selection_ids(args: Option<&Value>) -> Vec<String> {
        let ids = semio_framework_plugin::selection_ids(args);
        if !ids.is_empty() {
            return ids;
        }
        args.and_then(|value| value.get("nodeIds"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .unwrap_or_default()
    }

    fn split_endpoint(endpoint: &str) -> (String, String) {
        endpoint
            .split_once(':')
            .map(|(node, port)| (node.to_string(), port.to_string()))
            .unwrap_or_else(|| (endpoint.to_string(), "out".into()))
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct MediaGraphPortRecord {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct MediaGraphNodeRecord {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        inputs: Vec<MediaGraphPortRecord>,
        outputs: Vec<MediaGraphPortRecord>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct MediaGraphEdgeRecord {
        id: String,
        source_node_id: String,
        source_port_id: String,
        target_node_id: String,
        target_port_id: String,
    }

    fn fixture_to_media_graph(fixture: &DagFixture) -> (String, String) {
        let nodes: Vec<MediaGraphNodeRecord> = fixture
            .nodes
            .iter()
            .map(|node| MediaGraphNodeRecord {
                id: node.id.clone(),
                label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
                x: node.x,
                y: node.y,
                width: node.width,
                height: node.height,
                inputs: node
                    .inputs()
                    .iter()
                    .filter(|port| port.visible)
                    .map(|port| MediaGraphPortRecord {
                        id: format!("{}:{}", node.id, port.id),
                        label: Some(port.label.clone()),
                    })
                    .collect(),
                outputs: node
                    .outputs()
                    .iter()
                    .filter(|port| port.visible)
                    .map(|port| MediaGraphPortRecord {
                        id: format!("{}:{}", node.id, port.id),
                        label: Some(port.label.clone()),
                    })
                    .collect(),
            })
            .collect();
        let edges: Vec<MediaGraphEdgeRecord> = fixture
            .edges
            .iter()
            .map(|edge| {
                let (source_node_id, source_port_id) = split_endpoint(&edge.source);
                let (target_node_id, target_port_id) = split_endpoint(&edge.target);
                MediaGraphEdgeRecord {
                    id: edge.id.clone(),
                    source_node_id,
                    source_port_id,
                    target_node_id,
                    target_port_id,
                }
            })
            .collect();
        (
            serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
            serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
        )
    }

    fn widget_id(widget: &Widget) -> &str {
        match widget {
            Widget::Neuron { id, .. }
            | Widget::InputSlider { id, .. }
            | Widget::InputNote { id, .. }
            | Widget::InputImage { id, .. }
            | Widget::Variable { id, .. }
            | Widget::OutputPreview { id, .. }
            | Widget::OutputAction { id, .. }
            | Widget::OutputExport { id, .. }
            | Widget::Cluster { id, .. } => id,
        }
    }

    fn collect_drawing_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                    if handle.starts_with("drawing-") {
                        handles.push(handle.into());
                    }
                }
                for entry in map.values() {
                    collect_drawing_handles_from_eval(entry, handles);
                }
            }
            Value::Array(items) => {
                for item in items {
                    collect_drawing_handles_from_eval(item, handles);
                }
            }
            _ => {}
        }
    }

    fn affine_transform_array(value: &Value) -> [f64; 6] {
        if let Some(matrix) = value.as_array() {
            let mut out = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
            for (index, entry) in matrix.iter().take(6).enumerate() {
                out[index] = entry.as_f64().unwrap_or(if index == 0 || index == 3 { 1.0 } else { 0.0 });
            }
            return out;
        }
        if let Some(matrix) = value.get("0").and_then(|entry| entry.as_array()) {
            let wrapped = Value::Array(matrix.clone());
            return affine_transform_array(&wrapped);
        }
        [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
    }

    fn path_segments_from_node(node: &Value) -> Vec<Value> {
        if let Some(segments) = node.get("segments").and_then(|entry| entry.as_array()) {
            return segments.clone();
        }
        for key in ["path", "shape", "line", "polyline", "rect", "ellipse", "circle", "polygon"] {
            if let Some(inner) = node.get(key) {
                if let Some(segments) = inner.get("segments").and_then(|entry| entry.as_array()) {
                    return segments.clone();
                }
            }
        }
        Vec::new()
    }

    fn scene_layers_from_drawing_handle(handle: &str, prefix: &str) -> Vec<Value> {
        let scene_json = render_scene_json(handle);
        let Ok(scene) = serde_json::from_str::<Value>(&scene_json) else {
            return Vec::new();
        };
        if scene.get("error").is_some() {
            return Vec::new();
        }
        let Some(nodes) = scene.get("nodes").and_then(|entry| entry.as_array()) else {
            return Vec::new();
        };
        nodes
            .iter()
            .enumerate()
            .map(|(index, node)| {
                let node_body = node.get("node").unwrap_or(node);
                json!({
                    "id": format!("{prefix}-{handle}-{index}"),
                    "transform": affine_transform_array(node.get("transform").unwrap_or(&Value::Null)),
                    "segments": path_segments_from_node(node_body),
                    "fill": node.get("fill").cloned().unwrap_or(Value::Null),
                    "stroke": node.get("stroke").cloned().unwrap_or(Value::Null),
                    "opacity": node.get("opacity").and_then(|entry| entry.as_f64()).unwrap_or(1.0),
                    "blendMode": "normal",
                    "visible": true,
                    "needsKernel": false,
                })
            })
            .collect()
    }

    fn eval_preview_layers(play: &Procedural2dPlayView, preview: bool) -> String {
        // 🧵 Never evaluates: reads whatever the off-main-thread `flowEvalTick` chain (or an explicit
        // generation-preview/`setEvalOutputs` push) has cached so far — stale/empty is fine, the next
        // tick's scene refresh fills it in.
        let eval_json = play.runtime.eval_driver.eval_json();
        let prefix = if preview { "procedural2d-preview" } else { "procedural2d-main" };
        let mut layers = Vec::new();
        if let Ok(outputs) = serde_json::from_str::<Value>(eval_json) {
            let mut handles = Vec::new();
            collect_drawing_handles_from_eval(&outputs, &mut handles);
            handles.sort();
            handles.dedup();
            for handle in handles {
                layers.extend(scene_layers_from_drawing_handle(&handle, prefix));
            }
        }
        if play.runtime.show_mode == "wire" {
            let offset = if preview { 240.0 } else { 0.0 };
            for widget in &play.fixture.widgets {
                let id = widget_id(widget).to_string();
                if play.runtime.selected_ids.is_empty() || play.runtime.selected_ids.iter().any(|selected| selected == &id) {
                    let (x, y) = play
                        .fixture
                        .layout
                        .get(&id)
                        .map(|layout| (layout.x, layout.y))
                        .unwrap_or((offset + 48.0, 240.0));
                    layers.push(json!({
                        "id": format!("widget-{id}"),
                        "kind": "node",
                        "name": id,
                        "x": x,
                        "y": y,
                        "width": 96.0,
                        "height": 48.0,
                    }));
                }
            }
        }
        serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
    }

    fn evaluate_generation_preview(fixture: &FlowFixture, values: &serde_json::Map<String, Value>) -> String {
        let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
        let patched = apply_generation_values_to_fixture(&fixture_json, values);
        let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
        let mut host = FlowHost::from_fixture(patched_fixture);
        host.evaluate().unwrap_or_default()
    }

    fn generation_preview_layers(eval_json: &str) -> String {
        let prefix = "procedural2d-generate-preview";
        let mut layers = Vec::new();
        if let Ok(outputs) = serde_json::from_str::<Value>(eval_json) {
            let mut handles = Vec::new();
            collect_drawing_handles_from_eval(&outputs, &mut handles);
            handles.sort();
            handles.dedup();
            for handle in handles {
                layers.extend(scene_layers_from_drawing_handle(&handle, prefix));
            }
        }
        serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
    }

    /// 👁️ Recomputes the ephemeral generation preview for the currently selected generation and
    /// stores it on the runtime (never on the persisted document).
    fn refresh_generation_preview(
        runtime: &mut Procedural2dPlayRuntime,
        fixture: &FlowFixture,
        generation: &GenerationPlayState,
    ) {
        let Some(selected) = selected_generation(generation) else {
            runtime.generation_preview_text = None;
            return;
        };
        let preview = evaluate_generation_preview(fixture, &selected.values);
        runtime.generation_preview_text = Some(preview.clone());
        runtime.eval_driver.set_eval_json(preview);
    }
    //#endregion 🔖DocumentHelpers

    //#region 🔖Terminology
    semio_framework_plugin::app_labels! {
        /// 🗣️ Complete UI label set for the 2D flow app; one field per label makes every locale combination compile-checked.
        struct Procedural2dLabels {
            sources: &'static str = en: "Sources", de: "Quellen";
            components: &'static str = en: "Components", de: "Komponenten";
            sinks: &'static str = en: "Sinks", de: "Senken";
            show_mode_section: &'static str = en: "Show mode", de: "Anzeigemodus";
            show_prefix: &'static str = en: "Show", de: "Anzeigen";
            none: &'static str = en: "(none)", de: "(keine)";
            selection: &'static str = en: "Selection", de: "Auswahl";
            ids: &'static str = en: "Ids", de: "Kennungen";
            schema_prefix: &'static str = en: "Schema:", de: "Schema:";
            widgets_prefix: &'static str = en: "Widgets:", de: "Elemente:";
            show_mode_prefix: &'static str = en: "Show mode:", de: "Anzeigemodus:";
            generate_hint: &'static str = en: "Add a generation to edit input values.", de: "Erstelle eine Generation, um Eingabewerte zu bearbeiten.";
            preview_hint: &'static str = en: "(evaluate a generation to preview output)", de: "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)";
            source_slider: &'static str = en: "Slider", de: "Schieberegler";
            source_note: &'static str = en: "Note", de: "Notiz";
            component_add: &'static str = en: "Add", de: "Addieren";
            component_and: &'static str = en: "And", de: "Und";
            component_concat: &'static str = en: "Concat", de: "Verketten";
            sink_preview: &'static str = en: "Preview", de: "Vorschau";
            sink_export: &'static str = en: "Export", de: "Export";
            window_main: &'static str = en: "Flow", de: "Fluss";
            window_preview: &'static str = en: "Preview", de: "Vorschau";
            window_generations: &'static str = en: "Generations", de: "Generationen";
            window_generate_form: &'static str = en: "Form", de: "Formular";
            window_generate_preview: &'static str = en: "Preview", de: "Vorschau";
            delete_selection: &'static str = en: "Delete selection", de: "Auswahl löschen";
        }
    }

    /// 🗣️ Resolves the active label set from the shell-provided locale; falls back to native English.
    fn procedural2d_labels(view_state: &ViewState) -> &'static Procedural2dLabels {
        semio_framework_plugin::resolve_labels::<Procedural2dLabels>(view_state)
    }
    //#endregion 🔖Terminology

    //#region 🔖CommandLabels
    /// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_procedural2d_app`'s
    /// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
    /// palette and Actions rail get a translated label without threading locale through the whole builder chain.
    fn procedural2d_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
        const ENTRIES: &[(&str, &str, &str)] = &[
            ("nodeGraphViewport", "Set Viewport", "Ansicht festlegen"),
            ("nodeGraphEdit", "Edit Graph", "Graph bearbeiten"),
            ("moveMediaNode", "Move Node", "Knoten verschieben"),
            ("addWidget", "Add Widget", "Element hinzufügen"),
            ("removeWidget", "Remove Widget", "Element entfernen"),
            ("connectMediaPorts", "Connect Ports", "Ports verbinden"),
            ("reorganize", "Reorganize", "Neu anordnen"),
            ("addGeneration", "Add Generation", "Generation hinzufügen"),
            ("removeGeneration", "Remove Generation", "Generation entfernen"),
            ("renameGeneration", "Rename Generation", "Generation umbenennen"),
            ("updateGenerationValues", "Update Generation Values", "Generationswerte aktualisieren"),
            ("setSelection", "Set Selection", "Auswahl festlegen"),
            ("selectNode", "Select Node", "Knoten auswählen"),
            ("nodeGraphSelect", "Node Graph Select", "Graph-Auswahl"),
            ("nodeGraphHover", "Node Graph Hover", "Graph-Hover"),
            ("setShowMode", "Set Show Mode", "Anzeigemodus festlegen"),
            ("generate", "Generate", "Generieren"),
            ("setEvalOutputs", "Set Eval Outputs", "Auswertungsausgaben festlegen"),
            ("canvasPointerDown", "Canvas Pointer Down", "Canvas-Zeiger gedrückt"),
            ("canvasPointerMove", "Canvas Pointer Move", "Canvas-Zeiger bewegt"),
            ("canvasPointerUp", "Canvas Pointer Up", "Canvas-Zeiger losgelassen"),
            ("canvasWheel", "Canvas Wheel", "Canvas-Mausrad"),
            ("selectGeneration", "Select Generation", "Generation auswählen"),
        ];
        semio_framework_plugin::localized_label_map(is_de, ENTRIES)
    }
    //#endregion 🔖CommandLabels

    //#region 🔖Panels
    fn build_document_tree(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
        let widget_items: Vec<UiTreeItemNode> = play
            .fixture
            .widgets
            .iter()
            .map(|widget| {
                let id = widget_id(widget).to_string();
                tree_item_with_action(
                    format!("procedural2d-play-document.widget.{id}"),
                    id.clone(),
                    None,
                    procedural2d_action("setSelection", Some(json!({ "ids": [id] }))),
                )
            })
            .collect();
        PanelTreeBuilder::new("procedural2d-play-document")
            .section_or_placeholder(
                "procedural2d-play-document.widgets",
                Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
                true,
                widget_items,
                labels.none,
            )
            .selected(play.runtime.selected_ids.iter().map(|id| format!("procedural2d-play-document.widget.{id}")).collect())
            .selection_change(procedural2d_action("setSelection", None))
            .build()
    }

    fn build_catalogue_tree(labels: &Procedural2dLabels) -> UiNode {
        let sources = [("inputSlider", labels.source_slider), ("inputNote", labels.source_note)];
        let components = [("math.add", labels.component_add), ("logic.and", labels.component_and), ("text.concat", labels.component_concat)];
        let sinks = [("outputPreview", labels.sink_preview), ("outputExport", labels.sink_export)];
        PanelTreeBuilder::new("procedural2d-play-catalogue")
            .section(
                "procedural2d-play-catalogue.sources",
                Some(labels.sources.into()),
                true,
                sources
                    .iter()
                    .map(|(kind, label)| {
                        tree_item_with_action(
                            format!("procedural2d-play-catalogue.source.{kind}"),
                            *label,
                            None,
                            procedural2d_action("addWidget", Some(json!({ "kind": kind }))),
                        )
                    })
                    .collect(),
            )
            .section(
                "procedural2d-play-catalogue.components",
                Some(labels.components.into()),
                true,
                components
                    .iter()
                    .map(|(kind, label)| {
                        tree_item_with_action(
                            format!("procedural2d-play-catalogue.component.{kind}"),
                            *label,
                            None,
                            procedural2d_action("addWidget", Some(json!({ "kind": "neuron", "neuronKind": kind }))),
                        )
                    })
                    .collect(),
            )
            .section(
                "procedural2d-play-catalogue.sinks",
                Some(labels.sinks.into()),
                true,
                sinks
                    .iter()
                    .map(|(kind, label)| {
                        tree_item_with_action(
                            format!("procedural2d-play-catalogue.sink.{kind}"),
                            *label,
                            None,
                            procedural2d_action("addWidget", Some(json!({ "kind": kind }))),
                        )
                    })
                    .collect(),
            )
            .section(
                "procedural2d-play-catalogue.modes",
                Some(labels.show_mode_section.into()),
                false,
                ["preview", "generate", "wire"]
                    .iter()
                    .map(|mode| {
                        tree_item_with_action(
                            format!("procedural2d-play-catalogue.mode.{mode}"),
                            format!("{} {mode}", labels.show_prefix),
                            None,
                            procedural2d_action("setShowMode", Some(json!({ "value": mode }))),
                        )
                    })
                    .collect(),
            )
            .build()
    }

    fn build_inspector_tree(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
        if play.runtime.selected_ids.is_empty() {
            return ui_stack_vertical(vec![
                ui_text(format!("{} flow.fixture", labels.schema_prefix)),
                ui_text(format!("{} {}", labels.widgets_prefix, play.fixture.widgets.len())),
                ui_text(format!("{} {}", labels.show_mode_prefix, play.runtime.show_mode)),
            ]);
        }
        ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(),
            id: "procedural2d-play-inspector.selection".into(),
            label: labels.selection.into(),
            default_open: Some(true),
            fields: vec![ui_inspector_readonly_field(
                "procedural2d-play-inspector.ids",
                labels.ids,
                play.runtime.selected_ids.join(", "),
            )],
        }])
    }
    //#endregion 🔖Panels

    //#region 🔖Render
    fn render_main_graph(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
        let host = host_from_fixture(&play.fixture);
        let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
        let viewport_json = serde_json::to_string(&play.runtime.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
        let selection_json = if play.runtime.selected_ids.is_empty() {
            None
        } else {
            serde_json::to_string(&play.runtime.selected_ids).ok()
        };
        let flow_extras = flow_backed_node_graph_extras(&play.fixture, "", 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, Some(&play.runtime.eval_driver));
        let context_menu_json = serde_json::to_string(&json!([{
            "id": "delete-selection",
            "label": labels.delete_selection,
            "icon": "trash",
            "action": "nodeGraphEdit",
            "args": { "operations": [{ "operation": "deleteSelection" }] },
            "destructive": true,
        }]))
        .ok();
        build_node_graph_scene(
            PROCEDURAL2D_PLAY_SURFACE_MAIN,
            PROCEDURAL2D_PLAY_APP_ID,
            NodeGraphScene {
                editable: Some(true),
                operators_json: flow_extras.operators_json,
                capabilities_json: flow_extras.capabilities_json,
                lod_json: flow_extras.lod_json,
                fixture_json: flow_extras.fixture_json,
                selection_json,
                context_menu_json,
                ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
            },
        )
    }

    fn render_preview_canvas(play: &Procedural2dPlayView) -> UiNode {
        build_canvas_2d_scene(
            PROCEDURAL2D_PLAY_SURFACE_PREVIEW,
            PROCEDURAL2D_PLAY_APP_ID,
            Canvas2dScene {
                camera_x: play.runtime.camera.x,
                camera_y: play.runtime.camera.y,
                zoom: play.runtime.camera.zoom,
                layers_json: eval_preview_layers(play, true),
            },
        )
    }

    fn render_generate_generations(play: &Procedural2dPlayView) -> UiNode {
        render_generations_tree(
            PROCEDURAL2D_PLAY_APP_ID,
            "procedural2d-play-generate",
            &play.generation.generations,
            play.generation.selected_generation_id.as_deref(),
        )
    }

    fn render_generate_form(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
        let spec = flow_fixture_to_form_spec(&play.fixture);
        let Some(generation) = selected_generation(&play.generation) else {
            return ui_text(labels.generate_hint);
        };
        render_generation_form_body(
            &spec,
            &generation.values,
            PROCEDURAL2D_PLAY_APP_ID,
            "updateGenerationValues",
            &generation.id,
        )
    }

    fn render_generate_preview(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
        let eval_json = play
            .generation
            .preview_text
            .as_deref()
            .filter(|value| !value.is_empty())
            .unwrap_or("");
        if eval_json.is_empty() {
            return ui_text(labels.preview_hint);
        }
        let layers = generation_preview_layers(eval_json);
        if layers == "[]" {
            return render_generation_preview_text(
                PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW,
                PROCEDURAL2D_PLAY_APP_ID,
                eval_json,
            );
        }
        build_canvas_2d_scene(
            PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW,
            PROCEDURAL2D_PLAY_APP_ID,
            Canvas2dScene {
                camera_x: play.runtime.camera.x,
                camera_y: play.runtime.camera.y,
                zoom: play.runtime.camera.zoom,
                layers_json: layers,
            },
        )
    }
    //#endregion 🔖Render

    //#region 🔖Procedural2dPlayApp
    #[derive(Default)]
    pub struct Procedural2dPlayApp {
        runtime: Procedural2dPlayRuntime,
    }

    impl Procedural2dPlayApp {
        /// 🔀 Runs a host mutation seeded from the projection fixture and diffs the result into operations.
        /// Diffs against the host-normalized baseline (not the raw projection) so `FlowHost`'s own
        /// dedupe/dag-rebuild normalization does not leak spurious collection operations — only the actual
        /// mutation becomes an operation, which keeps concurrent disjoint edits mergeable on the backbone.
        fn ops_from_host_mutation(
            &self,
            fixture: &FlowFixture,
            mutate: impl FnOnce(&mut FlowHost),
        ) -> Vec<Procedural2dOperation> {
            let mut host = host_from_fixture(fixture);
            let baseline = host.fixture.clone();
            mutate(&mut host);
            procedural2d_fixture_operations(&baseline, &host.fixture)
        }

        /// 🧬 Emits generation operations for the generate-mode actions, updating ephemeral selection and
        /// preview from the post-operation state. `selectGeneration` is a view action (no operations).
        fn handle_generation(
            &mut self,
            action: &str,
            args: Option<&Value>,
            projection: &Procedural2dDocument,
        ) -> ActionEmit<Procedural2dOperation> {
            let spec = flow_fixture_to_form_spec(&projection.fixture);
            let mut state = projection.generation.clone();
            state.selected_generation_id = self.runtime.selected_generation_id.clone();
            if action == "selectGeneration" {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    select_generation(&mut state, id);
                }
                self.runtime.selected_generation_id = state.selected_generation_id.clone();
                refresh_generation_preview(&mut self.runtime, &projection.fixture, &state);
                return ActionEmit::default();
            }
            let Some(operations) = generation_operations(action, args, &state, &spec) else {
                return ActionEmit::default();
            };
            for operation in &operations {
                apply_generation_operation(&mut state, operation);
            }
            self.runtime.selected_generation_id = state.selected_generation_id.clone();
            refresh_generation_preview(&mut self.runtime, &projection.fixture, &state);
            let coalesce_key = (action == "updateGenerationValues").then(|| "generation-values".to_string());
            ActionEmit {
                operations: operations.into_iter().map(Procedural2dOperation::Generation).collect(),
                coalesce_key,
                ..Default::default()
            }
        }
    }

    impl DocumentApp for Procedural2dPlayApp {
        type Projection = Procedural2dDocument;
        type Operation = Procedural2dOperation;

        fn app_id(&self) -> &str {
            PROCEDURAL2D_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            PROCEDURAL_2D_SCHEMA
        }

        fn initial_projection(&self) -> Procedural2dDocument {
            default_projection()
        }

        fn handle_action(
            &mut self,
            action: &str,
            args: Option<&Value>,
            doc: &DocumentView<'_, Procedural2dDocument>,
            _view_state: &ViewState,
        ) -> ActionEmit<Procedural2dOperation> {
            let fixture = &doc.projection.fixture;
            match action {
                // 👁️ View actions — mutate ephemeral runtime, emit no operations.
                "setSelection" | "selectNode" | "nodeGraphSelect" => {
                    self.runtime.selected_ids = selection_ids(args);
                    ActionEmit::default()
                }
                "nodeGraphHover" => ActionEmit::default(),
                "setShowMode" => {
                    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                        self.runtime.show_mode = mode.into();
                    }
                    ActionEmit::default()
                }
                "generate" => {
                    self.runtime.show_mode = "generate".into();
                    ActionEmit::default()
                }
                "setEvalOutputs" => {
                    if let Some(outputs) = args.and_then(|value| value.get("outputs")) {
                        self.runtime.eval_driver.set_eval_json(outputs.to_string());
                    } else if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                        self.runtime.eval_driver.set_eval_json(json_text.into());
                    }
                    ActionEmit::default()
                }
                "flowEvalTick" => {
                    let mut host = host_from_fixture(fixture);
                    let more = self.runtime.eval_driver.tick(&mut host);
                    ActionEmit { effects: if more { vec![semio_framework_core::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() }, ..ActionEmit::default() }
                }
                "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" | "canvasWheel" => ActionEmit::default(),
                // 📷 Graph camera — ephemeral view state (never a document operation), same model as flow-play.
                "nodeGraphViewport" => {
                    if let Some(camera) = args
                        .and_then(|value| value.get("viewportJson"))
                        .and_then(|value| value.as_str())
                        .and_then(|json| serde_json::from_str(json).ok())
                    {
                        self.runtime.camera = camera;
                    }
                    ActionEmit::default()
                }
                // ✏️ Operations — compute the target fixture via the host, emit fixture operations.
                "nodeGraphEdit" => {
                    let sub_operations = args
                        .and_then(|value| value.get("operations"))
                        .and_then(|value| value.as_array())
                        .cloned()
                        .unwrap_or_default();
                    let selected = self.runtime.selected_ids.clone();
                    let mut cleared = false;
                    let operations = self.ops_from_host_mutation(fixture, |host| {
                        for operation in &sub_operations {
                            match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                                "setFixture" => {
                                    if let Some(fixture) = operation
                                        .get("fixtureJson")
                                        .and_then(|value| value.as_str())
                                        .and_then(|json| serde_json::from_str::<FlowFixture>(json).ok())
                                    {
                                        host.replace_fixture(fixture);
                                    }
                                }
                                "deleteSelection" => {
                                    for id in &selected {
                                        if host.remove_widget(id).is_ok() {
                                            cleared = true;
                                        }
                                    }
                                }
                                "connect" => {
                                    let from = operation.get("sourceNodeId").and_then(|value| value.as_str());
                                    let from_port = operation.get("sourcePortId").and_then(|value| value.as_str());
                                    let to = operation.get("targetNodeId").and_then(|value| value.as_str());
                                    let to_port = operation.get("targetPortId").and_then(|value| value.as_str());
                                    if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                                        let _ = host.connect_ports(from, from_port, to, to_port);
                                    }
                                }
                                _ => {}
                            }
                        }
                    });
                    if cleared {
                        self.runtime.selected_ids.clear();
                    }
                    ActionEmit::operations(operations)
                }
                "moveMediaNode" => {
                    let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(str::to_string);
                    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                    if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                        return ActionEmit::operations(self.ops_from_host_mutation(fixture, |host| {
                            let _ = host.move_widget(&node_id, x, y);
                        }));
                    }
                    ActionEmit::default()
                }
                "addWidget" => {
                    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("inputSlider");
                    let descriptor = match kind {
                        "neuron" => json!({
                            "kind": "neuron",
                            "neuronKind": args.and_then(|value| value.get("neuronKind")).and_then(|value| value.as_str()).unwrap_or("math.add"),
                        })
                        .to_string(),
                        other => json!({ "kind": other }).to_string(),
                    };
                    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                    let mut host = host_from_fixture(fixture);
                    let baseline = host.fixture.clone();
                    if let Ok(id) = host.add_widget(&descriptor, x, y) {
                        self.runtime.selected_ids = vec![id];
                        return ActionEmit::operations(procedural2d_fixture_operations(&baseline, &host.fixture));
                    }
                    ActionEmit::default()
                }
                "removeWidget" => {
                    let widget_id = args.and_then(|value| value.get("widgetId")).and_then(|value| value.as_str()).map(str::to_string);
                    if let Some(widget_id) = widget_id {
                        let operations = self.ops_from_host_mutation(fixture, |host| {
                            let _ = host.remove_widget(&widget_id);
                        });
                        if !operations.is_empty() {
                            self.runtime.selected_ids.retain(|id| id != &widget_id);
                        }
                        return ActionEmit::operations(operations);
                    }
                    ActionEmit::default()
                }
                "connectMediaPorts" => {
                    let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                    let from_port = args.and_then(|value| value.get("sourcePortId")).and_then(|value| value.as_str()).map(str::to_string);
                    let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                    let to_port = args.and_then(|value| value.get("targetPortId")).and_then(|value| value.as_str()).map(str::to_string);
                    if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                        return ActionEmit::operations(self.ops_from_host_mutation(fixture, |host| {
                            let _ = host.connect_ports(&from, &from_port, &to, &to_port);
                        }));
                    }
                    ActionEmit::default()
                }
                "reorganize" => ActionEmit::operations(self.ops_from_host_mutation(fixture, |host| {
                    let _ = host.reorganize(r#"{"orientation":"leftRight"}"#);
                })),
                "addGeneration" | "removeGeneration" | "selectGeneration" | "renameGeneration" | "updateGenerationValues" => {
                    self.handle_generation(action, args, doc.projection)
                }
                _ => ActionEmit::default(),
            }
        }

        /// 🧵 Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes —
        /// covers every mutation path (edits, undo/redo, remote operations) in one place instead of each
        /// action re-checking. `FlowEvalDriver::sync` is cheap when nothing changed.
        fn pending_effects(&mut self, doc: &DocumentView<'_, Procedural2dDocument>, _view_state: &ViewState) -> Vec<semio_framework_core::kernel::HostEffect> {
            let host = host_from_fixture(&doc.projection.fixture);
            if self.runtime.eval_driver.sync(&host) {
                vec![semio_framework_core::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
            } else {
                Vec::new()
            }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, Procedural2dDocument>, view_state: &ViewState) -> UiNode {
            let play = play_view(doc.projection, &self.runtime);
            let labels = procedural2d_labels(view_state);
            match body_key {
                PROCEDURAL2D_PLAY_BODY_MAIN => render_main_graph(&play, labels),
                PROCEDURAL2D_PLAY_BODY_PREVIEW => render_preview_canvas(&play),
                PROCEDURAL2D_PLAY_BODY_GENERATIONS => render_generate_generations(&play),
                PROCEDURAL2D_PLAY_BODY_GENERATE_FORM => render_generate_form(&play, labels),
                PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(&play, labels),
                PROCEDURAL2D_PLAY_BODY_DOCUMENT => build_document_tree(&play, labels),
                PROCEDURAL2D_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
                PROCEDURAL2D_PLAY_BODY_INSPECTION => build_inspector_tree(&play, labels),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
            let labels = procedural2d_labels(view_state);
            let is_de = semio_framework_plugin::is_de_locale(view_state);
            semio_framework_plugin::AppLabelsOverlay::default()
                .window_kind_label(PROCEDURAL2D_PLAY_WINDOW_MAIN, labels.window_main)
                .window_kind_label(PROCEDURAL2D_PLAY_WINDOW_PREVIEW, labels.window_preview)
                .window_kind_label(PROCEDURAL2D_PLAY_WINDOW_GENERATIONS, labels.window_generations)
                .window_kind_label(PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM, labels.window_generate_form)
                .window_kind_label(PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW, labels.window_generate_preview)
                .mode_label("edit", if is_de { "Bearbeiten" } else { "Edit" })
                .mode_label("generate", if is_de { "Generieren" } else { "Generate" })
                .action_labels(procedural2d_action_labels(is_de))
                .example_labels(semio_framework_plugin::localized_label_map(is_de, &[("default", "Default", "Standard")]))
        }
    }
    //#endregion 🔖Procedural2dPlayApp

    //#region 🔖Manifest
    pub fn create_procedural2d_app() -> App {
        App::from_builder(
            App::builder(PROCEDURAL2D_PLAY_APP_ID, "Procedural 2D").document(["semio", "procedural", "2d"])
                .resource_kind(ResourceKindSpec {
                    id: "2d.procedural".into(),
                    name: "2D Procedural".into(),
                    source_format: "procedural.2d".into(),
                    component_kind: "procedural2d".into(),
                    dimension: "2d".into(),
                    media_capability: OsMediaCapability::MeshOnly,
                    media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Flow },
                    schema: "procedural.2d".into(),
                    export_formats: vec![],
                    import_formats: vec![],
                })
                .icon_id("procedural2d")
                .mode("edit", "Edit")
                .mode("generate", "Generate")
                .default_mode_id("edit")
                .window_kind(PROCEDURAL2D_PLAY_WINDOW_MAIN, "Flow", PROCEDURAL2D_PLAY_BODY_MAIN, SurfaceKind::NodeGraph)
                .window_kind(PROCEDURAL2D_PLAY_WINDOW_PREVIEW, "Preview", PROCEDURAL2D_PLAY_BODY_PREVIEW, SurfaceKind::Canvas2d)
                .window_kind(
                    PROCEDURAL2D_PLAY_WINDOW_GENERATIONS,
                    "Generations",
                    PROCEDURAL2D_PLAY_BODY_GENERATIONS,
                    SurfaceKind::Canvas2d,
                )
                .window_kind(PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM, "Form", PROCEDURAL2D_PLAY_BODY_GENERATE_FORM, SurfaceKind::Canvas2d)
                .window_kind(
                    PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW,
                    "Preview",
                    PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW,
                    SurfaceKind::Canvas2d,
                )
                .default_layout(create_default_layout(
                    &[PROCEDURAL2D_PLAY_WINDOW_MAIN.into(), PROCEDURAL2D_PLAY_WINDOW_PREVIEW.into()],
                    "row",
                    Some(&[55.0, 45.0]),
                    Some(&["Main".into(), "Preview".into()]),
                ))
                .named_layout(create_named_layout(
                    "procedural2d-generate",
                    "Generate",
                    create_default_layout(
                        &[
                            PROCEDURAL2D_PLAY_WINDOW_GENERATIONS.into(),
                            PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM.into(),
                            PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW.into(),
                        ],
                        "row",
                        Some(&[22.0, 43.0, 35.0]),
                        Some(&["Generations".into(), "Form".into(), "Preview".into()]),
                    ),
                    "builtin",
                    Some("sparkles".into()),
                    None,
                ))
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                    PanelGroup::Workbench,
                    PROCEDURAL2D_PLAY_BODY_DOCUMENT,
                )
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                    PanelGroup::Workbench,
                    PROCEDURAL2D_PLAY_BODY_CATALOGUE,
                )
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                    PanelGroup::Details,
                    PROCEDURAL2D_PLAY_BODY_INSPECTION,
                )
                // ✏️ Document-mutating operations — dispatched as VCS operations with a true inverse.
                .operation("nodeGraphEdit", "Edit Graph")
                .operation("moveMediaNode", "Move Node")
                .operation("addWidget", "Add Widget")
                .operation("removeWidget", "Remove Widget")
                .operation("connectMediaPorts", "Connect Ports")
                .operation("reorganize", "Reorganize")
                .operation("addGeneration", "Add Generation")
                .operation("removeGeneration", "Remove Generation")
                .operation("renameGeneration", "Rename Generation")
                .operation("updateGenerationValues", "Update Generation Values")
                // 👁️ Ephemeral view actions — selection, hover, camera, the show-mode display toggle, and evaluation scratch (emit no operations).
                .view_action("nodeGraphViewport", "Set Viewport")
                .view_action("setSelection", "Set Selection")
                .view_action("selectNode", "Select Node")
                .view_action("nodeGraphSelect", "Node Graph Select")
                .view_action("nodeGraphHover", "Node Graph Hover")
                .view_action("setShowMode", "Set Show Mode")
                .view_action("generate", "Generate")
                .view_action("setEvalOutputs", "Set Eval Outputs")
                .view_action("canvasPointerDown", "Canvas Pointer Down")
                .view_action("canvasPointerMove", "Canvas Pointer Move")
                .view_action("canvasPointerUp", "Canvas Pointer Up")
                .view_action("canvasWheel", "Canvas Wheel")
                .view_action("selectGeneration", "Select Generation")
                // 📝 Staged argument form for the palette-visible add-widget action (default materialized host-side).
                .action_args("addWidget", vec![
                    ActionArgDef::select("kind", "Kind", vec![
                        ActionArgOption::new("inputSlider", "Slider"),
                        ActionArgOption::new("inputNote", "Note"),
                        ActionArgOption::new("neuron", "Component"),
                        ActionArgOption::new("outputPreview", "Preview"),
                        ActionArgOption::new("outputExport", "Export"),
                    ]).default_value("inputSlider"),
                ])
                .keybinding("mod+z", "undo")
                .keybinding("mod+shift+z", "redo"),
        )
        .example("default", "Default", serde_json::to_string(&default_projection()).unwrap())
        .program("procedural2d", "Procedural 2D", "layout")
    }

    fn procedural2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
        semio_framework_os::title_card_svg(value, "Procedural 2D", 1024, 768)
    }

    fn procedural2d_document_from_dwg(_drawing: &semio_framework_core::DwgDrawing) -> Result<Value, String> {
        serde_json::to_value(default_projection()).map_err(|err| err.to_string())
    }

    pub fn register_procedural2d_exports() {
        semio_framework_os::register_2d_export_handlers("2d.procedural", "procedural2d", procedural2d_document_json_to_svg);
        semio_framework_os::register_dwg_import_handler("2d.procedural", procedural2d_document_from_dwg);
    }
    //#endregion 🔖Manifest

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::testkit;
        use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};

        fn meta(actor: &str) -> ActionMeta {
            testkit::meta(actor)
        }

        fn new_app() -> VcsDocumentApp<Procedural2dPlayApp> {
            testkit::new_app::<Procedural2dPlayApp>()
        }

        /// 🧬 A wrapper carrying the real action registry so default-materialization + kind discipline run.
        fn new_app_with_registry() -> VcsDocumentApp<Procedural2dPlayApp> {
            testkit::new_app_with_registry::<Procedural2dPlayApp>(create_procedural2d_app)
        }

        #[test]
        fn add_widget_materializes_declared_kind_default_into_an_operation() {
            let mut app = new_app_with_registry();
            let before = app.projection().expect("projection").fixture.widgets.len();
            // addWidget fired with no args: the declared `kind` default must materialize into a real widget operation.
            app.handle_action("addWidget", None, &ViewState::default(), &meta("local")).expect("add widget");
            assert_eq!(app.projection().expect("projection").fixture.widgets.len(), before + 1, "materialized default kind produced a document operation");
        }

        #[test]
        fn renders_main_graph_scene() {
            let mut app = new_app();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
        }

        #[test]
        fn main_graph_scene_exports_flow_backed_node_graph_fields() {
            let mut app = new_app();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
            let value: Value = serde_json::from_str(&serde_json::to_string(&node).unwrap()).expect("ui node json");
            let graph = value.get("nodeGraph").expect("nodeGraph");
            assert!(graph.get("fixtureJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow.fixture")));
            assert!(graph.get("operatorsJson").and_then(|v| v.as_str()).is_some());
            assert!(graph.get("capabilitiesJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow")));
        }

        #[test]
        fn renders_preview_canvas_scene() {
            let mut app = new_app();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("canvas-2d"));
        }

        #[test]
        fn document_lists_widgets() {
            let mut app = new_app();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("procedural2d-play-document.widget.rect"));
        }

        #[test]
        fn catalogue_lists_show_modes() {
            let mut app = new_app();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("procedural2d-play-catalogue.mode.preview"));
        }

        #[test]
        fn add_widget_emits_op_and_grows_document() {
            let mut app = new_app();
            let before = app.projection().expect("projection").fixture.widgets.len();
            app.handle_action("addWidget", Some(&json!({ "kind": "inputNote" })), &ViewState::default(), &meta("local")).expect("add");
            assert_eq!(app.projection().expect("projection").fixture.widgets.len(), before + 1);
        }

        #[test]
        fn add_widget_undo_redo_round_trip() {
            let mut app = new_app();
            let before = app.projection().expect("projection").fixture.widgets.len();
            testkit::assert_undo_redo_round_trip(
                &mut app,
                "addWidget",
                Some(&json!({ "kind": "inputNote" })),
                |app| app.projection().expect("projection").fixture.widgets.len(),
                before,
                before + 1,
            );
        }

        #[test]
        fn generate_is_a_view_action_with_no_document_operations() {
            let mut app = new_app();
            let before = app.projection().expect("projection");
            app.handle_action("generate", None, &ViewState::default(), &meta("local")).expect("generate");
            assert_eq!(app.projection().expect("projection"), before, "generate must not mutate the document");
        }

        #[test]
        fn add_generation_records_an_undoable_generation_operation() {
            let mut app = new_app();
            testkit::assert_undo_redo_round_trip(
                &mut app,
                "addGeneration",
                None,
                |app| app.projection().expect("projection").generation.generations.len(),
                0,
                1,
            );
        }

        #[test]
        fn generate_mode_renders_surfaces() {
            let mut app = new_app();
            let generations = app.render(PROCEDURAL2D_PLAY_BODY_GENERATIONS, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&generations).unwrap().contains("addGeneration"));
        }

        #[test]
        fn document_from_dwg_returns_valid_default_projection() {
            let drawing = semio_framework_core::DwgDrawing::default();
            let document = procedural2d_document_from_dwg(&drawing).expect("dwg import document");
            let projection: Procedural2dDocument = serde_json::from_value(document).expect("parseable projection");
            assert_eq!(projection.fixture.schema, "flow.fixture");
        }

        #[test]
        fn two_instances_converge_disjoint_widget_moves() {
            let widgets: Vec<String> = new_app()
                .projection()
                .expect("projection")
                .fixture
                .widgets
                .iter()
                .map(|widget| widget_id(widget).to_string())
                .collect();
            assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
            let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
            testkit::assert_two_instances_converge::<Procedural2dPlayApp, (Option<f64>, Option<f64>)>(
                "mem://procedural2d-convergence",
                ("moveMediaNode", Some(&json!({ "nodeId": w0, "x": 111.0, "y": 5.0 }))),
                ("moveMediaNode", Some(&json!({ "nodeId": w1, "x": 222.0, "y": 6.0 }))),
                move |app| {
                    let layout = &app.projection().expect("projection").fixture.layout;
                    (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
                },
            );
        }

        #[test]
        fn procedural2d_labels_resolve_native_english_by_default() {
            let mut app = new_app();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("\"Sources\""));
            assert!(json.contains("\"Components\""));
            assert!(json.contains("\"Sinks\""));
            assert!(json.contains("\"Show mode\""));
            assert!(!json.contains("Quellen"));
        }

        #[test]
        fn procedural2d_labels_translate_catalogue_and_inspector_in_german() {
            let mut app = new_app();
            let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
            let catalogue = app.render(PROCEDURAL2D_PLAY_BODY_CATALOGUE, None, &view_state).expect("render");
            let catalogue_json = serde_json::to_string(&catalogue).unwrap();
            assert!(catalogue_json.contains("Quellen"));
            assert!(catalogue_json.contains("Komponenten"));
            assert!(catalogue_json.contains("Senken"));
            assert!(catalogue_json.contains("Anzeigemodus"));
            assert!(!catalogue_json.contains("\"Sources\""));
            let inspector = app.render(PROCEDURAL2D_PLAY_BODY_INSPECTION, None, &view_state).expect("render");
            let inspector_json = serde_json::to_string(&inspector).unwrap();
            assert!(inspector_json.contains("Elemente:"));
        }
    }
    //#endregion 🧪Tests
}
pub mod app_3d {
    //! 🧱 Procedural 3D plugin — flow-based procedural brep editor bundled as a hot-swappable WASM component.

    use flow_core::{
        dag::DagFixture,
        flow_backed_node_graph_extras,
        forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec},
        CameraJson, FlowEvalDriver, FlowFixture, FlowHost, Widget,
    };
    use flow_module_brep::tessellate_geometry_json;
    use procedural_3d::{procedural3d_fixture_operations, Procedural3dDocument, Procedural3dOperation, PROCEDURAL_3D_SCHEMA};
    use protocol::{
        apply_generation_operation, generation_operations, render_generation_form_body, render_generation_preview_text,
        render_generations_tree, select_generation, selected_generation, GenerationOperation, GenerationPlayState,
    };
    use semio_framework_plugin::{PanelGroup,
        apply_world3d_sun_action, build_node_graph_scene, build_world_3d_scene, create_default_layout,
        create_named_layout, merge_world_selection_ids,
        mesh_from_kind, tree_item_with_action, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_readonly_field,
        ui_stack_vertical, ui_text, ActionArgDef, ActionArgOption, ActionEmit, App, AppLabelsOverlayExt, DocumentApp, DocumentView, world3d_scene, world3d_selection_json, world3d_sun_measures,
        ActionDescriptor, MeasureSelectItem, MediaClass, MediaForm, MediaType, NodeGraphScene, OsMediaCapability, PanelTreeBuilder, ResourceKindSpec, UtilityDefinition,
        UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode, ViewState, WindowMeasure, WorldSunConfig,
        SET_ACTIVE_UTILITY_ACTION_ID,
        FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
        FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
        FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    };
    use semio_framework_core::mesh_from_indexed;
    use ui_wgpu::SurfaceKind;
    use std::collections::{hash_map::DefaultHasher, HashSet};
    use std::hash::{Hash, Hasher};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    //#region 🔖Constants
    const PROCEDURAL_3D_PLAY_APP_ID: &str = "procedural3d-play";
    const PROCEDURAL_3D_PLAY_CONTROLLER_ID: &str = "procedural3d-play";
    const PROCEDURAL_3D_PLAY_SURFACE_MAIN: &str = "procedural.play";
    const PROCEDURAL_3D_PLAY_SURFACE_PREVIEW: &str = "procedural.play.preview";
    const PROCEDURAL_3D_PLAY_BODY_MAIN: &str = "procedural.play.main";
    const PROCEDURAL_3D_PLAY_BODY_PREVIEW: &str = "procedural.play.preview";
    const PROCEDURAL_3D_PLAY_BODY_DOCUMENT: &str = "procedural.play.document";
    const PROCEDURAL_3D_PLAY_BODY_CATALOGUE: &str = "procedural.play.catalogue";
    const PROCEDURAL_3D_PLAY_BODY_INSPECTION: &str = "procedural.play.inspection";
    const PROCEDURAL_3D_PLAY_WINDOW_MAIN: &str = "procedural-main";
    const PROCEDURAL_3D_PLAY_WINDOW_PREVIEW: &str = "procedural-preview";
    const PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS: &str = "procedural3d-generations";
    const PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM: &str = "procedural3d-generate-form";
    const PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW: &str = "procedural3d-generate-preview";
    const PROCEDURAL_3D_PLAY_BODY_GENERATIONS: &str = "procedural.play.generations";
    const PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM: &str = "procedural.play.generate-form";
    const PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";
    const PROCEDURAL_3D_PLAY_SURFACE_GENERATIONS: &str = "procedural.play.generations";
    const PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";

    const PROCEDURAL_FALLBACK_MESH_KIND: &str = "box";
    const PROCEDURAL_EXAMPLE_HEX_COLUMN: &str = "hexagonal-mushroom-column";
    const PROCEDURAL_EXAMPLE_RECT_EXTRUDE: &str = "rectangle-extrude-volume";
    const PROCEDURAL_EXAMPLE_SPHERE_TORUS: &str = "sphere-cut-with-torus";

    const HEX_COLUMN_EXAMPLE_JSON: &str = include_str!("../../3d/example/hexagonal-mushroom-column.procedural.json");
    const RECT_EXTRUDE_EXAMPLE_JSON: &str = include_str!("../../3d/example/rectangle-extrude-volume.procedural.json");
    const SPHERE_TORUS_EXAMPLE_JSON: &str = include_str!("../../3d/example/sphere-cut-with-torus.procedural.json");

    const WIDGET_CATALOG: &[(&str, &str)] = &[
        ("neuron", "cpu"),
        ("inputSlider", "sliders-horizontal"),
        ("inputNote", "file-text"),
        ("outputPreview", "eye"),
    ];

    /// 🧰 The gumball utility active when the host has not yet set `view_state.active_utility_id` (first UtilityRef).
    const PROCEDURAL_3D_TRANSFORM_UTILITY_DEFAULT: &str = "move";
    //#endregion 🔖Constants

    //#region 🔖EvalCache
    /// 🧠 Process-wide [`flow_core::neural::NeuralCache`] shared across `FlowHost` reconstructions.
    ///
    /// `Procedural3dPlayView` is a stateless serde value rebuilt from `document_json` on every
    /// plugin dispatch, so a fresh `FlowHost::from_fixture` would otherwise discard per-node
    /// memoization (and the geometry handle stability that lets `flow_module_brep`'s mesh cache
    /// hit) on every single edit. Mirrors `flow_module_brep`'s single-instance `KERNEL`/`MESH_CACHE`
    /// `OnceLock` pattern — one shared cache per WASM instance, which matches one editor session.
    static PROCEDURAL_NEURAL_CACHE: std::sync::OnceLock<std::sync::Arc<flow_core::neural::NeuralCache>> = std::sync::OnceLock::new();

    fn procedural_neural_cache() -> std::sync::Arc<flow_core::neural::NeuralCache> {
        PROCEDURAL_NEURAL_CACHE.get_or_init(|| std::sync::Arc::new(flow_core::neural::NeuralCache::new())).clone()
    }
    //#endregion 🔖EvalCache

    //#region 🔖Types
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Procedural3dPreviewCamera {
        #[serde(default = "default_preview_cam_pos")]
        position: [f64; 3],
        #[serde(default = "default_preview_cam_target")]
        target: [f64; 3],
        #[serde(default = "default_preview_fov")]
        fov: f64,
    }

    impl Default for Procedural3dPreviewCamera {
        fn default() -> Self {
            Self {
                position: default_preview_cam_pos(),
                target: default_preview_cam_target(),
                fov: default_preview_fov(),
            }
        }
    }

    fn default_preview_cam_pos() -> [f64; 3] {
        [4.0, -4.0, 3.0]
    }

    fn default_preview_cam_target() -> [f64; 3] {
        [0.0, 0.0, 0.0]
    }

    fn default_preview_fov() -> f64 {
        45.0
    }

    /// 👁️ Ephemeral per-session view state — never part of the persisted document. Selection, hover,
    /// graph camera, preview camera, sun/LOD display options, the derived mesh preview caches, and the
    /// active generation selection/preview all live here on the app struct, out of the VCS document.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Procedural3dRuntime {
        selected_node_ids: Vec<String>,
        lod_mode: String,
        show_mode: String,
        selection_method: String,
        hovered_node_id: Option<String>,
        camera: CameraJson,
        preview_camera: Procedural3dPreviewCamera,
        preview_cache: Option<Procedural3dPreviewCache>,
        generation_preview_cache: Option<Procedural3dPreviewCache>,
        sun: WorldSunConfig,
        selected_generation_id: Option<String>,
        generation_preview_text: Option<String>,
        /// 🧵 Off-main-thread evaluation state — see `FlowEvalDriver`.
        eval_driver: FlowEvalDriver,
    }

    impl Default for Procedural3dRuntime {
        fn default() -> Self {
            Self {
                selected_node_ids: Vec::new(),
                lod_mode: String::new(),
                show_mode: default_show_mode(),
                selection_method: default_selection_method(),
                hovered_node_id: None,
                camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
                preview_camera: Procedural3dPreviewCamera::default(),
                preview_cache: None,
                generation_preview_cache: None,
                sun: WorldSunConfig::default(),
                selected_generation_id: None,
                generation_preview_text: None,
                eval_driver: FlowEvalDriver::default(),
            }
        }
    }

    fn default_show_mode() -> String {
        "solid".into()
    }

    fn default_selection_method() -> String {
        "rectangle".into()
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Procedural3dPreviewCache {
        signature: u64,
        meshes_json: String,
        instances_json: String,
    }

    /// 🧾 Transient render/action bundle — the persisted projection (fixture + generations) with the
    /// ephemeral runtime's selection, caches, and derived preview overlaid, so the pure panel/render
    /// helpers keep reading a single value. Assembled per call; never serialized as the document.
    struct Procedural3dPlayView {
        fixture: FlowFixture,
        runtime: Procedural3dRuntime,
        generation: GenerationPlayState,
    }

    /// 🧾 Overlays the ephemeral runtime's generation selection and derived preview onto the persisted
    /// generation state to build a {@link Procedural3dPlayView} for rendering.
    fn play_view(projection: &Procedural3dDocument, runtime: &Procedural3dRuntime) -> Procedural3dPlayView {
        let mut generation = projection.generation.clone();
        generation.selected_generation_id = runtime.selected_generation_id.clone();
        generation.preview_text = runtime.generation_preview_text.clone();
        Procedural3dPlayView { fixture: projection.fixture.clone(), runtime: runtime.clone(), generation }
    }
    //#endregion 🔖Types

    //#region 🔖DocumentHelpers
    fn default_fixture() -> FlowFixture {
        serde_json::from_str::<FlowFixture>(HEX_COLUMN_EXAMPLE_JSON).unwrap_or_default()
    }

    fn default_projection() -> Procedural3dDocument {
        Procedural3dDocument { fixture: default_fixture(), generation: GenerationPlayState::default() }
    }

    /// 🧾 Builds the initial projection for a named example (or the empty/default fixture).
    fn example_projection(example_id: &str) -> Procedural3dDocument {
        let fixture_json = match example_id {
            PROCEDURAL_EXAMPLE_HEX_COLUMN | "demo" => Some(HEX_COLUMN_EXAMPLE_JSON),
            PROCEDURAL_EXAMPLE_RECT_EXTRUDE => Some(RECT_EXTRUDE_EXAMPLE_JSON),
            PROCEDURAL_EXAMPLE_SPHERE_TORUS => Some(SPHERE_TORUS_EXAMPLE_JSON),
            "" => None,
            _ => None,
        };
        let fixture = fixture_json
            .and_then(|json| serde_json::from_str::<FlowFixture>(json).ok())
            .unwrap_or_default();
        Procedural3dDocument { fixture, generation: GenerationPlayState::default() }
    }

    /// 🧾 Serializes an example's bare projection for registration via `App::example`.
    fn example_document_json(example_id: &str) -> String {
        serde_json::to_string(&example_projection(example_id)).unwrap_or_default()
    }

    fn fixture_signature(fixture: &FlowFixture) -> u64 {
        let mut hasher = DefaultHasher::new();
        if let Ok(json) = serde_json::to_string(&fixture.widgets) {
            json.hash(&mut hasher);
        }
        if let Ok(json) = serde_json::to_string(&fixture.synapses) {
            json.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn generation_preview_signature(fixture: &FlowFixture, generation: &GenerationPlayState) -> u64 {
        let mut hasher = DefaultHasher::new();
        fixture_signature(fixture).hash(&mut hasher);
        if let Some(selected) = selected_generation(generation) {
            selected.id.hash(&mut hasher);
            if let Ok(json) = serde_json::to_string(&selected.values) {
                json.hash(&mut hasher);
            }
        }
        hasher.finish()
    }

    fn generation_fixture_for(fixture: &FlowFixture, generation: &GenerationPlayState) -> FlowFixture {
        if let Some(selected) = selected_generation(generation) {
            let patched = apply_generation_values_to_fixture(
                &serde_json::to_string(fixture).unwrap_or_default(),
                &selected.values,
            );
            FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone())
        } else {
            fixture.clone()
        }
    }

    fn refresh_preview_cache(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture) {
        let signature = fixture_signature(fixture);
        if runtime.preview_cache.as_ref().is_some_and(|entry| entry.signature == signature) {
            return;
        }
        let (meshes_json, instances_json) = evaluated_preview_payload(fixture, runtime);
        runtime.preview_cache = Some(Procedural3dPreviewCache {
            signature,
            meshes_json,
            instances_json,
        });
    }

    fn refresh_generation_preview_cache(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture, generation: &GenerationPlayState) {
        let signature = generation_preview_signature(fixture, generation);
        if runtime.generation_preview_cache.as_ref().is_some_and(|entry| entry.signature == signature) {
            return;
        }
        let (meshes_json, instances_json) = evaluated_preview_payload(fixture, runtime);
        runtime.generation_preview_cache = Some(Procedural3dPreviewCache {
            signature,
            meshes_json,
            instances_json,
        });
    }

    /// 🧵 Never evaluates: a signature mismatch (fixture changed since the cache was built) means a
    /// `flowEvalTick` chain is converging on the new fixture — this returns the stale cache as-is
    /// rather than blocking the render to recompute; the scene's `statusJson` reports "computing" in
    /// the meantime (see `pending_effects`/`FlowEvalDriver`). Only a cold start (no cache at all) falls
    /// back to a placeholder mesh per node kind.
    fn preview_payload_cached(runtime: &Procedural3dRuntime, fixture: &FlowFixture) -> (String, String) {
        if let Some(cache) = &runtime.preview_cache {
            return (cache.meshes_json.clone(), cache.instances_json.clone());
        }
        (preview_meshes_json_fallback(fixture), preview_instances_json_fallback(fixture, runtime))
    }

    /// 🗂️ Refreshes the ephemeral base + generation mesh preview caches after a mutation, so the next
    /// render hits instead of recomputing. `generation` carries the active selection from the runtime.
    fn refresh_all_caches(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture, generation: &GenerationPlayState) {
        refresh_preview_cache(runtime, fixture);
        if selected_generation(generation).is_none() {
            // 🪞 No active generation: `generation_fixture_for` would just return a clone of `fixture`,
            // so the generation preview is identical to the base preview — reuse the result just
            // computed above instead of evaluating the same fixture twice.
            let signature = generation_preview_signature(fixture, generation);
            let already_cached = runtime.generation_preview_cache.as_ref().is_some_and(|entry| entry.signature == signature);
            if !already_cached {
                if let Some(base) = runtime.preview_cache.clone() {
                    runtime.generation_preview_cache = Some(Procedural3dPreviewCache {
                        signature,
                        meshes_json: base.meshes_json,
                        instances_json: base.instances_json,
                    });
                }
            }
        } else {
            let generation_fixture = generation_fixture_for(fixture, generation);
            refresh_generation_preview_cache(runtime, &generation_fixture, generation);
        }
    }

    fn procedural_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor {
            controller_id: PROCEDURAL_3D_PLAY_CONTROLLER_ID.into(),
            action: action.into(),
            args,
        }
    }

    fn preview_camera_json(runtime: &Procedural3dRuntime) -> String {
        ui_wgpu::world3d_camera_json(
            runtime.preview_camera.position,
            runtime.preview_camera.target,
            runtime.preview_camera.fov,
        )
    }

    fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
        args.and_then(|value| value.get("ids"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .filter(|ids: &Vec<String>| !ids.is_empty())
            .unwrap_or_else(|| fallback.to_vec())
    }

    //#region 🔖GumballTransforms
    /// 🧭 Maps a gumball drag operation to the flow-graph transform neuron kind that persists it.
    fn gumball_xform_kind(operation: &str) -> &'static str {
        match operation {
            "rotate" => "brep.xform.rotate",
            "scale" => "brep.xform.scale",
            _ => "brep.xform.translate",
        }
    }

    /// 🪪 Deterministic id for the transform neuron generated by dragging `source_id`'s gumball for `operation`.
    fn gumball_widget_id(source_id: &str, operation: &str) -> String {
        format!("{source_id}__gumball_{operation}")
    }

    fn gumball_widget_json(host: &FlowHost, widget_id_str: &str) -> Option<Value> {
        host.fixture
            .widgets
            .iter()
            .find(|widget| widget_id(widget) == widget_id_str)
            .and_then(|widget| serde_json::to_value(widget).ok())
    }

    fn gumball_widget_offset(host: &FlowHost, widget_id_str: &str) -> [f64; 3] {
        let offset = gumball_widget_json(host, widget_id_str).and_then(|widget_json| widget_json.get("params").and_then(|params| params.get("offset")).cloned());
        [
            offset.as_ref().and_then(|value| value.get("x")).and_then(Value::as_f64).unwrap_or(0.0),
            offset.as_ref().and_then(|value| value.get("y")).and_then(Value::as_f64).unwrap_or(0.0),
            offset.as_ref().and_then(|value| value.get("z")).and_then(Value::as_f64).unwrap_or(0.0),
        ]
    }

    fn gumball_widget_number_param(host: &FlowHost, widget_id_str: &str, key: &str, default: f64) -> f64 {
        gumball_widget_json(host, widget_id_str)
            .and_then(|widget_json| widget_json.get("params").and_then(|params| params.get(key)).and_then(|entry| entry.get("value")).and_then(Value::as_f64))
            .unwrap_or(default)
    }

    fn gumball_translate_params_json(offset: [f64; 3]) -> String {
        json!({ "offset": { "$schema": "vector", "x": offset[0], "y": offset[1], "z": offset[2] } }).to_string()
    }

    fn gumball_rotate_params_json(axis: [f64; 3], angle: f64) -> String {
        json!({
            "axis": { "$schema": "vector", "x": axis[0], "y": axis[1], "z": axis[2] },
            "angle": { "$schema": "number", "value": angle },
        })
        .to_string()
    }

    fn gumball_scale_params_json(factor: f64) -> String {
        json!({
            "factor": { "$schema": "number", "value": factor },
            "center": { "$schema": "point", "x": 0.0, "y": 0.0, "z": 0.0 },
        })
        .to_string()
    }

    /// 🔀 Finds (or splices in) the transform neuron that persists `selected_id`'s gumball drag for `operation` into the flow graph, rewiring downstream consumers so the transformed geometry is what actually evaluates and exports.
    fn ensure_gumball_node(host: &mut FlowHost, selected_id: &str, operation: &str) -> Result<String, String> {
        let own_suffix = format!("__gumball_{operation}");
        if selected_id.ends_with(&own_suffix) && host.fixture.widgets.iter().any(|widget| widget_id(widget) == selected_id) {
            return Ok(selected_id.to_string());
        }
        let transform_id = gumball_widget_id(selected_id, operation);
        if host.fixture.widgets.iter().any(|widget| widget_id(widget) == transform_id) {
            return Ok(transform_id);
        }
        let (source_x, source_y) = widget_layout_position(&host.fixture, selected_id);
        let descriptor = json!({ "kind": "neuron", "id": transform_id, "neuronKind": gumball_xform_kind(operation) }).to_string();
        host.add_widget(&descriptor, source_x + 220.0, source_y).map_err(|err| err.to_string())?;
        let outgoing_port = host.fixture.synapses.iter().find(|synapse| synapse.from == selected_id).map(|synapse| synapse.from_port.clone());
        if let Some(port) = outgoing_port {
            host.insert_between(selected_id, &port, &transform_id, "geometry", "geometry").map_err(|err| err.to_string())?;
        } else {
            host.connect(selected_id, &transform_id).map_err(|err| err.to_string())?;
        }
        if let Some(Widget::Neuron { preview, .. }) = host.fixture.widgets.iter_mut().find(|widget| widget_id(widget) == selected_id) {
            *preview = false;
        }
        Ok(transform_id)
    }
    //#endregion 🔖GumballTransforms

    fn host_from_fixture(fixture: &FlowFixture) -> FlowHost {
        let mut host = FlowHost::from_fixture_with_cache(fixture.clone(), procedural_neural_cache());
        host.set_neuron_kind_infos_json(&flow_core::flow_neuron_kind_infos_json());
        host
    }

    fn split_endpoint(endpoint: &str) -> (String, String) {
        endpoint
            .split_once(':')
            .map(|(node, port)| (node.to_string(), port.to_string()))
            .unwrap_or_else(|| (endpoint.to_string(), "out".into()))
    }

    fn fixture_to_media_graph(fixture: &DagFixture) -> (String, String) {
        let nodes: Vec<Value> = fixture
            .nodes
            .iter()
            .map(|node| {
                json!({
                    "id": node.id,
                    "label": if node.name.is_empty() { &node.id } else { &node.name },
                    "x": node.x,
                    "y": node.y,
                    "width": node.width,
                    "height": node.height,
                    "inputs": node.inputs().iter().filter(|port| port.visible).map(|port| json!({
                        "id": format!("{}:{}", node.id, port.id),
                        "label": port.label,
                    })).collect::<Vec<_>>(),
                    "outputs": node.outputs().iter().filter(|port| port.visible).map(|port| json!({
                        "id": format!("{}:{}", node.id, port.id),
                        "label": port.label,
                    })).collect::<Vec<_>>(),
                })
            })
            .collect();
        let edges: Vec<Value> = fixture
            .edges
            .iter()
            .map(|edge| {
                let (source_node_id, source_port_id) = split_endpoint(&edge.source);
                let (target_node_id, target_port_id) = split_endpoint(&edge.target);
                json!({
                    "id": edge.id,
                    "sourceNodeId": source_node_id,
                    "sourcePortId": source_port_id,
                    "targetNodeId": target_node_id,
                    "targetPortId": target_port_id,
                })
            })
            .collect();
        (
            serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
            serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
        )
    }

    fn widget_id(widget: &Widget) -> &str {
        match widget {
            Widget::Neuron { id, .. }
            | Widget::InputSlider { id, .. }
            | Widget::InputNote { id, .. }
            | Widget::InputImage { id, .. }
            | Widget::Variable { id, .. }
            | Widget::OutputPreview { id, .. }
            | Widget::OutputAction { id, .. }
            | Widget::OutputExport { id, .. }
            | Widget::Cluster { id, .. } => id,
        }
    }

    fn neuron_mesh_kind(neuron_kind: &str) -> &'static str {
        match neuron_kind {
            "brep.prim3d.sphere" => "sphere",
            "brep.prim3d.cylinder" => "cylinder",
            "brep.prim3d.cone" => "cone",
            "brep.prim3d.torus" => "torus",
            "brep.prim3d.box" => "box",
            "brep.solid.extrude" | "brep.bool.cut" | "brep.bool.fuse" => "box",
            _ => PROCEDURAL_FALLBACK_MESH_KIND,
        }
    }

    fn widget_preview_mesh_kind(widget: &Widget) -> Option<&'static str> {
        match widget {
            Widget::Neuron { neuron_kind, preview, .. } if *preview => Some(neuron_mesh_kind(neuron_kind)),
            Widget::OutputPreview { .. } => Some(PROCEDURAL_FALLBACK_MESH_KIND),
            _ => None,
        }
    }

    fn widget_layout_position(fixture: &FlowFixture, widget_id: &str) -> (f64, f64) {
        fixture
            .layout
            .get(widget_id)
            .map(|layout| (layout.x, layout.y))
            .unwrap_or((0.0, 0.0))
    }

    fn is_brep_geometry_handle(handle: &str) -> bool {
        handle.starts_with("solid-")
            || handle.starts_with("shell-")
            || handle.starts_with("face-")
            || handle.starts_with("wire-")
            || handle.starts_with("edge-")
            || handle.starts_with("vertex-")
            || handle.starts_with("compound-")
            || handle.starts_with("curve-")
            || handle.starts_with("surface-")
    }

    fn collect_geometry_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                    if is_brep_geometry_handle(handle) {
                        handles.push(handle.into());
                    }
                }
                for entry in map.values() {
                    collect_geometry_handles_from_eval(entry, handles);
                }
            }
            Value::Array(items) => {
                for item in items {
                    collect_geometry_handles_from_eval(item, handles);
                }
            }
            _ => {}
        }
    }

    fn geometry_handle_for_widget(eval: &Value, widget_id: &str) -> Option<String> {
        let widget_eval = eval.get(widget_id)?;
        let channels = widget_eval.get("out").or_else(|| widget_eval.get("in"))?;
        let mut handles = Vec::new();
        collect_geometry_handles_from_eval(channels, &mut handles);
        handles.into_iter().next()
    }

    fn mesh_from_tessellation_json(mesh_json: &str) -> Option<semio_framework_plugin::MeshData> {
        let parsed: Value = serde_json::from_str(mesh_json).ok()?;
        if parsed.get("error").is_some() {
            return None;
        }
        let positions: Vec<f32> = parsed
            .get("position")
            .or_else(|| parsed.get("positions"))
            .and_then(|entry| entry.as_array())
            .map(|items| items.iter().filter_map(|value| value.as_f64().map(|number| number as f32)).collect())
            .filter(|items: &Vec<f32>| !items.is_empty())?;
        let normals: Vec<f32> = parsed
            .get("normal")
            .or_else(|| parsed.get("normals"))
            .and_then(|entry| entry.as_array())
            .map(|items| items.iter().filter_map(|value| value.as_f64().map(|number| number as f32)).collect())
            .unwrap_or_default();
        let indices: Vec<u32> = parsed
            .get("index")
            .or_else(|| parsed.get("indices"))
            .and_then(|entry| entry.as_array())
            .map(|items| items.iter().filter_map(|value| value.as_u64().map(|number| number as u32)).collect())
            .filter(|items: &Vec<u32>| !items.is_empty())?;
        Some(mesh_from_indexed(&positions, &normals, &indices))
    }

    fn evaluated_preview_payload(fixture: &FlowFixture, runtime: &Procedural3dRuntime) -> (String, String) {
        let mut host = FlowHost::from_fixture_with_cache(fixture.clone(), procedural_neural_cache());
        let eval_json = host.evaluate().unwrap_or_default();
        let eval: Value = serde_json::from_str(&eval_json).unwrap_or(json!({}));
        let mut meshes: Vec<Value> = Vec::new();
        let mut instances: Vec<Value> = Vec::new();
        for widget in &fixture.widgets {
            let id = widget_id(widget).to_string();
            let preview = matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. });
            if !preview {
                continue;
            }
            let Some(handle) = geometry_handle_for_widget(&eval, &id) else {
                continue;
            };
            let mesh_id = format!("eval-{id}");
            if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                let tessellation = tessellate_geometry_json(&handle, 0.05);
                if let Some(data) = mesh_from_tessellation_json(&tessellation) {
                    meshes.push(json!({ "id": mesh_id, "data": data }));
                }
            }
            if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                let (x, y) = widget_layout_position(fixture, &id);
                let selected = runtime.selected_node_ids.contains(&id);
                let hovered = runtime.hovered_node_id.as_deref() == Some(id.as_str());
                let position = [x * 0.01, -y * 0.01, 0.0];
                instances.push(json!({
                    "id": id,
                    "meshId": mesh_id,
                    "position": position,
                    "rotation": [0.0, 0.0, 0.0, 1.0],
                    "scale": [1.0, 1.0, 1.0],
                    "label": id,
                    "selected": selected,
                    "hovered": hovered,
                }));
            }
        }
        if meshes.is_empty() {
            let fallback = preview_meshes_json_fallback(fixture);
            let fallback_instances = preview_instances_json_fallback(fixture, runtime);
            return (fallback, fallback_instances);
        }
        (
            serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()),
            serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()),
        )
    }

    fn evaluate_generation_preview(fixture: &FlowFixture, values: &serde_json::Map<String, Value>) -> String {
        let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
        let patched = apply_generation_values_to_fixture(&fixture_json, values);
        let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
        let mut host = FlowHost::from_fixture_with_cache(patched_fixture, procedural_neural_cache());
        host.evaluate().unwrap_or_default()
    }

    /// 👁️ Recomputes the ephemeral generation preview text for the selected generation and stores it
    /// on the runtime (never on the persisted document).
    fn refresh_generation_preview(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture, generation: &GenerationPlayState) {
        let Some(selected) = selected_generation(generation) else {
            runtime.generation_preview_text = None;
            return;
        };
        runtime.generation_preview_text = Some(evaluate_generation_preview(fixture, &selected.values));
    }

    fn generation_preview_payload(view: &Procedural3dPlayView) -> (String, String) {
        let fixture = generation_fixture_for(&view.fixture, &view.generation);
        let signature = generation_preview_signature(&fixture, &view.generation);
        if let Some(cache) = &view.runtime.generation_preview_cache {
            if cache.signature == signature {
                return (cache.meshes_json.clone(), cache.instances_json.clone());
            }
        }
        evaluated_preview_payload(&fixture, &view.runtime)
    }

    fn preview_instances_json_fallback(fixture: &FlowFixture, runtime: &Procedural3dRuntime) -> String {
        let instances: Vec<Value> = fixture
            .widgets
            .iter()
            .filter_map(|widget| {
                let mesh_kind = widget_preview_mesh_kind(widget)?;
                let id = widget_id(widget).to_string();
                let (x, y) = widget_layout_position(fixture, &id);
                let selected = runtime.selected_node_ids.contains(&id);
                let hovered = runtime.hovered_node_id.as_deref() == Some(id.as_str());
                let position = [x * 0.01, -y * 0.01, 0.0];
                Some(json!({
                    "id": id,
                    "meshId": mesh_kind,
                    "position": position,
                    "rotation": [0.0, 0.0, 0.0, 1.0],
                    "scale": [1.0, 1.0, 1.0],
                    "label": id,
                    "selected": selected,
                    "hovered": hovered,
                }))
            })
            .collect();
        serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
    }

    fn preview_meshes_json_fallback(fixture: &FlowFixture) -> String {
        let kinds: Vec<String> = fixture
            .widgets
            .iter()
            .filter_map(|widget| widget_preview_mesh_kind(widget).map(str::to_string))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        let fallback_kinds = if kinds.is_empty() {
            vec![PROCEDURAL_FALLBACK_MESH_KIND.into()]
        } else {
            kinds
        };
        let meshes: Vec<Value> = fallback_kinds
            .iter()
            .map(|kind| {
                let data = mesh_from_kind(kind);
                json!({ "id": kind, "data": data })
            })
            .collect();
        serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
    }

    /// 🧭 World-3d selection payload with the host-owned gumball utility spliced in, so the transform
    /// handles follow `view_state.active_utility_id` instead of any document/runtime-stored utility.
    fn preview_selection_json(runtime: &Procedural3dRuntime, active_utility: &str) -> String {
        let mut value: Value = serde_json::from_str(&world3d_selection_json(
            &runtime.selection_method,
            &runtime.selected_node_ids,
            runtime.hovered_node_id.as_deref(),
        ))
        .unwrap_or_else(|_| json!({}));
        if let Some(object) = value.as_object_mut() {
            object.insert("transformMode".into(), json!(active_utility));
            object.insert("gumballActive".into(), json!(!runtime.selected_node_ids.is_empty()));
        }
        value.to_string()
    }

    /// 🎚️ Level-of-detail display measure for the flow window — the migrated home of the old LOD
    /// utility bar toggles (a display option, never an interactive utility). Dispatches `setLodMode` (a View action).
    fn procedural3d_lod_measure(lod_mode: &str) -> WindowMeasure {
        let current = if lod_mode.is_empty() { "solid" } else { lod_mode };
        WindowMeasure::Select {
            id: "procedural3d-measure-lod".into(),
            label: Some("LOD".into()),
            value: current.into(),
            items: vec![
                MeasureSelectItem { id: "procedural3d-measure-lod-solid".into(), value: "solid".into(), label: "Solid".into() },
                MeasureSelectItem { id: "procedural3d-measure-lod-wireframe".into(), value: "wireframe".into(), label: "Wireframe".into() },
            ],
            on_change: procedural_action("setLodMode", None),
        }
    }

    fn export_mesh_from_document(projection: &Procedural3dDocument) -> semio_framework_plugin::MeshData {
        let runtime = Procedural3dRuntime::default();
        let (meshes_json, _) = evaluated_preview_payload(&projection.fixture, &runtime);
        if let Ok(meshes) = serde_json::from_str::<Vec<Value>>(&meshes_json) {
            if let Some(first) = meshes.first() {
                if let Ok(data) = serde_json::from_value(first.get("data").cloned().unwrap_or(Value::Null)) {
                    return data;
                }
            }
        }
        let kind = projection
            .fixture
            .widgets
            .iter()
            .find_map(|widget| widget_preview_mesh_kind(widget))
            .unwrap_or(PROCEDURAL_FALLBACK_MESH_KIND);
        mesh_from_kind(kind)
    }
    //#endregion 🔖DocumentHelpers

    //#region 🔖Terminology
    semio_framework_plugin::app_labels! {
        /// 🗣️ Complete UI label set for the 3D flow app; one field per label makes every locale combination compile-checked.
        struct Procedural3dLabels {
            widgets: &'static str = en: "Widgets", de: "Elemente";
            schema_prefix: &'static str = en: "Schema:", de: "Schema:";
            widgets_prefix: &'static str = en: "Widgets:", de: "Elemente:";
            no_selection: &'static str = en: "No selection", de: "Keine Auswahl";
            id_field: &'static str = en: "Id", de: "ID";
            value_field: &'static str = en: "Value", de: "Wert";
            range_field: &'static str = en: "Range", de: "Bereich";
            widget_group: &'static str = en: "Widget", de: "Element";
            generate_hint: &'static str = en: "Add a generation to edit input values.", de: "Erstelle eine Generation, um Eingabewerte zu bearbeiten.";
            preview_hint: &'static str = en: "(evaluate a generation to preview output)", de: "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)";
            catalog_neuron: &'static str = en: "Neuron", de: "Neuron";
            catalog_slider: &'static str = en: "Slider", de: "Schieberegler";
            catalog_note: &'static str = en: "Note", de: "Notiz";
            catalog_preview: &'static str = en: "Preview", de: "Vorschau";
            window_flow: &'static str = en: "Flow", de: "Workflow";
            window_preview: &'static str = en: "Preview", de: "Vorschau";
            window_generations: &'static str = en: "Generations", de: "Generationen";
            window_generate_form: &'static str = en: "Form", de: "Formular";
            window_generate_preview: &'static str = en: "Preview", de: "Vorschau";
            delete_selection: &'static str = en: "Delete selection", de: "Auswahl löschen";
        }
    }

    /// 🗣️ Resolves the active label set from the shell-provided locale; falls back to native English.
    fn procedural3d_labels(view_state: &ViewState) -> &'static Procedural3dLabels {
        semio_framework_plugin::resolve_labels::<Procedural3dLabels>(view_state)
    }

    /// 🗣️ Resolves a catalogue widget kind's display label from its stable id; unknown kinds fall back to the id itself.
    fn procedural3d_catalog_label(kind: &'static str, labels: &Procedural3dLabels) -> &'static str {
        match kind {
            "neuron" => labels.catalog_neuron,
            "inputSlider" => labels.catalog_slider,
            "inputNote" => labels.catalog_note,
            "outputPreview" => labels.catalog_preview,
            _ => kind,
        }
    }
    //#endregion 🔖Terminology

    //#region 🔖CommandLabels
    /// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_procedural3d_app`'s
    /// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
    /// palette and Actions rail get a translated label without threading locale through the whole builder chain.
    fn procedural3d_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
        const ENTRIES: &[(&str, &str, &str)] = &[
            ("nodeGraphViewport", "Set Viewport", "Ansicht festlegen"),
            ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
            ("nodeGraphEdit", "Edit Graph", "Graph bearbeiten"),
            ("deleteSelection", "Delete Selection", "Auswahl löschen"),
            ("removeWidget", "Remove Widget", "Element entfernen"),
            ("moveMediaNode", "Move Node", "Knoten verschieben"),
            ("addWidget", "Add Widget", "Element hinzufügen"),
            ("patchFlowWidgets", "Patch Flow Widgets", "Flow-Elemente aktualisieren"),
            ("reorganize", "Reorganize", "Neu anordnen"),
            ("translateSelection", "Translate Selection", "Auswahl verschieben"),
            ("rotateSelection", "Rotate Selection", "Auswahl drehen"),
            ("scaleSelection", "Scale Selection", "Auswahl skalieren"),
            ("addGeneration", "Add Generation", "Generation hinzufügen"),
            ("removeGeneration", "Remove Generation", "Generation entfernen"),
            ("renameGeneration", "Rename Generation", "Generation umbenennen"),
            ("updateGenerationValues", "Update Generation Values", "Generationswerte aktualisieren"),
            ("setSelection", "Set Selection", "Auswahl festlegen"),
            ("selectNode", "Select Node", "Knoten auswählen"),
            ("nodeGraphSelect", "Node Graph Select", "Graph-Auswahl"),
            ("nodeGraphHover", "Node Graph Hover", "Graph-Hover"),
            ("setHover", "Set Hover", "Überfahren festlegen"),
            ("worldPointerDown", "World Pointer Down", "Welt-Zeiger gedrückt"),
            ("graphPointerDown", "Graph Pointer Down", "Graph-Zeiger gedrückt"),
            ("worldSelect", "World Select", "Welt auswählen"),
            ("worldHover", "World Hover", "Überfahren (Welt)"),
            ("setSelectionMethod", "Set Selection Method", "Auswahlmethode festlegen"),
            ("setLodMode", "Set Lod Mode", "LOD-Modus festlegen"),
            ("setShowMode", "Set Show Mode", "Anzeigemodus festlegen"),
            ("toggleSun", "Toggle Sun", "Sonne umschalten"),
            ("setSunAzimuth", "Set Sun Azimuth", "Sonnenazimut festlegen"),
            ("setSunElevation", "Set Sun Elevation", "Sonnenhöhe festlegen"),
            ("setSunIntensity", "Set Sun Intensity", "Sonnenintensität festlegen"),
            ("setCamera", "Set Camera", "Kamera festlegen"),
            ("selectGeneration", "Set Generation", "Generation auswählen"),
        ];
        semio_framework_plugin::localized_label_map(is_de, ENTRIES)
    }

    /// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_procedural3d_app`.
    fn procedural3d_utility_labels(is_de: bool) -> std::collections::HashMap<String, String> {
        const ENTRIES: &[(&str, &str, &str)] = &[
            ("move", "Move", "Verschieben"),
            ("rotate", "Rotate", "Drehen"),
            ("scale", "Scale", "Skalieren"),
        ];
        semio_framework_plugin::localized_label_map(is_de, ENTRIES)
    }
    //#endregion 🔖CommandLabels

    //#region 🔖Panels
    /// 🌳 SDK's `tree_item_with_action` plus an icon id — this crate's document/catalogue trees carry
    /// icons per item, which the shared helper doesn't model directly.
    fn tree_item_with_icon(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
        UiTreeItemNode { icon_id: icon_id.map(str::to_string), ..tree_item_with_action(id, label, None, action) }
    }

    fn build_document_tree(fixture: &FlowFixture, selected_node_ids: &[String], labels: &Procedural3dLabels) -> UiNode {
        let items: Vec<UiTreeItemNode> = fixture
            .widgets
            .iter()
            .map(|widget| {
                let id = widget_id(widget).to_string();
                tree_item_with_icon(
                    format!("procedural-widget:{id}"),
                    id.clone(),
                    Some("cpu"),
                    procedural_action("setSelection", Some(json!({ "ids": [id] }))),
                )
            })
            .collect();
        PanelTreeBuilder::new("procedural-play-document")
            .section("procedural-play-document.widgets", Some(labels.widgets.into()), true, items)
            .selected(selected_node_ids.iter().map(|id| format!("procedural-widget:{id}")).collect())
            .build()
    }

    fn build_catalogue_tree(labels: &Procedural3dLabels) -> UiNode {
        let items: Vec<UiTreeItemNode> = WIDGET_CATALOG
            .iter()
            .map(|(kind, icon)| {
                tree_item_with_icon(
                    format!("procedural-play-catalogue.{kind}"),
                    procedural3d_catalog_label(*kind, labels),
                    Some(icon),
                    procedural_action("addWidget", Some(json!({ "kind": kind }))),
                )
            })
            .collect();
        PanelTreeBuilder::new("procedural-play-catalogue")
            .section("procedural-play-catalogue.widgets", Some(labels.widgets.into()), true, items)
            .build()
    }

    fn build_inspector_tree(fixture: &FlowFixture, selected_node_ids: &[String], labels: &Procedural3dLabels) -> UiNode {
        let Some(selected_id) = selected_node_ids.first() else {
            return ui_stack_vertical(vec![
                ui_text(format!("{} {}", labels.schema_prefix, fixture.schema)),
                ui_text(format!("{} {}", labels.widgets_prefix, fixture.widgets.len())),
            ]);
        };
        let Some(widget) = fixture.widgets.iter().find(|entry| widget_id(entry) == selected_id) else {
            return ui_text(labels.no_selection.to_string());
        };
        let mut fields = vec![ui_inspector_readonly_field(
            "procedural-play-inspector.id",
            labels.id_field,
            widget_id(widget),
        )];
        if let Widget::InputSlider { value, min, max, .. } = widget {
            let mixed = ui_inspector_mixed_number(&[*value]);
            fields.push(UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "procedural-play-inspector.value".into(),
                label: labels.value_field.into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode { presence: UiPresence::default(),
                    id: "procedural-play-inspector.value.input".into(),
                    input_kind: "number".into(),
                    value: mixed.value.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: procedural_action(
                        "patchFlowWidgets",
                        Some(json!({ "widgetIds": [selected_id], "field": "value" })),
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
            fields.push(ui_inspector_readonly_field(
                "procedural-play-inspector.range",
                labels.range_field,
                &format!("{min}..{max}"),
            ));
        }
        ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(),
            id: "procedural-play-inspector.widget".into(),
            label: labels.widget_group.into(),
            default_open: None,
            fields,
        }])
    }
    //#endregion 🔖Panels

    //#region 🔖Render
    fn render_generate_generations(envelope: &Procedural3dPlayView) -> UiNode {
        render_generations_tree(
            PROCEDURAL_3D_PLAY_APP_ID,
            "procedural3d-play-generate",
            &envelope.generation.generations,
            envelope.generation.selected_generation_id.as_deref(),
        )
    }

    fn render_generate_form(envelope: &Procedural3dPlayView, labels: &Procedural3dLabels) -> UiNode {
        let spec = flow_fixture_to_form_spec(&envelope.fixture);
        let Some(generation) = selected_generation(&envelope.generation) else {
            return ui_text(labels.generate_hint);
        };
        render_generation_form_body(
            &spec,
            &generation.values,
            PROCEDURAL_3D_PLAY_APP_ID,
            "updateGenerationValues",
            &generation.id,
        )
    }

    fn render_generate_preview(envelope: &Procedural3dPlayView, labels: &Procedural3dLabels, active_utility: &str) -> UiNode {
        let (meshes_json, instances_json) = generation_preview_payload(envelope);
        if meshes_json == "[]" && instances_json == "[]" {
            let text = envelope
                .generation
                .preview_text
                .as_deref()
                .filter(|value| !value.is_empty())
                .unwrap_or(labels.preview_hint);
            return render_generation_preview_text(
                PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW,
                PROCEDURAL_3D_PLAY_APP_ID,
                text,
            );
        }
        build_world_3d_scene(
            PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW,
            PROCEDURAL_3D_PLAY_APP_ID,
            world3d_scene(
                preview_camera_json(&envelope.runtime),
                meshes_json,
                instances_json,
                preview_selection_json(&envelope.runtime, active_utility),
                &envelope.runtime.sun,
            ),
        )
    }
    //#endregion 🔖Render

    //#region 🔖Procedural3dPlayApp
    #[derive(Default)]
    pub struct Procedural3dPlayApp {
        runtime: Procedural3dRuntime,
    }

    impl Procedural3dPlayApp {
        /// 🔀 Diffs a mutated fixture into operations. Diffs against the host-normalized baseline of `before`
        /// (not the raw projection) so `FlowHost`'s own dedupe/dag-rebuild normalization does not leak
        /// spurious collection operations — only the actual mutation becomes an operation, keeping concurrent
        /// disjoint edits mergeable on the backbone. Never evaluates: `pending_effects` (called after
        /// every action's `refreshUi` pass) arms the `flowEvalTick` chain that refreshes the preview
        /// cache once the new fixture's dirty set resolves.
        fn commit_fixture(&mut self, before: &FlowFixture, target: &FlowFixture) -> Vec<Procedural3dOperation> {
            let baseline = host_from_fixture(before).fixture;
            procedural3d_fixture_operations(&baseline, target)
        }

        /// 🧬 Emits generation operations for the generate-mode actions, updating ephemeral selection and
        /// preview from the post-operation state. `selectGeneration` is a view action (no operations).
        fn handle_generation(
            &mut self,
            action: &str,
            args: Option<&Value>,
            projection: &Procedural3dDocument,
        ) -> ActionEmit<Procedural3dOperation> {
            let spec = flow_fixture_to_form_spec(&projection.fixture);
            let mut state = projection.generation.clone();
            state.selected_generation_id = self.runtime.selected_generation_id.clone();
            if action == "selectGeneration" {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    select_generation(&mut state, id);
                }
                self.runtime.selected_generation_id = state.selected_generation_id.clone();
                refresh_generation_preview(&mut self.runtime, &projection.fixture, &state);
                refresh_all_caches(&mut self.runtime, &projection.fixture, &state);
                return ActionEmit::default();
            }
            let Some(operations) = generation_operations(action, args, &state, &spec) else {
                return ActionEmit::default();
            };
            for operation in &operations {
                apply_generation_operation(&mut state, operation);
            }
            self.runtime.selected_generation_id = state.selected_generation_id.clone();
            refresh_generation_preview(&mut self.runtime, &projection.fixture, &state);
            refresh_all_caches(&mut self.runtime, &projection.fixture, &state);
            let coalesce_key = (action == "updateGenerationValues").then(|| "generation-values".to_string());
            ActionEmit {
                operations: operations.into_iter().map(Procedural3dOperation::Generation).collect(),
                coalesce_key,
                ..Default::default()
            }
        }

        /// 🧭 Runs a gumball transform (translate/rotate/scale) as a fixture operation, splicing transform
        /// neurons via `ensure_gumball_node` and re-selecting the resulting transform widgets.
        fn gumball_transform(
            &mut self,
            fixture: &FlowFixture,
            args: Option<&Value>,
            operation: &str,
            apply: impl Fn(&mut FlowHost, &str) -> bool,
        ) -> ActionEmit<Procedural3dOperation> {
            let ids = mesh_selection_ids(args, &self.runtime.selected_node_ids);
            let mut host = host_from_fixture(fixture);
            let mut new_selection = Vec::new();
            let mut changed = false;
            for id in &ids {
                if let Ok(transform_id) = ensure_gumball_node(&mut host, id, operation) {
                    if apply(&mut host, &transform_id) {
                        new_selection.push(transform_id);
                        changed = true;
                    }
                }
            }
            if changed {
                let operations = self.commit_fixture(fixture, &host.fixture);
                self.runtime.selected_node_ids = new_selection;
                return ActionEmit::amend(operations, format!("gumball-{operation}"));
            }
            ActionEmit::default()
        }
    }

    impl DocumentApp for Procedural3dPlayApp {
        type Projection = Procedural3dDocument;
        type Operation = Procedural3dOperation;

        fn app_id(&self) -> &str {
            PROCEDURAL_3D_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            PROCEDURAL_3D_SCHEMA
        }

        fn initial_projection(&self) -> Procedural3dDocument {
            default_projection()
        }

        fn handle_action(
            &mut self,
            action: &str,
            args: Option<&Value>,
            doc: &DocumentView<'_, Procedural3dDocument>,
            _view_state: &ViewState,
        ) -> ActionEmit<Procedural3dOperation> {
            let fixture = &doc.projection.fixture;
            match action {
                // 👁️ View actions — mutate ephemeral runtime, emit no operations.
                "setSelection" | "selectNode" | "nodeGraphSelect" => {
                    self.runtime.selected_node_ids = node_graph_selection_ids(args);
                    ActionEmit::default()
                }
                "nodeGraphHover" => {
                    if let Some(widget_id) = parse_node_graph_hover_widget_id(args) {
                        self.runtime.hovered_node_id = widget_id;
                    }
                    ActionEmit::default()
                }
                "setHover" => {
                    if args.is_none() || args.and_then(|value| value.get("objectId")).is_none() {
                        self.runtime.hovered_node_id = None;
                    } else {
                        self.runtime.hovered_node_id = args
                            .and_then(|value| value.get("objectId"))
                            .and_then(|value| value.as_str())
                            .map(str::to_string);
                    }
                    ActionEmit::default()
                }
                "worldPointerDown" | "graphPointerDown" => ActionEmit::default(),
                // 🧰 Host-owned active-utility switch — clear in-progress hover scratch, never emit operations.
                SET_ACTIVE_UTILITY_ACTION_ID => {
                    self.runtime.hovered_node_id = None;
                    ActionEmit::default()
                }
                "worldSelect" => {
                    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                    let ids: Vec<String> = args
                        .and_then(|value| value.get("ids"))
                        .and_then(|value| serde_json::from_value(value.clone()).ok())
                        .unwrap_or_default();
                    self.runtime.selected_node_ids = merge_world_selection_ids(&self.runtime.selected_node_ids, &ids, merge);
                    ActionEmit::default()
                }
                "worldHover" => {
                    self.runtime.hovered_node_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                    ActionEmit::default()
                }
                "setSelectionMethod" => {
                    self.runtime.selection_method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle").into();
                    ActionEmit::default()
                }
                "setLodMode" => {
                    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                        self.runtime.lod_mode = mode.into();
                    }
                    ActionEmit::default()
                }
                "setShowMode" => {
                    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                        self.runtime.show_mode = mode.into();
                    }
                    ActionEmit::default()
                }
                "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                    apply_world3d_sun_action(&mut self.runtime.sun, action, args);
                    ActionEmit::default()
                }
                "setCamera" => {
                    if let Some(camera) = args.and_then(|value| value.get("camera")) {
                        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                            self.runtime.preview_camera = parsed;
                        }
                    }
                    ActionEmit::default()
                }
                // 📷 Graph camera — ephemeral view state (never a document operation), same model as flow-play.
                "nodeGraphViewport" => {
                    if let Some(camera) = args
                        .and_then(|value| value.get("viewportJson"))
                        .and_then(|value| value.as_str())
                        .and_then(|json| serde_json::from_str(json).ok())
                    {
                        self.runtime.camera = camera;
                    }
                    ActionEmit::default()
                }
                // ✏️ Operations — compute the target fixture via the host, emit fixture operations.
                "setActiveExample" => {
                    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                    let target = example_projection(example_id);
                    let mut operations: Vec<Procedural3dOperation> = doc
                        .projection
                        .generation
                        .generations
                        .iter()
                        .map(|generation| Procedural3dOperation::Generation(GenerationOperation::Remove { id: generation.id.clone() }))
                        .collect();
                    operations.extend(procedural3d_fixture_operations(fixture, &target.fixture));
                    let camera = target.fixture.camera.clone();
                    self.runtime = Procedural3dRuntime { camera, ..Procedural3dRuntime::default() };
                    ActionEmit::operations(operations)
                }
                "nodeGraphEdit" => {
                    let sub_operations = args.and_then(|value| value.get("operations")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                    let selected = self.runtime.selected_node_ids.clone();
                    let mut host = host_from_fixture(fixture);
                    let mut cleared = false;
                    for operation in &sub_operations {
                        match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                            "setFixture" => {
                                if let Some(new_fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| serde_json::from_str::<FlowFixture>(json).ok()) {
                                    host.replace_fixture(new_fixture);
                                }
                            }
                            "deleteSelection" => {
                                for id in &selected {
                                    if host.remove_widget(id).is_ok() {
                                        cleared = true;
                                    }
                                }
                            }
                            "connect" => {
                                let from = operation.get("sourceNodeId").and_then(|value| value.as_str());
                                let from_port = operation.get("sourcePortId").and_then(|value| value.as_str());
                                let to = operation.get("targetNodeId").and_then(|value| value.as_str());
                                let to_port = operation.get("targetPortId").and_then(|value| value.as_str());
                                if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                                    let _ = host.connect_ports(from, from_port, to, to_port);
                                }
                            }
                            _ => {}
                        }
                    }
                    let operations = self.commit_fixture(fixture, &host.fixture);
                    if cleared {
                        self.runtime.selected_node_ids.clear();
                    }
                    ActionEmit::operations(operations)
                }
                "deleteSelection" => {
                    let selected = self.runtime.selected_node_ids.clone();
                    let mut host = host_from_fixture(fixture);
                    let mut cleared = false;
                    for id in &selected {
                        if host.remove_widget(id).is_ok() {
                            cleared = true;
                        }
                    }
                    let operations = self.commit_fixture(fixture, &host.fixture);
                    if cleared {
                        self.runtime.selected_node_ids.clear();
                    }
                    ActionEmit::operations(operations)
                }
                "removeWidget" => {
                    let target_id = args
                        .and_then(|value| value.get("widgetId"))
                        .or_else(|| args.and_then(|value| value.get("id")))
                        .and_then(|value| value.as_str())
                        .map(str::to_string);
                    if let Some(target_id) = target_id {
                        let mut host = host_from_fixture(fixture);
                        if host.remove_widget(&target_id).is_ok() {
                            let operations = self.commit_fixture(fixture, &host.fixture);
                            self.runtime.selected_node_ids.retain(|id| id != &target_id);
                            return ActionEmit::operations(operations);
                        }
                    }
                    ActionEmit::default()
                }
                "moveMediaNode" => {
                    let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(str::to_string);
                    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                    if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                        let mut host = host_from_fixture(fixture);
                        if host.move_widget(&node_id, x, y).is_ok() {
                            return ActionEmit::operations(self.commit_fixture(fixture, &host.fixture));
                        }
                    }
                    ActionEmit::default()
                }
                "addWidget" => {
                    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("inputSlider");
                    let descriptor = match kind {
                        "neuron" => json!({ "kind": "neuron", "neuronKind": "math.add" }).to_string(),
                        other => json!({ "kind": other }).to_string(),
                    };
                    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                    let mut host = host_from_fixture(fixture);
                    if let Ok(id) = host.add_widget(&descriptor, x, y) {
                        let operations = self.commit_fixture(fixture, &host.fixture);
                        self.runtime.selected_node_ids = vec![id];
                        return ActionEmit::operations(operations);
                    }
                    ActionEmit::default()
                }
                "patchFlowWidgets" => {
                    let widget_ids: Vec<String> = args.and_then(|value| value.get("widgetIds")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                    let raw_value = args.and_then(|value| value.get("value")).and_then(|entry| entry.as_f64());
                    let mut host = host_from_fixture(fixture);
                    let baseline = host.fixture.clone();
                    for widget in host.fixture.widgets.iter_mut() {
                        if !widget_ids.contains(&widget_id(widget).to_string()) {
                            continue;
                        }
                        if let (Widget::InputSlider { value: slider_value, .. }, Some(value)) = (widget, raw_value) {
                            if field == "value" {
                                *slider_value = value;
                            }
                        }
                    }
                    ActionEmit::operations(procedural3d_fixture_operations(&baseline, &host.fixture))
                }
                "reorganize" => {
                    let mut host = host_from_fixture(fixture);
                    if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
                        return ActionEmit::operations(self.commit_fixture(fixture, &host.fixture));
                    }
                    ActionEmit::default()
                }
                "translateSelection" => {
                    let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    self.gumball_transform(fixture, args, "translate", move |host, transform_id| {
                        let current = gumball_widget_offset(host, transform_id);
                        let next = [current[0] + dx, current[1] + dy, current[2] + dz];
                        host.set_neuron_params(transform_id, &gumball_translate_params_json(next)).is_ok()
                    })
                }
                "rotateSelection" => {
                    let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    self.gumball_transform(fixture, args, "rotate", move |host, transform_id| {
                        let current_angle = gumball_widget_number_param(host, transform_id, "angle", 0.0);
                        host.set_neuron_params(transform_id, &gumball_rotate_params_json([ax, ay, az], current_angle + angle)).is_ok()
                    })
                }
                "scaleSelection" => {
                    let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let uniform_factor = (sx + sy + sz) / 3.0;
                    self.gumball_transform(fixture, args, "scale", move |host, transform_id| {
                        let current_factor = gumball_widget_number_param(host, transform_id, "factor", 1.0);
                        host.set_neuron_params(transform_id, &gumball_scale_params_json(current_factor * uniform_factor)).is_ok()
                    })
                }
                "addGeneration" | "removeGeneration" | "selectGeneration" | "renameGeneration" | "updateGenerationValues" => {
                    self.handle_generation(action, args, doc.projection)
                }
                // 🧵 One budgeted evaluation step (see `FlowEvalDriver::tick`), off the main thread —
                // the plugin worker runs this, never the renderer. Chains itself via `DispatchAction`
                // until the fixture's dirty set is empty, then refreshes the mesh preview caches once
                // (cheap: every node hit the shared `procedural_neural_cache()` during ticking).
                "flowEvalTick" => {
                    let mut host = host_from_fixture(fixture);
                    let more = self.runtime.eval_driver.tick(&mut host);
                    if !more {
                        refresh_all_caches(&mut self.runtime, fixture, &doc.projection.generation);
                    }
                    ActionEmit {
                        effects: if more { vec![semio_framework_core::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() },
                        ..ActionEmit::default()
                    }
                }
                _ => ActionEmit::default(),
            }
        }

        /// 🧵 Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes —
        /// covers every mutation path (edits, undo/redo, example load, remote operations) in one place.
        fn pending_effects(&mut self, doc: &DocumentView<'_, Procedural3dDocument>, _view_state: &ViewState) -> Vec<semio_framework_core::kernel::HostEffect> {
            let host = host_from_fixture(&doc.projection.fixture);
            if self.runtime.eval_driver.sync(&host) {
                vec![semio_framework_core::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
            } else {
                Vec::new()
            }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, Procedural3dDocument>, view_state: &ViewState) -> UiNode {
            let envelope = play_view(doc.projection, &self.runtime);
            let host = host_from_fixture(&envelope.fixture);
            let labels = procedural3d_labels(view_state);
            let active_utility = view_state.active_utility_id.as_deref().unwrap_or(PROCEDURAL_3D_TRANSFORM_UTILITY_DEFAULT);
            match body_key {
                PROCEDURAL_3D_PLAY_BODY_MAIN => {
                    let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
                    let viewport_json =
                        serde_json::to_string(&envelope.runtime.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
                    let selection_json = if envelope.runtime.selected_node_ids.is_empty() {
                        None
                    } else {
                        serde_json::to_string(&envelope.runtime.selected_node_ids).ok()
                    };
                    let flow_extras = flow_backed_node_graph_extras(&envelope.fixture, &envelope.runtime.lod_mode, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, Some(&envelope.runtime.eval_driver));
                    let context_menu_json = serde_json::to_string(&json!([{
                        "id": "delete-selection",
                        "label": labels.delete_selection,
                        "icon": "trash",
                        "action": "nodeGraphEdit",
                        "args": { "operations": [{ "operation": "deleteSelection" }] },
                        "destructive": true,
                    }]))
                    .ok();
                    build_node_graph_scene(
                        PROCEDURAL_3D_PLAY_SURFACE_MAIN,
                        PROCEDURAL_3D_PLAY_APP_ID,
                        NodeGraphScene {
                            editable: Some(true),
                            operators_json: flow_extras.operators_json,
                            capabilities_json: flow_extras.capabilities_json,
                            lod_json: flow_extras.lod_json,
                            fixture_json: flow_extras.fixture_json,
                            eval_json: flow_extras.eval_json,
                            computing_json: flow_extras.computing_json,
                            selection_json,
                            hover_json: node_graph_hover_json(&envelope.runtime),
                            context_menu_json,
                            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
                        },
                    )
                }
                PROCEDURAL_3D_PLAY_BODY_PREVIEW => {
                    let (meshes_json, instances_json) = preview_payload_cached(&envelope.runtime, &envelope.fixture);
                    build_world_3d_scene(
                        PROCEDURAL_3D_PLAY_SURFACE_PREVIEW,
                        PROCEDURAL_3D_PLAY_APP_ID,
                        ui_wgpu::World3dScene {
                            status_json: envelope.runtime.eval_driver.pending().then(|| r#"{"computing":true}"#.to_string()),
                            ..world3d_scene(
                                preview_camera_json(&envelope.runtime),
                                meshes_json,
                                instances_json,
                                preview_selection_json(&envelope.runtime, active_utility),
                                &envelope.runtime.sun,
                            )
                        },
                    )
                }
                PROCEDURAL_3D_PLAY_BODY_GENERATIONS => render_generate_generations(&envelope),
                PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM => render_generate_form(&envelope, labels),
                PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(&envelope, labels, active_utility),
                PROCEDURAL_3D_PLAY_BODY_DOCUMENT => {
                    build_document_tree(&envelope.fixture, &envelope.runtime.selected_node_ids, labels)
                }
                PROCEDURAL_3D_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
                PROCEDURAL_3D_PLAY_BODY_INSPECTION => {
                    build_inspector_tree(&envelope.fixture, &envelope.runtime.selected_node_ids, labels)
                }
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn window_measures(
            &self,
            _doc: &DocumentView<'_, Procedural3dDocument>,
            _view_state: &ViewState,
        ) -> std::collections::HashMap<String, Vec<WindowMeasure>> {
            let measures = vec![world3d_sun_measures("procedural3d", &self.runtime.sun, procedural_action)];
            std::collections::HashMap::from([
                (PROCEDURAL_3D_PLAY_WINDOW_MAIN.to_string(), vec![procedural3d_lod_measure(&self.runtime.lod_mode)]),
                (PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.to_string(), measures.clone()),
                (PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), measures),
            ])
        }

        fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
            let labels = procedural3d_labels(view_state);
            let is_de = semio_framework_plugin::is_de_locale(view_state);
            semio_framework_plugin::AppLabelsOverlay::default()
                .window_kind_label(PROCEDURAL_3D_PLAY_WINDOW_MAIN, labels.window_flow)
                .window_kind_label(PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, labels.window_preview)
                .window_kind_label(PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS, labels.window_generations)
                .window_kind_label(PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM, labels.window_generate_form)
                .window_kind_label(PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW, labels.window_generate_preview)
                .mode_label("edit", if is_de { "Bearbeiten" } else { "Edit" })
                .mode_label("generate", if is_de { "Generieren" } else { "Generate" })
                .action_labels(procedural3d_action_labels(is_de))
                .utility_labels(procedural3d_utility_labels(is_de))
                .example_labels(semio_framework_plugin::localized_label_map(is_de, &[
                    (PROCEDURAL_EXAMPLE_HEX_COLUMN, "Hexagonal Mushroom Column", "Sechseckige Pilzsäule"),
                    (PROCEDURAL_EXAMPLE_RECT_EXTRUDE, "Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen"),
                    (PROCEDURAL_EXAMPLE_SPHERE_TORUS, "Sphere Cut With Torus", "Kugel mit Torus geschnitten"),
                ]))
        }
    }

    /// 🎯 Parses `nodeGraphHover` args into the hovered widget id — accepts `null`, `{ nodeId }`, or a
    /// `DagChannelRef` `{ widgetId, port, direction }` payload from the flow graph session.
    fn parse_node_graph_hover_widget_id(args: Option<&Value>) -> Option<Option<String>> {
        let hover = args?.get("hoverJson")?;
        if hover.is_null() {
            return Some(None);
        }
        let parsed = if let Some(text) = hover.as_str() {
            serde_json::from_str::<Value>(text).unwrap_or_else(|_| Value::String(text.to_string()))
        } else {
            hover.clone()
        };
        Some(
            parsed
                .get("widgetId")
                .or_else(|| parsed.get("nodeId"))
                .and_then(|value| value.as_str())
                .map(str::to_string),
        )
    }

    fn node_graph_hover_json(runtime: &Procedural3dRuntime) -> Option<String> {
        runtime.hovered_node_id.as_ref().map(|id| json!({ "nodeId": id }).to_string())
    }

    fn node_graph_selection_ids(args: Option<&Value>) -> Vec<String> {
        if let Some(ids) = args
            .and_then(|value| value.get("nodeIds"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
        {
            return ids;
        }
        selection_ids(args)
    }

    /// 🎯 `semio_framework_plugin::selection_ids`'s "ids" array plus a singular "nodeId" fallback —
    /// this app's actions accept either shape depending on the caller.
    fn selection_ids(args: Option<&Value>) -> Vec<String> {
        let ids = semio_framework_plugin::selection_ids(args);
        if !ids.is_empty() {
            return ids;
        }
        args.and_then(|value| value.get("nodeId"))
            .and_then(|value| value.as_str())
            .map(|id| vec![id.to_string()])
            .unwrap_or_default()
    }
    //#endregion 🔖Procedural3dPlayApp

    //#region 🔖Manifest
    pub fn create_procedural3d_app() -> App {
        App::from_builder(
            App::builder(PROCEDURAL_3D_PLAY_APP_ID, "Procedural 3D").document(["semio", "procedural", "3d"])
                .resource_kind(ResourceKindSpec {
                    id: "3d.procedural".into(),
                    name: "3D Procedural".into(),
                    source_format: "procedural.3d".into(),
                    component_kind: "procedural3d".into(),
                    dimension: "3d".into(),
                    media_capability: OsMediaCapability::MeshOnly,
                    media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
                    schema: "procedural.3d".into(),
                    export_formats: vec![],
                    import_formats: vec![],
                })
                .icon_id("workflow")
                .mode("edit", "Edit")
                .mode("generate", "Generate")
                .default_mode_id("edit")
                .mode_layout("generate", "procedural3d-generate")
                .window_kind(
                    PROCEDURAL_3D_PLAY_WINDOW_MAIN,
                    "Flow",
                    PROCEDURAL_3D_PLAY_BODY_MAIN,
                    SurfaceKind::NodeGraph,
                )
                .window_kind(
                    PROCEDURAL_3D_PLAY_WINDOW_PREVIEW,
                    "Preview",
                    PROCEDURAL_3D_PLAY_BODY_PREVIEW,
                    SurfaceKind::World3d,
                )
                .window_kind(
                    PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS,
                    "Generations",
                    PROCEDURAL_3D_PLAY_BODY_GENERATIONS,
                    SurfaceKind::Canvas2d,
                )
                .window_kind(
                    PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM,
                    "Form",
                    PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM,
                    SurfaceKind::Canvas2d,
                )
                .window_kind(
                    PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW,
                    "Preview",
                    PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW,
                    SurfaceKind::World3d,
                )
                .default_layout(create_default_layout(
                    &[PROCEDURAL_3D_PLAY_WINDOW_MAIN.into(), PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.into()],
                    "row",
                    Some(&[68.0, 32.0]),
                    Some(&["Flow".into(), "Preview".into()]),
                ))
                .named_layout(create_named_layout(
                    "procedural3d-generate",
                    "Generate",
                    create_default_layout(
                        &[
                            PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS.into(),
                            PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM.into(),
                            PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW.into(),
                        ],
                        "row",
                        Some(&[22.0, 43.0, 35.0]),
                        Some(&["Generations".into(), "Form".into(), "Preview".into()]),
                    ),
                    "builtin",
                    Some("sparkles".into()),
                    None,
                ))
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                    PanelGroup::Workbench,
                    PROCEDURAL_3D_PLAY_BODY_DOCUMENT,
                )
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                    PanelGroup::Workbench,
                    PROCEDURAL_3D_PLAY_BODY_CATALOGUE,
                )
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                    PanelGroup::Details,
                    PROCEDURAL_3D_PLAY_BODY_INSPECTION,
                )
                // ✏️ Document-mutating operations — dispatched as VCS operations with a true inverse.
                .operation("setActiveExample", "Set Active Example")
                .operation("nodeGraphEdit", "Edit Graph")
                .operation("deleteSelection", "Delete Selection")
                .operation("removeWidget", "Remove Widget")
                .operation("moveMediaNode", "Move Node")
                .operation("addWidget", "Add Widget")
                .operation("patchFlowWidgets", "Patch Flow Widgets")
                .operation("reorganize", "Reorganize")
                .operation("translateSelection", "Translate Selection")
                .operation("rotateSelection", "Rotate Selection")
                .operation("scaleSelection", "Scale Selection")
                .operation("addGeneration", "Add Generation")
                .operation("removeGeneration", "Remove Generation")
                .operation("renameGeneration", "Rename Generation")
                .operation("updateGenerationValues", "Update Generation Values")
                // 👁️ Ephemeral view actions — selection, hover, world picking, graph camera, sun/LOD/show-mode display toggles, preview camera (emit no operations).
                .view_action("nodeGraphViewport", "Set Viewport")
                .view_action("setSelection", "Set Selection")
                .view_action("selectNode", "Select Node")
                .view_action("nodeGraphSelect", "Node Graph Select")
                .view_action("nodeGraphHover", "Node Graph Hover")
                .view_action("setHover", "Set Hover")
                .view_action("worldPointerDown", "World Pointer Down")
                .view_action("graphPointerDown", "Graph Pointer Down")
                .view_action("worldSelect", "World Select")
                .view_action("worldHover", "World Hover")
                .view_action("setSelectionMethod", "Set Selection Method")
                .view_action("setLodMode", "Set LOD Mode")
                .view_action("setShowMode", "Set Show Mode")
                .view_action("toggleSun", "Toggle Sun")
                .view_action("setSunAzimuth", "Set Sun Azimuth")
                .view_action("setSunElevation", "Set Sun Elevation")
                .view_action("setSunIntensity", "Set Sun Intensity")
                .view_action("setCamera", "Set Camera")
                .view_action("selectGeneration", "Select Generation")
                // 📝 Staged argument forms for the palette-visible actions (defaults materialized host-side).
                .action_args("addWidget", vec![
                    ActionArgDef::select("kind", "Kind", vec![
                        ActionArgOption::new("neuron", "Neuron"),
                        ActionArgOption::new("inputSlider", "Slider"),
                        ActionArgOption::new("inputNote", "Note"),
                        ActionArgOption::new("outputPreview", "Preview"),
                    ]).default_value("inputSlider"),
                ])
                .action_args("setActiveExample", vec![
                    ActionArgDef::select("exampleId", "Example", vec![
                        ActionArgOption::new(PROCEDURAL_EXAMPLE_HEX_COLUMN, "Hexagonal Mushroom Column"),
                        ActionArgOption::new(PROCEDURAL_EXAMPLE_RECT_EXTRUDE, "Rectangle Extrude Volume"),
                        ActionArgOption::new(PROCEDURAL_EXAMPLE_SPHERE_TORUS, "Sphere Cut With Torus"),
                    ]).required(),
                ])
                // 🧰 Transform gumball — an exclusive utility group scoped to the 3D preview window (active utility is host-owned).
                .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", "Move", "move") })
                .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", "Rotate", "rotate-cw") })
                .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", "Scale", "maximize-2") })
                .window_kind_utilities(PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
                .keybinding("mod+z", "undo")
                .keybinding("mod+shift+z", "redo"),
        )
        .example(PROCEDURAL_EXAMPLE_HEX_COLUMN, "Hexagonal Mushroom Column", example_document_json(PROCEDURAL_EXAMPLE_HEX_COLUMN))
        .example(PROCEDURAL_EXAMPLE_RECT_EXTRUDE, "Rectangle Extrude Volume", example_document_json(PROCEDURAL_EXAMPLE_RECT_EXTRUDE))
        .example(PROCEDURAL_EXAMPLE_SPHERE_TORUS, "Sphere Cut With Torus", example_document_json(PROCEDURAL_EXAMPLE_SPHERE_TORUS))
        .program("procedural3d", "Procedural 3D", "brep")
    }

    fn procedural3d_mesh_from_document(doc: &serde_json::Value) -> Result<semio_framework_plugin::MeshData, String> {
        let projection: Procedural3dDocument = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        Ok(export_mesh_from_document(&projection))
    }

    fn procedural3d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
        serde_json::to_value(default_projection()).map_err(|err| err.to_string())
    }

    pub fn register_procedural3d_exports() {
        semio_framework_os::register_mesh_exporter("3d.procedural", "procedural", procedural3d_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
        semio_framework_os::register_mesh_exporter("3d.procedural", "procedural", procedural3d_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
        semio_framework_os::register_mesh_exporter("3d.procedural", "procedural", procedural3d_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
        semio_framework_os::register_mesh_dwg_export_handler("3d.procedural", "procedural", procedural3d_mesh_from_document);
        semio_framework_os::register_mesh_importer("3d.procedural", procedural3d_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
        semio_framework_os::register_mesh_importer("3d.procedural", procedural3d_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
        semio_framework_os::register_mesh_importer("3d.procedural", procedural3d_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
        semio_framework_os::register_mesh_dwg_import_handler("3d.procedural", procedural3d_document_from_mesh);
    }
    //#endregion 🔖Manifest

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use kernel_3d_scene::{
            aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Mesh3d, Vec3,
        };
        use semio_framework_plugin::testkit;
        use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};

        fn meta(actor: &str) -> ActionMeta {
            testkit::meta(actor)
        }

        fn new_app() -> VcsDocumentApp<Procedural3dPlayApp> {
            testkit::new_app::<Procedural3dPlayApp>()
        }

        /// 🧬 A wrapper carrying the real action registry so default-materialization + kind discipline run.
        fn new_app_with_registry() -> VcsDocumentApp<Procedural3dPlayApp> {
            testkit::new_app_with_registry::<Procedural3dPlayApp>(create_procedural3d_app)
        }

        /// 🧵 A `flowEvalTick` chain self-dispatches via `requestedEffects`, which only the JS renderer
        /// drains in production (see `applyHostEffects`'s `dispatchAction` branch) — a unit test has
        /// to do that draining itself. Mirrors `pending_effects`'s own arming logic so tests don't need
        /// to know whether a mutation left the driver already ticking.
        fn drain_flow_eval_ticks(app: &mut VcsDocumentApp<Procedural3dPlayApp>) {
            // 🧵 Arms the chain if it isn't already (a no-operation if a caller already armed it — `sync`
            // correctly declines to re-arm one already scheduled, so this must not gate on its return
            // value). A "flowEvalTick" dispatched with nothing pending is a harmless, immediate no-operation
            // (`evaluate_step`'s own early-return), so always ticking at least once is safe.
            app.pending_effects(&ViewState::default());
            for _ in 0..1000 {
                let result = app.handle_action("flowEvalTick", None, &ViewState::default(), &meta("local")).expect("flowEvalTick");
                if !result.requested_effects.iter().any(|effect| matches!(effect, semio_framework_core::kernel::HostEffect::DispatchAction { action, .. } if action == "flowEvalTick")) {
                    return;
                }
            }
            panic!("flowEvalTick chain did not converge within 1000 ticks");
        }

        #[test]
        fn set_active_example_arg_form_materializes_into_operations() {
            let mut app = new_app_with_registry();
            // The required `exampleId` staged arg drives an operation that rewrites the fixture.
            app.handle_action(
                "setActiveExample",
                Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("set example");
            let projection = app.projection().expect("projection");
            assert!(projection.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.prim3d.sphere")));
        }

        #[test]
        fn node_graph_hover_updates_preview_selection_and_graph_scene() {
            let mut app = new_app();
            app.handle_action(
                "nodeGraphHover",
                Some(&json!({ "hoverJson": { "nodeId": "extrude" } })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("node graph hover");
            let preview = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("preview");
            let preview_json = serde_json::to_string(&preview).expect("preview json");
            assert!(preview_json.contains(r#""hoveredId":"extrude""#) || preview_json.contains(r#""hoveredId": "extrude""#));
            let graph = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("graph");
            let graph_json = serde_json::to_string(&graph).expect("graph json");
            assert!(graph_json.contains(r#""hoverJson":"{\"nodeId\":\"extrude\"}""#) || graph_json.contains(r#""hoverJson": "{\"nodeId\":\"extrude\"}""#));
        }

        #[test]
        fn set_hover_from_world_updates_preview_and_graph_scene() {
            let mut app = new_app();
            app.handle_action("setHover", Some(&json!({ "objectId": "extrude" })), &ViewState::default(), &meta("local"))
                .expect("set hover");
            let preview = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("preview");
            let preview_json = serde_json::to_string(&preview).expect("preview json");
            assert!(preview_json.contains("extrude"));
            app.handle_action("setHover", None, &ViewState::default(), &meta("local")).expect("clear hover");
            let cleared = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("preview cleared");
            let cleared_json = serde_json::to_string(&cleared).expect("cleared json");
            assert!(!cleared_json.contains(r#""hoveredId":"extrude""#));
        }

        #[test]
        fn set_active_utility_switch_clears_scratch_and_emits_no_operations() {
            let mut app = new_app_with_registry();
            app.handle_action("worldHover", Some(&json!({ "id": "extrude" })), &ViewState::default(), &meta("local")).expect("hover");
            let before = app.projection().expect("projection");
            // Switching the gumball utility is the framework-injected View action: it clears scratch and emits no operations.
            let result = app
                .handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "rotate" })), &ViewState::default(), &meta("local"))
                .expect("switch utility");
            assert!(result.operations.is_empty(), "utility switching never emits document operations");
            assert_eq!(app.projection().expect("projection"), before, "utility switching records no history entry");
        }

        #[test]
        fn gumball_drag_coalesces_multi_tick_translate_into_one_edit() {
            let mut app = new_app();
            let before_widgets = app.projection().expect("projection").fixture.widgets.len();
            // A whole gumball drag (three ticks, same coalesce key) folds into ONE undoable edit, not one-operation-per-tick.
            for dx in [1.0, 1.0, 1.0] {
                app.handle_action(
                    "translateSelection",
                    Some(&json!({ "ids": ["extrude"], "dx": dx, "dy": 0.0, "dz": 0.0 })),
                    &ViewState::default(),
                    &meta("local"),
                )
                .expect("drag tick");
            }
            let transform_id = "extrude__gumball_translate";
            let dragged = app.projection().expect("projection");
            assert_eq!(gumball_widget_offset(&host_from_fixture(&dragged.fixture), transform_id), [3.0, 0.0, 0.0], "the three ticks accumulate on one transform node");
            // Undoing the coalesced drag reverts the whole gesture in a single step (splice + all ticks).
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            let restored = app.projection().expect("projection");
            assert_eq!(restored.fixture.widgets.len(), before_widgets, "one undo removes the entire coalesced gumball edit");
            assert!(!restored.fixture.widgets.iter().any(|widget| widget_id(widget) == transform_id), "the spliced transform node is gone after a single undo");
        }

        fn slider_value(projection: &Procedural3dDocument, id: &str) -> Option<f64> {
            projection.fixture.widgets.iter().find_map(|widget| match widget {
                Widget::InputSlider { id: widget_id, value, .. } if widget_id == id => Some(*value),
                _ => None,
            })
        }

        #[test]
        fn renders_node_graph_scene() {
            let mut app = new_app();
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
        }

        #[test]
        fn main_graph_scene_exports_flow_backed_node_graph_fields() {
            let mut app = new_app();
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
            let json = serde_json::to_string(&node).unwrap();
            let value: Value = serde_json::from_str(&json).expect("ui node json");
            let graph = value.get("nodeGraph").expect("nodeGraph");
            assert!(graph.get("fixtureJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow.fixture")));
            assert!(graph.get("operatorsJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("math.add") || s.contains("brep.")));
            let capabilities = graph.get("capabilitiesJson").and_then(|v| v.as_str()).unwrap_or_default();
            assert!(capabilities.contains("flow"), "missing flow engine capability: {capabilities}");
        }

        #[test]
        fn set_lod_mode_is_a_view_action_with_no_document_operations() {
            let mut app = new_app();
            let before = app.projection().expect("projection");
            app.handle_action("setLodMode", Some(&json!({ "value": "wireframe" })), &ViewState::default(), &meta("local")).expect("lod");
            assert_eq!(app.projection().expect("projection"), before, "setLodMode must not mutate the document");
        }

        #[test]
        fn sun_measures_are_exposed_on_preview_windows() {
            let mut app = new_app();
            let measures = app.window_measures(&ViewState::default());
            assert!(measures.contains_key(PROCEDURAL_3D_PLAY_WINDOW_PREVIEW));
            assert!(measures.contains_key(PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW));
            // 👁️ Sun toggling is a view action: it must not record a document operation.
            let before = app.projection().expect("projection");
            app.handle_action("toggleSun", None, &ViewState::default(), &meta("local")).expect("toggle sun");
            assert_eq!(app.projection().expect("projection"), before, "toggleSun must not mutate the document");
        }

        #[test]
        fn set_active_example_loads_sphere_fixture() {
            let mut app = new_app();
            app.handle_action(
                "setActiveExample",
                Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("set example");
            let projection = app.projection().expect("projection");
            assert!(projection.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.prim3d.sphere")));
        }

        #[test]
        fn sphere_cut_example_preview_renders_meshes() {
            // 🧵 Loading the example never evaluates synchronously anymore (see `pending_effects`) —
            // draining the `flowEvalTick` chain here simulates what the JS renderer's `applyHostEffects`
            // does automatically after every refresh, so the render below sees the real evaluated
            // geometry rather than the cold-start placeholder mesh.
            let mut app = new_app();
            app.handle_action(
                "setActiveExample",
                Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("set example");
            drain_flow_eval_ticks(&mut app);
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("render");
            let parsed: ui_wgpu::UiNode = serde_json::from_str(&serde_json::to_string(&node).unwrap()).expect("preview ui json");
            match parsed {
                ui_wgpu::UiNode::ComponentScene(scene) => {
                    let world = scene.world_3d.expect("world_3d payload");
                    assert_ne!(world.meshes_json, "[]");
                    assert_ne!(world.instances_json, "[]");
                }
                other => panic!("expected component scene, got {other:?}"),
            }
        }

        #[test]
        fn sphere_cut_example_computing_chrome_clears_once_ticks_converge() {
            let mut app = new_app();
            app.handle_action(
                "setActiveExample",
                Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("set example");
            let main_graph = |app: &mut VcsDocumentApp<Procedural3dPlayApp>| -> ui_wgpu::NodeGraphScene {
                let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
                match serde_json::from_str::<ui_wgpu::UiNode>(&serde_json::to_string(&node).unwrap()).expect("graph ui json") {
                    ui_wgpu::UiNode::ComponentScene(scene) => scene.node_graph.expect("node_graph payload"),
                    other => panic!("expected component scene, got {other:?}"),
                }
            };
            // 🧵 In production, `pending_effects` runs after every `refreshUi` pass — a test driving
            // `render` directly has to call it explicitly to arm the driver the same way. Before any
            // tick runs, the graph must flag the cut node (and its downstream preview) as computing —
            // this is what drives the dag canvas's animated loading border.
            assert!(!app.pending_effects(&ViewState::default()).is_empty(), "loading the example must arm a tick chain");
            assert!(main_graph(&mut app).computing_json.is_some(), "pending nodes must be reported before the chain runs");
            drain_flow_eval_ticks(&mut app);
            assert!(main_graph(&mut app).computing_json.is_none(), "computing chrome clears once the chain converges");
        }

        #[test]
        fn patch_flow_widgets_edits_slider_value() {
            let mut app = new_app();
            app.handle_action(
                "patchFlowWidgets",
                Some(&json!({ "widgetIds": ["height"], "field": "value", "value": 9.5 })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("patch");
            assert_eq!(slider_value(&app.projection().expect("projection"), "height"), Some(9.5));
        }

        #[test]
        fn preview_payload_has_meshes_and_instances() {
            let projection = default_projection();
            let runtime = Procedural3dRuntime::default();
            let (meshes_json, instances_json) = evaluated_preview_payload(&projection.fixture, &runtime);
            assert_ne!(meshes_json, "[]", "meshes_json was empty");
            assert_ne!(instances_json, "[]", "instances_json was empty");
            let meshes: Vec<serde_json::Value> = serde_json::from_str(&meshes_json).expect("meshes json");
            let instances: Vec<serde_json::Value> = serde_json::from_str(&instances_json).expect("instances json");
            assert!(!meshes.is_empty());
            assert!(!instances.is_empty());
            for mesh in &meshes {
                let data: semio_framework_core::MeshData =
                    serde_json::from_value(mesh.get("data").cloned().unwrap_or_default()).expect("mesh data");
                assert!(data.positions.len() >= 9, "mesh has too few positions");
                assert!(data.indices.len() >= 3, "mesh has too few indices");
            }
            let camera = Camera3d {
                position: Vec3::from_array([
                    runtime.preview_camera.position[0] as f32,
                    runtime.preview_camera.position[1] as f32,
                    runtime.preview_camera.position[2] as f32,
                ]),
                target: Vec3::from_array([
                    runtime.preview_camera.target[0] as f32,
                    runtime.preview_camera.target[1] as f32,
                    runtime.preview_camera.target[2] as f32,
                ]),
                up: Vec3::new(0.0, 0.0, 1.0),
                fov_y: runtime.preview_camera.fov as f32 * std::f32::consts::PI / 180.0,
                near: 0.1,
                far: 1000.0,
            };
            let view_proj = camera.view_proj(0.6);
            let planes = frustum_planes(view_proj);
            let mut visible = 0usize;
            for instance in instances {
                let mesh_id = instance
                    .get("meshId")
                    .or_else(|| instance.get("mesh_id"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("box");
                let mesh = meshes
                    .iter()
                    .find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id))
                    .expect("mesh record");
                let data: semio_framework_core::MeshData =
                    serde_json::from_value(mesh.get("data").cloned().unwrap_or_default()).expect("mesh data");
                let mesh3d = Mesh3d::from_buffers(data.positions, data.normals, data.indices);
                let position = instance
                    .get("position")
                    .and_then(|value| value.as_array())
                    .map(|items| {
                        [
                            items[0].as_f64().unwrap_or(0.0) as f32,
                            items[1].as_f64().unwrap_or(0.0) as f32,
                            items[2].as_f64().unwrap_or(0.0) as f32,
                        ]
                    })
                    .unwrap_or([0.0, 0.0, 0.0]);
                let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]);
                let (min, max) = transform_aabb(model, mesh3d.aabb_min, mesh3d.aabb_max);
                if aabb_intersects_frustum(&planes, min, max) {
                    visible += 1;
                }
            }
            assert!(visible > 0, "no preview instances intersect camera frustum");
        }

        #[test]
        fn renders_world_preview_scene() {
            let mut app = new_app();
            drain_flow_eval_ticks(&mut app);
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("render");
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("world-3d"));
            let parsed: ui_wgpu::UiNode = serde_json::from_str(&json).expect("preview ui json");
            match parsed {
                ui_wgpu::UiNode::ComponentScene(scene) => {
                    assert_eq!(scene.component_kind, SurfaceKind::World3d);
                    let world = scene.world_3d.expect("world_3d payload");
                    assert_ne!(world.meshes_json, "[]");
                    assert_ne!(world.instances_json, "[]");
                }
                other => panic!("expected component scene, got {other:?}"),
            }
        }

        #[test]
        fn add_widget_action_appends_widget() {
            let mut app = new_app();
            let before = app.projection().expect("projection").fixture.widgets.len();
            app.handle_action("addWidget", Some(&json!({ "kind": "inputNote" })), &ViewState::default(), &meta("local")).expect("add");
            assert!(app.projection().expect("projection").fixture.widgets.len() > before);
        }

        #[test]
        fn generate_mode_renders_surfaces() {
            let mut app = new_app();
            let generations = app.render(PROCEDURAL_3D_PLAY_BODY_GENERATIONS, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&generations).unwrap().contains("addGeneration"));
        }

        #[test]
        fn add_generation_records_an_undoable_generation_operation() {
            let mut app = new_app();
            testkit::assert_undo_redo_round_trip(
                &mut app,
                "addGeneration",
                None,
                |app| app.projection().expect("projection").generation.generations.len(),
                0,
                1,
            );
        }

        #[test]
        fn translate_selection_persists_transform_into_flow_graph() {
            let mut app = new_app();
            let before = app.projection().expect("projection");
            assert!(before.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"));
            app.handle_action(
                "translateSelection",
                Some(&json!({ "ids": ["extrude"], "dx": 1.0, "dy": 2.0, "dz": 3.0 })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("translate");
            let projection = app.projection().expect("projection");
            let transform_id = "extrude__gumball_translate";
            let transform = projection.fixture.widgets.iter().find(|widget| widget_id(widget) == transform_id).expect("transform neuron created");
            assert!(matches!(transform, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.translate"));
            let offset = gumball_widget_offset(&host_from_fixture(&projection.fixture), transform_id);
            assert_eq!(offset, [1.0, 2.0, 3.0]);
            let source = projection.fixture.widgets.iter().find(|widget| widget_id(widget) == "extrude").expect("source widget");
            assert!(matches!(source, Widget::Neuron { preview, .. } if !*preview), "source preview should turn off once gumball-transformed");
            assert!(projection.fixture.synapses.iter().any(|synapse| synapse.from == transform_id && synapse.to == "column-preview"), "downstream rewired through transform node");
            assert!(!projection.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"), "old direct edge removed");

            // Re-grabbing the same transform accumulates the delta instead of creating a second node.
            app.handle_action(
                "translateSelection",
                Some(&json!({ "ids": [transform_id], "dx": 1.0, "dy": 0.0, "dz": 0.0 })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("translate again");
            let projection2 = app.projection().expect("projection");
            assert_eq!(projection2.fixture.widgets.iter().filter(|widget| widget_id(widget) == transform_id).count(), 1);
            assert_eq!(gumball_widget_offset(&host_from_fixture(&projection2.fixture), transform_id), [2.0, 2.0, 3.0]);
        }

        #[test]
        fn rotate_and_scale_selection_persist_into_flow_graph() {
            let mut app = new_app();
            app.handle_action(
                "rotateSelection",
                Some(&json!({ "ids": ["extrude"], "angle": std::f64::consts::FRAC_PI_2 })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("rotate");
            let rotated = app.projection().expect("projection");
            let rotate_id = "extrude__gumball_rotate";
            assert!(rotated.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == rotate_id && neuron_kind == "brep.xform.rotate")));
            assert_eq!(gumball_widget_number_param(&host_from_fixture(&rotated.fixture), rotate_id, "angle", 0.0), std::f64::consts::FRAC_PI_2);

            let mut scale_app = new_app();
            scale_app.handle_action(
                "scaleSelection",
                Some(&json!({ "ids": ["extrude"], "sx": 2.0, "sy": 2.0, "sz": 2.0 })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("scale");
            let scaled = scale_app.projection().expect("projection");
            let scale_id = "extrude__gumball_scale";
            assert!(scaled.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == scale_id && neuron_kind == "brep.xform.scale")));
            assert_eq!(gumball_widget_number_param(&host_from_fixture(&scaled.fixture), scale_id, "factor", 1.0), 2.0);
        }

        #[test]
        fn undo_redo_round_trips_flow_graph_edits() {
            let mut app = new_app();
            let before = app.projection().expect("projection").fixture.widgets.len();
            testkit::assert_undo_redo_round_trip(
                &mut app,
                "addWidget",
                Some(&json!({ "kind": "inputNote" })),
                |app| app.projection().expect("projection").fixture.widgets.len(),
                before,
                before + 1,
            );
        }

        #[test]
        fn remove_widget_action_deletes_by_id_and_supports_undo() {
            let mut app = new_app();
            assert!(app.projection().expect("projection").fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"));
            testkit::assert_undo_redo_round_trip(
                &mut app,
                "removeWidget",
                Some(&json!({ "widgetId": "sides" })),
                |app| app.projection().expect("projection").fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"),
                true,
                false,
            );
        }

        #[test]
        fn two_instances_converge_disjoint_widget_moves() {
            let widgets: Vec<String> = new_app()
                .projection()
                .expect("projection")
                .fixture
                .widgets
                .iter()
                .map(|widget| widget_id(widget).to_string())
                .collect();
            assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
            let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
            testkit::assert_two_instances_converge::<Procedural3dPlayApp, (Option<f64>, Option<f64>)>(
                "mem://procedural3d-convergence",
                ("moveMediaNode", Some(&json!({ "nodeId": w0, "x": 111.0, "y": 5.0 }))),
                ("moveMediaNode", Some(&json!({ "nodeId": w1, "x": 222.0, "y": 6.0 }))),
                move |app| {
                    let layout = &app.projection().expect("projection").fixture.layout;
                    (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
                },
            );
        }

        #[test]
        fn document_from_mesh_returns_valid_default_projection() {
            let mesh = semio_framework_plugin::MeshData::default();
            let document = procedural3d_document_from_mesh(&mesh).expect("dwg mesh import document");
            let projection: Procedural3dDocument = serde_json::from_value(document).expect("parseable projection");
            assert_eq!(projection.fixture.schema, "flow.fixture");
        }

        #[test]
        fn procedural3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs() {
            use semio_framework_plugin::{
                GlbExporter, GlbImporter, MeshExporter, MeshImporter, ObjExporter, ObjImporter, StlExporter, StlImporter,
            };
            let document_json = serde_json::to_value(default_projection()).expect("projection json");
            let mesh = procedural3d_mesh_from_document(&document_json).expect("mesh from document");
            assert!(!mesh.positions.is_empty());

            let obj_bytes = ObjExporter.export(&mesh).expect("obj export");
            let obj_mesh = ObjImporter.import(&obj_bytes).expect("obj import");
            let obj_document = procedural3d_document_from_mesh(&obj_mesh).expect("obj document from mesh");
            let _: Procedural3dDocument = serde_json::from_value(obj_document).expect("parseable obj projection");

            let glb_bytes = GlbExporter.export(&mesh).expect("glb export");
            let glb_mesh = GlbImporter.import(&glb_bytes).expect("glb import");
            let glb_document = procedural3d_document_from_mesh(&glb_mesh).expect("glb document from mesh");
            let _: Procedural3dDocument = serde_json::from_value(glb_document).expect("parseable glb projection");

            let stl_bytes = StlExporter.export(&mesh).expect("stl export");
            let stl_mesh = StlImporter.import(&stl_bytes).expect("stl import");
            let stl_document = procedural3d_document_from_mesh(&stl_mesh).expect("stl document from mesh");
            let _: Procedural3dDocument = serde_json::from_value(stl_document).expect("parseable stl projection");
        }

        #[test]
        fn procedural3d_labels_resolve_native_english_by_default() {
            let mut app = new_app();
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("\"Widgets\""));
            assert!(json.contains("\"Slider\""));
            assert!(!json.contains("Elemente"));
        }

        #[test]
        fn procedural3d_labels_translate_catalogue_and_inspector_in_german() {
            let mut app = new_app();
            let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
            let catalogue = app.render(PROCEDURAL_3D_PLAY_BODY_CATALOGUE, None, &view_state).expect("render");
            let catalogue_json = serde_json::to_string(&catalogue).unwrap();
            assert!(catalogue_json.contains("\"Elemente\""));
            assert!(catalogue_json.contains("Schieberegler"));
            assert!(!catalogue_json.contains("\"Widgets\""));
            let inspector = app.render(PROCEDURAL_3D_PLAY_BODY_INSPECTION, None, &view_state).expect("render");
            let inspector_json = serde_json::to_string(&inspector).unwrap();
            assert!(inspector_json.contains("Elemente:"));
        }
    }
    //#endregion 🧪Tests
}

//#region 🔖Bundle
fn register_procedural_exports() {
    app_2d::register_procedural2d_exports();
    app_3d::register_procedural3d_exports();
}

semio_framework_plugin::semio_plugin! {
    id: "procedural",
    label: "Procedural",
    version: "0.1.0",
    setup: register_procedural_exports,
    apps: [
        app_2d::create_procedural2d_app => app_2d::Procedural2dPlayApp,
        app_3d::create_procedural3d_app => app_3d::Procedural3dPlayApp,
    ]
}
//#endregion 🔖Bundle
