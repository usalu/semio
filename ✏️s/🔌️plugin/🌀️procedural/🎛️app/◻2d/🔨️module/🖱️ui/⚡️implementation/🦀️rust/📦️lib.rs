//! 🎲️ Procedural 2D app — DocumentApp impl, render, manifest (constitutional: ui).

use flow_core::forms_bridge::flow_fixture_to_form_spec;
use flow_core::{flow_backed_node_graph_extras, CameraJson, FlowFixture, FlowHost, Widget};
use playbook::{apply_generation_operation, generation_operations, render_generation_form_body, render_generation_preview_text, render_generations_tree, select_generation, selected_generation, GenerationPlayState};
use procedural_2d::{widget_id, Procedural2dDocument, PROCEDURAL_2D_SCHEMA};
use procedural_2d_engine::{
    collect_drawing_handles_from_eval, default_projection, eval_driver_json_for, evaluate_generation_preview, fixture_to_workflow, generation_preview_layers, host_from_fixture, host_from_fixture_with_driver, procedural2d_io,
    refresh_generation_preview, scene_layers_from_drawing_handle, Procedural2dConfig,
};
use procedural_2d_op::{procedural2d_fixture_operations, Procedural2dConfigOperation, Procedural2dOperation};
use procedural_2d_protocol::Procedural2dCommand;
use semio_framework_plugin::{
    build_canvas_2d_scene, build_node_graph_scene, create_default_layout, create_named_layout, tree_item_with_action, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption,
    ActionDefinition, ActionDescriptor, ActionKind, App, AppLabels, ArtifactKindSpec, Canvas2dScene, ConfigView, DocumentApp, DocumentView, Emit, Label, Locale, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType,
    NodeGraphScene, NodeGraphViewport, OsMediaCapability, PanelGroup, PanelTreeBuilder, SurfaceKind, Terminology, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

use serde_json::{json, Value};

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
/// config's selection and derived preview overlaid, so the pure panel/render helpers keep reading a
/// single value. Assembled per call; never serialized as the document.
struct Procedural2dPlayView {
    fixture: FlowFixture,
    config: Procedural2dConfig,
    generation: GenerationPlayState,
}

/// 🧾️ Overlays the config's ephemeral selection and derived preview onto the persisted generation
/// state to build a {@link Procedural2dPlayView} for rendering.
fn play_view(projection: &Procedural2dDocument, config: &Procedural2dConfig) -> Procedural2dPlayView {
    let mut generation = projection.generation.clone();
    generation.selected_generation_id = config.selected_generation_id.clone();
    generation.preview_text = config.generation_preview_text.clone();
    Procedural2dPlayView { fixture: projection.fixture.clone(), config: config.clone(), generation }
}
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
fn procedural2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PROCEDURAL2D_PLAY_APP_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

fn eval_preview_layers(play: &Procedural2dPlayView, preview: bool) -> String {
    // 🧵️ Never evaluates: reads whatever the off-main-thread `flowEvalTick` chain (or an explicit
    // generation-preview/`setEvalOutputs` push) has cached so far — stale/empty is fine, the next
    // tick's scene refresh fills it in.
    let driver = play.config.eval_driver();
    let eval_json = driver.eval_json();
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
    if play.config.show_mode == "wire" {
        let offset = if preview { 240.0 } else { 0.0 };
        for widget in &play.fixture.widgets {
            let id = widget_id(widget).to_string();
            if play.config.selected_ids.is_empty() || play.config.selected_ids.iter().any(|selected| selected == &id) {
                let (x, y) = play.fixture.layout.get(&id).map(|layout| (layout.x, layout.y)).unwrap_or((offset + 48.0, 240.0));
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
        sources: native_en "Sources", native_de "Quellen", reuse_en "Sources", reuse_de "Quellen";
        components: native_en "Components", native_de "Komponenten", reuse_en "Components", reuse_de "Komponenten";
        sinks: native_en "Sinks", native_de "Senken", reuse_en "Sinks", reuse_de "Senken";
        show_mode_section: native_en "Show mode", native_de "Anzeigemodus", reuse_en "Show mode", reuse_de "Anzeigemodus";
        show_prefix: native_en "Show", native_de "Anzeigen", reuse_en "Show", reuse_de "Anzeigen";
        none: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        selection: native_en "Selection", native_de "Auswahl", reuse_en "Selection", reuse_de "Auswahl";
        ids: native_en "Ids", native_de "Kennungen", reuse_en "Ids", reuse_de "Kennungen";
        schema_prefix: native_en "Schema:", native_de "Schema:", reuse_en "Schema:", reuse_de "Schema:";
        widgets_prefix: native_en "Widgets:", native_de "Elemente:", reuse_en "Widgets:", reuse_de "Elemente:";
        show_mode_prefix: native_en "Show mode:", native_de "Anzeigemodus:", reuse_en "Show mode:", reuse_de "Anzeigemodus:";
        generate_hint: native_en "Add a generation to edit input values.", native_de "Erstelle eine Generation, um Eingabewerte zu bearbeiten.", reuse_en "Add a generation to edit input values.", reuse_de "Erstelle eine Generation, um Eingabewerte zu bearbeiten.";
        preview_hint: native_en "(evaluate a generation to preview output)", native_de "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)", reuse_en "(evaluate a generation to preview output)", reuse_de "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)";
        source_slider: native_en "Slider", native_de "Schieberegler", reuse_en "Slider", reuse_de "Schieberegler";
        source_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        component_add: native_en "Add", native_de "Addieren", reuse_en "Add", reuse_de "Addieren";
        component_and: native_en "And", native_de "Und", reuse_en "And", reuse_de "Und";
        component_concat: native_en "Concat", native_de "Verketten", reuse_en "Concat", reuse_de "Verketten";
        sink_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        sink_export: native_en "Export", native_de "Export", reuse_en "Export", reuse_de "Export";
        window_main: native_en "Flow", native_de "Fluss", reuse_en "Flow", reuse_de "Fluss";
        window_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        window_generations: native_en "Generations", native_de "Generationen", reuse_en "Generations", reuse_de "Generationen";
        window_generate_form: native_en "Form", native_de "Formular", reuse_en "Form", reuse_de "Formular";
        window_generate_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        delete_selection: native_en "Delete selection", native_de "Auswahl löschen", reuse_en "Delete selection", reuse_de "Auswahl löschen";
    }
}

/// 🗣️ Wave-2: `cfg.locale`-driven counterpart to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — matches `shooting_ui`'s own local
/// re-derivation exactly (`DocumentApp::render`/etc no longer receive a `ViewState`).
fn is_de_locale(cfg: &Procedural2dConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn procedural2d_locale(cfg: &Procedural2dConfig) -> Locale {
    if is_de_locale(cfg) {
        Locale::De
    } else {
        Locale::En
    }
}

fn resolve_labels<L: AppLabels>(cfg: &Procedural2dConfig) -> &'static L {
    L::labels(procedural2d_locale(cfg), Terminology::Native)
}

/// 🗣️ Resolves the active label set from the config-carried locale; falls back to native English.
fn procedural2d_labels(cfg: &Procedural2dConfig) -> &'static Procedural2dLabels {
    resolve_labels::<Procedural2dLabels>(cfg)
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
fn build_document_tree(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
    let widget_items: Vec<UiTreeItemNode> = play
        .fixture
        .widgets
        .iter()
        .map(|widget| {
            let id = widget_id(widget).to_string();
            tree_item_with_action(format!("procedural2d-play-document.widget.{id}"), Label::data(id.clone()), None, procedural2d_action("setSelection", Some(json!({ "ids": [id] }))))
        })
        .collect();
    PanelTreeBuilder::new("procedural2d-play-document")
        .section_or_placeholder("procedural2d-play-document.widgets", Some(Label::data(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL)), true, widget_items, labels.none)
        .selected(play.config.selected_ids.iter().map(|id| format!("procedural2d-play-document.widget.{id}")).collect())
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
            sources.iter().map(|(kind, label)| tree_item_with_action(format!("procedural2d-play-catalogue.source.{kind}"), *label, None, procedural2d_action("addWidget", Some(json!({ "kind": kind }))))).collect(),
        )
        .section(
            "procedural2d-play-catalogue.components",
            Some(labels.components.into()),
            true,
            components.iter().map(|(kind, label)| tree_item_with_action(format!("procedural2d-play-catalogue.component.{kind}"), *label, None, procedural2d_action("addWidget", Some(json!({ "kind": "neuron", "neuronKind": kind }))))).collect(),
        )
        .section(
            "procedural2d-play-catalogue.sinks",
            Some(labels.sinks.into()),
            true,
            sinks.iter().map(|(kind, label)| tree_item_with_action(format!("procedural2d-play-catalogue.sink.{kind}"), *label, None, procedural2d_action("addWidget", Some(json!({ "kind": kind }))))).collect(),
        )
        .section(
            "procedural2d-play-catalogue.modes",
            Some(labels.show_mode_section.into()),
            false,
            ["preview", "generate", "wire"]
                .iter()
                .map(|mode| tree_item_with_action(format!("procedural2d-play-catalogue.mode.{mode}"), Label::data(format!("{} {mode}", labels.show_prefix.as_str())), None, procedural2d_action("setShowMode", Some(json!({ "value": mode })))))
                .collect(),
        )
        .build()
}

fn build_inspector_tree(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
    if play.config.selected_ids.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "procedural2d-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![
                ui_text(Label::data(format!("{} flow.fixture", labels.schema_prefix.as_str()))),
                ui_text(Label::data(format!("{} {}", labels.widgets_prefix.as_str(), play.fixture.widgets.len()))),
                ui_text(Label::data(format!("{} {}", labels.show_mode_prefix.as_str(), play.config.show_mode))),
            ],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        presence: UiPresence::default(),
        id: "procedural2d-play-inspector.selection".into(),
        label: labels.selection.into(),
        default_open: Some(true),
        fields: vec![ui_inspector_readonly_field("procedural2d-play-inspector.ids", labels.ids, play.config.selected_ids.join(", "))],
    }])
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_main_graph(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
    let host = host_from_fixture(&play.fixture);
    let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
    let viewport = NodeGraphViewport { x: play.config.camera.x, y: play.config.camera.y, zoom: play.config.camera.zoom };
    let selection = play.config.selected_ids.clone();
    let driver = play.config.eval_driver();
    let flow_extras = flow_backed_node_graph_extras(&play.fixture, "", 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, Some(&driver));
    build_node_graph_scene(
        PROCEDURAL2D_PLAY_SURFACE_MAIN,
        PROCEDURAL2D_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            operators: flow_extras.operators,
            capabilities_json: flow_extras.capabilities_json,
            lod_json: flow_extras.lod_json,
            fixture_json: flow_extras.fixture_json,
            selection,
            ..NodeGraphScene::base(nodes, edges, viewport)
        },
    )
}

