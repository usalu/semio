//! 🧱️ Procedural 3D app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait
//! pilot conversion — `Procedural3dPlayApp` is a unit struct; every former `Procedural3dRuntime` field
//! (selection, hover, cameras, LOD/show display options, sun display options, generation
//! selection/preview, off-main-thread eval driver) now lives in `procedural_3d_engine::Procedural3dConfig`,
//! written via `procedural_3d_op::Procedural3dConfigOperation`s (real `backwards`, no ad hoc
//! `InverseAction`); every action dispatches through the single typed
//! `procedural_3d_protocol::Procedural3dCommand` channel via `DocumentApp::handle`.

use flow_core::forms_bridge::flow_fixture_to_form_spec;
use flow_core::{flow_backed_node_graph_extras, CameraJson, FlowEvalDriver, FlowFixture, FlowHost, Widget};
use playbook::{apply_generation_operation, generation_operations, render_generation_form_body, render_generation_preview_text, render_generations_tree, select_generation, selected_generation, GenerationOperation, GenerationPlayState};
use procedural_3d::{widget_id, Procedural3dDocument, PROCEDURAL_3D_SCHEMA};
use procedural_3d_engine::{
    default_projection, ensure_gumball_node, evaluate_generation_preview, example_projection, fixture_to_workflow, generation_fixture_for, gumball_rotate_params_json, gumball_scale_params_json, gumball_translate_params_json, gumball_widget_number_param, gumball_widget_offset,
    host_from_fixture, host_from_fixture_with_driver, is_procedural3d_example_id, widget_id_from_instance_id, Procedural3dConfig, Procedural3dPreviewCamera, PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_BOX_SHELL, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_HEX_COLUMN,
    PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_RECT_EXTRUDE, PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, PROCEDURAL_EXAMPLE_SPHERE_TORUS,
};
use procedural_3d_op::{procedural3d_fixture_operations, Procedural3dConfigOperation, Procedural3dOperation};
use procedural_3d_protocol::Procedural3dCommand;
use semio_framework_plugin::{
        apply_world3d_sun_action, build_node_graph_scene, build_world_3d_scene, create_default_layout, create_named_layout, merge_world_selection_ids, tree_item_with_action, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_readonly_field, ui_text, world3d_scene, world3d_sun_measures, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppLabels, ArtifactKindSpec, ConfigView, DocumentApp,
    DocumentView, Emit, Fault, HostEffect, Label, Locale, LocalizedLabel, MeasureSelectItem, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, NodeGraphHover, NodeGraphScene, NodeGraphViewport, OsMediaCapability, OsMediaFormat, PanelGroup,
    PanelTreeBuilder, SelectionSet, SurfaceKind, Terminology, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode, UtilityDefinition, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, SET_ACTIVE_UTILITY_ACTION_ID,
};
use serde_json::{json, Value};
use store::DocumentPack;

//#region 🔖️Constants
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
const PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";


//#endregion 🔖️Constants

//#region 🔖️Locale

//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn procedural_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(PROCEDURAL_3D_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🎯️ B1: the typed-command counterpart of the pre-B1 `mesh_selection_ids` (JSON-args) — falls back
/// to the current config selection when the command carries no explicit ids.
fn mesh_selection_ids_typed(ids: &[String], fallback: &[String]) -> Vec<String> {
    if ids.is_empty() {
        fallback.to_vec()
    } else {
        ids.to_vec()
    }
}

/// 🧵️ Recomputes the driver's live "still pending?"/"still computing?" state fresh from the fixture
/// against the persisted baseline (`cfg.eval_driver()`'s `previous_snapshot`/`previous_channels`) —
/// this local mutation is thrown away (never persisted), it only exists to answer "is there pending
/// work right now" for THIS render/`pending_effects` call, matching the "render is a pure projection"
/// contract: the real, persisted driver mutation only ever happens inside the `flowEvalTick` command.
fn live_eval_driver(fixture: &FlowFixture, cfg: &Procedural3dConfig) -> FlowEvalDriver {
    let mut driver = cfg.eval_driver();
    let host = host_from_fixture_with_driver(fixture, Some(&driver));
    driver.sync(&host);
    driver
}

/// 🧾️ Overlays the config's ephemeral generation selection/preview onto the persisted generation
/// state — the B1 replacement for the deleted `Procedural3dPlayView`/`play_view`.
fn generation_view(projection: &Procedural3dDocument, cfg: &Procedural3dConfig) -> GenerationPlayState {
    let mut generation = projection.generation.clone();
    generation.selected_generation_id = cfg.selected_generation_id.clone();
    generation.preview_text = cfg.generation_preview_text.clone();
    generation
}

/// ▶️ Rebuilds the fixture the flow host would normalize `before` to, then diffs `target` against
/// that baseline — the B1 free-function replacement for `Procedural3dPlayApp::commit_fixture`.
fn commit_fixture(before: &FlowFixture, target: &FlowFixture) -> Vec<Procedural3dOperation> {
    let baseline = host_from_fixture(before).fixture;
    procedural3d_fixture_operations(&baseline, target)
}

/// 🧭️ Runs a gumball transform (translate/rotate/scale) as a fixture operation, splicing transform
/// neurons via `ensure_gumball_node` and re-selecting the resulting transform widgets. `None` when no
/// transform actually changed anything (nothing to commit).
fn gumball_transform(fixture: &FlowFixture, ids: &[String], operation: &str, apply: impl Fn(&mut FlowHost, &str) -> bool) -> Option<(Vec<Procedural3dOperation>, Vec<String>)> {
    let mut host = host_from_fixture(fixture);
    let mut new_selection = Vec::new();
    let mut changed = false;
    for id in ids {
        if let Ok(transform_id) = ensure_gumball_node(&mut host, id, operation) {
            if apply(&mut host, &transform_id) {
                new_selection.push(transform_id);
                changed = true;
            }
        }
    }
    if changed {
        Some((commit_fixture(fixture, &host.fixture), new_selection))
    } else {
        None
    }
}

/// 🧬️ Emits generation operations for the generate-mode document-mutating commands — reuses
/// `playbook::generation_operations`'s id-generation/values-seeding logic via a synthetic JSON args
/// value built from the typed command fields (mirrors `apply_world3d_sun_action`'s call shape below).
/// `selectGeneration` (a config-only view command) is handled separately in `handle`, never here.
fn handle_generation(action: &str, args: Option<&Value>, projection: &Procedural3dDocument, cfg: &Procedural3dConfig) -> Emit<Procedural3dOperation, Procedural3dConfigOperation> {
    let spec = flow_fixture_to_form_spec(&projection.fixture);
    let mut state = projection.generation.clone();
    state.selected_generation_id = cfg.selected_generation_id.clone();
    let Some(operations) = generation_operations(action, args, &state, &spec) else {
        return Ok(Emit::default();
    };
    for operation in &operations {
        apply_generation_operation(&mut state, operation);
    }
    let generation_preview_text = selected_generation(&state).map(|selected| evaluate_generation_preview(&projection.fixture, &selected.values));
    let coalesce_key = (action == "updateGenerationValues").then(|| "generation-values".to_string());
    Ok(Emit {
        document_operations: operations.into_iter().map(Procedural3dOperation::Generation).collect(),
        config_operations: vec![Procedural3dConfigOperation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }],
        coalesce_key,
        ..Default::default()
    })
}

/// 🎚️ Level-of-detail tessellation deflection for the flow window.
fn procedural3d_lod_measure(lod_mode: &str) -> WindowMeasure {
    let current = if lod_mode.is_empty() { "medium" } else { lod_mode };
    WindowMeasure::Select {
        id: "procedural3d-measure-lod".into(),
        label: Some("LOD".into()),
        value: current.into(),
        items: vec![
            MeasureSelectItem { id: "procedural3d-measure-lod-coarse".into(), value: "coarse".into(), label: "Coarse".into() },
            MeasureSelectItem { id: "procedural3d-measure-lod-medium".into(), value: "medium".into(), label: "Medium".into() },
            MeasureSelectItem { id: "procedural3d-measure-lod-fine".into(), value: "fine".into(), label: "Fine".into() },
        ],
        on_change: procedural_action("setLodMode", None),
    }
}

/// 👁️ Preview shading mode for the world-3d window.
fn procedural3d_show_mode_measure(show_mode: &str) -> WindowMeasure {
    let current = if show_mode.is_empty() { "shaded" } else { show_mode };
    WindowMeasure::Select {
        id: "procedural3d-measure-show".into(),
        label: Some("Show".into()),
        value: current.into(),
        items: vec![
            MeasureSelectItem { id: "procedural3d-measure-show-shaded".into(), value: "shaded".into(), label: "Shaded".into() },
            MeasureSelectItem { id: "procedural3d-measure-show-edges".into(), value: "shaded+edges".into(), label: "Shaded + edges".into() },
            MeasureSelectItem { id: "procedural3d-measure-show-wireframe".into(), value: "wireframe".into(), label: "Wireframe".into() },
            MeasureSelectItem { id: "procedural3d-measure-show-points".into(), value: "points".into(), label: "Points".into() },
        ],
        on_change: procedural_action("setShowMode", None),
    }
}
fn config_after_example_load(previous: &Procedural3dConfig, flow_camera: &CameraJson) -> Procedural3dConfig {
    Procedural3dConfig {
        camera: flow_camera.clone(),
        selected_node_ids: Vec::new(),
        hovered_node_id: None,
        eval_driver_json: String::new(),
        selected_generation_id: None,
        generation_preview_text: None,
        preview_camera: previous.preview_camera.clone(),
        lod_mode: previous.lod_mode.clone(),
        show_mode: previous.show_mode.clone(),
        selection_method: previous.selection_method.clone(),
        sun_json: previous.sun_json.clone(),
        active_utility_id: previous.active_utility_id.clone(),
        locale: previous.locale.clone(),
        contributions_json: previous.contributions_json.clone(),
    }
}

fn parse_flow_camera_json(args: &Value) -> CameraJson {
    if let Some(camera) = args.get("camera") {
        if let Ok(parsed) = serde_json::from_value::<CameraJson>(camera.clone()) {
            return parsed;
        }
    }
    CameraJson {
        x: args.get("x").and_then(Value::as_f64).unwrap_or(0.0),
        y: args.get("y").and_then(Value::as_f64).unwrap_or(0.0),
        zoom: args.get("zoom").and_then(Value::as_f64).unwrap_or(1.0),
    }
}

