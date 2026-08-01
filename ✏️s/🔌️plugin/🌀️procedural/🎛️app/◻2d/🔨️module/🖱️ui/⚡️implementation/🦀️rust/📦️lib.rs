//! 🎲️ Procedural 2D app — DocumentApp impl, render, manifest (constitutional: ui).

use flow_core::forms_bridge::flow_fixture_to_form_spec;
use flow_core::{flow_backed_node_graph_extras, FlowFixture, FlowHost};
use playbook::{
    apply_generation_operation, generation_operations, render_generation_form_body, render_generation_preview_text, render_generations_tree, select_generation, selected_generation, GenerationPlayState,
};
use procedural_2d::{widget_id, Procedural2dDocument, PROCEDURAL_2D_SCHEMA};
use procedural_2d_engine::{
    collect_drawing_handles_from_eval, default_projection, fixture_to_workflow, generation_preview_layers, host_from_fixture, host_from_fixture_with_driver, refresh_generation_preview,
    scene_layers_from_drawing_handle, Procedural2dPlayRuntime,
};
use procedural_2d_op::{procedural2d_fixture_operations, Procedural2dOperation};
use semio_framework_plugin::{
    build_canvas_2d_scene, build_node_graph_scene, create_default_layout, create_named_layout, tree_item_with_action, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text,
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionEmit, App, AppLabelsOverlayExt, ArtifactKindSpec, Canvas2dScene, DocumentApp, DocumentView, MediaClass, MediaForm, MediaType, NodeGraphScene,
    OsMediaCapability, PanelGroup, PanelTreeBuilder, SurfaceKind, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::{json, Value};
use std::cell::RefCell;

//#region 🔖️Constants
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
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🧾️ Transient render/action bundle — the persisted projection (fixture + generations) with the
/// ephemeral runtime's selection and derived preview overlaid, so the pure panel/render helpers
/// keep reading a single value. Assembled per call; never serialized as the document.
struct Procedural2dPlayView {
    fixture: FlowFixture,
    runtime: Procedural2dPlayRuntime,
    generation: GenerationPlayState,
}

/// 🧾️ Overlays the ephemeral runtime's selection and derived preview onto the persisted
/// generation state to build a {@link Procedural2dPlayView} for rendering.
fn play_view(projection: &Procedural2dDocument, runtime: &Procedural2dPlayRuntime) -> Procedural2dPlayView {
    let mut generation = projection.generation.clone();
    generation.selected_generation_id = runtime.selected_generation_id.clone();
    generation.preview_text = runtime.generation_preview_text.clone();
    Procedural2dPlayView { fixture: projection.fixture.clone(), runtime: runtime.clone(), generation }
}
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
fn procedural2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: PROCEDURAL2D_PLAY_APP_ID.into(),
        action: action.into(),
        args,
    }
}

/// 🎯️ `semio_framework_plugin::selection_ids`'s "ids" array plus a singular "nodeIds" fallback —
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