fn render_preview_canvas(play: &Procedural2dPlayView) -> UiNode {
    build_canvas_2d_scene(PROCEDURAL2D_PLAY_SURFACE_PREVIEW, PROCEDURAL2D_PLAY_APP_ID, Canvas2dScene { camera_x: play.config.camera.x, camera_y: play.config.camera.y, zoom: play.config.camera.zoom, layers_json: eval_preview_layers(play, true) })
}

fn render_generate_generations(play: &Procedural2dPlayView) -> UiNode {
    render_generations_tree(PROCEDURAL2D_PLAY_APP_ID, "procedural2d-play-generate", &play.generation.generations, play.generation.selected_generation_id.as_deref())
}

fn render_generate_form(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
    let spec = flow_fixture_to_form_spec(&play.fixture);
    let Some(generation) = selected_generation(&play.generation) else {
        return ui_text(labels.generate_hint);
    };
    render_generation_form_body(&spec, &generation.values, PROCEDURAL2D_PLAY_APP_ID, "updateGenerationValues", &generation.id)
}

fn render_generate_preview(play: &Procedural2dPlayView, labels: &Procedural2dLabels) -> UiNode {
    let eval_json = play.generation.preview_text.as_deref().filter(|value| !value.is_empty()).unwrap_or("");
    if eval_json.is_empty() {
        return ui_text(labels.preview_hint);
    }
    let layers = generation_preview_layers(eval_json);
    if layers == "[]" {
        return render_generation_preview_text(PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW, PROCEDURAL2D_PLAY_APP_ID, eval_json);
    }
    build_canvas_2d_scene(PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW, PROCEDURAL2D_PLAY_APP_ID, Canvas2dScene { camera_x: play.config.camera.x, camera_y: play.config.camera.y, zoom: play.config.camera.zoom, layers_json: layers })
}
//#endregion 🔖️Render