fn parse_preview_camera_json(args: &Value) -> Procedural3dPreviewCamera {
    if let Some(camera) = args.get("camera") {
        if let Ok(parsed) = serde_json::from_value::<Procedural3dPreviewCamera>(camera.clone()) {
            return parsed;
        }
    }
    Procedural3dPreviewCamera::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the 3D flow app; one field per label makes every locale combination compile-checked.
    struct Procedural3dLabels {
        widgets: native_en "Widgets", native_de "Elemente", reuse_en "Widgets", reuse_de "Elemente";
        schema_prefix: native_en "Schema:", native_de "Schema:", reuse_en "Schema:", reuse_de "Schema:";
        widgets_prefix: native_en "Widgets:", native_de "Elemente:", reuse_en "Widgets:", reuse_de "Elemente:";
        no_selection: native_en "No selection", native_de "Keine Auswahl", reuse_en "No selection", reuse_de "Keine Auswahl";
        id_field: native_en "Id", native_de "ID", reuse_en "Id", reuse_de "ID";
        value_field: native_en "Value", native_de "Wert", reuse_en "Value", reuse_de "Wert";
        range_field: native_en "Range", native_de "Bereich", reuse_en "Range", reuse_de "Bereich";
        widget_group: native_en "Widget", native_de "Element", reuse_en "Widget", reuse_de "Element";
        generate_hint: native_en "Add a generation to edit input values.", native_de "Erstelle eine Generation, um Eingabewerte zu bearbeiten.", reuse_en "Add a generation to edit input values.", reuse_de "Erstelle eine Generation, um Eingabewerte zu bearbeiten.";
        preview_hint: native_en "(evaluate a generation to preview output)", native_de "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)", reuse_en "(evaluate a generation to preview output)", reuse_de "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)";
        catalog_neuron: native_en "Neuron", native_de "Neuron", reuse_en "Neuron", reuse_de "Neuron";
        catalog_slider: native_en "Slider", native_de "Schieberegler", reuse_en "Slider", reuse_de "Schieberegler";
        catalog_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        catalog_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        window_flow: native_en "Flow", native_de "Workflow", reuse_en "Flow", reuse_de "Workflow";
        window_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        window_generations: native_en "Generations", native_de "Generationen", reuse_en "Generations", reuse_de "Generationen";
        window_generate_form: native_en "Form", native_de "Formular", reuse_en "Form", reuse_de "Formular";
        window_generate_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        delete_selection: native_en "Delete selection", native_de "Auswahl löschen", reuse_en "Delete selection", reuse_de "Auswahl löschen";
    }
}

/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
fn procedural3d_labels(cfg: &Procedural3dConfig) -> &'static Procedural3dLabels {
    semio_framework_plugin::resolve_labels_for_locale::<Procedural3dLabels>(&cfg.locale)
}

/// 🗣️ Resolves a catalogue widget kind's display label from its stable id; unknown kinds fall back to the id itself.
fn procedural3d_catalog_label(kind: &'static str, labels: &Procedural3dLabels) -> &'static str {
    match kind {
        "neuron" => labels.catalog_neuron.as_str(),
        "inputSlider" => labels.catalog_slider.as_str(),
        "inputNote" => labels.catalog_note.as_str(),
        "outputPreview" => labels.catalog_preview.as_str(),
        _ => kind,
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
/// 🌳️ SDK's `tree_item_with_action` plus an icon id — this crate's document/catalogue trees carry
/// icons per item, which the shared helper doesn't model directly.
fn tree_item_with_icon(id: impl Into<String>, label: impl Into<Label>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: icon_id.map(Into::into), menu: None, ..tree_item_with_action(id, label, None, action) }
}

fn build_document_tree(fixture: &FlowFixture, selected_node_ids: &[String], labels: &Procedural3dLabels) -> UiNode {
    let items: Vec<UiTreeItemNode> = fixture
        .widgets
        .iter()
        .map(|widget| {
            let id = widget_id(widget).to_string();
            tree_item_with_icon(format!("procedural-widget:{id}"), Label::data(id.clone()), Some("cpu"), procedural_action("setSelection", Some(json!({ "ids": [id] }))))
        })
        .collect();
    PanelTreeBuilder::new("procedural-play-document").section("procedural-play-document.widgets", Some(labels.widgets.into()), true, items).selected(selected_node_ids.iter().map(|id| format!("procedural-widget:{id}")).collect()).build()
}

fn build_catalogue_tree(labels: &Procedural3dLabels) -> UiNode {
    let sections = flow_core::flow_palette_catalogue_sections();
    let items: Vec<UiTreeItemNode> = sections
        .iter()
        .flat_map(|section| section.items.iter().map(|item| {
            let action_kind = if item.kind == "neuron" {
                format!("neuron|{}", item.neuron_kind.as_deref().unwrap_or("math.add"))
            } else {
                item.kind.clone()
            };
            let icon = if item.icon.starts_with("emoji:") { "box" } else { item.icon.as_str() };
            tree_item_with_icon(
                format!("procedural-play-catalogue.{}", item.neuron_kind.as_deref().unwrap_or(&item.kind)),
                Label::data(item.name.clone()),
                Some(icon),
                procedural_action("addWidget", Some(json!({ "kind": action_kind }))),
            )
        }))
        .collect();
    PanelTreeBuilder::new("procedural-play-catalogue").section("procedural-play-catalogue.widgets", Some(labels.widgets.into()), true, items).build()
}

fn build_inspector_tree(fixture: &FlowFixture, selected_node_ids: &[String], labels: &Procedural3dLabels) -> UiNode {
    let Some(selected_id) = selected_node_ids.first() else {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "procedural-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(Label::data(format!("{} {}", labels.schema_prefix.as_str(), fixture.schema))), ui_text(Label::data(format!("{} {}", labels.widgets_prefix.as_str(), fixture.widgets.len())))],
            presence: UiPresence::default(),
            menu: None,
        }]);
    };
    let Some(widget) = fixture.widgets.iter().find(|entry| widget_id(entry) == selected_id) else {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "procedural-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(labels.no_selection)],
            presence: UiPresence::default(),
            menu: None,
        }]);
    };
    let mut fields = vec![ui_inspector_readonly_field("procedural-play-inspector.id", labels.id_field, widget_id(widget))];
    if let Widget::InputSlider { value, min, max, .. } = widget {
        let mixed = ui_inspector_mixed_number(&[*value]);
        fields.push(UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "procedural-play-inspector.value".into(),
            label: labels.value_field.into(),
            child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                presence: UiPresence::default(),
                id: "procedural-play-inspector.value.input".into(),
                input_kind: "number".into(),
                value: mixed.value.to_string(),
                placeholder: None,
                commit: None,
                on_change: procedural_action("patchFlowWidgets", Some(json!({ "widgetIds": [selected_id], "field": "value" }))),
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
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.range", labels.range_field, &format!("{min}..{max}")));
    }
    if let Widget::InputNote { text, .. } = widget {
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.note", labels.value_field, text));
    }
    if let Widget::Neuron { neuron_kind, .. } = widget {
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.neuron-kind", labels.id_field, neuron_kind));
    }
    if let Widget::Variable { name, schema, .. } = widget {
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.variable-name", labels.value_field, name));
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.variable-schema", labels.range_field, schema));
    }
    if let Widget::OutputAction { action, .. } = widget {
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.action", labels.value_field, action));
    }
    if let Widget::OutputExport { format, .. } = widget {
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.export-format", labels.value_field, format));
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(), id: "procedural-play-inspector.widget".into(), label: labels.widget_group.into(), default_open: None, fields }])
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_generate_generations(generation: &GenerationPlayState, locale: Locale, terminology: Terminology) -> UiNode {
    render_generations_tree(PROCEDURAL_3D_PLAY_APP_ID, "procedural3d-play-generate", &generation.generations, generation.selected_generation_id.as_deref(), locale, terminology)
}

fn render_generate_form(fixture: &FlowFixture, generation: &GenerationPlayState, labels: &Procedural3dLabels) -> UiNode {
    let spec = flow_fixture_to_form_spec(fixture);
    let Some(current) = selected_generation(generation) else {
        return ui_text(labels.generate_hint);
    };
    render_generation_form_body(&spec, &current.values, PROCEDURAL_3D_PLAY_APP_ID, "updateGenerationValues", &current.id)
}

fn render_generate_preview(fixture: &FlowFixture, generation: &GenerationPlayState, cfg: &Procedural3dConfig, labels: &Procedural3dLabels, active_utility: &str) -> UiNode {
    let (meshes_json, instances_json) = match selected_generation(generation) {
        Some(_) => {
            let gen_fixture = generation_fixture_for(fixture, generation);
            let eval_json = generation.preview_text.clone().unwrap_or_default();
            procedural_3d_engine::preview_payload_from_eval(&eval_json, &gen_fixture, cfg)
        }
        None => ("[]".into(), "[]".into()),
    };
    if meshes_json == "[]" && instances_json == "[]" {
        let text = generation.preview_text.as_deref().filter(|value| !value.is_empty()).unwrap_or(labels.preview_hint.as_str());
        return render_generation_preview_text(PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW, PROCEDURAL_3D_PLAY_APP_ID, text);
    }
    let sun = cfg.sun();
    build_world_3d_scene(
        PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW,
        PROCEDURAL_3D_PLAY_APP_ID,
        world3d_scene(procedural_3d_engine::preview_camera_json(cfg), meshes_json, instances_json, procedural_3d_engine::preview_selection_json(cfg, active_utility), &sun),
    )
}
//#endregion 🔖️Render

//#region 🔖️Procedural3dPlayApp
/// 🧪️ B1: unit struct — every former `Procedural3dRuntime`/`self.runtime` field now lives in
/// `procedural_3d_engine::Procedural3dConfig` (see `DocumentApp::Config`), written through
/// `procedural_3d_op::Procedural3dConfigOperation`s.
#[derive(Default)]
pub struct Procedural3dPlayApp;

impl DocumentApp for Procedural3dPlayApp {
    type Projection = Procedural3dDocument;
    type Operation = Procedural3dOperation;
    type Config = Procedural3dConfig;
    type ConfigOperation = Procedural3dConfigOperation;
    type Command = Procedural3dCommand;