fn eval_preview_layers(play: &Procedural2dPlayView, preview: bool) -> String {
    // 🧵️ Never evaluates: reads whatever the off-main-thread `flowEvalTick` chain (or an explicit
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
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
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
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
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
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
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
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_main_graph(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
    let host = host_from_fixture(&play.fixture);
    let (nodes_json, edges_json) = fixture_to_workflow(&host.dag.fixture);
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
//#endregion 🔖️Render

//#region 🔖️Procedural2dPlayApp
#[derive(Default)]
pub struct Procedural2dPlayApp {
    runtime: RefCell<Procedural2dPlayRuntime>,
}

impl Procedural2dPlayApp {
    /// 🔀️ Runs a host mutation seeded from the projection fixture and diffs the result into operations.
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

    /// 🧬️ Emits generation operations for the generate-mode actions, updating ephemeral selection and
    /// preview from the post-operation state. `selectGeneration` is a view action (no operations).
    fn handle_generation(
        &self,
        action: &str,
        args: Option<&Value>,
        projection: &Procedural2dDocument,
    ) -> ActionEmit<Procedural2dOperation> {
        let spec = flow_fixture_to_form_spec(&projection.fixture);
        let mut state = projection.generation.clone();
        let mut runtime = self.runtime.borrow_mut();
        state.selected_generation_id = runtime.selected_generation_id.clone();
        if action == "selectGeneration" {
            if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                select_generation(&mut state, id);
            }
            runtime.selected_generation_id = state.selected_generation_id.clone();
            refresh_generation_preview(&mut runtime, &projection.fixture, &state);
            return ActionEmit::default();
        }
        let Some(operations) = generation_operations(action, args, &state, &spec) else {
            return ActionEmit::default();
        };
        for operation in &operations {
            apply_generation_operation(&mut state, operation);
        }
        runtime.selected_generation_id = state.selected_generation_id.clone();
        refresh_generation_preview(&mut runtime, &projection.fixture, &state);
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
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

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
        &self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, Procedural2dDocument>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        _view_state: &ViewState,
    ) -> ActionEmit<Procedural2dOperation> {
        let fixture = &doc.projection.fixture;
        match action {
            // 👁️ View actions — mutate ephemeral runtime, emit no operations.
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                self.runtime.borrow_mut().selected_ids = selection_ids(args);
                ActionEmit::default()
            }
            "nodeGraphHover" => ActionEmit::default(),
            "setShowMode" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    self.runtime.borrow_mut().show_mode = mode.into();
                }
                ActionEmit::default()
            }
            "generate" => {
                self.runtime.borrow_mut().show_mode = "generate".into();
                ActionEmit::default()
            }
            "setEvalOutputs" => {
                if let Some(outputs) = args.and_then(|value| value.get("outputs")) {
                    self.runtime.borrow_mut().eval_driver.set_eval_json(outputs.to_string());
                } else if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    self.runtime.borrow_mut().eval_driver.set_eval_json(json_text.into());
                }
                ActionEmit::default()
            }
            "flowEvalTick" => {
                let mut runtime = self.runtime.borrow_mut();
                let mut host = host_from_fixture_with_driver(fixture, Some(&runtime.eval_driver));
                let more = runtime.eval_driver.tick(&mut host);
                ActionEmit { effects: if more { vec![semio_framework_core::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() }, ..ActionEmit::default() }
            }
            "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" | "canvasWheel" => ActionEmit::default(),
            // 📷️ Graph camera — ephemeral view state (never a document operation), same model as flow-play.
            "nodeGraphViewport" => {
                if let Some(camera) = args
                    .and_then(|value| value.get("viewportJson"))
                    .and_then(|value| value.as_str())
                    .and_then(|json| serde_json::from_str(json).ok())
                {
                    self.runtime.borrow_mut().camera = camera;
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
                let selected = self.runtime.borrow().selected_ids.clone();
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
                    self.runtime.borrow_mut().selected_ids.clear();
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
                    self.runtime.borrow_mut().selected_ids = vec![id];
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
                        self.runtime.borrow_mut().selected_ids.retain(|id| id != &widget_id);
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

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes —
    /// covers every mutation path (edits, undo/redo, remote operations) in one place instead of each
    /// action re-checking. `FlowEvalDriver::sync` is cheap when nothing changed.
    fn pending_effects(
        &self,
        doc: &DocumentView<'_, Procedural2dDocument>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        _view_state: &ViewState,
    ) -> Vec<semio_framework_core::kernel::HostEffect> {
        let mut runtime = self.runtime.borrow_mut();
        let host = host_from_fixture_with_driver(&doc.projection.fixture, Some(&runtime.eval_driver));
        if runtime.eval_driver.sync(&host) {
            vec![semio_framework_core::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
        } else {
            Vec::new()
        }
    }

    fn render(
        &self,
        body_key: &str,
        doc: &DocumentView<'_, Procedural2dDocument>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
    ) -> UiNode {
        let play = play_view(doc.projection, &self.runtime.borrow());
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
//#endregion 🔖️Procedural2dPlayApp

//#region 🔖️Manifest
pub fn create_procedural2d_app() -> App {
    App::from_builder(
        App::builder(PROCEDURAL2D_PLAY_APP_ID, "Procedural 2D").document(["semio", "procedural", "2d"])
            .artifact_kind(ArtifactKindSpec {
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
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_MAIN, "Flow", PROCEDURAL2D_PLAY_BODY_MAIN, SurfaceKind::NodeGraph, "flow-graph")
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_PREVIEW, "Preview", PROCEDURAL2D_PLAY_BODY_PREVIEW, SurfaceKind::Canvas2d, "preview")
            .window_kind(
                PROCEDURAL2D_PLAY_WINDOW_GENERATIONS,
                "Generations",
                PROCEDURAL2D_PLAY_BODY_GENERATIONS,
                SurfaceKind::Canvas2d,
                "sparkles",
            )
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM, "Form", PROCEDURAL2D_PLAY_BODY_GENERATE_FORM, SurfaceKind::Canvas2d, "clipboard-list")
            .window_kind(
                PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW,
                "Preview",
                PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW,
                SurfaceKind::Canvas2d,
                "preview",
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
            // 📝️ Staged argument form for the palette-visible add-widget action (default materialized host-side).
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
    .workflow("procedural2d", "Procedural 2D", "layout")
}
//#endregion 🔖️Manifest

//#region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use store::create_document_envelope;
    use procedural_2d_engine::empty_procedural2d_projection;
    use procedural_2d_op::{Procedural2dEnvelope, Procedural2dStore};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Procedural2dDocumentVcs {
        store: RefCell<Procedural2dStore>,
    }

    #[wasm_bindgen]
    impl Procedural2dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Procedural2dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Procedural2dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Procedural2dStore::new(envelope)
                }
                None => Procedural2dStore::new(create_document_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", empty_procedural2d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
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

    /// 🧬️ A wrapper carrying the real action registry so default-materialization + kind discipline run.
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
        let document = procedural_2d_engine::procedural2d_document_from_dwg(&drawing).expect("dwg import document");
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
//#endregion 🧪️Tests