//#region 🔖️Procedural2dPlayApp
/// 🧪️ Wave-2: unit struct — every former `Procedural2dPlayRuntime`/`self.runtime` field now lives in
/// `procedural_2d_engine::Procedural2dConfig` (see `DocumentApp::Config`), written through
/// `procedural_2d_op::Procedural2dConfigOperation`s.
#[derive(Default)]
pub struct Procedural2dPlayApp;

impl Procedural2dPlayApp {
    /// 🔀️ Runs a host mutation seeded from the projection fixture and diffs the result into operations.
    /// Diffs against the host-normalized baseline (not the raw projection) so `FlowHost`'s own
    /// dedupe/dag-rebuild normalization does not leak spurious collection operations — only the actual
    /// mutation becomes an operation, which keeps concurrent disjoint edits mergeable on the backbone.
    fn ops_from_host_mutation(&self, fixture: &FlowFixture, mutate: impl FnOnce(&mut FlowHost)) -> Vec<Procedural2dOperation> {
        let mut host = host_from_fixture(fixture);
        let baseline = host.fixture.clone();
        mutate(&mut host);
        procedural2d_fixture_operations(&baseline, &host.fixture)
    }

    /// 🧬️ Emits generation operations for the generate-mode commands, updating the config's ephemeral
    /// selection and preview from the post-operation state via a whole-config `Snapshot`.
    /// `selectGeneration` is config-only (no document operations).
    fn handle_generation(&self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, Procedural2dDocument>, cfg: &ConfigView<'_, Procedural2dConfig>) -> Emit<Procedural2dOperation, Procedural2dConfigOperation> {
        let projection = doc.projection;
        let spec = flow_fixture_to_form_spec(&projection.fixture);
        let mut state = projection.generation.clone();
        state.selected_generation_id = cfg.projection.selected_generation_id.clone();
        let mut next_config = cfg.projection.clone();
        if action == "selectGeneration" {
            if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                select_generation(&mut state, id);
            }
            next_config.selected_generation_id = state.selected_generation_id.clone();
            refresh_generation_preview(&mut next_config, &projection.fixture, &state);
            return Emit::config(vec![Procedural2dConfigOperation::Snapshot { config: next_config }]);
        }
        let Some(operations) = generation_operations(action, args, &state, &spec) else {
            return Emit::default();
        };
        for operation in &operations {
            apply_generation_operation(&mut state, operation);
        }
        next_config.selected_generation_id = state.selected_generation_id.clone();
        refresh_generation_preview(&mut next_config, &projection.fixture, &state);
        let coalesce_key = (action == "updateGenerationValues").then(|| "generation-values".to_string());
        Emit { document_operations: operations.into_iter().map(Procedural2dOperation::Generation).collect(), config_operations: vec![Procedural2dConfigOperation::Snapshot { config: next_config }], coalesce_key, ..Default::default() }
    }
}