    fn app_id(&self) -> &str {
        PROCEDURAL_3D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PROCEDURAL_3D_SCHEMA
    }

    fn initial_projection(&self) -> Procedural3dDocument {
        default_projection()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(procedural_3d_engine::procedural3d_io())
    }

    /// 🎞️ `geometry:out` (see `procedural_3d_engine::export_mesh_from_document`) plus the inherited
    /// `document:out` default (the pack of `doc.projection`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, Procedural3dDocument>) -> Result<Media, MediaError> {
        match port {
            "geometry:out" => {
                let mesh = procedural_3d_engine::export_mesh_from_document(doc.projection);
                Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "3d.mesh".into(), json: serde_json::to_string(&mesh).unwrap_or_default() } })
            }
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `params:in` — patches matching `InputSlider` widgets from a `{widgetId: number}` JSON object;
    /// unmatched keys/non-slider widgets are silently ignored. Every other port (including
    /// `document:in`, since `Procedural3dOperation` has no whole-document-replace variant today —
    /// see `whole_document_operation`'s default) is `NotImplemented`.
    fn import_media(&self, port: &str, media: &Media, doc: &DocumentView<'_, Procedural3dDocument>) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, MediaError> {
        match port {
            "params:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "params:in importer only accepts a Structured JSON object payload".into()));
                };
                let object: serde_json::Map<String, Value> = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let fixture = &doc.projection.fixture;
                let mut operations = Vec::new();
                for (target_id, value) in &object {
                    let Some(number) = value.as_f64() else { continue };
                    let Some((index, widget)) = fixture.widgets.iter().enumerate().find(|(_, widget)| widget_id(widget) == target_id) else { continue };
                    if let Widget::InputSlider { id, min, max, step, .. } = widget {
                        operations.push(Procedural3dOperation::SetWidget { index, widget: Widget::InputSlider { id: id.clone(), value: number, min: *min, max: *max, step: *step } });
                    }
                }
                Ok(Emit::operations(operations))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `Procedural3dCommand` variant back to the action id it was declared under in
    /// `create_procedural3d_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &Procedural3dCommand) -> &str {
        match command {
            Procedural3dCommand::SetActiveExample { .. } => "setActiveExample",
            Procedural3dCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            Procedural3dCommand::DeleteSelection => "deleteSelection",
            Procedural3dCommand::RemoveWidget { .. } => "removeWidget",
            Procedural3dCommand::MoveMediaNode { .. } => "moveMediaNode",
            Procedural3dCommand::AddWidget { .. } => "addWidget",
            Procedural3dCommand::PatchFlowWidgets { .. } => "patchFlowWidgets",
            Procedural3dCommand::Reorganize => "reorganize",
            Procedural3dCommand::TranslateSelection { .. } => "translateSelection",
            Procedural3dCommand::RotateSelection { .. } => "rotateSelection",
            Procedural3dCommand::ScaleSelection { .. } => "scaleSelection",
            Procedural3dCommand::AddGeneration => "addGeneration",
            Procedural3dCommand::RemoveGeneration { .. } => "removeGeneration",
            Procedural3dCommand::RenameGeneration { .. } => "renameGeneration",
            Procedural3dCommand::UpdateGenerationValues { .. } => "updateGenerationValues",
            Procedural3dCommand::NodeGraphViewport { .. } => "nodeGraphViewport",
            Procedural3dCommand::SetSelection { .. } => "setSelection",
            Procedural3dCommand::SelectNode { .. } => "selectNode",
            Procedural3dCommand::NodeGraphSelect { .. } => "nodeGraphSelect",
            Procedural3dCommand::NodeGraphHover { .. } => "nodeGraphHover",
            Procedural3dCommand::SetHover { .. } => "setHover",
            Procedural3dCommand::WorldPointerDown => "worldPointerDown",
            Procedural3dCommand::GraphPointerDown => "graphPointerDown",
            Procedural3dCommand::WorldSelect { .. } => "worldSelect",
            Procedural3dCommand::WorldHover { .. } => "worldHover",
            Procedural3dCommand::SetSelectionMethod { .. } => "setSelectionMethod",
            Procedural3dCommand::SetLodMode { .. } => "setLodMode",
            Procedural3dCommand::SetShowMode { .. } => "setShowMode",
            Procedural3dCommand::ToggleSun => "toggleSun",
            Procedural3dCommand::SetSunAzimuth { .. } => "setSunAzimuth",
            Procedural3dCommand::SetSunElevation { .. } => "setSunElevation",
            Procedural3dCommand::SetSunIntensity { .. } => "setSunIntensity",
            Procedural3dCommand::SetCamera { .. } => "setCamera",
            Procedural3dCommand::SelectGeneration { .. } => "selectGeneration",
            Procedural3dCommand::SetActiveUtility { .. } => SET_ACTIVE_UTILITY_ACTION_ID,
            Procedural3dCommand::SetLocale { .. } => "setLocale",
            Procedural3dCommand::SetContributions { .. } => "setContributions",
            Procedural3dCommand::FlowEvalTick => "flowEvalTick",
            Procedural3dCommand::FlowEvalResolve { .. } => "flowEvalResolve",
        }
    }

    /// 🎯️ Maps host action id + JSON args onto `Procedural3dCommand` — React/wgpu still speak the
    /// stringly `{action,args}` wire until those call sites send `OpBinary` bytes directly.
    fn command_from_action(&self, action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(Value::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        let object_args = args.as_object().cloned().unwrap_or_default();
        match action {
            "setActiveExample" => Ok(Procedural3dCommand::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() }),
            "nodeGraphEdit" => Ok(Procedural3dCommand::NodeGraphEdit {
                operations_json: str_arg(&["operationsJson", "operations_json"]).or_else(|| args.get("operations").map(|value| value.to_string())).unwrap_or_else(|| "[]".into()),
            }),
            "deleteSelection" => Ok(Procedural3dCommand::DeleteSelection),
            "removeWidget" => Ok(Procedural3dCommand::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() }),
            "moveMediaNode" => Ok(Procedural3dCommand::MoveMediaNode {
                node_id: str_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(),
                x: f64_arg(&["x"]).unwrap_or(0.0),
                y: f64_arg(&["y"]).unwrap_or(0.0),
            }),
            "addWidget" => Ok(Procedural3dCommand::AddWidget {
                kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()),
                x: f64_arg(&["x"]),
                y: f64_arg(&["y"]),
            }),
            "patchFlowWidgets" => Ok(Procedural3dCommand::PatchFlowWidgets {
                widget_ids: {
                    let mut ids = string_list("widgetIds");
                    if ids.is_empty() {
                        ids = string_list("widget_ids");
                    }
                    ids
                },
                field: str_arg(&["field"]).unwrap_or_default(),
                value: f64_arg(&["value"]),
            }),
            "reorganize" => Ok(Procedural3dCommand::Reorganize),
            "translateSelection" => Ok(Procedural3dCommand::TranslateSelection {
                node_ids: {
                    let mut ids = string_list("nodeIds");
                    if ids.is_empty() {
                        ids = string_list("node_ids");
                    }
                    if ids.is_empty() {
                        ids = string_list("ids");
                    }
                    ids
                },
                dx: f64_arg(&["dx"]).unwrap_or(0.0),
                dy: f64_arg(&["dy"]).unwrap_or(0.0),
                dz: f64_arg(&["dz"]).unwrap_or(0.0),
            }),
            "rotateSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Procedural3dCommand::RotateSelection {
                    node_ids,
                    ax: f64_arg(&["ax"]).unwrap_or(0.0),
                    ay: f64_arg(&["ay"]).unwrap_or(0.0),
                    az: f64_arg(&["az"]).unwrap_or(0.0),
                    angle: f64_arg(&["angle"]).unwrap_or(0.0),
                })
            }
            "scaleSelection" => {
                let mut node_ids = string_list("nodeIds");
                if node_ids.is_empty() {
                    node_ids = string_list("node_ids");
                }
                if node_ids.is_empty() {
                    node_ids = string_list("ids");
                }
                Ok(Procedural3dCommand::ScaleSelection {
                    node_ids,
                    sx: f64_arg(&["sx"]).unwrap_or(1.0),
                    sy: f64_arg(&["sy"]).unwrap_or(1.0),
                    sz: f64_arg(&["sz"]).unwrap_or(1.0),
                })
            }
            "addGeneration" => Ok(Procedural3dCommand::AddGeneration),
            "removeGeneration" => Ok(Procedural3dCommand::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() }),
            "renameGeneration" => Ok(Procedural3dCommand::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name"]).unwrap_or_default() }),
            "updateGenerationValues" => {
                let value = args.get("value").map(|entry| dsl::to_dsl_value(entry).unwrap_or(dsl::DslValue::Null)).unwrap_or(dsl::DslValue::Null);
                Ok(Procedural3dCommand::UpdateGenerationValues {
                    generation_id: str_arg(&["generationId", "generation_id"]),
                    question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                    value,
                })
            }
            "nodeGraphViewport" => Ok(Procedural3dCommand::NodeGraphViewport { camera: parse_flow_camera_json(&args) }),
            "setSelection" => Ok(Procedural3dCommand::SetSelection { node_ids: string_list("ids") }),
            "selectNode" => Ok(Procedural3dCommand::SelectNode { node_ids: string_list("ids").into_iter().chain(string_list("nodeIds")).collect() }),
            "nodeGraphSelect" => Ok(Procedural3dCommand::NodeGraphSelect { node_ids: string_list("ids").into_iter().chain(string_list("nodeIds")).collect() }),
            "nodeGraphHover" => Ok(Procedural3dCommand::NodeGraphHover { widget_id: str_arg(&["widgetId", "widget_id"]) }),
            "setHover" => Ok(Procedural3dCommand::SetHover { object_id: str_arg(&["objectId", "object_id", "id"]) }),
            "worldPointerDown" => Ok(Procedural3dCommand::WorldPointerDown),
            "graphPointerDown" => Ok(Procedural3dCommand::GraphPointerDown),
            "worldSelect" => Ok(Procedural3dCommand::WorldSelect { ids: string_list("ids"), merge: str_arg(&["merge"]).unwrap_or_else(|| "replace".into()) }),
            "worldHover" => Ok(Procedural3dCommand::WorldHover { id: str_arg(&["id", "objectId", "object_id"]) }),
            "setSelectionMethod" => Ok(Procedural3dCommand::SetSelectionMethod { method: str_arg(&["value", "method", "selectionMethod"]).unwrap_or_default() }),
            "setLodMode" => Ok(Procedural3dCommand::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() }),
            "setShowMode" => Ok(Procedural3dCommand::SetShowMode { value: str_arg(&["value", "showMode", "show_mode"]).unwrap_or_default() }),
            "toggleSun" => Ok(Procedural3dCommand::ToggleSun),
            "setSunAzimuth" => Ok(Procedural3dCommand::SetSunAzimuth { value: f64_arg(&["value"]).unwrap_or(0.0) }),
            "setSunElevation" => Ok(Procedural3dCommand::SetSunElevation { value: f64_arg(&["value"]).unwrap_or(0.0) }),
            "setSunIntensity" => Ok(Procedural3dCommand::SetSunIntensity { value: f64_arg(&["value"]).unwrap_or(1.0) }),
            "setCamera" => Ok(Procedural3dCommand::SetCamera { camera: parse_preview_camera_json(&args) }),
            "selectGeneration" => Ok(Procedural3dCommand::SelectGeneration { id: str_arg(&["id"]).unwrap_or_default() }),
            SET_ACTIVE_UTILITY_ACTION_ID => Ok(Procedural3dCommand::SetActiveUtility { utility_id: str_arg(&["utilityId", "utility_id"]).unwrap_or_default() }),
            "setLocale" => Ok(Procedural3dCommand::SetLocale { value: str_arg(&["value", "locale"]).unwrap_or_default() }),
            "setContributions" => Ok(Procedural3dCommand::SetContributions {
                json: str_arg(&["json", "contributionsJson", "contributions_json"]).or_else(|| args.get("contributions").map(|value| value.to_string())).unwrap_or_else(|| "[]".into()),
            }),
            "flowEvalTick" => Ok(Procedural3dCommand::FlowEvalTick),
            "flowEvalResolve" => Ok(Procedural3dCommand::FlowEvalResolve {
                node_hash: args.get("nodeHash").or_else(|| args.get("node_hash")).and_then(Value::as_u64).unwrap_or(0),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_else(|| "{}".into()),
            }),
            other => {
                let _ = object_args;
                Err(format!(
                    "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                     app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
                ).into())
            }
        }
    }

    fn handle(&self, command: &Procedural3dCommand, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let fixture = &doc.projection.fixture;
        let config = cfg.projection;
        match command {
            // ✏️ Operations — compute the target fixture via the host, emit fixture operations.
            Procedural3dCommand::SetActiveExample { example_id } => {
                let target = if example_id.is_empty() {
                    default_projection()
                } else if is_procedural3d_example_id(example_id) {
                    example_projection(example_id).unwrap_or_default()
                } else {
                    return Ok(Emit::default();
                };
                let mut operations: Vec<Procedural3dOperation> = doc.projection.generation.generations.iter().map(|generation| Procedural3dOperation::Generation(GenerationOperation::Remove { id: generation.id.clone() })).collect();
                operations.extend(procedural3d_fixture_operations(fixture, &target.fixture));
                let camera = target.fixture.camera.clone();
                Ok(Emit {
                    document_operations: operations,
                    config_operations: vec![Procedural3dConfigOperation::Snapshot { config: config_after_example_load(config, &camera) }],
                    ..Default::default()
                })
            }
            Procedural3dCommand::NodeGraphEdit { operations_json } => {
                let sub_operations: Vec<Value> = serde_json::from_str(operations_json).unwrap_or_default();
                let selected = config.selected_node_ids.clone();
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
                let operations = commit_fixture(fixture, &host.fixture);
                let config_operations = if cleared { vec![Procedural3dConfigOperation::SetSelection { node_ids: Vec::new() }] } else { Vec::new() };
                Emit { document_operations: operations, config_operations, ..Default::default() }
            }
            Procedural3dCommand::DeleteSelection => {
                let selected = config.selected_node_ids.clone();
                let mut host = host_from_fixture(fixture);
                let mut cleared = false;
                for id in &selected {
                    if host.remove_widget(id).is_ok() {
                        cleared = true;
                    }
                }
                let operations = commit_fixture(fixture, &host.fixture);
                let config_operations = if cleared { vec![Procedural3dConfigOperation::SetSelection { node_ids: Vec::new() }] } else { Vec::new() };
                Emit { document_operations: operations, config_operations, ..Default::default() }
            }
            Procedural3dCommand::RemoveWidget { widget_id: target_id } => {
                let mut host = host_from_fixture(fixture);
                if host.remove_widget(target_id).is_ok() {
                    let operations = commit_fixture(fixture, &host.fixture);
                    let mut remaining = config.selected_node_ids.clone();
                    remaining.retain(|id| id != target_id);
                    Emit { document_operations: operations, config_operations: vec![Procedural3dConfigOperation::SetSelection { node_ids: remaining }], ..Default::default() }
                } else {
                    Ok(Emit::default()
                }
            }
            Procedural3dCommand::MoveMediaNode { node_id, x, y } => {
                let mut host = host_from_fixture(fixture);
                if host.move_widget(node_id, *x, *y).is_ok() {
                    Ok(Emit::operations(commit_fixture(fixture, &host.fixture))
                } else {
                    Ok(Emit::default()
                }
            }
            Procedural3dCommand::AddWidget { kind, x, y } => {
                let descriptor = if let Some((base, neuron)) = kind.split_once('|') {
                    if base == "neuron" {
                        json!({ "kind": "neuron", "neuronKind": neuron }).to_string()
                    } else {
                        json!({ "kind": kind }).to_string()
                    }
                } else {
                    json!({ "kind": kind }).to_string()
                };
                let x = x.unwrap_or(120.0);
                let y = y.unwrap_or(120.0);
                let mut host = host_from_fixture(fixture);
                if let Ok(id) = host.add_widget(&descriptor, x, y) {
                    let operations = commit_fixture(fixture, &host.fixture);
                    Emit { document_operations: operations, config_operations: vec![Procedural3dConfigOperation::SetSelection { node_ids: vec![id] }], ..Default::default() }
                } else {
                    Ok(Emit::default()
                }
            }
            Procedural3dCommand::PatchFlowWidgets { widget_ids, field, value } => {
                let mut host = host_from_fixture(fixture);
                let baseline = host.fixture.clone();
                for widget in host.fixture.widgets.iter_mut() {
                    if !widget_ids.contains(&widget_id(widget).to_string()) {
                        continue;
                    }
                    if let (Widget::InputSlider { value: slider_value, .. }, Some(new_value)) = (widget, value) {
                        if field == "value" {
                            *slider_value = *new_value;
                        }
                    }
                }
                Ok(Emit::operations(procedural3d_fixture_operations(&baseline, &host.fixture))
            }
            Procedural3dCommand::Reorganize => {
                let mut host = host_from_fixture(fixture);
                if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
                    Ok(Emit::operations(commit_fixture(fixture, &host.fixture))
                } else {
                    Ok(Emit::default()
                }
            }
            Procedural3dCommand::TranslateSelection { node_ids, dx, dy, dz } => {
                let ids = mesh_selection_ids_typed(node_ids, &config.selected_node_ids);
                let (dx, dy, dz) = (*dx, *dy, *dz);
                match gumball_transform(fixture, &ids, "translate", move |host, transform_id| {
                    let current = gumball_widget_offset(host, transform_id);
                    let next = [current[0] + dx, current[1] + dy, current[2] + dz];
                    host.set_neuron_params(transform_id, &gumball_translate_params_json(next)).is_ok()
                }) {
                    Some((operations, new_selection)) => {
                        Emit { document_operations: operations, config_operations: vec![Procedural3dConfigOperation::SetSelection { node_ids: new_selection }], coalesce_key: Some("gumball-translate".into()), ..Default::default() }
                    }
                    None => Ok(Emit::default()),
                }
            }
            Procedural3dCommand::RotateSelection { node_ids, ax, ay, az, angle } => {
                let ids = mesh_selection_ids_typed(node_ids, &config.selected_node_ids);
                let (ax, ay, az, angle) = (*ax, *ay, *az, *angle);
                match gumball_transform(fixture, &ids, "rotate", move |host, transform_id| {
                    let current_angle = gumball_widget_number_param(host, transform_id, "angle", 0.0);
                    host.set_neuron_params(transform_id, &gumball_rotate_params_json([ax, ay, az], current_angle + angle)).is_ok()
                }) {
                    Some((operations, new_selection)) => {
                        Emit { document_operations: operations, config_operations: vec![Procedural3dConfigOperation::SetSelection { node_ids: new_selection }], coalesce_key: Some("gumball-rotate".into()), ..Default::default() }
                    }
                    None => Ok(Emit::default()),
                }
            }
            Procedural3dCommand::ScaleSelection { node_ids, sx, sy, sz } => {
                let ids = mesh_selection_ids_typed(node_ids, &config.selected_node_ids);
                let uniform_factor = (sx + sy + sz) / 3.0;
                match gumball_transform(fixture, &ids, "scale", move |host, transform_id| {
                    let current_factor = gumball_widget_number_param(host, transform_id, "factor", 1.0);
                    host.set_neuron_params(transform_id, &gumball_scale_params_json(current_factor * uniform_factor)).is_ok()
                }) {
                    Some((operations, new_selection)) => {
                        Emit { document_operations: operations, config_operations: vec![Procedural3dConfigOperation::SetSelection { node_ids: new_selection }], coalesce_key: Some("gumball-scale".into()), ..Default::default() }
                    }
                    None => Ok(Emit::default()),
                }
            }
            Procedural3dCommand::AddGeneration => Ok(handle_generation("addGeneration", None, doc.projection, config),
            Procedural3dCommand::RemoveGeneration { id } => Ok(handle_generation("removeGeneration", Some(&json!({ "id": id })), doc.projection, config),
            Procedural3dCommand::RenameGeneration { id, name } => Ok(handle_generation("renameGeneration", Some(&json!({ "id": id, "name": name })), doc.projection, config),
            Procedural3dCommand::UpdateGenerationValues { generation_id, question_id, value } => {
                let value_json = dsl::from_dsl_value(value.clone()).unwrap_or(Value::Null);
                handle_generation("updateGenerationValues", Some(&json!({ "generationId": generation_id, "questionId": question_id, "value": value_json })), doc.projection, config)
            }

            // 👁️ Config-only — mutate ephemeral config, emit no document operations.
            Procedural3dCommand::NodeGraphViewport { camera } => Ok(Emit::config(vec![Procedural3dConfigOperation::SetCamera { camera: camera.clone() }])),
            Procedural3dCommand::SetSelection { node_ids } | Procedural3dCommand::SelectNode { node_ids } | Procedural3dCommand::NodeGraphSelect { node_ids } => {
                Ok(Emit::config(vec![Procedural3dConfigOperation::SetSelection { node_ids: node_ids.clone() }])
            }
            Procedural3dCommand::NodeGraphHover { widget_id } => Ok(Emit::config(vec![Procedural3dConfigOperation::SetHover { node_id: widget_id.clone() }])),
            Procedural3dCommand::SetHover { object_id } => Ok(Emit::config(vec![Procedural3dConfigOperation::SetHover { node_id: object_id.clone() }])),
            Procedural3dCommand::WorldPointerDown | Procedural3dCommand::GraphPointerDown => Ok(Emit::default()),
            Procedural3dCommand::WorldSelect { ids, merge } => {
                let mapped: Vec<String> = ids.iter().map(|id| widget_id_from_instance_id(id).to_string()).collect();
                let merged = merge_world_selection_ids(&SelectionSet::from_ids(config.selected_node_ids.clone()), &mapped, merge).to_vec();
                Ok(Emit::config(vec![Procedural3dConfigOperation::SetSelection { node_ids: merged }])
            }
            Procedural3dCommand::WorldHover { id } => {
                let resolved = id.as_deref().map(|id| widget_id_from_instance_id(id).to_string());
                Ok(Emit::config(vec![Procedural3dConfigOperation::SetHover { node_id: resolved }])
            }
            Procedural3dCommand::SetSelectionMethod { method } => Ok(Emit::config(vec![Procedural3dConfigOperation::SetSelectionMethod { method: method.clone() }])),
            Procedural3dCommand::SetLodMode { value } => Ok(Emit::config(vec![Procedural3dConfigOperation::SetLodMode { value: value.clone() }])),
            Procedural3dCommand::SetShowMode { value } => Ok(Emit::config(vec![Procedural3dConfigOperation::SetShowMode { value: value.clone() }])),
            Procedural3dCommand::ToggleSun => {
                let mut sun = config.sun();
                apply_world3d_sun_action(&mut sun, "toggleSun", None);
                Ok(Emit::config(vec![Procedural3dConfigOperation::SetSun { json: serde_json::to_string(&sun).unwrap_or_default() }])
            }
            Procedural3dCommand::SetSunAzimuth { value } => {
                let mut sun = config.sun();
                apply_world3d_sun_action(&mut sun, "setSunAzimuth", Some(&json!({ "value": value })));
                Ok(Emit::config(vec![Procedural3dConfigOperation::SetSun { json: serde_json::to_string(&sun).unwrap_or_default() }])
            }
            Procedural3dCommand::SetSunElevation { value } => {
                let mut sun = config.sun();
                apply_world3d_sun_action(&mut sun, "setSunElevation", Some(&json!({ "value": value })));
                Ok(Emit::config(vec![Procedural3dConfigOperation::SetSun { json: serde_json::to_string(&sun).unwrap_or_default() }])
            }
            Procedural3dCommand::SetSunIntensity { value } => {
                let mut sun = config.sun();
                apply_world3d_sun_action(&mut sun, "setSunIntensity", Some(&json!({ "value": value })));
                Ok(Emit::config(vec![Procedural3dConfigOperation::SetSun { json: serde_json::to_string(&sun).unwrap_or_default() }])
            }
            Procedural3dCommand::SetCamera { camera } => Ok(Emit::config(vec![Procedural3dConfigOperation::SetPreviewCamera { camera: camera.clone() }])),
            Procedural3dCommand::SelectGeneration { id } => {
                let mut state = doc.projection.generation.clone();
                state.selected_generation_id = config.selected_generation_id.clone();
                select_generation(&mut state, id);
                let generation_preview_text = selected_generation(&state).map(|selected| evaluate_generation_preview(fixture, &selected.values));
                Ok(Emit::config(vec![Procedural3dConfigOperation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }])
            }
            // 🧰️ Host-owned active-utility switch — clear in-progress hover scratch, never emit document operations.
            Procedural3dCommand::SetActiveUtility { utility_id } => Ok(Emit::config(vec![Procedural3dConfigOperation::SetActiveUtility { utility_id: utility_id.clone() }, Procedural3dConfigOperation::SetHover { node_id: None }])),
            Procedural3dCommand::SetLocale { value } => Ok(Emit::config(vec![Procedural3dConfigOperation::SetLocale { value: value.clone() }])),
            Procedural3dCommand::SetContributions { json } => Ok(Emit::config(vec![Procedural3dConfigOperation::SetContributions { json: json.clone() }])),
            // 🧵️ One budgeted evaluation step (see `FlowEvalDriver::tick`), off the main thread —
            // the plugin worker runs this, never the renderer. Chains itself via `HostEffect::DispatchAction`
            // until the fixture's dirty set is empty; persists the driver's new baseline/eval json via
            // `SetEvalDriver` so the next render/`pending_effects` call sees the converged state.
            Procedural3dCommand::FlowEvalTick => {
                let mut driver = config.eval_driver();
                let mut host = host_from_fixture_with_driver(fixture, Some(&driver));
                let more = driver.tick(&mut host);
                let mut effects = if more { vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() };
                if let Some(pending) = host.take_pending_extension_eval() {
                    if let Some(plugin_id) = flow_core::flow_extension_plugin_id(&pending.extension_id) {
                        let request_json = serde_json::json!({
                            "operatorId": pending.operator_id,
                            "inputJson": pending.input_json,
                            "nodeHash": pending.node_hash,
                        })
                        .to_string();
                        effects.push(HostEffect::RequestPluginExchange {
                            plugin_id,
                            app_id: "flow-extension-eval".into(),
                            request_json,
                            response_action: "flowEvalResolve".into(),
                        });
                    }
                }
                Ok(Emit {
                    config_operations: vec![Procedural3dConfigOperation::SetEvalDriver { json: serde_json::to_string(&driver).unwrap_or_default() }],
                    effects,
                    ..Default::default()
                })
            }
            Procedural3dCommand::FlowEvalResolve { node_hash, output_json } => {
                let _ = procedural_3d_engine::resolve_flow_eval_node(*node_hash, output_json);
                Emit { effects: vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }], ..Default::default() }
            }
        }
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes —
    /// covers every mutation path (edits, undo/redo, example load, remote operations) in one place.
    /// Pure: recomputes the "is anything pending" probe fresh from the fixture and the driver's
    /// persisted baseline each call, never mutates anything durably.
    fn pending_effects(&self, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>) -> Vec<HostEffect> {
        let mut driver = cfg.projection.eval_driver();
        let host = host_from_fixture_with_driver(&doc.projection.fixture, Some(&driver));
        if driver.sync(&host) {
            vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
        } else {
            Vec::new()
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>) -> UiNode {
        let fixture = &doc.projection.fixture;
        let config = cfg.projection;
        let labels = procedural3d_labels(config);
        let active_utility = config.active_utility_id.as_str();
        match body_key {
            PROCEDURAL_3D_PLAY_BODY_MAIN => {
                procedural_3d_engine::sync_flow_extension_contributions(&cfg.projection.contributions_json);
                let host = host_from_fixture(fixture);
                let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
                let viewport = NodeGraphViewport { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom };
                let selection = config.selected_node_ids.clone();
                let live_driver = live_eval_driver(fixture, config);
                let flow_extras = flow_backed_node_graph_extras(fixture, &config.lod_mode, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, Some(&live_driver));
                build_node_graph_scene(
                    PROCEDURAL_3D_PLAY_SURFACE_MAIN,
                    PROCEDURAL_3D_PLAY_APP_ID,
                    NodeGraphScene {
                        editable: Some(true),
                        operators: flow_extras.operators,
                        catalogue_json: flow_extras.catalogue_json,
                        capabilities_json: flow_extras.capabilities_json,
                        lod_json: flow_extras.lod_json,
                        fixture_json: flow_extras.fixture_json,
                        eval_json: flow_extras.eval_json,
                        computing_json: flow_extras.computing_json,
                        selection,
                        hover: config.hovered_node_id.as_ref().map(|id| NodeGraphHover { node_id: Some(id.clone()) }),
                        ..NodeGraphScene::base(nodes, edges, viewport)
                    },
                )
            }
            PROCEDURAL_3D_PLAY_BODY_PREVIEW => {
                let live_driver = live_eval_driver(fixture, config);
                let eval_json = live_driver.eval_json().to_string();
                let (meshes_json, instances_json) = procedural_3d_engine::preview_payload_from_eval(&eval_json, fixture, config);
                let preview_status = procedural_3d_engine::preview_status_json(&eval_json, fixture);
                let sun = config.sun();
                build_world_3d_scene(
                    PROCEDURAL_3D_PLAY_SURFACE_PREVIEW,
                    PROCEDURAL_3D_PLAY_APP_ID,
                    ui_wgpu::World3dScene {
                        status_json: procedural_3d_engine::preview_scene_status_json(&live_driver, preview_status),
                        ..world3d_scene(procedural_3d_engine::preview_camera_json(config), meshes_json, instances_json, procedural_3d_engine::preview_selection_json(config, active_utility), &sun)
                    },
                )
            }
            PROCEDURAL_3D_PLAY_BODY_GENERATIONS => render_generate_generations(&generation_view(doc.projection, config), semio_framework_plugin::locale_from_str(&config.locale), Terminology::default()),
            PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM => render_generate_form(fixture, &generation_view(doc.projection, config), labels),
            PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(fixture, &generation_view(doc.projection, config), config, labels, active_utility),
            PROCEDURAL_3D_PLAY_BODY_DOCUMENT => build_document_tree(fixture, &config.selected_node_ids, labels),
            PROCEDURAL_3D_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            PROCEDURAL_3D_PLAY_BODY_INSPECTION => build_inspector_tree(fixture, &config.selected_node_ids, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_measures(&self, _doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>) -> std::collections::HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let sun = config.sun();
        let mut measures = vec![procedural3d_show_mode_measure(&config.show_mode)];
        measures.push(world3d_sun_measures("procedural3d", &sun, procedural_action));
        std::collections::HashMap::from([
            (PROCEDURAL_3D_PLAY_WINDOW_MAIN.to_string(), vec![procedural3d_lod_measure(&config.lod_mode)]),
            (PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.to_string(), measures.clone()),
            (PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), measures),
        ])
    }

    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &DocumentView<'_, Procedural3dDocument>,
        cfg: &ConfigView<'_, Procedural3dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};
        let config = cfg.projection;
        let labels = procedural3d_labels(config);
        let is_de = config.locale.starts_with("de");
        let selected = config.selected_node_ids.clone();
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]);
        let has_selection = !nodes.is_empty() || !edges.is_empty();
        // 🗂️ Grouped disclosure: `reorganize`/`translateSelection`/`rotateSelection`/`scaleSelection`
        // stay top-level (the 3-5 most frequent 3D-editing verbs); creation, removal and generation
        // methods fold into taxonomy groups; `delete-selection` stays a direct destructive item last —
        // `organize_context_menu` (applied automatically at the `VcsDocumentApp::context_menu` funnel)
        // sorts the groups into `RIBBON_PARENT_CATEGORIES` order and inserts the pre-destructive
        // separator itself.
        let mut menu = Menu::of(registry).action("reorganize");
        menu = menu.when(has_selection, |m| m.action("translateSelection").action("rotateSelection").action("scaleSelection"));
        menu = menu.group("create", |m| m.action("addWidget").action("addGeneration"));
        menu = menu.when(has_selection, |m| m.group("targets", |m2| m2.action("removeWidget").action("removeGeneration")));
        menu = menu.group("methods", |m| m.action("renameGeneration").action("updateGenerationValues").action("patchFlowWidgets"));
        if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
            menu = menu.item(spec);
        }
        menu.build()
    }
}
//#endregion 🔖️Procedural3dPlayApp

//#region 🔖️Manifest
pub fn create_procedural3d_app() -> App {
    App::from_builder(
        App::builder(PROCEDURAL_3D_PLAY_APP_ID, LocalizedLabel::native("Procedural 3D", "Procedural 3D")).document(["semio", "procedural", "3d"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.procedural".into(),
                name: "3D Procedural".into(),
                source_format: "procedural.3d".into(),
                component_kind: "procedural3d".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
                schema: "procedural.3d".into(),
                export_formats: vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl],
                import_formats: vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl],
            })
            .icon_id("workflow")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .mode("generate", LocalizedLabel::native("Generate", "Generieren"), "sparkles")
            .default_mode_id("edit")
            .mode_layout("generate", "procedural3d-generate")
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_MAIN,
                LocalizedLabel::native("Flow", "Workflow"),
                PROCEDURAL_3D_PLAY_BODY_MAIN,
                SurfaceKind::NodeGraph,
                "flow-graph",
            )
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_PREVIEW,
                LocalizedLabel::native("Preview", "Vorschau"),
                PROCEDURAL_3D_PLAY_BODY_PREVIEW,
                SurfaceKind::World3d,
                "preview",
            )
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS,
                LocalizedLabel::native("Generations", "Generationen"),
                PROCEDURAL_3D_PLAY_BODY_GENERATIONS,
                SurfaceKind::Canvas2d,
                "sparkles",
            )
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM,
                LocalizedLabel::native("Form", "Formular"),
                PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM,
                SurfaceKind::Canvas2d,
                "clipboard-list",
            )
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW,
                LocalizedLabel::native("Preview", "Vorschau"),
                PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW,
                SurfaceKind::World3d,
                "preview",
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
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
                PanelGroup::Workbench,
                PROCEDURAL_3D_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                PROCEDURAL_3D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                PROCEDURAL_3D_PLAY_BODY_INSPECTION,
            )
            // ✏️ Document-mutating operations — dispatched as VCS operations with a true inverse.
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .operation("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"))
            .operation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            // 🗂️ Referenced by Procedural3dPlayApp::context_menu — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::new_catalog("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"), ActionKind::Operation).with_category("targets"))
            .operation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(ActionDefinition::new_catalog("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Operation).with_category("create"))
            .action_with(ActionDefinition::new_catalog("patchFlowWidgets", LocalizedLabel::native("Patch Flow Widgets", "Flow-Elemente aktualisieren"), ActionKind::Operation).with_category("methods"))
            .action_with(ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Operation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Operation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Operation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Operation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Operation).with_category("create"))
            .action_with(ActionDefinition::new_catalog("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"), ActionKind::Operation).with_category("targets"))
            .action_with(ActionDefinition::new_catalog("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"), ActionKind::Operation).with_category("methods"))
            .action_with(ActionDefinition::new_catalog("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"), ActionKind::Operation).with_category("methods"))
            // 👁️ Ephemeral view actions — selection, hover, world picking, graph camera, sun/LOD/show-mode display toggles, preview camera (emit no operations).
            .view_action("nodeGraphViewport", LocalizedLabel::native("Set Viewport", "Ansicht festlegen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("selectNode", LocalizedLabel::native("Select Node", "Knoten auswählen"))
            .view_action("nodeGraphSelect", LocalizedLabel::native("Node Graph Select", "Graph-Auswahl"))
            .view_action("nodeGraphHover", LocalizedLabel::native("Node Graph Hover", "Graph-Hover"))
            .view_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"))
            .view_action("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"))
            .view_action("graphPointerDown", LocalizedLabel::native("Graph Pointer Down", "Graph-Zeiger gedrückt"))
            .view_action("worldSelect", LocalizedLabel::native("World Select", "Welt auswählen"))
            .view_action("worldHover", LocalizedLabel::native("World Hover", "Überfahren (Welt)"))
            .view_action("setSelectionMethod", LocalizedLabel::native("Set Selection Method", "Auswahlmethode festlegen"))
            .view_action("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"))
            .view_action("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"))
            .view_action("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"))
            .view_action("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"))
            .view_action("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"))
            .view_action("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("selectGeneration", LocalizedLabel::native("Set Generation", "Generation auswählen"))
            // 📝️ Staged argument forms for the palette-visible actions (defaults materialized host-side).
            .action_args("addWidget", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("neuron", LocalizedLabel::native("Neuron", "Neuron")),
                    ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
                    ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
                    ActionArgOption::new("outputPreview", LocalizedLabel::native("Preview", "Vorschau")),
                ]).default_value("inputSlider"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(PROCEDURAL_EXAMPLE_HEX_COLUMN, LocalizedLabel::native("Hexagonal Mushroom Column", "Sechseckige Pilzsäule")),
                    ActionArgOption::new(PROCEDURAL_EXAMPLE_RECT_EXTRUDE, LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen")),
                    ActionArgOption::new(PROCEDURAL_EXAMPLE_SPHERE_TORUS, LocalizedLabel::native("Sphere Cut With Torus", "Kugel mit Torus geschnitten")),
                    ActionArgOption::new(PROCEDURAL_EXAMPLE_BOX_FILLET, LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau")),
                    ActionArgOption::new(PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen")),
                    ActionArgOption::new(PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, LocalizedLabel::native("Face Sweep Extrude", "Fläche extrudieren")),
                    ActionArgOption::new(PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau")),
                    ActionArgOption::new(PROCEDURAL_EXAMPLE_BOX_SHELL, LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau")),
                ]).required(),
            ])
            // 🧰️ Transform gumball — an exclusive utility group scoped to the 3D preview window (active utility is host-owned).
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move", "Verschieben"), "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") })
            .window_kind_utilities(PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1 /
            // WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2) — `config_spec()`/
            // `procedural3d_io()` are this same information's single source of truth, reused here rather
            // than duplicated.
            .config(Procedural3dPlayApp::default().config_spec())
            .io(procedural_3d_engine::procedural3d_io()),
    )
    .example(PROCEDURAL_EXAMPLE_HEX_COLUMN, LocalizedLabel::native("Hexagonal Mushroom Column", "Sechseckige Pilzsäule"), procedural_3d_engine::example_document_json(PROCEDURAL_EXAMPLE_HEX_COLUMN), "hexagon")
    .example(PROCEDURAL_EXAMPLE_RECT_EXTRUDE, LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen"), procedural_3d_engine::example_document_json(PROCEDURAL_EXAMPLE_RECT_EXTRUDE), "box")
    .example(PROCEDURAL_EXAMPLE_SPHERE_TORUS, LocalizedLabel::native("Sphere Cut With Torus", "Kugel mit Torus geschnitten"), procedural_3d_engine::example_document_json(PROCEDURAL_EXAMPLE_SPHERE_TORUS), "circle")
    .example(PROCEDURAL_EXAMPLE_BOX_FILLET, LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau"), procedural_3d_engine::example_document_json(PROCEDURAL_EXAMPLE_BOX_FILLET), "box")
    .example(PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen"), procedural_3d_engine::example_document_json(PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE), "combine")
    .example(PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, LocalizedLabel::native("Face Sweep Extrude", "Fläche extrudieren"), procedural_3d_engine::example_document_json(PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE), "layers")
    .example(PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau"), procedural_3d_engine::example_document_json(PROCEDURAL_EXAMPLE_RECTANGLE_WIRE), "square")
    .example(PROCEDURAL_EXAMPLE_BOX_SHELL, LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau"), procedural_3d_engine::example_document_json(PROCEDURAL_EXAMPLE_BOX_SHELL), "box")
    .workflow("procedural3d", "Procedural 3D", "brep")
}
//#endregion 🔖️Manifest

//#region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use procedural_3d_engine::empty_procedural3d_projection;
    use procedural_3d_protocol::{Procedural3dEnvelope, Procedural3dStore};
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Procedural3dDocumentVcs {
        store: RefCell<Procedural3dStore>,
    }

    #[wasm_bindgen]
    impl Procedural3dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Procedural3dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Procedural3dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Procedural3dStore::new(envelope)
                }
                None => Procedural3dStore::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_projection(), None)),
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
    use semio_framework_plugin::{PluginApp, VcsDocumentApp, ViewState};
    use std::sync::{Mutex, MutexGuard};

    /// 🧵️ `procedural_3d_engine::procedural_neural_cache()` is a process-wide `OnceLock` shared across
    /// every `FlowHost` this crate's tests build (mirrors `procedural_3d_engine`'s own `TEST_SERIAL` —
    /// see that crate's test module doc comment) — cargo's default multi-threaded test runner races
    /// concurrent evaluations against it otherwise, observed as flaky empty-mesh/empty-eval results.
    static TEST_SERIAL: Mutex<()> = Mutex::new(());

    fn test_serial() -> MutexGuard<'static, ()> {
        TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn meta(actor: &str) -> semio_framework_plugin::ActionMeta {
        testkit::meta(actor)
    }

    fn new_app() -> VcsDocumentApp<Procedural3dPlayApp> {
        testkit::new_app::<Procedural3dPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real action registry so default-materialization + kind discipline run.
    fn new_app_with_registry() -> VcsDocumentApp<Procedural3dPlayApp> {
        testkit::new_app_with_registry::<Procedural3dPlayApp>(create_procedural3d_app)
    }

    /// 🧵️ A `flowEvalTick` chain self-dispatches via `requestedEffects`, which only the JS renderer
    /// drains in production (see `applyHostEffects`'s `dispatchAction` branch) — a unit test has
    /// to do that draining itself. Mirrors `pending_effects`'s own arming logic so tests don't need
    /// to know whether a mutation left the driver already ticking.
    fn drain_flow_eval_ticks(app: &mut VcsDocumentApp<Procedural3dPlayApp>) {
        // 🧵️ Arms the chain if it isn't already (a no-operation if a caller already armed it). A
        // "flowEvalTick" dispatched with nothing pending is a harmless, immediate no-operation, so
        // always ticking at least once is safe.
        app.pending_effects();
        for _ in 0..1000 {
            let result = app.dispatch_typed(Procedural3dCommand::FlowEvalTick, &meta("local")).expect("flowEvalTick");
            if !result.requested_effects.iter().any(|effect| matches!(effect, semio_framework_core::kernel::HostEffect::DispatchAction { action, .. } if action == "flowEvalTick")) {
                return;
            }
        }
        panic!("flowEvalTick chain did not converge within 1000 ticks");
    }

    #[test]
    fn declared_actions_bridge_to_commands() {
        let _serial = test_serial();
        testkit::assert_declared_actions_bridge_to_commands::<Procedural3dPlayApp>(create_procedural3d_app);
    }

    #[test]
    fn set_active_example_via_string_action_loads_fixture() {
        let _serial = test_serial();
        let mut app = new_app_with_registry();
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_FILLET })), &meta("local")).expect("set example");
        let projection = app.projection().expect("projection");
        assert!(projection.fixture.widgets.iter().any(|widget| widget_id(widget).contains("fillet") || matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind.contains("fillet") || neuron_kind.contains("box"))));
    }

    #[test]
    fn unknown_example_id_is_a_no_op() {
        let _serial = test_serial();
        let mut app = new_app();
        let before = app.projection().expect("projection").clone();
        app.dispatch_typed(Procedural3dCommand::SetActiveExample { example_id: "not-a-real-example".into() }, &meta("local")).expect("noop example");
        assert_eq!(app.projection().expect("projection"), before);
    }

    fn preview_mesh_count(app: &mut VcsDocumentApp<Procedural3dPlayApp>) -> usize {
        drain_flow_eval_ticks(app);
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("preview");
        let json = serde_json::to_string(&node).unwrap();
        let parsed: ui_wgpu::UiNode = serde_json::from_str(&json).expect("preview ui json");
        match parsed {
            ui_wgpu::UiNode::ComponentScene(scene) => {
                let world = scene.world_3d.expect("world_3d");
                let meshes: Vec<Value> = serde_json::from_str(&world.meshes_json).unwrap_or_default();
                meshes.len()
            }
            other => panic!("expected component scene, got {other:?}"),
        }
    }

    fn fixture_widget_id_set(projection: &Procedural3dDocument) -> std::collections::BTreeSet<String> {
        projection.fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect()
    }

    #[test]
    fn each_example_loads_distinct_fixture_and_preview_geometry() {
        let _serial = test_serial();
        let examples = [
            PROCEDURAL_EXAMPLE_HEX_COLUMN,
            PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
            PROCEDURAL_EXAMPLE_SPHERE_TORUS,
            PROCEDURAL_EXAMPLE_BOX_FILLET,
            PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
            PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE,
            PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
            PROCEDURAL_EXAMPLE_BOX_SHELL,
        ];
        let fixture_only_preview = [PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_BOX_SHELL];
        let mut signatures = std::collections::BTreeSet::new();
        for example_id in examples {
            let mut app = new_app();
            app.dispatch_typed(Procedural3dCommand::SetActiveExample { example_id: example_id.into() }, &meta("local")).expect("set example");
            let signature = format!("{:?}", fixture_widget_id_set(&app.projection().expect("projection")));
            assert!(signatures.insert(signature.clone()), "duplicate fixture signature for {example_id}: {signature}");
            if !fixture_only_preview.contains(&example_id) {
                let mesh_count = preview_mesh_count(&mut app);
                assert!(mesh_count > 0, "example {example_id} should tessellate at least one preview mesh, got {mesh_count}");
            }
        }
    }

    #[test]
    fn refresh_pending_effects_arms_flow_eval_tick_chain() {
        let _serial = test_serial();
        let mut app = new_app();
        app.dispatch_typed(Procedural3dCommand::SetActiveExample { example_id: PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() }, &meta("local")).expect("set example");
        let effects = app.pending_effects();
        assert!(effects.iter().any(|effect| matches!(effect, semio_framework_core::kernel::HostEffect::DispatchAction { action, .. } if action == "flowEvalTick")));
        app.handle_action("flowEvalTick", None, &meta("local")).expect("tick via string action");
        drain_flow_eval_ticks(&mut app);
        assert!(preview_mesh_count(&mut app) > 0);
    }

    #[test]
    fn set_active_example_arg_form_materializes_into_operations() {
        let _serial = test_serial();
        let mut app = new_app_with_registry();
        app.dispatch_typed(Procedural3dCommand::SetActiveExample { example_id: PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() }, &meta("local")).expect("set example");
        let projection = app.projection().expect("projection");
        assert!(projection.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.prim3d.sphere")));
    }

    #[test]
    fn node_graph_hover_updates_preview_selection_and_graph_scene() {
        let _serial = test_serial();
        let mut app = new_app();
        app.dispatch_typed(Procedural3dCommand::NodeGraphHover { widget_id: Some("extrude".into()) }, &meta("local")).expect("node graph hover");
        let preview = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("preview");
        let preview_json = serde_json::to_string(&preview).expect("preview json");
        // 🩹️ `selectionJson` is a nested JSON-encoded STRING field, so its inner quotes are
        // backslash-escaped in the outer serialized `UiNode` — match the escaped form.
        assert!(preview_json.contains(r#"\"hoveredId\":\"extrude\""#));
        let graph = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("graph");
        let graph_json = serde_json::to_string(&graph).expect("graph json");
        assert!(graph_json.contains(r#""hover":{"nodeId":"extrude"}"#));
    }

    #[test]
    fn set_hover_from_world_updates_preview_and_graph_scene() {
        let _serial = test_serial();
        let mut app = new_app();
        app.dispatch_typed(Procedural3dCommand::SetHover { object_id: Some("extrude".into()) }, &meta("local")).expect("set hover");
        let preview = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("preview");
        let preview_json = serde_json::to_string(&preview).expect("preview json");
        assert!(preview_json.contains("extrude"));
        app.dispatch_typed(Procedural3dCommand::SetHover { object_id: None }, &meta("local")).expect("clear hover");
        let cleared = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("preview cleared");
        let cleared_json = serde_json::to_string(&cleared).expect("cleared json");
        assert!(!cleared_json.contains(r#""hoveredId":"extrude""#));
    }

    #[test]
    fn set_active_utility_switch_clears_scratch_and_emits_no_operations() {
        let _serial = test_serial();
        let mut app = new_app_with_registry();
        app.dispatch_typed(Procedural3dCommand::WorldHover { id: Some("extrude".into()) }, &meta("local")).expect("hover");
        let before = app.projection().expect("projection");
        // Switching the gumball utility is the framework-injected View command: it clears scratch and emits no operations.
        let result = app.dispatch_typed(Procedural3dCommand::SetActiveUtility { utility_id: "rotate".into() }, &meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.projection().expect("projection"), before, "utility switching records no history entry");
    }

    #[test]
    fn gumball_drag_coalesces_multi_tick_translate_into_one_edit() {
        let _serial = test_serial();
        let mut app = new_app();
        let before_widgets = app.projection().expect("projection").fixture.widgets.len();
        // A whole gumball drag (three ticks, same coalesce key) folds into ONE undoable edit, not one-operation-per-tick.
        for dx in [1.0, 1.0, 1.0] {
            app.dispatch_typed(Procedural3dCommand::TranslateSelection { node_ids: vec!["extrude".into()], dx, dy: 0.0, dz: 0.0 }, &meta("local")).expect("drag tick");
        }
        let transform_id = "extrude__gumball_translate";
        let dragged = app.projection().expect("projection");
        assert_eq!(gumball_widget_offset(&host_from_fixture(&dragged.fixture), transform_id), [3.0, 0.0, 0.0], "the three ticks accumulate on one transform node");
        // Undoing the coalesced drag reverts the whole gesture in a single step (splice + all ticks).
        app.handle_action("undo", None, &meta("local")).expect("undo");
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
        let _serial = test_serial();
        let mut app = new_app();
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
    }

    #[test]
    fn main_graph_scene_exports_flow_backed_node_graph_fields() {
        let _serial = test_serial();
        let mut app = new_app();
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        let value: Value = serde_json::from_str(&json).expect("ui node json");
        let graph = value.get("nodeGraph").expect("nodeGraph");
        assert!(graph.get("fixtureJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow.fixture")));
        // 🩹️ `operators` is a nested JSON ARRAY field (`Vec<NodeGraphOperatorRecord>`), not a
        // "operatorsJson" string field — check the catalogue entries' `id`s directly.
        let operators = graph.get("operators").and_then(|value| value.as_array()).expect("operators array");
        assert!(operators.iter().any(|operator| operator.get("id").and_then(|value| value.as_str()).is_some_and(|id| id.contains("math.add") || id.contains("brep."))), "missing math/brep operator catalogue entries");
        let capabilities = graph.get("capabilitiesJson").and_then(|v| v.as_str()).unwrap_or_default();
        assert!(capabilities.contains("flow"), "missing flow engine capability: {capabilities}");
    }

    #[test]
    fn set_lod_mode_is_a_view_action_with_no_document_operations() {
        let _serial = test_serial();
        let mut app = new_app();
        let before = app.projection().expect("projection");
        app.dispatch_typed(Procedural3dCommand::SetLodMode { value: "wireframe".into() }, &meta("local")).expect("lod");
        assert_eq!(app.projection().expect("projection"), before, "setLodMode must not mutate the document");
    }

    #[test]
    fn sun_measures_are_exposed_on_preview_windows() {
        let _serial = test_serial();
        let mut app = new_app();
        let measures = app.window_measures();
        assert!(measures.contains_key(PROCEDURAL_3D_PLAY_WINDOW_PREVIEW));
        assert!(measures.contains_key(PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW));
        // 👁️ Sun toggling is a view command: it must not record a document operation.
        let before = app.projection().expect("projection");
        app.dispatch_typed(Procedural3dCommand::ToggleSun, &meta("local")).expect("toggle sun");
        assert_eq!(app.projection().expect("projection"), before, "toggleSun must not mutate the document");
    }

    #[test]
    fn set_active_example_loads_sphere_fixture() {
        let _serial = test_serial();
        let mut app = new_app();
        app.dispatch_typed(Procedural3dCommand::SetActiveExample { example_id: PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() }, &meta("local")).expect("set example");
        let projection = app.projection().expect("projection");
        assert!(projection.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.prim3d.sphere")));
    }

    #[test]
    fn sphere_cut_example_preview_renders_meshes() {
        let _serial = test_serial();
        // 🧵️ Loading the example never evaluates synchronously anymore (see `pending_effects`) —
        // draining the `flowEvalTick` chain here simulates what the JS renderer's `applyHostEffects`
        // does automatically after every refresh, so the render below sees the real evaluated
        // geometry rather than the cold-start placeholder mesh.
        let mut app = new_app();
        app.dispatch_typed(Procedural3dCommand::SetActiveExample { example_id: PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() }, &meta("local")).expect("set example");
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
        let _serial = test_serial();
        let mut app = new_app();
        app.dispatch_typed(Procedural3dCommand::SetActiveExample { example_id: PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() }, &meta("local")).expect("set example");
        let main_graph = |app: &mut VcsDocumentApp<Procedural3dPlayApp>| -> ui_wgpu::NodeGraphScene {
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
            match serde_json::from_str::<ui_wgpu::UiNode>(&serde_json::to_string(&node).unwrap()).expect("graph ui json") {
                ui_wgpu::UiNode::ComponentScene(scene) => scene.node_graph.expect("node_graph payload"),
                other => panic!("expected component scene, got {other:?}"),
            }
        };
        // 🧵️ In production, `pending_effects` runs after every `refreshUi` pass — a test driving
        // `render` directly has to call it explicitly to arm the driver the same way. Before any
        // tick runs, the graph must flag the cut node (and its downstream preview) as computing —
        // this is what drives the dag canvas's animated loading border.
        assert!(!app.pending_effects().is_empty(), "loading the example must arm a tick chain");
        assert!(main_graph(&mut app).computing_json.is_some(), "pending nodes must be reported before the chain runs");
        drain_flow_eval_ticks(&mut app);
        assert!(preview_mesh_count(&mut app) > 0, "eval chain should produce preview geometry after convergence");
        app.dispatch_typed(Procedural3dCommand::PatchFlowWidgets { widget_ids: vec!["slider_2".into()], field: "value".into(), value: Some(4.5) }, &meta("local")).expect("patch slider");
        assert!(!app.pending_effects().is_empty(), "slider mutation must re-arm evaluation");
        let computing = main_graph(&mut app).computing_json.expect("computing chrome after slider edit");
        assert!(computing.contains("brep_prim3d_sphere_3") || computing.contains("brep_bool_cut_5"), "downstream sphere/cut branch must be marked computing, got {computing}");
    }

    #[test]
    fn patch_flow_widgets_edits_slider_value() {
        let _serial = test_serial();
        let mut app = new_app();
        app.dispatch_typed(Procedural3dCommand::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }, &meta("local")).expect("patch");
        assert_eq!(slider_value(&app.projection().expect("projection"), "height"), Some(9.5));
    }

    #[test]
    fn renders_world_preview_scene() {
        let _serial = test_serial();
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
        let _serial = test_serial();
        let mut app = new_app();
        let before = app.projection().expect("projection").fixture.widgets.len();
        app.dispatch_typed(Procedural3dCommand::AddWidget { kind: "inputNote".into(), x: None, y: None }, &meta("local")).expect("add");
        assert!(app.projection().expect("projection").fixture.widgets.len() > before);
    }

    #[test]
    fn generate_mode_renders_surfaces() {
        let _serial = test_serial();
        let mut app = new_app();
        let generations = app.render(PROCEDURAL_3D_PLAY_BODY_GENERATIONS, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&generations).unwrap().contains("addGeneration"));
    }

    #[test]
    fn add_generation_records_an_undoable_generation_operation() {
        let _serial = test_serial();
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(&mut app, Procedural3dCommand::AddGeneration, |app| app.projection().expect("projection").generation.generations.len(), 0, 1);
    }

    #[test]
    fn translate_selection_persists_transform_into_flow_graph() {
        let _serial = test_serial();
        let mut app = new_app();
        let before = app.projection().expect("projection");
        assert!(before.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"));
        app.dispatch_typed(Procedural3dCommand::TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 }, &meta("local")).expect("translate");
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
        app.dispatch_typed(Procedural3dCommand::TranslateSelection { node_ids: vec![transform_id.into()], dx: 1.0, dy: 0.0, dz: 0.0 }, &meta("local")).expect("translate again");
        let projection2 = app.projection().expect("projection");
        assert_eq!(projection2.fixture.widgets.iter().filter(|widget| widget_id(widget) == transform_id).count(), 1);
        assert_eq!(gumball_widget_offset(&host_from_fixture(&projection2.fixture), transform_id), [2.0, 2.0, 3.0]);
    }

    #[test]
    fn rotate_and_scale_selection_persist_into_flow_graph() {
        let _serial = test_serial();
        let mut app = new_app();
        app.dispatch_typed(Procedural3dCommand::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_2 }, &meta("local")).expect("rotate");
        let rotated = app.projection().expect("projection");
        let rotate_id = "extrude__gumball_rotate";
        assert!(rotated.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == rotate_id && neuron_kind == "brep.xform.rotate")));
        assert_eq!(gumball_widget_number_param(&host_from_fixture(&rotated.fixture), rotate_id, "angle", 0.0), std::f64::consts::FRAC_PI_2);

        let mut scale_app = new_app();
        scale_app.dispatch_typed(Procedural3dCommand::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0 }, &meta("local")).expect("scale");
        let scaled = scale_app.projection().expect("projection");
        let scale_id = "extrude__gumball_scale";
        assert!(scaled.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == scale_id && neuron_kind == "brep.xform.scale")));
        assert_eq!(gumball_widget_number_param(&host_from_fixture(&scaled.fixture), scale_id, "factor", 1.0), 2.0);
    }

    #[test]
    fn undo_redo_round_trips_flow_graph_edits() {
        let _serial = test_serial();
        let mut app = new_app();
        let before = app.projection().expect("projection").fixture.widgets.len();
        testkit::assert_undo_redo_round_trip(&mut app, Procedural3dCommand::AddWidget { kind: "inputNote".into(), x: None, y: None }, |app| app.projection().expect("projection").fixture.widgets.len(), before, before + 1);
    }

    #[test]
    fn remove_widget_action_deletes_by_id_and_supports_undo() {
        let _serial = test_serial();
        let mut app = new_app();
        assert!(app.projection().expect("projection").fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"));
        testkit::assert_undo_redo_round_trip(&mut app, Procedural3dCommand::RemoveWidget { widget_id: "sides".into() }, |app| app.projection().expect("projection").fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"), true, false);
    }

    #[test]
    fn two_instances_converge_disjoint_widget_moves() {
        let _serial = test_serial();
        let widgets: Vec<String> = new_app().projection().expect("projection").fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
        assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
        let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
        testkit::assert_two_instances_converge::<Procedural3dPlayApp, (Option<f64>, Option<f64>)>(
            "mem://procedural3d-convergence",
            Procedural3dCommand::MoveMediaNode { node_id: w0.clone(), x: 111.0, y: 5.0 },
            Procedural3dCommand::MoveMediaNode { node_id: w1.clone(), x: 222.0, y: 6.0 },
            move |app| {
                let layout = &app.projection().expect("projection").fixture.layout;
                (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
            },
        );
    }

    #[test]
    fn procedural3d_labels_resolve_native_english_by_default() {
        let _serial = test_serial();
        let mut app = new_app();
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"Widgets\""));
        assert!(json.contains("\"Slider\""));
        assert!(!json.contains("Elemente"));
    }

    #[test]
    fn procedural3d_labels_translate_catalogue_and_inspector_in_german() {
        let _serial = test_serial();
        let mut app = new_app();
        app.dispatch_typed(Procedural3dCommand::SetLocale { value: "de-DE".into() }, &meta("local")).expect("set locale");
        let catalogue = app.render(PROCEDURAL_3D_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("\"Elemente\""));
        assert!(!catalogue_json.contains("\"Widgets\""));
        let inspector = app.render(PROCEDURAL_3D_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let inspector_json = serde_json::to_string(&inspector).unwrap();
        assert!(inspector_json.contains("Elemente:"));
    }

    #[test]
    fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        let _serial = test_serial();
        let mut app = new_app_with_registry();
        let widgets: Vec<String> = app.projection().expect("projection").fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
        assert!(!widgets.is_empty(), "default fixture needs at least one widget for the test");
        app.dispatch_typed(Procedural3dCommand::SetSelection { node_ids: widgets.clone() }, &meta("local")).expect("set selection");
        let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true);
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).map(|child| child.destructive == Some(true)).unwrap_or(false);
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must be last: {menu:?}");
    }
}
//#endregion 🧪️Tests
