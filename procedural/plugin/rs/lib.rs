//! 🔧 Procedural plugin — 2D and 3D flow apps in one hot-swappable WASM component.

pub mod app_2d {
    //! 🎲 Procedural 2D plugin — procedural flow play app bundled as a hot-swappable WASM component.

    use flow_core::{dag::DagFixture, flow_backed_node_graph_extras, flow_neuron_kind_infos_json, forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec}, FlowFixture, FlowHost, Widget};
    use flow_module_draw::render_scene_json;
    use procedural_2d::{procedural2d_fixture_ops, Procedural2dDocument, Procedural2dOp, PROCEDURAL_2D_SCHEMA};
    use semio_framework_plugin::{SurfaceKind, PanelGroup,
        apply_generation_op, build_canvas_2d_scene, build_node_graph_scene, create_default_layout, create_named_layout,
        generation_ops, render_generation_form_body, render_generation_preview_text, render_generations_tree,
        select_generation, selected_generation, ui_inspector_groups_to_tree, ui_inspector_readonly_field,
        ui_stack_vertical, ui_text, ActionEmit, App, Canvas2dScene, ActionDescriptor, DocumentApp, DocumentView,
        GenerationPlayState, NodeGraphScene, UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode,
        UiTreeSectionNode, ViewState,
        FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
        FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    };
    use serde::{Deserialize, Serialize};
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

    //#region 🔖Terminology
    /// 🗣️ Complete UI label set for the 2D flow app; one field per label makes every locale combination compile-checked.
    struct Procedural2dLabels {
        sources: &'static str,
        components: &'static str,
        sinks: &'static str,
        show_mode_section: &'static str,
        show_prefix: &'static str,
        none: &'static str,
        selection: &'static str,
        ids: &'static str,
        schema_prefix: &'static str,
        widgets_prefix: &'static str,
        show_mode_prefix: &'static str,
        generate_hint: &'static str,
        preview_hint: &'static str,
        source_slider: &'static str,
        source_stepper: &'static str,
        source_note: &'static str,
        component_add: &'static str,
        component_and: &'static str,
        component_concat: &'static str,
        sink_preview: &'static str,
        sink_export: &'static str,
        window_main: &'static str,
        window_preview: &'static str,
        window_generations: &'static str,
        window_generate_form: &'static str,
        window_generate_preview: &'static str,
    }

    const PROCEDURAL2D_LABELS_NATIVE_EN: Procedural2dLabels = Procedural2dLabels {
        sources: "Sources",
        components: "Components",
        sinks: "Sinks",
        show_mode_section: "Show mode",
        show_prefix: "Show",
        none: "(none)",
        selection: "Selection",
        ids: "Ids",
        schema_prefix: "Schema:",
        widgets_prefix: "Widgets:",
        show_mode_prefix: "Show mode:",
        generate_hint: "Add a generation to edit input values.",
        preview_hint: "(evaluate a generation to preview output)",
        source_slider: "Slider",
        source_stepper: "Stepper",
        source_note: "Note",
        component_add: "Add",
        component_and: "And",
        component_concat: "Concat",
        sink_preview: "Preview",
        sink_export: "Export",
        window_main: "Flow",
        window_preview: "Preview",
        window_generations: "Generations",
        window_generate_form: "Form",
        window_generate_preview: "Preview",
    };

    const PROCEDURAL2D_LABELS_NATIVE_DE: Procedural2dLabels = Procedural2dLabels {
        sources: "Quellen",
        components: "Komponenten",
        sinks: "Senken",
        show_mode_section: "Anzeigemodus",
        show_prefix: "Anzeigen",
        none: "(keine)",
        selection: "Auswahl",
        ids: "Kennungen",
        schema_prefix: "Schema:",
        widgets_prefix: "Elemente:",
        show_mode_prefix: "Anzeigemodus:",
        generate_hint: "Erstelle eine Generation, um Eingabewerte zu bearbeiten.",
        preview_hint: "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)",
        source_slider: "Schieberegler",
        source_stepper: "Schrittzähler",
        source_note: "Notiz",
        component_add: "Addieren",
        component_and: "Und",
        component_concat: "Verketten",
        sink_preview: "Vorschau",
        sink_export: "Export",
        window_main: "Fluss",
        window_preview: "Vorschau",
        window_generations: "Generationen",
        window_generate_form: "Formular",
        window_generate_preview: "Vorschau",
    };

    /// 🗣️ Resolves the active label set from the shell-provided locale; falls back to native English.
    fn procedural2d_labels(view_state: &ViewState) -> &'static Procedural2dLabels {
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        if is_de { &PROCEDURAL2D_LABELS_NATIVE_DE } else { &PROCEDURAL2D_LABELS_NATIVE_EN }
    }
    //#endregion 🔖Terminology

    //#region 🔖Types
    /// 👁️ Ephemeral per-session view state — never part of the persisted document. Selection, the
    /// active show mode, the last evaluation outputs, and the derived generation preview all live
    /// here on the app struct, out of the VCS document.
    #[derive(Clone, Debug)]
    struct Procedural2dPlayRuntime {
        selected_ids: Vec<String>,
        show_mode: String,
        eval_outputs_json: String,
        selected_generation_id: Option<String>,
        generation_preview_text: Option<String>,
    }

    impl Default for Procedural2dPlayRuntime {
        fn default() -> Self {
            Self {
                selected_ids: Vec::new(),
                show_mode: default_show_mode(),
                eval_outputs_json: String::new(),
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

    fn host_from_fixture(fixture: &FlowFixture) -> FlowHost {
        let mut host = FlowHost::from_fixture(fixture.clone());
        host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
        host
    }

    fn selection_ids(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("nodeIds"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .or_else(|| {
                args.and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
            })
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
            | Widget::InputStepper { id, .. }
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
        let mut host = host_from_fixture(&play.fixture);
        let eval_json = if play.runtime.eval_outputs_json.is_empty() {
            host.evaluate().unwrap_or_default()
        } else {
            host.apply_eval_outputs_json(&play.runtime.eval_outputs_json);
            play.runtime.eval_outputs_json.clone()
        };
        let prefix = if preview { "procedural2d-preview" } else { "procedural2d-main" };
        let mut layers = Vec::new();
        if let Ok(outputs) = serde_json::from_str::<Value>(&eval_json) {
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
        runtime.eval_outputs_json = preview;
    }
    //#endregion 🔖DocumentHelpers

    //#region 🔖Panels
    fn tree_item(id: impl Into<String>, label: impl Into<String>, action: Option<ActionDescriptor>) -> UiTreeItemNode {
        UiTreeItemNode {
            id: id.into(),
            label: label.into(),
            description: None,
            icon_id: None,
            selected: None,
            default_open: None,
            hover_action: None,
            unhover_action: None,
            actions: None,
            action,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        }
    }

    fn build_document_tree(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
        let widget_items: Vec<UiTreeItemNode> = play
            .fixture
            .widgets
            .iter()
            .map(|widget| {
                let id = widget_id(widget).to_string();
                tree_item(
                    format!("procedural2d-play-document.widget.{id}"),
                    id.clone(),
                    Some(procedural2d_action("setSelection", Some(json!({ "ids": [id] })))),
                )
            })
            .collect();
        UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode {
                id: "procedural2d-play-document.widgets".into(),
                label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
                default_open: Some(true),
                items: if widget_items.is_empty() {
                    vec![tree_item("procedural2d-play-document.empty", labels.none, None)]
                } else {
                    widget_items
                },
            }],
            selected_ids: Some(
                play.runtime
                    .selected_ids
                    .iter()
                    .map(|id| format!("procedural2d-play-document.widget.{id}"))
                    .collect(),
            ),
            highlighted_ids: None,
            selection_change: Some(procedural2d_action("setSelection", None)),
            drop_action: None,
        })
    }

    fn build_catalogue_tree(labels: &Procedural2dLabels) -> UiNode {
        let sources = [("inputSlider", labels.source_slider), ("inputStepper", labels.source_stepper), ("inputNote", labels.source_note)];
        let components = [("math.add", labels.component_add), ("logic.and", labels.component_and), ("text.concat", labels.component_concat)];
        let sinks = [("outputPreview", labels.sink_preview), ("outputExport", labels.sink_export)];
        UiNode::Tree(UiTreeNode {
            sections: vec![
                UiTreeSectionNode {
                    id: "procedural2d-play-catalogue.sources".into(),
                    label: Some(labels.sources.into()),
                    default_open: Some(true),
                    items: sources
                        .iter()
                        .map(|(kind, label)| {
                            tree_item(
                                format!("procedural2d-play-catalogue.source.{kind}"),
                                *label,
                                Some(procedural2d_action("addWidget", Some(json!({ "kind": kind })))),
                            )
                        })
                        .collect(),
                },
                UiTreeSectionNode {
                    id: "procedural2d-play-catalogue.components".into(),
                    label: Some(labels.components.into()),
                    default_open: Some(true),
                    items: components
                        .iter()
                        .map(|(kind, label)| {
                            tree_item(
                                format!("procedural2d-play-catalogue.component.{kind}"),
                                *label,
                                Some(procedural2d_action(
                                    "addWidget",
                                    Some(json!({ "kind": "neuron", "neuronKind": kind })),
                                )),
                            )
                        })
                        .collect(),
                },
                UiTreeSectionNode {
                    id: "procedural2d-play-catalogue.sinks".into(),
                    label: Some(labels.sinks.into()),
                    default_open: Some(true),
                    items: sinks
                        .iter()
                        .map(|(kind, label)| {
                            tree_item(
                                format!("procedural2d-play-catalogue.sink.{kind}"),
                                *label,
                                Some(procedural2d_action("addWidget", Some(json!({ "kind": kind })))),
                            )
                        })
                        .collect(),
                },
                UiTreeSectionNode {
                    id: "procedural2d-play-catalogue.modes".into(),
                    label: Some(labels.show_mode_section.into()),
                    default_open: Some(false),
                    items: ["preview", "generate", "wire"]
                        .iter()
                        .map(|mode| {
                            tree_item(
                                format!("procedural2d-play-catalogue.mode.{mode}"),
                                format!("{} {mode}", labels.show_prefix),
                                Some(procedural2d_action("setShowMode", Some(json!({ "value": mode })))),
                            )
                        })
                        .collect(),
                },
            ],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        })
    }

    fn build_inspector_tree(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
        if play.runtime.selected_ids.is_empty() {
            return ui_stack_vertical(vec![
                ui_text(format!("{} flow.fixture", labels.schema_prefix)),
                ui_text(format!("{} {}", labels.widgets_prefix, play.fixture.widgets.len())),
                ui_text(format!("{} {}", labels.show_mode_prefix, play.runtime.show_mode)),
            ]);
        }
        ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
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
    fn render_main_graph(play: &Procedural2dPlayView) -> UiNode {
        let host = host_from_fixture(&play.fixture);
        let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
        let viewport_json = serde_json::to_string(&play.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
        let selection_json = if play.runtime.selected_ids.is_empty() {
            None
        } else {
            serde_json::to_string(&play.runtime.selected_ids).ok()
        };
        let flow_extras = flow_backed_node_graph_extras(&play.fixture, "", 0.0);
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
                context_menu_json: Some(
                    r#"[{"id":"delete-selection","label":"Delete selection","action":"nodeGraphEdit","args":{"ops":[{"op":"deleteSelection"}]}}]"#.into(),
                ),
                ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
            },
        )
    }

    fn render_main_canvas(play: &Procedural2dPlayView) -> UiNode {
        build_canvas_2d_scene(
            PROCEDURAL2D_PLAY_SURFACE_MAIN,
            PROCEDURAL2D_PLAY_APP_ID,
            Canvas2dScene {
                camera_x: play.fixture.camera.x,
                camera_y: play.fixture.camera.y,
                zoom: play.fixture.camera.zoom,
                layers_json: eval_preview_layers(play, false),
            },
        )
    }

    fn render_preview_canvas(play: &Procedural2dPlayView) -> UiNode {
        build_canvas_2d_scene(
            PROCEDURAL2D_PLAY_SURFACE_PREVIEW,
            PROCEDURAL2D_PLAY_APP_ID,
            Canvas2dScene {
                camera_x: play.fixture.camera.x,
                camera_y: play.fixture.camera.y,
                zoom: play.fixture.camera.zoom,
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
                camera_x: play.fixture.camera.x,
                camera_y: play.fixture.camera.y,
                zoom: play.fixture.camera.zoom,
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
        /// 🔀 Runs a host mutation seeded from the projection fixture and diffs the result into ops.
        fn ops_from_host_mutation(
            &self,
            fixture: &FlowFixture,
            mutate: impl FnOnce(&mut FlowHost),
        ) -> Vec<Procedural2dOp> {
            let mut host = host_from_fixture(fixture);
            mutate(&mut host);
            procedural2d_fixture_ops(fixture, &host.fixture)
        }

        /// 🧬 Emits generation ops for the generate-mode actions, updating ephemeral selection and
        /// preview from the post-op state. `selectGeneration` is a view action (no ops).
        fn handle_generation(
            &mut self,
            action: &str,
            args: Option<&Value>,
            projection: &Procedural2dDocument,
        ) -> ActionEmit<Procedural2dOp> {
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
            let Some(ops) = generation_ops(action, args, &state, &spec) else {
                return ActionEmit::default();
            };
            for op in &ops {
                apply_generation_op(&mut state, op);
            }
            self.runtime.selected_generation_id = state.selected_generation_id.clone();
            refresh_generation_preview(&mut self.runtime, &projection.fixture, &state);
            let coalesce_key = (action == "updateGenerationValues").then(|| "generation-values".to_string());
            ActionEmit {
                ops: ops.into_iter().map(Procedural2dOp::Generation).collect(),
                coalesce_key,
                ..Default::default()
            }
        }
    }

    impl DocumentApp for Procedural2dPlayApp {
        type Projection = Procedural2dDocument;
        type Op = Procedural2dOp;

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
        ) -> ActionEmit<Procedural2dOp> {
            let fixture = &doc.projection.fixture;
            match action {
                // 👁️ View actions — mutate ephemeral runtime, emit no ops.
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
                    self.runtime.eval_outputs_json = host_from_fixture(fixture).evaluate().unwrap_or_default();
                    self.runtime.show_mode = "generate".into();
                    ActionEmit::default()
                }
                "setEvalOutputs" => {
                    if let Some(outputs) = args.and_then(|value| value.get("outputs")) {
                        self.runtime.eval_outputs_json = outputs.to_string();
                    } else if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                        self.runtime.eval_outputs_json = json_text.into();
                    }
                    ActionEmit::default()
                }
                "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" | "canvasWheel" => ActionEmit::default(),
                // 📷 Camera — a coalesced scalar op so a pan/zoom gesture is one undo step.
                "nodeGraphViewport" => {
                    if let Some(camera) = args
                        .and_then(|value| value.get("viewportJson"))
                        .and_then(|value| value.as_str())
                        .and_then(|json| serde_json::from_str(json).ok())
                    {
                        return ActionEmit {
                            ops: vec![Procedural2dOp::SetCamera { camera }],
                            coalesce_key: Some("camera".into()),
                            ..Default::default()
                        };
                    }
                    ActionEmit::default()
                }
                // ✏️ Operations — compute the target fixture via the host, emit fixture ops.
                "nodeGraphEdit" => {
                    let sub_ops = args
                        .and_then(|value| value.get("ops"))
                        .and_then(|value| value.as_array())
                        .cloned()
                        .unwrap_or_default();
                    let selected = self.runtime.selected_ids.clone();
                    let mut cleared = false;
                    let ops = self.ops_from_host_mutation(fixture, |host| {
                        for op in &sub_ops {
                            match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
                                "setFixture" => {
                                    if let Some(fixture) = op
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
                                    let from = op.get("sourceNodeId").and_then(|value| value.as_str());
                                    let from_port = op.get("sourcePortId").and_then(|value| value.as_str());
                                    let to = op.get("targetNodeId").and_then(|value| value.as_str());
                                    let to_port = op.get("targetPortId").and_then(|value| value.as_str());
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
                    ActionEmit::ops(ops)
                }
                "moveMediaNode" => {
                    let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(str::to_string);
                    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                    if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                        return ActionEmit::ops(self.ops_from_host_mutation(fixture, |host| {
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
                    if let Ok(id) = host.add_widget(&descriptor, x, y) {
                        self.runtime.selected_ids = vec![id];
                        return ActionEmit::ops(procedural2d_fixture_ops(fixture, &host.fixture));
                    }
                    ActionEmit::default()
                }
                "removeWidget" => {
                    let widget_id = args.and_then(|value| value.get("widgetId")).and_then(|value| value.as_str()).map(str::to_string);
                    if let Some(widget_id) = widget_id {
                        let ops = self.ops_from_host_mutation(fixture, |host| {
                            let _ = host.remove_widget(&widget_id);
                        });
                        if !ops.is_empty() {
                            self.runtime.selected_ids.retain(|id| id != &widget_id);
                        }
                        return ActionEmit::ops(ops);
                    }
                    ActionEmit::default()
                }
                "connectMediaPorts" => {
                    let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                    let from_port = args.and_then(|value| value.get("sourcePortId")).and_then(|value| value.as_str()).map(str::to_string);
                    let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                    let to_port = args.and_then(|value| value.get("targetPortId")).and_then(|value| value.as_str()).map(str::to_string);
                    if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                        return ActionEmit::ops(self.ops_from_host_mutation(fixture, |host| {
                            let _ = host.connect_ports(&from, &from_port, &to, &to_port);
                        }));
                    }
                    ActionEmit::default()
                }
                "reorganize" => ActionEmit::ops(self.ops_from_host_mutation(fixture, |host| {
                    let _ = host.reorganize(r#"{"orientation":"leftRight"}"#);
                })),
                "addGeneration" | "removeGeneration" | "selectGeneration" | "renameGeneration" | "updateGenerationValues" => {
                    self.handle_generation(action, args, doc.projection)
                }
                _ => ActionEmit::default(),
            }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, Procedural2dDocument>, view_state: &ViewState) -> UiNode {
            let play = play_view(doc.projection, &self.runtime);
            let labels = procedural2d_labels(view_state);
            match body_key {
                PROCEDURAL2D_PLAY_BODY_MAIN => render_main_graph(&play),
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
            semio_framework_plugin::AppLabelsOverlay {
                app_label: None,
                window_kind_labels: std::collections::HashMap::from([
                    (PROCEDURAL2D_PLAY_WINDOW_MAIN.to_string(), labels.window_main.to_string()),
                    (PROCEDURAL2D_PLAY_WINDOW_PREVIEW.to_string(), labels.window_preview.to_string()),
                    (PROCEDURAL2D_PLAY_WINDOW_GENERATIONS.to_string(), labels.window_generations.to_string()),
                    (PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM.to_string(), labels.window_generate_form.to_string()),
                    (PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), labels.window_generate_preview.to_string()),
                ]),
                panel_tab_labels: std::collections::HashMap::new(),
                mode_labels: std::collections::HashMap::new(),
            }
        }
    }
    //#endregion 🔖Procedural2dPlayApp

    //#region 🔖AppFactory
    pub fn create_procedural2d_app() -> App {
        App::from_builder(
            App::builder(PROCEDURAL2D_PLAY_APP_ID, "Procedural 2D").document(["semio", "procedural", "2d"])
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
    //#endregion 🔖AppFactory

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::PluginApp;

        #[test]
        fn renders_main_graph_scene() {
            let app = Procedural2dPlayApp;
            let document = app.initial_document_json();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_MAIN, &document, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("node-graph"));
        }

        #[test]
        fn main_graph_scene_exports_flow_backed_node_graph_fields() {
            let app = Procedural2dPlayApp;
            let document = app.initial_document_json();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_MAIN, &document, &ViewState::default());
            let value: Value = serde_json::from_str(&serde_json::to_string(&node).unwrap()).expect("ui node json");
            let graph = value.get("nodeGraph").expect("nodeGraph");
            assert!(graph.get("fixtureJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow.fixture")));
            assert!(graph.get("operatorsJson").and_then(|v| v.as_str()).is_some());
            assert!(graph.get("capabilitiesJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow")));
        }

        #[test]
        fn renders_preview_canvas_scene() {
            let app = Procedural2dPlayApp;
            let document = app.initial_document_json();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_PREVIEW, &document, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("canvas-2d"));
        }

        #[test]
        fn document_lists_widgets() {
            let app = Procedural2dPlayApp;
            let document = app.initial_document_json();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("procedural2d-play-document.widget.rect"));
        }

        #[test]
        fn catalogue_lists_show_modes() {
            let app = Procedural2dPlayApp;
            let document = app.initial_document_json();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("procedural2d-play-catalogue.mode.preview"));
        }

        #[test]
        fn generate_action_sets_eval_outputs() {
            let mut app = Procedural2dPlayApp;
            let document = app.initial_document_json();
            let ops = app.handle_action_patch_ops("generate", None, &document, &ViewState::default());
            assert_eq!(ops.len(), 1);
            let payload: Value = serde_json::from_str(&ops[0]).unwrap();
            let next: Procedural2dPlayView = serde_json::from_value(payload["document"].clone()).unwrap();
            assert_eq!(next.runtime.show_mode, "generate");
        }

        #[test]
        fn set_show_mode_updates_runtime() {
            let mut app = Procedural2dPlayApp;
            let document = app.initial_document_json();
            let ops = app.handle_action_patch_ops("setShowMode", Some(&json!({ "value": "wire" })), &document, &ViewState::default());
            let payload: Value = serde_json::from_str(&ops[0]).unwrap();
            let next: Procedural2dPlayView = serde_json::from_value(payload["document"].clone()).unwrap();
            assert_eq!(next.runtime.show_mode, "wire");
        }

        #[test]
        fn generate_mode_renders_surfaces() {
            let app = Procedural2dPlayApp;
            let document = app.initial_document_json();
            let generations = app.render(PROCEDURAL2D_PLAY_BODY_GENERATIONS, &document, &ViewState::default());
            assert!(serde_json::to_string(&generations).unwrap().contains("addGeneration"));
        }

        #[test]
        fn add_generation_evaluates_preview() {
            let mut app = Procedural2dPlayApp;
            let document = app.initial_document_json();
            let ops = app.handle_action_patch_ops("addGeneration", None, &document, &ViewState::default());
            let updated: Procedural2dPlayView =
                serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
            assert_eq!(updated.generation.generations.len(), 1);
            assert!(updated.generation.preview_text.as_deref().unwrap_or("").len() > 2);
        }

        #[test]
        fn document_from_dwg_returns_valid_default_envelope() {
            let drawing = semio_framework_core::DwgDrawing::default();
            let document = procedural2d_document_from_dwg(&drawing).expect("dwg import document");
            let envelope: Procedural2dPlayView = serde_json::from_value(document).expect("parseable envelope");
            assert_eq!(envelope.fixture.schema, "flow.fixture");
        }

        #[test]
        fn procedural2d_labels_resolve_native_english_by_default() {
            let app = Procedural2dPlayApp;
            let document = app.initial_document_json();
            let node = app.render(PROCEDURAL2D_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("\"Sources\""));
            assert!(json.contains("\"Components\""));
            assert!(json.contains("\"Sinks\""));
            assert!(json.contains("\"Show mode\""));
            assert!(!json.contains("Quellen"));
        }

        #[test]
        fn procedural2d_labels_translate_catalogue_and_inspector_in_german() {
            let app = Procedural2dPlayApp;
            let document = app.initial_document_json();
            let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
            let catalogue = app.render(PROCEDURAL2D_PLAY_BODY_CATALOGUE, &document, &view_state);
            let catalogue_json = serde_json::to_string(&catalogue).unwrap();
            assert!(catalogue_json.contains("Quellen"));
            assert!(catalogue_json.contains("Komponenten"));
            assert!(catalogue_json.contains("Senken"));
            assert!(catalogue_json.contains("Anzeigemodus"));
            assert!(!catalogue_json.contains("\"Sources\""));
            let inspector = app.render(PROCEDURAL2D_PLAY_BODY_INSPECTION, &document, &view_state);
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
        FlowFixture, FlowHost, Widget,
    };
    use flow_module_brep::tessellate_geometry_json;
    use semio_framework_plugin::{PanelGroup,
        apply_world3d_sun_action, build_node_graph_scene, build_world_3d_scene, create_default_layout, create_named_layout,
        handle_generation_action, merge_world_selection_ids,
        mesh_from_kind, render_generation_form_body, render_generation_preview_text, render_generations_tree,
        selected_generation, tool_button, tool_collection, tool_toggle, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_readonly_field,
        ui_stack_vertical, ui_text, App, world3d_scene, world3d_selection_json, world3d_sun_measures,
        ActionDescriptor, GenerationPlayState, NodeGraphScene, PluginApp, PluginBundle, ToolCategory, ToolNode, UiControlNode,
        UiFieldNode, UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowMeasure, WorldSunConfig,
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
    use std::sync::LazyLock;

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
    //#endregion 🔖Constants

    //#region 🔖EvalCache
    /// 🧠 Process-wide [`flow_core::neural::NeuralCache`] shared across `FlowHost` reconstructions.
    ///
    /// `Procedural3dEnvelope` is a stateless serde value rebuilt from `document_json` on every
    /// plugin dispatch, so a fresh `FlowHost::from_fixture` would otherwise discard per-node
    /// memoization (and the geometry handle stability that lets `flow_module_brep`'s mesh cache
    /// hit) on every single edit. Mirrors `flow_module_brep`'s single-instance `KERNEL`/`MESH_CACHE`
    /// `OnceLock` pattern — one shared cache per WASM instance, which matches one editor session.
    static PROCEDURAL_NEURAL_CACHE: std::sync::OnceLock<std::sync::Arc<flow_core::neural::NeuralCache>> = std::sync::OnceLock::new();

    fn procedural_neural_cache() -> std::sync::Arc<flow_core::neural::NeuralCache> {
        PROCEDURAL_NEURAL_CACHE.get_or_init(|| std::sync::Arc::new(flow_core::neural::NeuralCache::new())).clone()
    }
    //#endregion 🔖EvalCache

    //#region 🔖Terminology
    /// 🗣️ Complete UI label set for the 3D flow app; one field per label makes every locale combination compile-checked.
    struct Procedural3dLabels {
        widgets: &'static str,
        schema_prefix: &'static str,
        widgets_prefix: &'static str,
        no_selection: &'static str,
        id_field: &'static str,
        value_field: &'static str,
        range_field: &'static str,
        widget_group: &'static str,
        generate_hint: &'static str,
        preview_hint: &'static str,
        catalog_neuron: &'static str,
        catalog_slider: &'static str,
        catalog_note: &'static str,
        catalog_preview: &'static str,
        window_flow: &'static str,
        window_preview: &'static str,
        window_generations: &'static str,
        window_generate_form: &'static str,
        window_generate_preview: &'static str,
    }

    const PROCEDURAL3D_LABELS_NATIVE_EN: Procedural3dLabels = Procedural3dLabels {
        widgets: "Widgets",
        schema_prefix: "Schema:",
        widgets_prefix: "Widgets:",
        no_selection: "No selection",
        id_field: "Id",
        value_field: "Value",
        range_field: "Range",
        widget_group: "Widget",
        generate_hint: "Add a generation to edit input values.",
        preview_hint: "(evaluate a generation to preview output)",
        catalog_neuron: "Neuron",
        catalog_slider: "Slider",
        catalog_note: "Note",
        catalog_preview: "Preview",
        window_flow: "Flow",
        window_preview: "Preview",
        window_generations: "Generations",
        window_generate_form: "Form",
        window_generate_preview: "Preview",
    };

    const PROCEDURAL3D_LABELS_NATIVE_DE: Procedural3dLabels = Procedural3dLabels {
        widgets: "Elemente",
        schema_prefix: "Schema:",
        widgets_prefix: "Elemente:",
        no_selection: "Keine Auswahl",
        id_field: "ID",
        value_field: "Wert",
        range_field: "Bereich",
        widget_group: "Element",
        generate_hint: "Erstelle eine Generation, um Eingabewerte zu bearbeiten.",
        preview_hint: "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)",
        catalog_neuron: "Neuron",
        catalog_slider: "Schieberegler",
        catalog_note: "Notiz",
        catalog_preview: "Vorschau",
        window_flow: "Workflow",
        window_preview: "Vorschau",
        window_generations: "Generationen",
        window_generate_form: "Formular",
        window_generate_preview: "Vorschau",
    };

    /// 🗣️ Resolves the active label set from the shell-provided locale; falls back to native English.
    fn procedural3d_labels(view_state: &ViewState) -> &'static Procedural3dLabels {
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        if is_de { &PROCEDURAL3D_LABELS_NATIVE_DE } else { &PROCEDURAL3D_LABELS_NATIVE_EN }
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

    //#region 🔖Document
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Procedural3dRuntime {
        #[serde(default)]
        selected_node_ids: Vec<String>,
        #[serde(default)]
        lod_mode: String,
        #[serde(default = "default_show_mode")]
        show_mode: String,
        #[serde(default = "default_selection_method")]
        selection_method: String,
        #[serde(default)]
        hovered_node_id: Option<String>,
        #[serde(default)]
        preview_camera: Procedural3dPreviewCamera,
        /// ⏮️ Flow-graph snapshots for undo, pushed before structural edits (add/remove/connect/gumball).
        #[serde(default)]
        undo_fixtures: Vec<FlowFixture>,
        /// ⏭️ Flow-graph snapshots for redo, cleared whenever a new edit is snapshotted.
        #[serde(default)]
        redo_fixtures: Vec<FlowFixture>,
        #[serde(default)]
        preview_cache: Option<Procedural3dPreviewCache>,
        #[serde(default)]
        generation_preview_cache: Option<Procedural3dPreviewCache>,
        #[serde(default)]
        sun: WorldSunConfig,
    }

    impl Default for Procedural3dRuntime {
        fn default() -> Self {
            Self {
                selected_node_ids: Vec::new(),
                lod_mode: String::new(),
                show_mode: default_show_mode(),
                selection_method: default_selection_method(),
                hovered_node_id: None,
                preview_camera: Procedural3dPreviewCamera::default(),
                undo_fixtures: Vec::new(),
                redo_fixtures: Vec::new(),
                preview_cache: None,
                generation_preview_cache: None,
                sun: WorldSunConfig::default(),
            }
        }
    }

    fn snapshot_procedural3d(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture) {
        runtime.undo_fixtures.push(fixture.clone());
        runtime.redo_fixtures.clear();
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Procedural3dEnvelope {
        fixture: FlowFixture,
        #[serde(default)]
        runtime: Procedural3dRuntime,
        #[serde(default)]
        generation: GenerationPlayState,
    }

    fn default_envelope() -> Procedural3dEnvelope {
        envelope_from_fixture_json(HEX_COLUMN_EXAMPLE_JSON).unwrap_or_else(|| Procedural3dEnvelope {
            fixture: FlowFixture::default(),
            runtime: Procedural3dRuntime::default(),
            generation: GenerationPlayState::default(),
        })
    }

    fn envelope_from_fixture_json(json_text: &str) -> Option<Procedural3dEnvelope> {
        serde_json::from_str::<FlowFixture>(json_text).ok().map(|fixture| {
            let mut envelope = Procedural3dEnvelope {
                fixture,
                runtime: Procedural3dRuntime::default(),
                generation: GenerationPlayState::default(),
            };
            refresh_preview_cache(&mut envelope.runtime, &envelope.fixture);
            envelope
        })
    }

    fn parse_envelope(document_json: &str) -> Procedural3dEnvelope {
        serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
    }

    fn set_document_op(envelope: &Procedural3dEnvelope) -> String {
        json!({ "op": "setDocument", "document": envelope }).to_string()
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

    fn generation_fixture_for_envelope(envelope: &Procedural3dEnvelope) -> FlowFixture {
        if let Some(generation) = selected_generation(&envelope.generation) {
            let patched = apply_generation_values_to_fixture(
                &serde_json::to_string(&envelope.fixture).unwrap_or_default(),
                &generation.values,
            );
            FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| envelope.fixture.clone())
        } else {
            envelope.fixture.clone()
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

    fn preview_payload_cached(runtime: &Procedural3dRuntime, fixture: &FlowFixture) -> (String, String) {
        let signature = fixture_signature(fixture);
        if let Some(cache) = &runtime.preview_cache {
            if cache.signature == signature {
                return (cache.meshes_json.clone(), cache.instances_json.clone());
            }
        }
        evaluated_preview_payload(fixture, runtime)
    }

    fn finalize_document_op(envelope: &mut Procedural3dEnvelope) -> String {
        refresh_preview_cache(&mut envelope.runtime, &envelope.fixture);
        if selected_generation(&envelope.generation).is_none() {
            // 🪞 No active generation: `generation_fixture_for_envelope` would just return a clone
            // of `envelope.fixture`, so the generation preview is identical to the base preview —
            // reuse the result just computed above instead of evaluating the same fixture twice.
            let signature = generation_preview_signature(&envelope.fixture, &envelope.generation);
            let already_cached = envelope.runtime.generation_preview_cache.as_ref().is_some_and(|entry| entry.signature == signature);
            if !already_cached {
                if let Some(base) = envelope.runtime.preview_cache.clone() {
                    envelope.runtime.generation_preview_cache = Some(Procedural3dPreviewCache {
                        signature,
                        meshes_json: base.meshes_json,
                        instances_json: base.instances_json,
                    });
                }
            }
        } else {
            let generation_fixture = generation_fixture_for_envelope(envelope);
            refresh_generation_preview_cache(&mut envelope.runtime, &generation_fixture, &envelope.generation);
        }
        set_document_op(envelope)
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
    /// 🧭 Maps a gumball drag op to the flow-graph transform neuron kind that persists it.
    fn gumball_xform_kind(op: &str) -> &'static str {
        match op {
            "rotate" => "brep.xform.rotate",
            "scale" => "brep.xform.scale",
            _ => "brep.xform.translate",
        }
    }

    /// 🪪 Deterministic id for the transform neuron generated by dragging `source_id`'s gumball for `op`.
    fn gumball_widget_id(source_id: &str, op: &str) -> String {
        format!("{source_id}__gumball_{op}")
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

    /// 🔀 Finds (or splices in) the transform neuron that persists `selected_id`'s gumball drag for `op` into the flow graph, rewiring downstream consumers so the transformed geometry is what actually evaluates and exports.
    fn ensure_gumball_node(host: &mut FlowHost, selected_id: &str, op: &str) -> Result<String, String> {
        let own_suffix = format!("__gumball_{op}");
        if selected_id.ends_with(&own_suffix) && host.fixture.widgets.iter().any(|widget| widget_id(widget) == selected_id) {
            return Ok(selected_id.to_string());
        }
        let transform_id = gumball_widget_id(selected_id, op);
        if host.fixture.widgets.iter().any(|widget| widget_id(widget) == transform_id) {
            return Ok(transform_id);
        }
        let (source_x, source_y) = widget_layout_position(&host.fixture, selected_id);
        let descriptor = json!({ "kind": "neuron", "id": transform_id, "neuronKind": gumball_xform_kind(op) }).to_string();
        host.add_widget(&descriptor, source_x + 220.0, source_y)?;
        let outgoing_port = host.fixture.synapses.iter().find(|synapse| synapse.from == selected_id).map(|synapse| synapse.from_port.clone());
        if let Some(port) = outgoing_port {
            host.insert_between(selected_id, &port, &transform_id, "geometry", "geometry")?;
        } else {
            host.connect(selected_id, &transform_id)?;
        }
        if let Some(Widget::Neuron { preview, .. }) = host.fixture.widgets.iter_mut().find(|widget| widget_id(widget) == selected_id) {
            *preview = false;
        }
        Ok(transform_id)
    }
    //#endregion 🔖GumballTransforms

    fn host_from_envelope(envelope: &Procedural3dEnvelope) -> FlowHost {
        let mut host = FlowHost::from_fixture_with_cache(envelope.fixture.clone(), procedural_neural_cache());
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
            | Widget::InputStepper { id, .. }
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
            Widget::Neuron { neuronKind, preview, .. } if *preview => Some(neuron_mesh_kind(neuronKind)),
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

    fn evaluate_generation_preview(envelope: &Procedural3dEnvelope, values: &serde_json::Map<String, Value>) -> String {
        let fixture_json = serde_json::to_string(&envelope.fixture).unwrap_or_default();
        let patched = apply_generation_values_to_fixture(&fixture_json, values);
        let fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| envelope.fixture.clone());
        let mut host = FlowHost::from_fixture_with_cache(fixture.clone(), procedural_neural_cache());
        host.evaluate().unwrap_or_default()
    }

    fn refresh_generation_preview(envelope: &mut Procedural3dEnvelope) {
        let Some(generation) = selected_generation(&envelope.generation) else {
            envelope.generation.preview_text = None;
            return;
        };
        let preview = evaluate_generation_preview(envelope, &generation.values);
        envelope.generation.preview_text = Some(preview);
    }

    fn generation_preview_payload(envelope: &Procedural3dEnvelope) -> (String, String) {
        let fixture = generation_fixture_for_envelope(envelope);
        let signature = generation_preview_signature(&fixture, &envelope.generation);
        if let Some(cache) = &envelope.runtime.generation_preview_cache {
            if cache.signature == signature {
                return (cache.meshes_json.clone(), cache.instances_json.clone());
            }
        }
        evaluated_preview_payload(&fixture, &envelope.runtime)
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

    fn preview_selection_json(runtime: &Procedural3dRuntime) -> String {
        world3d_selection_json(
            &runtime.selection_method,
            &runtime.selected_node_ids,
            runtime.hovered_node_id.as_deref(),
        )
    }

    fn export_mesh_from_envelope(envelope: &Procedural3dEnvelope) -> semio_framework_plugin::MeshData {
        let (meshes_json, _) = preview_payload_cached(&envelope.runtime, &envelope.fixture);
        if let Ok(meshes) = serde_json::from_str::<Vec<Value>>(&meshes_json) {
            if let Some(first) = meshes.first() {
                if let Ok(data) = serde_json::from_value(first.get("data").cloned().unwrap_or(Value::Null)) {
                    return data;
                }
            }
        }
        let kind = envelope
            .fixture
            .widgets
            .iter()
            .find_map(|widget| widget_preview_mesh_kind(widget))
            .unwrap_or(PROCEDURAL_FALLBACK_MESH_KIND);
        mesh_from_kind(kind)
    }
    //#endregion 🔖Document

    //#region 🔖Panels
    fn tree_item_with_action(
        id: impl Into<String>,
        label: impl Into<String>,
        icon_id: Option<&str>,
        action: ActionDescriptor,
    ) -> UiTreeItemNode {
        UiTreeItemNode {
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

    fn build_document_tree(fixture: &FlowFixture, selected_node_ids: &[String], labels: &Procedural3dLabels) -> UiNode {
        let items: Vec<UiTreeItemNode> = fixture
            .widgets
            .iter()
            .map(|widget| {
                let id = widget_id(widget).to_string();
                tree_item_with_action(
                    format!("procedural-widget:{id}"),
                    id.clone(),
                    Some("cpu"),
                    procedural_action("setSelection", Some(json!({ "ids": [id] }))),
                )
            })
            .collect();
        UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode {
                id: "procedural-play-document.widgets".into(),
                label: Some(labels.widgets.into()),
                default_open: Some(true),
                items,
            }],
            selected_ids: Some(selected_node_ids.iter().map(|id| format!("procedural-widget:{id}")).collect()),
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        })
    }

    fn build_catalogue_tree(labels: &Procedural3dLabels) -> UiNode {
        let items: Vec<UiTreeItemNode> = WIDGET_CATALOG
            .iter()
            .map(|(kind, icon)| {
                tree_item_with_action(
                    format!("procedural-play-catalogue.{kind}"),
                    procedural3d_catalog_label(*kind, labels),
                    Some(icon),
                    procedural_action("addWidget", Some(json!({ "kind": kind }))),
                )
            })
            .collect();
        UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode {
                id: "procedural-play-catalogue.widgets".into(),
                label: Some(labels.widgets.into()),
                default_open: Some(true),
                items,
            }],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        })
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
                id: "procedural-play-inspector.value".into(),
                label: labels.value_field.into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
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
        ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
            id: "procedural-play-inspector.widget".into(),
            label: labels.widget_group.into(),
            default_open: None,
            fields,
        }])
    }
    //#endregion 🔖Panels

    //#region 🔖GenerateRender
    fn render_generate_generations(envelope: &Procedural3dEnvelope) -> UiNode {
        render_generations_tree(
            PROCEDURAL_3D_PLAY_APP_ID,
            "procedural3d-play-generate",
            &envelope.generation.generations,
            envelope.generation.selected_generation_id.as_deref(),
        )
    }

    fn render_generate_form(envelope: &Procedural3dEnvelope, labels: &Procedural3dLabels) -> UiNode {
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

    fn render_generate_preview(envelope: &Procedural3dEnvelope, labels: &Procedural3dLabels) -> UiNode {
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
                preview_selection_json(&envelope.runtime),
                &envelope.runtime.sun,
            ),
        )
    }
    //#endregion 🔖GenerateRender

    //#region 🔖Procedural3dPlayApp
    #[derive(Default)]
    pub struct Procedural3dPlayApp;

    impl PluginApp for Procedural3dPlayApp {
        fn app_id(&self) -> &str {
            PROCEDURAL_3D_PLAY_APP_ID
        }

        fn initial_document_json(&self) -> String {
            serde_json::to_string(&default_envelope()).expect("procedural3d envelope json")
        }

        fn handle_action_patch_ops(
            &mut self,
            action: &str,
            args: Option<&Value>,
            document_json: &str,
            _view_state: &ViewState,
        ) -> Vec<String> {
            let mut envelope = parse_envelope(document_json);
            let mut host = host_from_envelope(&envelope);
            match action {
                "setDocument" => {
                    if let Some(document) = args.and_then(|value| value.get("document")) {
                        if let Ok(mut parsed) = serde_json::from_value::<Procedural3dEnvelope>(document.clone()) {
                            return vec![finalize_document_op(&mut parsed)];
                        }
                    }
                }
                "setActiveExample" => {
                    let example_id = args
                        .and_then(|value| value.get("exampleId"))
                        .and_then(|value| value.as_str())
                        .unwrap_or("");
                    envelope = if example_id.is_empty() || example_id == "empty" {
                        Procedural3dEnvelope {
                            fixture: FlowFixture::default(),
                            runtime: Procedural3dRuntime::default(),
                            generation: GenerationPlayState::default(),
                        }
                    } else if example_id == PROCEDURAL_EXAMPLE_HEX_COLUMN || example_id == "demo" {
                        envelope_from_fixture_json(HEX_COLUMN_EXAMPLE_JSON).unwrap_or_else(default_envelope)
                    } else if example_id == PROCEDURAL_EXAMPLE_RECT_EXTRUDE {
                        envelope_from_fixture_json(RECT_EXTRUDE_EXAMPLE_JSON).unwrap_or_else(default_envelope)
                    } else if example_id == PROCEDURAL_EXAMPLE_SPHERE_TORUS {
                        envelope_from_fixture_json(SPHERE_TORUS_EXAMPLE_JSON).unwrap_or_else(default_envelope)
                    } else {
                        envelope
                    };
                    return vec![finalize_document_op(&mut envelope)];
                }
                "setSelection" | "selectNode" | "nodeGraphSelect" => {
                    envelope.runtime.selected_node_ids = node_graph_selection_ids(args);
                    return vec![finalize_document_op(&mut envelope)];
                }
                "nodeGraphHover" => return Vec::new(),
                "nodeGraphViewport" => {
                    if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                        if let Ok(camera) = serde_json::from_str(viewport_json) {
                            envelope.fixture.camera = camera;
                            return vec![finalize_document_op(&mut envelope)];
                        }
                    }
                }
                "nodeGraphEdit" => {
                    let ops = args
                        .and_then(|value| value.get("ops"))
                        .and_then(|value| value.as_array())
                        .cloned()
                        .unwrap_or_default();
                    let mut changed = false;
                    for op in ops {
                        match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
                            "setFixture" => {
                                if let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) {
                                    if let Ok(fixture) = serde_json::from_str::<FlowFixture>(fixture_json) {
                                        envelope.fixture = fixture;
                                        changed = true;
                                    }
                                }
                            }
                            "deleteSelection" => {
                                for node_id in envelope.runtime.selected_node_ids.clone() {
                                    if !changed {
                                        snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                                    }
                                    if host.remove_widget(&node_id).is_ok() {
                                        changed = true;
                                    }
                                }
                                if changed {
                                    envelope.runtime.selected_node_ids.clear();
                                }
                            }
                            "connect" => {
                                let from = op.get("sourceNodeId").and_then(|value| value.as_str());
                                let from_port = op.get("sourcePortId").and_then(|value| value.as_str());
                                let to = op.get("targetNodeId").and_then(|value| value.as_str());
                                let to_port = op.get("targetPortId").and_then(|value| value.as_str());
                                if let (Some(from), Some(from_port), Some(to), Some(to_port)) =
                                    (from, from_port, to, to_port)
                                {
                                    snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                                    if host.connect_ports(from, from_port, to, to_port).is_ok() {
                                        changed = true;
                                    } else {
                                        envelope.runtime.undo_fixtures.pop();
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    if changed {
                        envelope.fixture = host.fixture;
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "deleteSelection" => {
                    for node_id in envelope.runtime.selected_node_ids.clone() {
                        snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                        if host.remove_widget(&node_id).is_ok() {
                            envelope.fixture = host.fixture;
                            envelope.runtime.selected_node_ids.retain(|id| id != &node_id);
                            return vec![finalize_document_op(&mut envelope)];
                        }
                        envelope.runtime.undo_fixtures.pop();
                    }
                }
                "removeWidget" => {
                    let target_id = args
                        .and_then(|value| value.get("widgetId"))
                        .or_else(|| args.and_then(|value| value.get("id")))
                        .and_then(|value| value.as_str());
                    if let Some(target_id) = target_id {
                        if envelope.fixture.widgets.iter().any(|widget| widget_id(widget) == target_id) {
                            snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                            if host.remove_widget(target_id).is_ok() {
                                envelope.fixture = host.fixture;
                                envelope.runtime.selected_node_ids.retain(|id| id != target_id);
                                return vec![finalize_document_op(&mut envelope)];
                            }
                            envelope.runtime.undo_fixtures.pop();
                        }
                    }
                }
                "undo" => {
                    if let Some(previous) = envelope.runtime.undo_fixtures.pop() {
                        envelope.runtime.redo_fixtures.push(envelope.fixture.clone());
                        envelope.fixture = previous;
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "redo" => {
                    if let Some(next) = envelope.runtime.redo_fixtures.pop() {
                        envelope.runtime.undo_fixtures.push(envelope.fixture.clone());
                        envelope.fixture = next;
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                    apply_world3d_sun_action(&mut envelope.runtime.sun, action, args);
                    return vec![finalize_document_op(&mut envelope)];
                }
                "setLodMode" => {
                    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                        envelope.runtime.lod_mode = mode.into();
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "setShowMode" => {
                    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                        envelope.runtime.show_mode = mode.into();
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "moveMediaNode" => {
                    let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str());
                    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                    if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                        if host.move_widget(node_id, x, y).is_ok() {
                            envelope.fixture = host.fixture;
                            return vec![finalize_document_op(&mut envelope)];
                        }
                    }
                }
                "addWidget" => {
                    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("inputSlider");
                    let descriptor = match kind {
                        "neuron" => json!({ "kind": "neuron", "neuronKind": "math.add" }).to_string(),
                        other => json!({ "kind": other }).to_string(),
                    };
                    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                    snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                    if let Ok(id) = host.add_widget(&descriptor, x, y) {
                        envelope.fixture = host.fixture;
                        envelope.runtime.selected_node_ids = vec![id];
                        return vec![finalize_document_op(&mut envelope)];
                    }
                    envelope.runtime.undo_fixtures.pop();
                }
                "patchFlowWidgets" => {
                    let widget_ids: Vec<String> = args
                        .and_then(|value| value.get("widgetIds"))
                        .and_then(|value| serde_json::from_value(value.clone()).ok())
                        .unwrap_or_default();
                    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                    let raw_value = args.and_then(|value| value.get("value"));
                    snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                    for widget in envelope.fixture.widgets.iter_mut() {
                        if !widget_ids.contains(&widget_id(widget).to_string()) {
                            continue;
                        }
                        if let (Widget::InputSlider { value: ref mut slider_value, .. }, Some(value)) =
                            (widget, raw_value.and_then(|entry| entry.as_f64()))
                        {
                            if field == "value" {
                                *slider_value = value;
                            }
                        }
                    }
                    return vec![finalize_document_op(&mut envelope)];
                }
                "reorganize" => {
                    snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                    if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
                        envelope.fixture = host.fixture;
                        return vec![finalize_document_op(&mut envelope)];
                    }
                    envelope.runtime.undo_fixtures.pop();
                }
                "worldSelect" => {
                    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                    let ids: Vec<String> = args
                        .and_then(|value| value.get("ids"))
                        .and_then(|value| serde_json::from_value(value.clone()).ok())
                        .unwrap_or_default();
                    envelope.runtime.selected_node_ids =
                        merge_world_selection_ids(&envelope.runtime.selected_node_ids, &ids, merge);
                    return vec![finalize_document_op(&mut envelope)];
                }
                "worldHover" => {
                    envelope.runtime.hovered_node_id = args
                        .and_then(|value| value.get("id"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string);
                    return vec![finalize_document_op(&mut envelope)];
                }
                "setSelectionMethod" => {
                    let method = args
                        .and_then(|value| value.get("method"))
                        .and_then(|value| value.as_str())
                        .unwrap_or("rectangle");
                    envelope.runtime.selection_method = method.into();
                    return vec![finalize_document_op(&mut envelope)];
                }
                "setCamera" => {
                    if let Some(camera) = args.and_then(|value| value.get("camera")) {
                        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                            envelope.runtime.preview_camera = parsed;
                            return vec![finalize_document_op(&mut envelope)];
                        }
                    }
                }
                "translateSelection" => {
                    let ids = mesh_selection_ids(args, &envelope.runtime.selected_node_ids);
                    let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let mut new_selection = Vec::new();
                    let mut changed = false;
                    for id in &ids {
                        let is_new = !host.fixture.widgets.iter().any(|widget| widget_id(widget) == gumball_widget_id(id, "translate")) && !id.ends_with("__gumball_translate");
                        if is_new {
                            snapshot_procedural3d(&mut envelope.runtime, &host.fixture);
                        }
                        match ensure_gumball_node(&mut host, id, "translate") {
                            Ok(transform_id) => {
                                let current = gumball_widget_offset(&host, &transform_id);
                                let next = [current[0] + dx, current[1] + dy, current[2] + dz];
                                if host.set_neuron_params(&transform_id, &gumball_translate_params_json(next)).is_ok() {
                                    new_selection.push(transform_id);
                                    changed = true;
                                } else if is_new {
                                    envelope.runtime.undo_fixtures.pop();
                                }
                            }
                            Err(_) if is_new => {
                                envelope.runtime.undo_fixtures.pop();
                            }
                            Err(_) => {}
                        }
                    }
                    if changed {
                        envelope.fixture = host.fixture;
                        envelope.runtime.selected_node_ids = new_selection;
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "rotateSelection" => {
                    let ids = mesh_selection_ids(args, &envelope.runtime.selected_node_ids);
                    let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let mut new_selection = Vec::new();
                    let mut changed = false;
                    for id in &ids {
                        let is_new = !host.fixture.widgets.iter().any(|widget| widget_id(widget) == gumball_widget_id(id, "rotate")) && !id.ends_with("__gumball_rotate");
                        if is_new {
                            snapshot_procedural3d(&mut envelope.runtime, &host.fixture);
                        }
                        match ensure_gumball_node(&mut host, id, "rotate") {
                            Ok(transform_id) => {
                                let current_angle = gumball_widget_number_param(&host, &transform_id, "angle", 0.0);
                                let params = gumball_rotate_params_json([ax, ay, az], current_angle + angle);
                                if host.set_neuron_params(&transform_id, &params).is_ok() {
                                    new_selection.push(transform_id);
                                    changed = true;
                                } else if is_new {
                                    envelope.runtime.undo_fixtures.pop();
                                }
                            }
                            Err(_) if is_new => {
                                envelope.runtime.undo_fixtures.pop();
                            }
                            Err(_) => {}
                        }
                    }
                    if changed {
                        envelope.fixture = host.fixture;
                        envelope.runtime.selected_node_ids = new_selection;
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "scaleSelection" => {
                    let ids = mesh_selection_ids(args, &envelope.runtime.selected_node_ids);
                    let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                    let uniform_factor = (sx + sy + sz) / 3.0;
                    let mut new_selection = Vec::new();
                    let mut changed = false;
                    for id in &ids {
                        let is_new = !host.fixture.widgets.iter().any(|widget| widget_id(widget) == gumball_widget_id(id, "scale")) && !id.ends_with("__gumball_scale");
                        if is_new {
                            snapshot_procedural3d(&mut envelope.runtime, &host.fixture);
                        }
                        match ensure_gumball_node(&mut host, id, "scale") {
                            Ok(transform_id) => {
                                let current_factor = gumball_widget_number_param(&host, &transform_id, "factor", 1.0);
                                let params = gumball_scale_params_json(current_factor * uniform_factor);
                                if host.set_neuron_params(&transform_id, &params).is_ok() {
                                    new_selection.push(transform_id);
                                    changed = true;
                                } else if is_new {
                                    envelope.runtime.undo_fixtures.pop();
                                }
                            }
                            Err(_) if is_new => {
                                envelope.runtime.undo_fixtures.pop();
                            }
                            Err(_) => {}
                        }
                    }
                    if changed {
                        envelope.fixture = host.fixture;
                        envelope.runtime.selected_node_ids = new_selection;
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                "worldPointerDown" | "graphPointerDown" => return Vec::new(),
                "addGeneration" | "removeGeneration" | "selectGeneration" | "renameGeneration" | "updateGenerationValues" => {
                    let spec = flow_fixture_to_form_spec(&envelope.fixture);
                    if handle_generation_action(action, args, &mut envelope.generation, &spec, PROCEDURAL_3D_PLAY_APP_ID)
                    {
                        if matches!(action, "addGeneration" | "selectGeneration" | "updateGenerationValues") {
                            refresh_generation_preview(&mut envelope);
                        }
                        return vec![finalize_document_op(&mut envelope)];
                    }
                }
                _ => {}
            }
            Vec::new()
        }

        fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
            let envelope = parse_envelope(document_json);
            let host = host_from_envelope(&envelope);
            let labels = procedural3d_labels(view_state);
            match body_key {
                PROCEDURAL_3D_PLAY_BODY_MAIN => {
                    let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
                    let viewport_json =
                        serde_json::to_string(&envelope.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
                    let selection_json = if envelope.runtime.selected_node_ids.is_empty() {
                        None
                    } else {
                        serde_json::to_string(&envelope.runtime.selected_node_ids).ok()
                    };
                    let flow_extras = flow_backed_node_graph_extras(&envelope.fixture, &envelope.runtime.lod_mode, 0.0);
                    build_node_graph_scene(
                        PROCEDURAL_3D_PLAY_SURFACE_MAIN,
                        PROCEDURAL_3D_PLAY_APP_ID,
                        NodeGraphScene {
                            editable: Some(true),
                            operators_json: flow_extras.operators_json,
                            capabilities_json: flow_extras.capabilities_json,
                            lod_json: flow_extras.lod_json,
                            fixture_json: flow_extras.fixture_json,
                            selection_json,
                            context_menu_json: Some(
                                r#"[{"id":"delete-selection","label":"Delete selection","action":"nodeGraphEdit","args":{"ops":[{"op":"deleteSelection"}]}}]"#.into(),
                            ),
                            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
                        },
                    )
                }
                PROCEDURAL_3D_PLAY_BODY_PREVIEW => {
                    let (meshes_json, instances_json) = preview_payload_cached(&envelope.runtime, &envelope.fixture);
                    build_world_3d_scene(
                        PROCEDURAL_3D_PLAY_SURFACE_PREVIEW,
                        PROCEDURAL_3D_PLAY_APP_ID,
                        world3d_scene(
                            preview_camera_json(&envelope.runtime),
                            meshes_json,
                            instances_json,
                            preview_selection_json(&envelope.runtime),
                            &envelope.runtime.sun,
                        ),
                    )
                }
                PROCEDURAL_3D_PLAY_BODY_GENERATIONS => render_generate_generations(&envelope),
                PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM => render_generate_form(&envelope, labels),
                PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(&envelope, labels),
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

        fn window_measures(&self, document_json: &str, _view_state: &ViewState) -> std::collections::HashMap<String, Vec<WindowMeasure>> {
            let envelope = parse_envelope(document_json);
            let measures = vec![world3d_sun_measures("procedural3d", &envelope.runtime.sun, procedural3d_action)];
            std::collections::HashMap::from([
                (PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.to_string(), measures.clone()),
                (PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), measures),
            ])
        }

        fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
            let labels = procedural3d_labels(view_state);
            semio_framework_plugin::AppLabelsOverlay {
                app_label: None,
                window_kind_labels: std::collections::HashMap::from([
                    (PROCEDURAL_3D_PLAY_WINDOW_MAIN.to_string(), labels.window_flow.to_string()),
                    (PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.to_string(), labels.window_preview.to_string()),
                    (PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS.to_string(), labels.window_generations.to_string()),
                    (PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM.to_string(), labels.window_generate_form.to_string()),
                    (
                        PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(),
                        labels.window_generate_preview.to_string(),
                    ),
                ]),
                panel_tab_labels: std::collections::HashMap::new(),
                mode_labels: std::collections::HashMap::new(),
            }
        }
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

    fn selection_ids(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("ids"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .or_else(|| {
                args.and_then(|value| value.get("nodeId"))
                    .and_then(|value| value.as_str())
                    .map(|id| vec![id.to_string()])
            })
            .unwrap_or_default()
    }
    //#endregion 🔖Procedural3dPlayApp

    fn procedural3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor {
            controller_id: PROCEDURAL_3D_PLAY_CONTROLLER_ID.into(),
            action: action.into(),
            args,
        }
    }

    fn procedural3d_edit_tools() -> Vec<ToolNode> {
        vec![
            tool_collection(
                "procedural3d-tools-lod",
                "layers",
                "LOD",
                vec![
                    tool_toggle(
                        "procedural3d-tools-lod-solid",
                        "box",
                        "Solid",
                        true,
                        procedural3d_action("setLodMode", Some(json!({ "value": "solid" }))),
                    ),
                    tool_toggle(
                        "procedural3d-tools-lod-wireframe",
                        "git-commit-horizontal",
                        "Wireframe",
                        false,
                        procedural3d_action("setLodMode", Some(json!({ "value": "wireframe" }))),
                    ),
                ],
            )
            .with_category(ToolCategory::Tools),
        ]
    }

    fn procedural3d_generate_tools() -> Vec<ToolNode> {
        vec![tool_button(
            "procedural3d-tools-add-generation",
            "plus",
            "Add Generation",
            procedural3d_action("addGeneration", None),
        )
        .with_category(ToolCategory::Actions)]
    }

    //#region 🔖Manifest
    pub fn create_procedural3d_app() -> App {
        App::from_builder(
            App::builder(PROCEDURAL_3D_PLAY_APP_ID, "Procedural 3D").document(["semio", "procedural", "3d"])
                .icon_id("workflow")
                .mode("edit", "Edit")
                .mode("generate", "Generate")
                .default_mode_id("edit")
                .mode_layout("generate", "procedural3d-generate")
                .mode_tools("edit", procedural3d_edit_tools())
                .mode_tools("generate", procedural3d_generate_tools())
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
                .keybinding("mod+z", "undo")
                .keybinding("mod+shift+z", "redo"),
        )
        .example(PROCEDURAL_EXAMPLE_HEX_COLUMN, "Hexagonal Mushroom Column", HEX_COLUMN_EXAMPLE_JSON)
        .example(PROCEDURAL_EXAMPLE_RECT_EXTRUDE, "Rectangle Extrude Volume", RECT_EXTRUDE_EXAMPLE_JSON)
        .example(PROCEDURAL_EXAMPLE_SPHERE_TORUS, "Sphere Cut With Torus", SPHERE_TORUS_EXAMPLE_JSON)
        .program("procedural3d", "Procedural 3D", "brep")
    }

    fn procedural3d_mesh_from_document(doc: &serde_json::Value) -> Result<semio_framework_plugin::MeshData, String> {
        let envelope: Procedural3dEnvelope = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        Ok(export_mesh_from_envelope(&envelope))
    }

    fn procedural3d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
        serde_json::to_value(default_envelope()).map_err(|err| err.to_string())
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

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use kernel_3d_scene::{
            aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Mesh3d, Vec3,
        };
        use semio_framework_plugin::PluginApp;

        #[test]
        fn renders_node_graph_scene() {
            let app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, &document, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("node-graph"));
        }

        #[test]
        fn main_graph_scene_exports_flow_backed_node_graph_fields() {
            let app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, &document, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            let value: Value = serde_json::from_str(&json).expect("ui node json");
            let graph = value.get("nodeGraph").expect("nodeGraph");
            assert!(graph.get("fixtureJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow.fixture")));
            assert!(graph.get("operatorsJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("math.add") || s.contains("brep.")));
            let capabilities = graph.get("capabilitiesJson").and_then(|v| v.as_str()).unwrap_or_default();
            assert!(capabilities.contains("flow"), "missing flow engine capability: {capabilities}");
        }

        #[test]
        fn set_lod_mode_reads_value_arg() {
            let mut app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let ops = app.handle_action_patch_ops("setLodMode", Some(&json!({ "value": "wireframe" })), &document, &ViewState::default());
            let envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
            assert_eq!(envelope.runtime.lod_mode, "wireframe");
        }

        #[test]
        fn toggle_sun_round_trips_through_runtime_and_defaults_off() {
            let mut app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let envelope = parse_envelope(&document);
            assert!(!envelope.runtime.sun.enabled, "sun must be off by default");
            let measures = app.window_measures(&document, &ViewState::default());
            assert!(measures.contains_key(PROCEDURAL_3D_PLAY_WINDOW_PREVIEW));
            assert!(measures.contains_key(PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW));
            let ops = app.handle_action_patch_ops("toggleSun", None, &document, &ViewState::default());
            let next: Procedural3dEnvelope = apply_ops(&envelope, &ops);
            assert!(next.runtime.sun.enabled);
        }

        #[test]
        fn set_active_example_loads_sphere_fixture() {
            let mut app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let ops = app.handle_action_patch_ops(
                "setActiveExample",
                Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
                &document,
                &ViewState::default(),
            );
            let envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
            assert!(envelope.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { neuronKind, .. } if neuronKind == "brep.prim3d.sphere")));
        }

        #[test]
        fn sphere_cut_example_preview_renders_meshes() {
            let app = Procedural3dPlayApp;
            let mut envelope = envelope_from_fixture_json(SPHERE_TORUS_EXAMPLE_JSON).expect("sphere fixture");
            refresh_preview_cache(&mut envelope.runtime, &envelope.fixture);
            let document = serde_json::to_string(&envelope).expect("envelope json");
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, &document, &ViewState::default());
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
        fn viewport_action_preserves_preview_cache() {
            let mut app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let load_ops = app.handle_action_patch_ops(
                "setActiveExample",
                Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
                &document,
                &ViewState::default(),
            );
            let mut envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &load_ops);
            let cached = envelope.runtime.preview_cache.clone().expect("preview cache");
            let document = serde_json::to_string(&envelope).unwrap();
            let viewport_ops = app.handle_action_patch_ops(
                "nodeGraphViewport",
                Some(&json!({ "viewportJson": r#"{"x":12,"y":24,"zoom":2}"# })),
                &document,
                &ViewState::default(),
            );
            let next: Procedural3dEnvelope = apply_ops(&envelope, &viewport_ops);
            let next_cache = next.runtime.preview_cache.expect("preview cache after viewport");
            assert_eq!(next_cache.signature, cached.signature);
            assert_eq!(next_cache.meshes_json, cached.meshes_json);
            assert_eq!(next_cache.instances_json, cached.instances_json);
        }

        #[test]
        fn patch_flow_widgets_refreshes_preview_cache() {
            let mut app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let before: Procedural3dEnvelope = parse_envelope(&document);
            let cached = before.runtime.preview_cache.clone().expect("preview cache");
            let ops = app.handle_action_patch_ops(
                "patchFlowWidgets",
                Some(&json!({ "widgetIds": ["height"], "field": "value", "value": 9.5 })),
                &document,
                &ViewState::default(),
            );
            let after: Procedural3dEnvelope = apply_ops(&before, &ops);
            let next_cache = after.runtime.preview_cache.expect("preview cache after patch");
            assert_ne!(next_cache.signature, cached.signature);
        }

        #[test]
        fn preview_payload_has_meshes_and_instances() {
            let envelope = default_envelope();
            let (meshes_json, instances_json) = evaluated_preview_payload(&envelope.fixture, &envelope.runtime);
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
                    envelope.runtime.preview_camera.position[0] as f32,
                    envelope.runtime.preview_camera.position[1] as f32,
                    envelope.runtime.preview_camera.position[2] as f32,
                ]),
                target: Vec3::from_array([
                    envelope.runtime.preview_camera.target[0] as f32,
                    envelope.runtime.preview_camera.target[1] as f32,
                    envelope.runtime.preview_camera.target[2] as f32,
                ]),
                up: Vec3::new(0.0, 0.0, 1.0),
                fov_y: envelope.runtime.preview_camera.fov as f32 * std::f32::consts::PI / 180.0,
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
            let app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, &document, &ViewState::default());
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
            let mut app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let before = parse_envelope(&document).fixture.widgets.len();
            let ops = app.handle_action_patch_ops("addWidget", Some(&json!({ "kind": "inputNote" })), &document, &ViewState::default());
            let envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
            assert!(envelope.fixture.widgets.len() > before);
        }

        #[test]
        fn generate_mode_renders_surfaces() {
            let app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let generations = app.render(PROCEDURAL_3D_PLAY_BODY_GENERATIONS, &document, &ViewState::default());
            assert!(serde_json::to_string(&generations).unwrap().contains("addGeneration"));
        }

        #[test]
        fn add_generation_evaluates_preview() {
            let mut app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let ops = app.handle_action_patch_ops("addGeneration", None, &document, &ViewState::default());
            let envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
            assert_eq!(envelope.generation.generations.len(), 1);
            assert!(envelope.generation.preview_text.as_deref().unwrap_or("").len() > 2);
        }

        #[test]
        fn translate_selection_persists_transform_into_flow_graph() {
            let mut app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let before = parse_envelope(&document);
            assert!(before.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"));
            let ops = app.handle_action_patch_ops(
                "translateSelection",
                Some(&json!({ "ids": ["extrude"], "dx": 1.0, "dy": 2.0, "dz": 3.0 })),
                &document,
                &ViewState::default(),
            );
            let envelope = apply_ops(&before, &ops);
            let transform_id = "extrude__gumball_translate";
            let transform = envelope.fixture.widgets.iter().find(|widget| widget_id(widget) == transform_id).expect("transform neuron created");
            assert!(matches!(transform, Widget::Neuron { neuronKind, .. } if neuronKind == "brep.xform.translate"));
            let offset = gumball_widget_offset(&host_from_envelope(&envelope), transform_id);
            assert_eq!(offset, [1.0, 2.0, 3.0]);
            let source = envelope.fixture.widgets.iter().find(|widget| widget_id(widget) == "extrude").expect("source widget");
            assert!(matches!(source, Widget::Neuron { preview, .. } if !*preview), "source preview should turn off once gumball-transformed");
            assert!(envelope.fixture.synapses.iter().any(|synapse| synapse.from == transform_id && synapse.to == "column-preview"), "downstream rewired through transform node");
            assert!(!envelope.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"), "old direct edge removed");
            assert_eq!(envelope.runtime.selected_node_ids, vec![transform_id.to_string()]);
            assert_eq!(envelope.runtime.undo_fixtures.len(), 1);

            // Re-grabbing the same transform accumulates the delta instead of creating a second node.
            let document2 = serde_json::to_string(&envelope).unwrap();
            let ops2 = app.handle_action_patch_ops(
                "translateSelection",
                Some(&json!({ "ids": [transform_id], "dx": 1.0, "dy": 0.0, "dz": 0.0 })),
                &document2,
                &ViewState::default(),
            );
            let envelope2 = apply_ops(&envelope, &ops2);
            assert_eq!(envelope2.fixture.widgets.iter().filter(|widget| widget_id(widget) == transform_id).count(), 1);
            assert_eq!(gumball_widget_offset(&host_from_envelope(&envelope2), transform_id), [2.0, 2.0, 3.0]);
            assert_eq!(envelope2.runtime.undo_fixtures.len(), 1, "re-grab updates in place without an extra undo snapshot");
        }

        #[test]
        fn rotate_and_scale_selection_persist_into_flow_graph() {
            let mut app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let envelope = parse_envelope(&document);
            let rotate_ops = app.handle_action_patch_ops(
                "rotateSelection",
                Some(&json!({ "ids": ["extrude"], "angle": std::f64::consts::FRAC_PI_2 })),
                &document,
                &ViewState::default(),
            );
            let rotated = apply_ops(&envelope, &rotate_ops);
            let rotate_id = "extrude__gumball_rotate";
            assert!(rotated.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuronKind, .. } if id == rotate_id && neuronKind == "brep.xform.rotate")));
            assert_eq!(gumball_widget_number_param(&host_from_envelope(&rotated), rotate_id, "angle", 0.0), std::f64::consts::FRAC_PI_2);

            let scale_ops = app.handle_action_patch_ops(
                "scaleSelection",
                Some(&json!({ "ids": ["extrude"], "sx": 2.0, "sy": 2.0, "sz": 2.0 })),
                &document,
                &ViewState::default(),
            );
            let scaled = apply_ops(&envelope, &scale_ops);
            let scale_id = "extrude__gumball_scale";
            assert!(scaled.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuronKind, .. } if id == scale_id && neuronKind == "brep.xform.scale")));
            assert_eq!(gumball_widget_number_param(&host_from_envelope(&scaled), scale_id, "factor", 1.0), 2.0);
        }

        #[test]
        fn undo_redo_round_trips_flow_graph_edits() {
            let mut app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let before = parse_envelope(&document);
            let add_ops = app.handle_action_patch_ops("addWidget", Some(&json!({ "kind": "inputNote" })), &document, &ViewState::default());
            let after_add = apply_ops(&before, &add_ops);
            assert!(after_add.fixture.widgets.len() > before.fixture.widgets.len());
            assert_eq!(after_add.runtime.undo_fixtures.len(), 1);

            let document_after_add = serde_json::to_string(&after_add).unwrap();
            let undo_ops = app.handle_action_patch_ops("undo", None, &document_after_add, &ViewState::default());
            let after_undo = apply_ops(&after_add, &undo_ops);
            assert_eq!(after_undo.fixture.widgets.len(), before.fixture.widgets.len());
            assert_eq!(after_undo.runtime.undo_fixtures.len(), 0);
            assert_eq!(after_undo.runtime.redo_fixtures.len(), 1);

            let document_after_undo = serde_json::to_string(&after_undo).unwrap();
            let redo_ops = app.handle_action_patch_ops("redo", None, &document_after_undo, &ViewState::default());
            let after_redo = apply_ops(&after_undo, &redo_ops);
            assert_eq!(after_redo.fixture.widgets.len(), after_add.fixture.widgets.len());
            assert_eq!(after_redo.runtime.redo_fixtures.len(), 0);
        }

        #[test]
        fn remove_widget_action_deletes_by_id_and_supports_undo() {
            let mut app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let before = parse_envelope(&document);
            assert!(before.fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"));
            let ops = app.handle_action_patch_ops("removeWidget", Some(&json!({ "widgetId": "sides" })), &document, &ViewState::default());
            let after = apply_ops(&before, &ops);
            assert!(!after.fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"));
            assert_eq!(after.runtime.undo_fixtures.len(), 1);

            let document_after = serde_json::to_string(&after).unwrap();
            let undo_ops = app.handle_action_patch_ops("undo", None, &document_after, &ViewState::default());
            let restored = apply_ops(&after, &undo_ops);
            assert!(restored.fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"));
        }

        #[test]
        fn document_from_mesh_returns_valid_default_envelope() {
            let mesh = semio_framework_plugin::MeshData::default();
            let document = procedural3d_document_from_mesh(&mesh).expect("dwg mesh import document");
            let envelope: Procedural3dEnvelope = serde_json::from_value(document).expect("parseable envelope");
            assert_eq!(envelope.fixture.schema, "flow.fixture");
        }

        #[test]
        fn procedural3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs() {
            use semio_framework_plugin::{
                GlbExporter, GlbImporter, MeshExporter, MeshImporter, ObjExporter, ObjImporter, StlExporter, StlImporter,
            };
            let app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let document_json: Value = serde_json::from_str(&document).expect("initial document json");
            let mesh = procedural3d_mesh_from_document(&document_json).expect("mesh from document");
            assert!(!mesh.positions.is_empty());

            let obj_bytes = ObjExporter.export(&mesh).expect("obj export");
            let obj_mesh = ObjImporter.import(&obj_bytes).expect("obj import");
            let obj_document = procedural3d_document_from_mesh(&obj_mesh).expect("obj document from mesh");
            let _: Procedural3dEnvelope = serde_json::from_value(obj_document).expect("parseable obj envelope");

            let glb_bytes = GlbExporter.export(&mesh).expect("glb export");
            let glb_mesh = GlbImporter.import(&glb_bytes).expect("glb import");
            let glb_document = procedural3d_document_from_mesh(&glb_mesh).expect("glb document from mesh");
            let _: Procedural3dEnvelope = serde_json::from_value(glb_document).expect("parseable glb envelope");

            let stl_bytes = StlExporter.export(&mesh).expect("stl export");
            let stl_mesh = StlImporter.import(&stl_bytes).expect("stl import");
            let stl_document = procedural3d_document_from_mesh(&stl_mesh).expect("stl document from mesh");
            let _: Procedural3dEnvelope = serde_json::from_value(stl_document).expect("parseable stl envelope");
        }

        fn apply_ops(envelope: &Procedural3dEnvelope, ops: &[String]) -> Procedural3dEnvelope {
            let mut next = envelope.clone();
            for op_json in ops {
                if let Ok(op) = serde_json::from_str::<Value>(op_json) {
                    if let Some(document) = op.get("document") {
                        if let Ok(parsed) = serde_json::from_value(document.clone()) {
                            next = parsed;
                        }
                    }
                }
            }
            next
        }

        #[test]
        fn procedural3d_labels_resolve_native_english_by_default() {
            let app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("\"Widgets\""));
            assert!(json.contains("\"Slider\""));
            assert!(!json.contains("Elemente"));
        }

        #[test]
        fn procedural3d_labels_translate_catalogue_and_inspector_in_german() {
            let app = Procedural3dPlayApp;
            let document = app.initial_document_json();
            let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
            let catalogue = app.render(PROCEDURAL_3D_PLAY_BODY_CATALOGUE, &document, &view_state);
            let catalogue_json = serde_json::to_string(&catalogue).unwrap();
            assert!(catalogue_json.contains("\"Elemente\""));
            assert!(catalogue_json.contains("Schieberegler"));
            assert!(!catalogue_json.contains("\"Widgets\""));
            let inspector = app.render(PROCEDURAL_3D_PLAY_BODY_INSPECTION, &document, &view_state);
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