impl DocumentApp for Procedural2dPlayApp {
    type Projection = Procedural2dDocument;
    type Operation = Procedural2dOperation;
    type Config = Procedural2dConfig;
    type ConfigOperation = Procedural2dConfigOperation;
    type Command = Procedural2dCommand;

    fn app_id(&self) -> &str {
        PROCEDURAL2D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PROCEDURAL_2D_SCHEMA
    }

    fn initial_projection(&self) -> Procedural2dDocument {
        default_projection()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(procedural2d_io())
    }

    /// 🏷️ Maps each `Procedural2dCommand` variant back to the action id it was declared under in
    /// `create_procedural2d_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Operation kind-discipline check.
    fn command_id(&self, command: &Procedural2dCommand) -> &str {
        match command {
            Procedural2dCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            Procedural2dCommand::MoveMediaNode { .. } => "moveMediaNode",
            Procedural2dCommand::AddWidget { .. } => "addWidget",
            Procedural2dCommand::RemoveWidget { .. } => "removeWidget",
            Procedural2dCommand::ConnectMediaPorts { .. } => "connectMediaPorts",
            Procedural2dCommand::Reorganize => "reorganize",
            Procedural2dCommand::AddGeneration => "addGeneration",
            Procedural2dCommand::RemoveGeneration { .. } => "removeGeneration",
            Procedural2dCommand::RenameGeneration { .. } => "renameGeneration",
            Procedural2dCommand::UpdateGenerationValues { .. } => "updateGenerationValues",
            Procedural2dCommand::NodeGraphViewport { .. } => "nodeGraphViewport",
            Procedural2dCommand::SetSelection { .. } => "setSelection",
            Procedural2dCommand::SelectNode { .. } => "selectNode",
            Procedural2dCommand::NodeGraphSelect { .. } => "nodeGraphSelect",
            Procedural2dCommand::NodeGraphHover => "nodeGraphHover",
            Procedural2dCommand::SetShowMode { .. } => "setShowMode",
            Procedural2dCommand::Generate => "generate",
            Procedural2dCommand::SetEvalOutputs { .. } => "setEvalOutputs",
            Procedural2dCommand::CanvasPointerDown => "canvasPointerDown",
            Procedural2dCommand::CanvasPointerMove => "canvasPointerMove",
            Procedural2dCommand::CanvasPointerUp => "canvasPointerUp",
            Procedural2dCommand::CanvasWheel => "canvasWheel",
            Procedural2dCommand::SelectGeneration { .. } => "selectGeneration",
            Procedural2dCommand::FlowEvalTick => "flowEvalTick",
            Procedural2dCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(&self, command: &Procedural2dCommand, doc: &DocumentView<'_, Procedural2dDocument>, cfg: &ConfigView<'_, Procedural2dConfig>) -> Emit<Procedural2dOperation, Procedural2dConfigOperation> {
        let fixture = &doc.projection.fixture;
        let config = cfg.projection;
        match command {
            // 👁️ Config-only — ephemeral selection/hover/show-mode/eval-scratch, never document operations.
            Procedural2dCommand::SetSelection { ids } | Procedural2dCommand::SelectNode { ids } | Procedural2dCommand::NodeGraphSelect { ids } => Emit::config(vec![Procedural2dConfigOperation::SetSelection { ids: ids.clone() }]),
            Procedural2dCommand::NodeGraphHover => Emit::default(),
            Procedural2dCommand::SetShowMode { value } => Emit::config(vec![Procedural2dConfigOperation::SetShowMode { value: value.clone() }]),
            Procedural2dCommand::Generate => Emit::config(vec![Procedural2dConfigOperation::SetShowMode { value: "generate".into() }]),
            Procedural2dCommand::SetEvalOutputs { outputs_json } => {
                let mut driver = config.eval_driver();
                driver.set_eval_json(outputs_json.clone());
                Emit::config(vec![Procedural2dConfigOperation::SetEvalDriver { json: eval_driver_json_for(&driver) }])
            }
            Procedural2dCommand::FlowEvalTick => {
                let mut driver = config.eval_driver();
                let mut host = host_from_fixture_with_driver(fixture, Some(&driver));
                let more = driver.tick(&mut host);
                Emit {
                    config_operations: vec![Procedural2dConfigOperation::SetEvalDriver { json: eval_driver_json_for(&driver) }],
                    effects: if more { vec![semio_framework_core::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() },
                    ..Default::default()
                }
            }
            Procedural2dCommand::CanvasPointerDown | Procedural2dCommand::CanvasPointerMove | Procedural2dCommand::CanvasPointerUp | Procedural2dCommand::CanvasWheel => Emit::default(),
            // 📷️ Graph camera — config-only (never a document operation), same model as flow-play.
            Procedural2dCommand::NodeGraphViewport { viewport_json } => match serde_json::from_str::<CameraJson>(viewport_json) {
                Ok(camera) => Emit::config(vec![Procedural2dConfigOperation::SetCamera { camera }]),
                Err(_) => Emit::default(),
            },
            // ✏️ Operations — compute the target fixture via the host, emit fixture operations.
            Procedural2dCommand::NodeGraphEdit { operations_json } => {
                let sub_operations: Vec<Value> = serde_json::from_str(operations_json).unwrap_or_default();
                let selected = config.selected_ids.clone();
                let mut cleared = false;
                let operations = self.ops_from_host_mutation(fixture, |host| {
                    for operation in &sub_operations {
                        match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                            "setFixture" => {
                                if let Some(fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| serde_json::from_str::<FlowFixture>(json).ok()) {
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
                let config_operations = if cleared { vec![Procedural2dConfigOperation::SetSelection { ids: Vec::new() }] } else { Vec::new() };
                Emit { document_operations: operations, config_operations, ..Default::default() }
            }
            Procedural2dCommand::MoveMediaNode { node_id, x, y } => Emit::operations(self.ops_from_host_mutation(fixture, |host| {
                let _ = host.move_widget(node_id, *x, *y);
            })),
            Procedural2dCommand::AddWidget { kind, neuron_kind, x, y } => {
                let descriptor = match kind.as_str() {
                    "neuron" => json!({ "kind": "neuron", "neuronKind": neuron_kind.clone().unwrap_or_else(|| "math.add".into()) }).to_string(),
                    other => json!({ "kind": other }).to_string(),
                };
                let mut host = host_from_fixture(fixture);
                let baseline = host.fixture.clone();
                if let Ok(id) = host.add_widget(&descriptor, x.unwrap_or(120.0), y.unwrap_or(120.0)) {
                    return Emit { document_operations: procedural2d_fixture_operations(&baseline, &host.fixture), config_operations: vec![Procedural2dConfigOperation::SetSelection { ids: vec![id] }], ..Default::default() };
                }
                Emit::default()
            }
            Procedural2dCommand::RemoveWidget { widget_id } => {
                let operations = self.ops_from_host_mutation(fixture, |host| {
                    let _ = host.remove_widget(widget_id);
                });
                if operations.is_empty() {
                    return Emit::default();
                }
                let remaining: Vec<String> = config.selected_ids.iter().filter(|id| *id != widget_id).cloned().collect();
                Emit { document_operations: operations, config_operations: vec![Procedural2dConfigOperation::SetSelection { ids: remaining }], ..Default::default() }
            }
            Procedural2dCommand::ConnectMediaPorts { source_node_id, source_port_id, target_node_id, target_port_id } => Emit::operations(self.ops_from_host_mutation(fixture, |host| {
                let _ = host.connect_ports(source_node_id, source_port_id, target_node_id, target_port_id);
            })),
            Procedural2dCommand::Reorganize => Emit::operations(self.ops_from_host_mutation(fixture, |host| {
                let _ = host.reorganize(r#"{"orientation":"leftRight"}"#);
            })),
            Procedural2dCommand::AddGeneration => self.handle_generation("addGeneration", None, doc, cfg),
            Procedural2dCommand::RemoveGeneration { id } => self.handle_generation("removeGeneration", Some(&json!({ "id": id })), doc, cfg),
            Procedural2dCommand::RenameGeneration { id, name } => self.handle_generation("renameGeneration", Some(&json!({ "id": id, "name": name })), doc, cfg),
            Procedural2dCommand::UpdateGenerationValues { generation_id, question_id, value } => {
                let value_json = dsl::from_dsl_value(value.clone()).unwrap_or(Value::Null);
                self.handle_generation("updateGenerationValues", Some(&json!({ "generationId": generation_id, "questionId": question_id, "value": value_json })), doc, cfg)
            }
            Procedural2dCommand::SelectGeneration { id } => self.handle_generation("selectGeneration", Some(&json!({ "id": id })), doc, cfg),
            Procedural2dCommand::SetLocale { value } => Emit::config(vec![Procedural2dConfigOperation::SetLocale { value: value.clone() }]),
        }
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes —
    /// covers every mutation path (edits, undo/redo, remote operations) in one place instead of each
    /// action re-checking. `FlowEvalDriver::sync` is cheap when nothing changed.
    fn pending_effects(&self, doc: &DocumentView<'_, Procedural2dDocument>, cfg: &ConfigView<'_, Procedural2dConfig>) -> Vec<semio_framework_core::kernel::HostEffect> {
        let mut driver = cfg.projection.eval_driver();
        let host = host_from_fixture_with_driver(&doc.projection.fixture, Some(&driver));
        if driver.sync(&host) {
            vec![semio_framework_core::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
        } else {
            Vec::new()
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Procedural2dDocument>, cfg: &ConfigView<'_, Procedural2dConfig>) -> UiNode {
        let play = play_view(doc.projection, cfg.projection);
        let labels = procedural2d_labels(cfg.projection);
        match body_key {
            PROCEDURAL2D_PLAY_BODY_MAIN => render_main_graph(&play, labels),
            PROCEDURAL2D_PLAY_BODY_PREVIEW => render_preview_canvas(&play),
            PROCEDURAL2D_PLAY_BODY_GENERATIONS => render_generate_generations(&play),
            PROCEDURAL2D_PLAY_BODY_GENERATE_FORM => render_generate_form(&play, labels),
            PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(&play, labels),
            PROCEDURAL2D_PLAY_BODY_DOCUMENT => build_document_tree(&play, labels),
            PROCEDURAL2D_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            PROCEDURAL2D_PLAY_BODY_INSPECTION => build_inspector_tree(&play, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🗂️ Grouped disclosure: `addWidget`/`reorganize`/`generate` stay top-level (the most frequent
    /// verbs on a procedural-2d canvas); the display-mode toggle, generation authoring, and generation
    /// selection each fold into their own taxonomy group; the delete-selection item stays a direct
    /// destructive item last — `organize_context_menu` (applied automatically at the
    /// `VcsDocumentApp::context_menu` funnel) sorts the groups into `RIBBON_PARENT_CATEGORIES` order
    /// and inserts the pre-destructive separator itself, so this emitter needs no separator/ordering
    /// logic of its own.
    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &DocumentView<'_, Procedural2dDocument>,
        cfg: &ConfigView<'_, Procedural2dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let labels = resolve_labels::<Procedural2dLabels>(cfg.projection);
        let is_de = is_de_locale(cfg.projection);
        let selected = cfg.projection.selected_ids.clone();
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]);
        let mut menu = Menu::of(registry).action("addWidget").action("reorganize").action("generate").group("mode", |m| m.action("setShowMode")).group("create", |m| m.action("addGeneration")).group("methods", |m| m.action("selectGeneration"));
        if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
            menu = menu.item(spec);
        }
        menu.build()
    }

    /// 🎞️ Declares `export_media`'s default document schema — pack-encodes `doc.projection`,
    /// wrapped `Structured{schema: self.document_schema(), json: base64}` (identical to the trait
    /// default, restated because this override also handles `"drawing:out"`).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Procedural2dDocument>) -> Result<Media, MediaError> {
        match port {
            "drawing:out" => {
                let eval_json = evaluate_generation_preview(&doc.projection.fixture, &serde_json::Map::new());
                let layers_json = generation_preview_layers(&eval_json);
                Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: layers_json } })
            }
            "document:out" => {
                let bytes = store::DocumentPack::encode_pack(doc.projection);
                Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Flow }, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `"params:in"`: a generic Data×Value JSON object `{widgetId: number}` — patches matching
    /// `InputSlider` widgets' `value` field, leaving unmatched keys/widget kinds untouched (total,
    /// lenient — a workflow parameter feed is never a hard error on a stale/foreign key).
    fn import_media(&self, port: &str, media: &Media, doc: &DocumentView<'_, Procedural2dDocument>) -> Result<Emit<Procedural2dOperation, Procedural2dConfigOperation>, MediaError> {
        if port != "params:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "params:in expects a Structured JSON object payload".into()));
        };
        let parsed: Value = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let Some(object) = parsed.as_object() else {
            return Err(MediaError::Payload(port.to_string(), "params:in payload must be a JSON object".into()));
        };
        let mut operations = Vec::new();
        for (widget_id_key, value) in object {
            let Some(number) = value.as_f64() else { continue };
            let Some((index, widget)) = doc.projection.fixture.widgets.iter().enumerate().find(|(_, widget)| widget_id(widget) == widget_id_key.as_str()) else { continue };
            if let Widget::InputSlider { id, min, max, step, .. } = widget {
                operations.push(Procedural2dOperation::SetWidget { index, widget: Widget::InputSlider { id: id.clone(), value: number, min: *min, max: *max, step: *step } });
            }
        }
        Ok(Emit::operations(operations))
    }
}
//#endregion 🔖️Procedural2dPlayApp

//#region 🔖️Manifest
pub fn create_procedural2d_app() -> App {
    App::from_builder(
        App::builder(PROCEDURAL2D_PLAY_APP_ID, LocalizedLabel::native("Procedural 2D", "Procedural 2D")).document(["semio", "procedural", "2d"])
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
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .mode("generate", LocalizedLabel::native("Generate", "Generieren"), "sparkles")
            .default_mode_id("edit")
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_MAIN, LocalizedLabel::native("Flow", "Fluss"), PROCEDURAL2D_PLAY_BODY_MAIN, SurfaceKind::NodeGraph, "flow-graph")
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_PREVIEW, LocalizedLabel::native("Preview", "Vorschau"), PROCEDURAL2D_PLAY_BODY_PREVIEW, SurfaceKind::Canvas2d, "preview")
            .window_kind(
                PROCEDURAL2D_PLAY_WINDOW_GENERATIONS,
                LocalizedLabel::native("Generations", "Generationen"),
                PROCEDURAL2D_PLAY_BODY_GENERATIONS,
                SurfaceKind::Canvas2d,
                "sparkles",
            )
            .window_kind(PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM, LocalizedLabel::native("Form", "Formular"), PROCEDURAL2D_PLAY_BODY_GENERATE_FORM, SurfaceKind::Canvas2d, "clipboard-list")
            .window_kind(
                PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW,
                LocalizedLabel::native("Preview", "Vorschau"),
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
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
                PanelGroup::Workbench,
                PROCEDURAL2D_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                PROCEDURAL2D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                PROCEDURAL2D_PLAY_BODY_INSPECTION,
            )
            // ✏️ Document-mutating operations — dispatched as VCS operations with a true inverse.
            // 🗂️ Referenced by `Procedural2dPlayApp::context_menu` — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::new_catalog("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"), ActionKind::Operation).with_category("selection"))
            .operation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(ActionDefinition::new_catalog("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Operation).with_category("create"))
            .operation("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"))
            .operation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Ports verbinden"))
            .action_with(ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Operation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Operation).with_category("create"))
            .operation("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"))
            .operation("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"))
            .operation("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"))
            // 👁️ Ephemeral view actions — selection, hover, camera, the show-mode display toggle, and evaluation scratch (emit no operations).
            .view_action("nodeGraphViewport", LocalizedLabel::native("Set Viewport", "Ansicht festlegen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("selectNode", LocalizedLabel::native("Select Node", "Knoten auswählen"))
            .view_action("nodeGraphSelect", LocalizedLabel::native("Node Graph Select", "Graph-Auswahl"))
            .view_action("nodeGraphHover", LocalizedLabel::native("Node Graph Hover", "Graph-Hover"))
            // 🗂️ Referenced by `Procedural2dPlayApp::context_menu` — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::new_catalog("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"), ActionKind::View).with_category("mode"))
            .action_with(ActionDefinition::new_catalog("generate", LocalizedLabel::native("Generate", "Generieren"), ActionKind::View).with_category("actions"))
            .view_action("setEvalOutputs", LocalizedLabel::native("Set Eval Outputs", "Auswertungsausgaben festlegen"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Canvas-Zeiger gedrückt"))
            .view_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Canvas-Zeiger bewegt"))
            .view_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Canvas-Zeiger losgelassen"))
            .view_action("canvasWheel", LocalizedLabel::native("Canvas Wheel", "Canvas-Mausrad"))
            .action_with(ActionDefinition::new_catalog("selectGeneration", LocalizedLabel::native("Select Generation", "Generation auswählen"), ActionKind::View).with_category("methods"))
            // 📝️ Staged argument form for the palette-visible add-widget action (default materialized host-side).
            .action_args("addWidget", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
                    ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
                    ActionArgOption::new("neuron", LocalizedLabel::native("Component", "Komponente")),
                    ActionArgOption::new("outputPreview", LocalizedLabel::native("Preview", "Vorschau")),
                    ActionArgOption::new("outputExport", LocalizedLabel::native("Export", "Export")),
                ]).default_value("inputSlider"),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🎯️ Wave-2 typed channel + workflow ports — `config_spec()`/`procedural2d_io()` are this
            // same information's single source of truth, reused here rather than duplicated.
            .config(Procedural2dPlayApp::default().config_spec())
            .io(procedural2d_io()),
    )
    .example("default", LocalizedLabel::native("Default", "Standard"), serde_json::to_string(&default_projection()).unwrap(), "file")
    .workflow("procedural2d", "Procedural 2D", "layout")
}
//#endregion 🔖️Manifest

//#region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use procedural_2d_engine::empty_procedural2d_projection;
    use procedural_2d_op::{Procedural2dEnvelope, Procedural2dStore};
    use std::cell::RefCell;
    use store::create_document_envelope;
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
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp, ViewState};

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
        // addWidget with the declared `kind` default: must materialize into a real widget operation.
        app.dispatch_typed(Procedural2dCommand::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: None, y: None }, &meta("local")).expect("add widget");
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
        // 🩹️ `operators` is now a typed `Vec<NodeGraphOperatorRecord>` field (a concurrent, unrelated
        // W5 typed-`NodeGraphScene` migration renamed this off the old `operatorsJson` string field —
        // see `🧰️framework/🔨️module/🖱️ui/🧊️wgpu`'s `NodeGraphScene`) — assert the new shape instead.
        assert!(graph.get("operators").and_then(|v| v.as_array()).is_some_and(|items| !items.is_empty()));
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
        app.dispatch_typed(Procedural2dCommand::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: None, y: None }, &meta("local")).expect("add");
        assert_eq!(app.projection().expect("projection").fixture.widgets.len(), before + 1);
    }

    #[test]
    fn add_widget_undo_redo_round_trip() {
        let mut app = new_app();
        let before = app.projection().expect("projection").fixture.widgets.len();
        testkit::assert_undo_redo_round_trip(&mut app, Procedural2dCommand::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: None, y: None }, |app| app.projection().expect("projection").fixture.widgets.len(), before, before + 1);
    }

    #[test]
    fn generate_is_a_view_action_with_no_document_operations() {
        let mut app = new_app();
        let before = app.projection().expect("projection");
        app.dispatch_typed(Procedural2dCommand::Generate, &meta("local")).expect("generate");
        assert_eq!(app.projection().expect("projection"), before, "generate must not mutate the document");
    }

    #[test]
    fn add_generation_records_an_undoable_generation_operation() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(&mut app, Procedural2dCommand::AddGeneration, |app| app.projection().expect("projection").generation.generations.len(), 0, 1);
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
        let widgets: Vec<String> = new_app().projection().expect("projection").fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
        assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
        let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
        testkit::assert_two_instances_converge::<Procedural2dPlayApp, (Option<f64>, Option<f64>)>(
            "mem://procedural2d-convergence",
            Procedural2dCommand::MoveMediaNode { node_id: w0.clone(), x: 111.0, y: 5.0 },
            Procedural2dCommand::MoveMediaNode { node_id: w1.clone(), x: 222.0, y: 6.0 },
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
        // 🗣️ Wave-2: locale now lives on the config artifact, set via `SetLocale` (dispatched by the
        // shell session, mirroring `shooting_ui`'s `ShootingCommand::SetLocale`) — no longer a
        // per-render `ViewState` override.
        let mut app = new_app();
        app.dispatch_typed(Procedural2dCommand::SetLocale { value: "de".into() }, &meta("local")).expect("set locale");
        let catalogue = app.render(PROCEDURAL2D_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("Quellen"));
        assert!(catalogue_json.contains("Komponenten"));
        assert!(catalogue_json.contains("Senken"));
        assert!(catalogue_json.contains("Anzeigemodus"));
        assert!(!catalogue_json.contains("\"Sources\""));
        let inspector = app.render(PROCEDURAL2D_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let inspector_json = serde_json::to_string(&inspector).unwrap();
        assert!(inspector_json.contains("Elemente:"));
    }

    //#region 🔖️ContextMenuTests
    #[test]
    fn context_menu_stays_within_disclosure_budget_with_destructive_last() {
        let mut app = new_app_with_registry();
        app.dispatch_typed(Procedural2dCommand::SetSelection { ids: vec!["rect".into()] }, &meta("local")).expect("select");
        let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let items = app.context_menu(&request);
        assert!(items.len() <= 9, "top-level menu rows (leaves + groups + separator) must stay within disclosure budget, got {}", items.len());
        assert_eq!(items.last().map(|item| item.id.as_str()), Some("delete-selection"), "the destructive delete row must be last");
        assert_eq!(items.last().and_then(|item| item.destructive), Some(true));
    }
    //#endregion 🔖️ContextMenuTests

    //#region 🔖️PortTests
    #[test]
    fn export_drawing_out_returns_vector_media() {
        let mut app = new_app();
        let media = app.export_media("drawing:out").expect("export drawing:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "2d.drawing");
                let _: Value = serde_json::from_str(&json).expect("layers json parses");
            }
            other => panic!("expected Structured payload, got {other:?}"),
        }
    }

    #[test]
    fn export_document_out_returns_flow_media() {
        let mut app = new_app();
        let media = app.export_media("document:out").expect("export document:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Flow });
        assert!(matches!(media.payload, MediaPayload::Structured { schema, .. } if schema == PROCEDURAL_2D_SCHEMA));
    }

    #[test]
    fn export_unknown_port_is_not_implemented() {
        let mut app = new_app();
        assert!(matches!(app.export_media("bogus:out"), Err(MediaError::NotImplemented)));
    }

    #[test]
    fn import_params_in_patches_matching_input_slider() {
        // 🌱️ The default `🌀️default.procedural2d` fixture has no `InputSlider` widget of its own
        // (just draw-shape neurons + an output-preview) — add one via `AddWidget` first.
        let mut app = new_app();
        app.dispatch_typed(Procedural2dCommand::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: None, y: None }, &meta("local")).expect("add slider");
        let slider_id = app
            .projection()
            .expect("projection")
            .fixture
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::InputSlider { id, .. } => Some(id.clone()),
                _ => None,
            })
            .expect("just-added input slider");
        let media = Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "params".into(), json: json!({ slider_id.clone(): 42.0 }).to_string() } };
        app.import_media("params:in", &media, &meta("local")).expect("import params");
        let value = app.projection().expect("projection").fixture.widgets.iter().find_map(|widget| match widget {
            Widget::InputSlider { id, value, .. } if id == &slider_id => Some(*value),
            _ => None,
        });
        assert_eq!(value, Some(42.0));
    }

    #[test]
    fn import_params_in_ignores_unmatched_keys() {
        let mut app = new_app();
        let before = app.projection().expect("projection");
        let media = Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "params".into(), json: json!({ "does-not-exist": 1.0 }).to_string() } };
        app.import_media("params:in", &media, &meta("local")).expect("import params is total/lenient");
        assert_eq!(app.projection().expect("projection"), before, "unmatched keys must not mutate the document");
    }

    #[test]
    fn media_ports_declare_params_in_and_drawing_out() {
        // 🔌️ `media_ports()` lives on `DocumentApp` directly (not the object-safe `PluginApp` wrapper
        // surface `VcsDocumentApp` exposes) — exercised on the raw unit-struct app.
        let ports = DocumentApp::media_ports(&Procedural2dPlayApp::default());
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let params_in = ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
        assert_eq!(params_in.media_type, MediaType { class: MediaClass::Data, form: MediaForm::Value });
        let drawing_out = ports.iter().find(|port| port.id == "drawing:out").expect("drawing:out declared");
        assert_eq!(drawing_out.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
        assert_eq!(drawing_out.kind_id.as_deref(), Some("2d.drawing"));
    }
    //#endregion 🔖️PortTests
}
//#endregion 🧪️Tests
